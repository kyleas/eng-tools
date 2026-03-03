use crate::backend::CeaBackend;
use crate::error::CeaError;
use crate::model::{EquilibriumProblem, EquilibriumResult, RocketProblem, RocketResult};
use std::ffi::CString;
use std::sync::Mutex;
use tf_cea_sys::*;

/// Native in-process NASA CEA backend using prebuilt binaries
///
/// This backend calls NASA CEA directly via FFI to the bundled prebuilt libraries
/// in `third_party/cea/`. No external executable or Fortran compiler required.
///
/// # Thread Safety
///
/// NASA CEA has global state and is not fully thread-safe. This implementation
/// uses a Mutex to serialize all CEA calls.
///
/// # Initialization
///
/// CEA initializes on first call, loading thermochemistry data from `thermo.lib`.
pub struct NativeCeaBackend {
    cea_lock: Mutex<CeaState>,
}

struct CeaState {
    initialized: bool,
}

impl NativeCeaBackend {
    /// Create a new native CEA backend (initialization deferred)
    pub fn new() -> Self {
        Self {
            cea_lock: Mutex::new(CeaState { initialized: false }),
        }
    }

    /// Ensure CEA is initialized once (must hold lock when calling)
    fn ensure_initialized_locked(state: &mut CeaState) -> Result<(), CeaError> {
        if state.initialized {
            return Ok(());
        }

        // Get data directory from environment
        // Priority:
        // 1. Runtime TF_CEA_DATA_DIR env var (for manual override)
        // 2. Compile-time CEA_DATA_DIR (set by build.rs from vendored location)
        // 3. Relative path fallback
        let data_dir = std::env::var("TF_CEA_DATA_DIR").unwrap_or_else(|_| {
            option_env!("CEA_DATA_DIR")
                .unwrap_or("third_party/cea/data")
                .to_string()
        });

        // CEA expects the full path to thermo.lib file
        let thermo_path = std::path::Path::new(&data_dir).join("thermo.lib");

        // Verify thermo.lib exists
        if !thermo_path.exists() {
            return Err(CeaError::BackendError(format!(
                "thermo.lib not found at {:?}. Set TF_CEA_DATA_DIR environment variable to the correct path.",
                thermo_path
            )));
        }

        // Convert to absolute path for CEA initialization
        let thermo_abs = thermo_path.canonicalize().map_err(|e| {
            CeaError::BackendError(format!("Failed to resolve thermo.lib path: {}", e))
        })?;

        let thermo_str = thermo_abs
            .to_str()
            .ok_or_else(|| CeaError::BackendError("Invalid thermo.lib path".to_string()))?;

        // Set CEA_DATA_PATH environment variable for NASA CEA library
        // Some versions of CEA look for this environment variable
        // SAFETY: Setting environment variables is generally unsafe in multi-threaded contexts,
        // but we hold a mutex lock that serializes all CEA operations
        unsafe {
            std::env::set_var("CEA_DATA_PATH", thermo_abs.parent().unwrap());
        }

        let thermo_cstr = CString::new(thermo_str)
            .map_err(|e| CeaError::BackendError(format!("Invalid path: {}", e)))?;

        // Call CEA initialization with thermo database
        // Try init with specific file first
        let status = unsafe { cea_init_thermo(thermo_cstr.as_ptr()) };

        if status != CEA_OK {
            // If that fails, try basic init (which may use environment variables)
            let status2 = unsafe { cea_init() };
            if status2 != CEA_OK {
                return Err(CeaError::BackendError(format!(
                    "CEA initialization failed. init_thermo error: {}, init error: {}. Attempted thermo.lib path: {:?}",
                    status, status2, thermo_abs
                )));
            }
        }

        state.initialized = true;
        Ok(())
    }
}

impl Default for NativeCeaBackend {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// RAII Wrappers for automatic cleanup
// ============================================================================

/// RAII wrapper for cea_mixture
struct CeaMixture(cea_mixture);

impl CeaMixture {
    fn from_reactants(reactants: &[CString]) -> Result<Self, CeaError> {
        let mut mixture: cea_mixture = std::ptr::null_mut();
        let reactant_ptrs: Vec<*const i8> = reactants.iter().map(|s| s.as_ptr()).collect();

        let status = unsafe {
            cea_mixture_create_from_reactants(
                &mut mixture,
                reactant_ptrs.len() as cea_int,
                reactant_ptrs.as_ptr(),
                0, // no omitted species
                std::ptr::null(),
            )
        };

        if status != CEA_OK {
            return Err(cea_error("Mixture creation failed", status));
        }
        Ok(CeaMixture(mixture))
    }

    /// Create a simple mixture with only the specified species (not derived products)
    fn from_species(species: &[CString]) -> Result<Self, CeaError> {
        let mut mixture: cea_mixture = std::ptr::null_mut();
        let species_ptrs: Vec<*const i8> = species.iter().map(|s| s.as_ptr()).collect();

        let status = unsafe {
            cea_mixture_create(
                &mut mixture,
                species_ptrs.len() as cea_int,
                species_ptrs.as_ptr(),
            )
        };

        if status != CEA_OK {
            return Err(cea_error("Simple mixture creation failed", status));
        }
        Ok(CeaMixture(mixture))
    }

    fn get(&self) -> cea_mixture {
        self.0
    }
}

impl Drop for CeaMixture {
    fn drop(&mut self) {
        if !self.0.is_null() {
            unsafe {
                let _ = cea_mixture_destroy(&mut self.0);
            }
        }
    }
}

/// RAII wrapper for cea_eqsolver
struct CeaEqSolver(cea_eqsolver);

impl CeaEqSolver {
    fn with_reactants(products: &CeaMixture, reactants: &CeaMixture) -> Result<Self, CeaError> {
        let mut solver: cea_eqsolver = std::ptr::null_mut();
        let status = unsafe {
            cea_eqsolver_create_with_reactants(&mut solver, products.get(), reactants.get())
        };

        if status != CEA_OK {
            return Err(cea_error("Equilibrium solver creation failed", status));
        }
        Ok(CeaEqSolver(solver))
    }

    fn get(&self) -> cea_eqsolver {
        self.0
    }

    #[allow(dead_code)]
    fn get_num_products(&self) -> Result<usize, CeaError> {
        let mut num: cea_int = 0;
        let status =
            unsafe { cea_eqsolver_get_size(self.0, cea_equilibrium_size::NumProducts, &mut num) };
        if status != CEA_OK {
            return Err(cea_error("Failed to get number of products", status));
        }
        Ok(num as usize)
    }
}

impl Drop for CeaEqSolver {
    fn drop(&mut self) {
        if !self.0.is_null() {
            unsafe {
                let _ = cea_eqsolver_destroy(&mut self.0);
            }
        }
    }
}

/// RAII wrapper for cea_eqsolution
struct CeaEqSolution(cea_eqsolution);

impl CeaEqSolution {
    fn new(solver: &CeaEqSolver) -> Result<Self, CeaError> {
        let mut solution: cea_eqsolution = std::ptr::null_mut();
        let status = unsafe { cea_eqsolution_create(&mut solution, solver.get()) };

        if status != CEA_OK {
            return Err(cea_error("Equilibrium solution creation failed", status));
        }
        Ok(CeaEqSolution(solution))
    }

    fn get(&self) -> cea_eqsolution {
        self.0
    }

    fn get_property(&self, prop: cea_property_type) -> Result<f64, CeaError> {
        let mut value: cea_real = 0.0;
        let status = unsafe { cea_eqsolution_get_property(self.0, prop, &mut value) };
        if status != CEA_OK {
            return Err(cea_error("Failed to get property", status));
        }
        Ok(value)
    }

    fn is_converged(&self) -> Result<bool, CeaError> {
        let mut converged: cea_int = 0;
        let status = unsafe { cea_eqsolution_get_converged(self.0, &mut converged) };
        if status != CEA_OK {
            return Err(cea_error("Failed to check convergence", status));
        }
        Ok(converged != 0)
    }

    #[allow(dead_code)]
    fn get_species_moles(&self, num_species: usize) -> Result<Vec<f64>, CeaError> {
        let mut amounts = vec![0.0; num_species];
        let status = unsafe {
            cea_eqsolution_get_species_amounts(
                self.0,
                num_species as cea_int,
                amounts.as_mut_ptr(),
                false, // get moles, not mass
            )
        };
        if status != CEA_OK {
            return Err(cea_error("Failed to get species amounts", status));
        }
        Ok(amounts)
    }
}

impl Drop for CeaEqSolution {
    fn drop(&mut self) {
        if !self.0.is_null() {
            unsafe {
                let _ = cea_eqsolution_destroy(&mut self.0);
            }
        }
    }
}

/// RAII wrapper for cea_rocket_solver
struct CeaRocketSolver(cea_rocket_solver);

impl CeaRocketSolver {
    fn with_reactants(products: &CeaMixture, reactants: &CeaMixture) -> Result<Self, CeaError> {
        let mut solver: cea_rocket_solver = std::ptr::null_mut();
        let status = unsafe {
            cea_rocket_solver_create_with_reactants(&mut solver, products.get(), reactants.get())
        };

        if status != CEA_OK {
            return Err(cea_error("Rocket solver creation failed", status));
        }
        Ok(CeaRocketSolver(solver))
    }

    fn get(&self) -> cea_rocket_solver {
        self.0
    }
}

impl Drop for CeaRocketSolver {
    fn drop(&mut self) {
        if !self.0.is_null() {
            unsafe {
                let _ = cea_rocket_solver_destroy(&mut self.0);
            }
        }
    }
}

/// RAII wrapper for cea_rocket_solution
struct CeaRocketSolution(cea_rocket_solution);

impl CeaRocketSolution {
    fn new(solver: &CeaRocketSolver) -> Result<Self, CeaError> {
        let mut solution: cea_rocket_solution = std::ptr::null_mut();
        let status = unsafe { cea_rocket_solution_create(&mut solution, solver.get()) };

        if status != CEA_OK {
            return Err(cea_error("Rocket solution creation failed", status));
        }
        Ok(CeaRocketSolution(solution))
    }

    fn get(&self) -> cea_rocket_solution {
        self.0
    }

    fn get_num_points(&self) -> Result<usize, CeaError> {
        let mut num_pts: cea_int = 0;
        let status = unsafe { cea_rocket_solution_get_size(self.0, &mut num_pts) };
        if status != CEA_OK {
            return Err(cea_error("Failed to get number of points", status));
        }
        Ok(num_pts as usize)
    }

    fn get_property(
        &self,
        prop: cea_rocket_property_type,
        num_pts: usize,
    ) -> Result<Vec<f64>, CeaError> {
        let mut values = vec![0.0; num_pts];
        let status = unsafe {
            cea_rocket_solution_get_property(self.0, prop, num_pts as cea_int, values.as_mut_ptr())
        };
        if status != CEA_OK {
            return Err(cea_error("Failed to get rocket property", status));
        }
        Ok(values)
    }

    fn is_converged(&self) -> Result<bool, CeaError> {
        let mut converged: cea_int = 0;
        let status = unsafe { cea_rocket_solution_get_converged(self.0, &mut converged) };
        if status != CEA_OK {
            return Err(cea_error("Failed to check convergence", status));
        }
        Ok(converged != 0)
    }
}

impl Drop for CeaRocketSolution {
    fn drop(&mut self) {
        if !self.0.is_null() {
            unsafe {
                let _ = cea_rocket_solution_destroy(&mut self.0);
            }
        }
    }
}

// Helper to create error with CEA error code
fn cea_error(context: &str, status: cea_err) -> CeaError {
    // Note: cea_strerror may not be available in all CEA builds
    // For now, just use the error code
    let err_msg = format!("{}: CEA error code {}", context, status);
    CeaError::BackendError(err_msg)
}

// ============================================================================
// Implementation of CeaBackend trait
// ============================================================================

impl CeaBackend for NativeCeaBackend {
    fn run_equilibrium(&self, problem: &EquilibriumProblem) -> Result<EquilibriumResult, CeaError> {
        let mut state = self.cea_lock.lock().unwrap();
        NativeCeaBackend::ensure_initialized_locked(&mut state)?;

        // Prepare reactant strings for CEA (format: "NAME moles [temperature]")
        let mut reactant_strings = Vec::new();
        for reactant in &problem.reactants {
            // NASA CEA reactant format is just the species name
            // The amount and temperature are specified elsewhere in the API
            let reactant_str = reactant.name.clone();
            reactant_strings.push(
                CString::new(reactant_str).map_err(|e| {
                    CeaError::BackendError(format!("Invalid reactant string: {}", e))
                })?,
            );
        }

        // Create mixtures
        let reactants_mix = CeaMixture::from_reactants(&reactant_strings)?;
        let products_mix = CeaMixture::from_reactants(&reactant_strings)?; // Same for products

        // Create equilibrium solver
        let solver = CeaEqSolver::with_reactants(&products_mix, &reactants_mix)?;
        let solution = CeaEqSolution::new(&solver)?;

        // Determine equilibrium type and state variables from problem
        let (eq_type, state1, state2) = match &problem.state {
            crate::model::ThermoState::PressureTemperature {
                pressure_pa,
                temperature_k,
            } => (cea_equilibrium_type::TP, *temperature_k, *pressure_pa),
            crate::model::ThermoState::PressureEnthalpy {
                pressure_pa,
                enthalpy_j_per_kg,
            } => (cea_equilibrium_type::HP, *enthalpy_j_per_kg, *pressure_pa),
        };

        // Prepare reactant amounts (all go into amounts array)
        let amounts: Vec<f64> = problem.reactants.iter().map(|r| r.amount_moles).collect();

        // Solve equilibrium
        let status = unsafe {
            cea_eqsolver_solve(
                solver.get(),
                eq_type,
                state1,
                state2,
                amounts.as_ptr(),
                solution.get(),
            )
        };

        if status != CEA_OK {
            return Err(cea_error("Equilibrium solve failed", status));
        }

        // Check convergence
        if !solution.is_converged()? {
            return Err(CeaError::BackendError(
                "CEA equilibrium did not converge".to_string(),
            ));
        }

        // Extract properties
        let temperature_k = solution.get_property(cea_property_type::Temperature)?;
        let pressure_pa = solution.get_property(cea_property_type::Pressure)?;
        let mw = solution.get_property(cea_property_type::Mw)?;
        let gamma = solution.get_property(cea_property_type::GammaS)?;

        // Get species composition (placeholder for now - requires species name extraction)
        // TODO: Implement species name and mole fraction extraction
        let species_mole_fractions = Vec::new();

        Ok(EquilibriumResult {
            temperature_k,
            pressure_pa,
            mean_molecular_weight_kg_per_kmol: mw,
            gamma,
            species_mole_fractions,
        })
    }

    fn run_rocket(&self, problem: &RocketProblem) -> Result<RocketResult, CeaError> {
        let mut state = self.cea_lock.lock().unwrap();
        NativeCeaBackend::ensure_initialized_locked(&mut state)?;

        // Prepare reactant strings (oxi and fuel)
        // NASA CEA expects just the species names, not amounts
        let ox_str = problem.oxidizer.name.clone();
        let fuel_str = problem.fuel.name.clone();
        let reactant_strings = vec![
            CString::new(ox_str)
                .map_err(|e| CeaError::BackendError(format!("Invalid oxidizer string: {}", e)))?,
            CString::new(fuel_str)
                .map_err(|e| CeaError::BackendError(format!("Invalid fuel string: {}", e)))?,
        ];

        // Create mixtures
        // Reactants: simple mixture with just the input species (not expanded)
        let reactants_mix = CeaMixture::from_species(&reactant_strings)?;
        // Products: expanded from reactants to include all possible combustion products
        let products_mix = CeaMixture::from_reactants(&reactant_strings)?;

        // Create rocket solver
        let solver = CeaRocketSolver::with_reactants(&products_mix, &reactants_mix)?;
        let solution = CeaRocketSolution::new(&solver)?;

        // Prepare reactant weights using requested O/F mass ratio.
        // Native CEA mixture APIs consume relative mass weights, so map O/F as:
        //   w_fuel = fuel_amount (reference)
        //   w_oxidizer = O/F * w_fuel
        let fuel_weight = problem.fuel.amount_moles.max(1.0e-9);
        let ox_weight = (problem.mixture_ratio_oxidizer_to_fuel * fuel_weight).max(1.0e-9);
        let weights = [ox_weight, fuel_weight];
        let pc = problem.chamber_pressure_pa;

        // Calculate chamber enthalpy/temperature specification
        // For IAC mode: either specify hc (enthalpy) or tc (temperature)
        // - use_hc=true: hc_or_tc is chamber enthalpy (h/R in K), tc_est unused
        // - use_hc=false: hc_or_tc is chamber temperature (K), tc_est is convergence hint
        let (hc_or_tc, use_hc, tc_est, use_tc_est) =
            if problem.oxidizer.temperature_k.is_some() || problem.fuel.temperature_k.is_some() {
                let ox_temp = problem.oxidizer.temperature_k.unwrap_or(298.15);
                let fuel_temp = problem.fuel.temperature_k.unwrap_or(298.15);

                // Calculate adiabatic flame temperature by solving HP equilibrium
                // 1. Calculate enthalpy of reactants at their input temperatures
                // 2. Solve HP equilibrium at chamber pressure with that enthalpy
                // 3. The resulting temperature is the adiabatic flame temperature
                let temps = [ox_temp, fuel_temp];

                let mut enthalpy_j_per_kg: f64 = 0.0;
                let status = unsafe {
                    cea_mixture_calc_property_multitemp(
                        reactants_mix.get(), // Use the simple reactants mixture
                        cea_property_type::Enthalpy,
                        weights.len() as cea_int,
                        weights.as_ptr(),
                        temps.len() as cea_int,
                        temps.as_ptr(),
                        &mut enthalpy_j_per_kg,
                    )
                };

                if status == CEA_OK {
                    const R_UNIVERSAL: f64 = 8314.46261815324;
                    let hc_value = enthalpy_j_per_kg / R_UNIVERSAL;
                    // Use enthalpy mode - CEA will calculate adiabatic flame temperature
                    (hc_value, true, 0.0, false)
                } else {
                    // Fallback: if enthalpy calculation fails, use a reasonable estimate
                    // This should rarely happen
                    (3500.0, false, 0.0, false)
                }
            } else {
                // No temperature specified - assume standard conditions (298.15 K)
                // Calculate adiabatic flame temperature from standard enthalpy
                let temps = [298.15, 298.15];

                let mut enthalpy_j_per_kg: f64 = 0.0;
                let status = unsafe {
                    cea_mixture_calc_property_multitemp(
                        reactants_mix.get(),
                        cea_property_type::Enthalpy,
                        weights.len() as cea_int,
                        weights.as_ptr(),
                        temps.len() as cea_int,
                        temps.as_ptr(),
                        &mut enthalpy_j_per_kg,
                    )
                };

                if status == CEA_OK {
                    const R_UNIVERSAL: f64 = 8314.46261815324;
                    let hc_value = enthalpy_j_per_kg / R_UNIVERSAL;
                    (hc_value, true, 0.0, false)
                } else {
                    (3500.0, false, 0.0, false)
                }
            };

        // Calculate area ratios for stations
        let expansion_ratio = problem.expansion_ratio;
        // Use standard subsonic (chamber/throat) area ratio from CEA examples
        let subar = [1.5];
        let supar = [expansion_ratio]; // Supersonic (exit)

        // No pressure ratio points, using area ratios instead
        let pi_p: Vec<f64> = Vec::new();

        // Solve rocket problem
        let status = unsafe {
            cea_rocket_solver_solve_iac(
                solver.get(),
                solution.get(),
                weights.as_ptr(),
                pc,
                pi_p.as_ptr(),
                0, // n_pi_p = 0
                subar.as_ptr(),
                subar.len() as cea_int,
                supar.as_ptr(),
                supar.len() as cea_int,
                0,          // n_frz = 0 (all shifting equilibrium)
                hc_or_tc,   // Chamber enthalpy (K) or temperature (K)
                use_hc,     // use_hc: true for cryogenic, false for standard
                tc_est,     // Chamber temperature estimate (K) when not using hc
                use_tc_est, // use_tc_est: provide estimate for standard combustion
            )
        };

        if status != CEA_OK {
            return Err(cea_error("Rocket solve failed", status));
        }

        // Check convergence
        if !solution.is_converged()? {
            return Err(CeaError::BackendError(
                "CEA rocket solution did not converge".to_string(),
            ));
        }

        // Get number of solution points (should be 2+: chamber, throat, exit)
        let num_pts = solution.get_num_points()?;

        eprintln!(
            "[DEBUG] CEA rocket solve completed: {} solution points",
            num_pts
        );

        // Extract chamber properties (first station, index 0)
        let temperatures = solution.get_property(cea_rocket_property_type::Temperature, num_pts)?;
        let gammas = solution.get_property(cea_rocket_property_type::GammaS, num_pts)?;
        let mws = solution.get_property(cea_rocket_property_type::Mw, num_pts)?;

        eprintln!("[DEBUG] All temperatures: {:?}", temperatures);
        eprintln!("[DEBUG] All gammas: {:?}", gammas);
        eprintln!("[DEBUG] All MWs: {:?}", mws);

        // Extract rocket performance properties
        let cstar_values = solution.get_property(cea_rocket_property_type::Cstar, num_pts)?;
        let isp_vac_values = solution.get_property(cea_rocket_property_type::IspVacuum, num_pts)?;
        let cf_values =
            solution.get_property(cea_rocket_property_type::CoefficientOfThrust, num_pts)?;

        eprintln!("[DEBUG] All c*: {:?}", cstar_values);
        eprintln!("[DEBUG] All Isp_vac: {:?}", isp_vac_values);
        eprintln!("[DEBUG] All Cf_vac: {:?}", cf_values);

        // Chamber values are at station 0
        let chamber_temperature_k = temperatures[0];
        let chamber_gamma = gammas[0];
        let chamber_molecular_weight_kg_per_kmol = mws[0];

        // C* is typically a rocket-level property (same for all stations, take first)
        let characteristic_velocity_m_per_s = cstar_values[0];

        // For vacuum performance, use the exit station (last value)
        let specific_impulse_vac_s = isp_vac_values[num_pts - 1];
        let thrust_coefficient_vac = cf_values[num_pts - 1];

        Ok(RocketResult {
            chamber_temperature_k,
            chamber_gamma,
            chamber_molecular_weight_kg_per_kmol,
            characteristic_velocity_m_per_s,
            specific_impulse_vac_s,
            thrust_coefficient_vac,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_native_backend_creation() {
        let _backend = NativeCeaBackend::new();
        // Just verify backend creates without errors
    }

    #[test]
    fn test_raii_mixture_cleanup() {
        // This test verifies RAII wrapper drops properly
        // CEA must be initialized before creating mixtures
        let backend = NativeCeaBackend::new();
        let mut state = backend.cea_lock.lock().unwrap();

        // Initialize CEA
        if let Err(e) = NativeCeaBackend::ensure_initialized_locked(&mut state) {
            // If initialization fails (e.g., thermo.lib not found), skip test
            println!("Skipping test: CEA initialization failed: {}", e);
            return;
        }

        // Now test mixture creation and cleanup
        // Use just the species name, not amounts (CEA expects just names)
        let reactants = vec![CString::new("H2").unwrap()];
        let result = CeaMixture::from_reactants(&reactants);
        // Mixture should be dropped here automatically
        assert!(
            result.is_ok(),
            "Mixture creation should succeed after CEA initialization"
        );
    }
}
