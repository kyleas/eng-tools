use std::{collections::HashMap, str::FromStr};

use eng_core::units::typed::QuantityKind;
use eng_core::units::{
    convert_equation_value_from_si, ensure_signature_matches_dimension,
    parse_equation_quantity_to_si,
};
use eng_fluids::{FluidProperty, FluidState};
use eng_materials::MaterialState;

use crate::{
    EquationError, Result, constants,
    model::{EquationDef, ResolverKind},
    solve_engine::SolveMethod,
};

#[derive(Debug, Clone)]
pub enum ContextBinding {
    Fluid(FluidState),
    Material(MaterialState),
}

impl From<FluidState> for ContextBinding {
    fn from(value: FluidState) -> Self {
        Self::Fluid(value)
    }
}

impl From<MaterialState> for ContextBinding {
    fn from(value: MaterialState) -> Self {
        Self::Material(value)
    }
}

#[derive(Debug, Clone)]
pub struct ContextSolveBuilder {
    facade: crate::EqFacade,
    equation_id: String,
    target: Option<String>,
    givens: HashMap<String, crate::SolveInputValue>,
    constant_overrides: HashMap<String, crate::SolveInputValue>,
    method: SolveMethod,
    branch: Option<String>,
    contexts: HashMap<String, ContextBinding>,
}

impl ContextSolveBuilder {
    pub(crate) fn new(facade: crate::EqFacade, equation_id: &str) -> Self {
        Self {
            facade,
            equation_id: equation_id.to_string(),
            target: None,
            givens: HashMap::new(),
            constant_overrides: HashMap::new(),
            method: SolveMethod::Auto,
            branch: None,
            contexts: HashMap::new(),
        }
    }

    pub fn for_target(mut self, target: &str) -> Self {
        self.target = Some(target.to_string());
        self
    }

    pub fn given<K, V>(mut self, name: K, value: V) -> Self
    where
        K: Into<String>,
        V: crate::IntoSolveInput,
    {
        self.givens.insert(name.into(), value.into_solve_input());
        self
    }

    pub fn givens<I, K, V>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = (K, V)>,
        K: Into<String>,
        V: crate::IntoSolveInput,
    {
        for (k, v) in values {
            self.givens.insert(k.into(), v.into_solve_input());
        }
        self
    }

    pub fn override_constant<V>(mut self, name: &str, value: V) -> Self
    where
        V: crate::IntoSolveInput,
    {
        self.constant_overrides
            .insert(name.to_string(), value.into_solve_input());
        self
    }

    pub fn method(mut self, method: SolveMethod) -> Self {
        self.method = method;
        self
    }

    pub fn branch(mut self, branch: &str) -> Self {
        self.branch = Some(branch.to_string());
        self
    }

    pub fn context<C>(mut self, name: &str, context: C) -> Self
    where
        C: Into<ContextBinding>,
    {
        self.contexts.insert(name.to_string(), context.into());
        self
    }

    pub fn fluid(mut self, state: FluidState) -> Self {
        self.contexts
            .insert("fluid".to_string(), ContextBinding::Fluid(state));
        self
    }

    pub fn material(mut self, state: MaterialState) -> Self {
        self.contexts
            .insert("material".to_string(), ContextBinding::Material(state));
        self
    }

    pub fn value(self) -> Result<f64> {
        Ok(self.result()?.value_si)
    }

    pub fn value_in(self, unit: &str) -> Result<f64> {
        let (equation, result) = self.execute()?;
        let target = equation.variables.get(&result.target).ok_or_else(|| {
            EquationError::Validation(format!("target '{}' missing", result.target))
        })?;
        convert_equation_value_from_si(&target.dimension, unit, result.value_si).map_err(|e| {
            EquationError::Unit {
                variable: result.target,
                message: e.to_string(),
            }
        })
    }

    pub fn result(self) -> Result<crate::SolveResult> {
        let (_equation, result) = self.execute()?;
        Ok(result)
    }

    fn execute(self) -> Result<(&'static EquationDef, crate::SolveResult)> {
        let target = self.target.ok_or_else(|| {
            EquationError::Validation("missing solve target: call for_target(...)".to_string())
        })?;
        let equation = self.facade.equation(&self.equation_id)?;
        if !equation.variables.contains_key(&target) {
            return Err(EquationError::InvalidSolveTarget {
                equation_key: equation.key.clone(),
                target,
                valid_targets: equation
                    .variables
                    .keys()
                    .cloned()
                    .collect::<Vec<_>>()
                    .join(", "),
            });
        }
        if let Some(branch) = self.branch.as_deref()
            && !equation.branches.iter().any(|b| b.name == branch)
        {
            let valid: Vec<String> = equation.branches.iter().map(|b| b.name.clone()).collect();
            return Err(EquationError::InvalidBranch {
                equation_key: equation.key.clone(),
                branch: branch.to_string(),
                valid_branches: valid.join(", "),
                suggestion: String::new(),
            });
        }
        let constant_overrides_si = convert_constant_overrides_to_si(&self.constant_overrides)?;
        let mut auto_constants_si = resolve_auto_constants_for_equation(
            equation,
            &target,
            &self.givens,
            &constant_overrides_si,
        );
        apply_constant_overrides_to_symbol_map(&mut auto_constants_si, &constant_overrides_si);

        let mut knowns_si = convert_givens_to_si(equation, &self.givens)?;
        let mut resolved_context = resolve_context_variables(
            equation,
            &target,
            &knowns_si,
            &auto_constants_si,
            &self.contexts,
        )?;
        knowns_si.extend(auto_constants_si);
        knowns_si.extend(resolved_context.drain());

        let missing: Vec<String> = equation
            .variables
            .keys()
            .filter(|k| *k != &target && !knowns_si.contains_key(*k))
            .cloned()
            .collect();
        if !missing.is_empty() {
            return Err(EquationError::Validation(format!(
                "equation '{}' target '{}' missing inputs after context resolution: {}",
                equation.key,
                target,
                missing.join(", ")
            )));
        }

        let solved =
            equation.solve_with_method(&target, knowns_si, self.method, self.branch.as_deref())?;
        let residual_abs = solved.residual.abs();
        let residual_rel = residual_abs / solved.value_si.abs().max(1.0);
        let result = crate::SolveResult {
            equation_id: self.equation_id,
            target: solved.target,
            method: solved.method_used,
            branch: self.branch,
            value_si: solved.value_si,
            residual_abs,
            residual_rel,
            warnings: Vec::new(),
            notes: Vec::new(),
        };
        Ok((equation, result))
    }
}

fn resolve_context_variables(
    equation: &EquationDef,
    target: &str,
    knowns_si: &HashMap<String, f64>,
    auto_constants_si: &HashMap<String, f64>,
    contexts: &HashMap<String, ContextBinding>,
) -> Result<HashMap<String, f64>> {
    let mut out = HashMap::new();
    for (key, var) in &equation.variables {
        if key == target || knowns_si.contains_key(key) || auto_constants_si.contains_key(key) {
            continue;
        }
        let Some(resolver) = &var.resolver else {
            continue;
        };
        let context = contexts.get(&resolver.source).ok_or_else(|| {
            EquationError::Validation(format!(
                "equation '{}' variable '{}' needs context '{}' for resolver {}:{}",
                equation.key, key, resolver.source, resolver.kind, resolver.property
            ))
        })?;
        let value = match (&resolver.kind, context) {
            (ResolverKind::FluidProperty, ContextBinding::Fluid(state)) => {
                let prop = FluidProperty::from_str(&resolver.property).map_err(|e| {
                    EquationError::Validation(format!(
                        "equation '{}' variable '{}' resolver property '{}': {}",
                        equation.key, key, resolver.property, e
                    ))
                })?;
                state.property(prop).map_err(|e| {
                    EquationError::Validation(format!(
                        "equation '{}' variable '{}' fluid property resolution failed: {}",
                        equation.key, key, e
                    ))
                })?
            }
            (ResolverKind::MaterialProperty, ContextBinding::Material(state)) => {
                state.property(&resolver.property).map_err(|e| {
                    EquationError::Validation(format!(
                        "equation '{}' variable '{}' material property resolution failed: {}",
                        equation.key, key, e
                    ))
                })?
            }
            (ResolverKind::FluidProperty, ContextBinding::Material(_)) => {
                return Err(EquationError::Validation(format!(
                    "equation '{}' variable '{}' expects fluid context '{}' but received material context",
                    equation.key, key, resolver.source
                )));
            }
            (ResolverKind::MaterialProperty, ContextBinding::Fluid(_)) => {
                return Err(EquationError::Validation(format!(
                    "equation '{}' variable '{}' expects material context '{}' but received fluid context",
                    equation.key, key, resolver.source
                )));
            }
        };
        out.insert(key.clone(), value);
    }
    Ok(out)
}

fn convert_givens_to_si(
    equation: &EquationDef,
    givens: &HashMap<String, crate::SolveInputValue>,
) -> Result<HashMap<String, f64>> {
    let mut out = HashMap::with_capacity(givens.len());
    let valid_vars: Vec<String> = equation.variables.keys().cloned().collect();
    for (name, value) in givens {
        let var = equation.variables.get(name).ok_or_else(|| {
            EquationError::Validation(format!(
                "equation '{}' unknown given variable '{}'. valid variables: {}",
                equation.key,
                name,
                valid_vars.join(", ")
            ))
        })?;
        let v_si = match value {
            crate::SolveInputValue::Si(v) => *v,
            crate::SolveInputValue::WithUnit(text) => {
                parse_equation_quantity_to_si(&var.dimension, text).map_err(|e| {
                    EquationError::Unit {
                        variable: name.clone(),
                        message: format!(
                            "{} (input='{}'; expected formats like '2.5 MPa' or SI numeric)",
                            e, text
                        ),
                    }
                })?
            }
            crate::SolveInputValue::Typed(q) => {
                ensure_kind_matches_dimension(q.kind, &var.dimension).map_err(|msg| {
                    EquationError::Unit {
                        variable: name.clone(),
                        message: msg,
                    }
                })?;
                q.value_si
            }
            crate::SolveInputValue::Expr(q) => {
                ensure_signature_matches_dimension(q.signature, &var.dimension).map_err(|e| {
                    EquationError::Unit {
                        variable: name.clone(),
                        message: format!(
                            "{} (expression input must reduce to {})",
                            e, var.dimension
                        ),
                    }
                })?;
                q.value_si
            }
        };
        out.insert(name.clone(), v_si);
    }
    Ok(out)
}

fn convert_constant_overrides_to_si(
    overrides: &HashMap<String, crate::SolveInputValue>,
) -> Result<HashMap<String, f64>> {
    let mut out = HashMap::new();
    for (name, value) in overrides {
        let constant = constants::get_by_identifier(name).ok_or_else(|| {
            EquationError::Validation(format!(
                "unknown constant override '{}'. use a constant key/symbol/alias from equations::constants",
                name
            ))
        })?;
        let v_si = match value {
            crate::SolveInputValue::Si(v) => *v,
            crate::SolveInputValue::WithUnit(text) => {
                parse_equation_quantity_to_si(&constant.dimension, text).map_err(|e| {
                    EquationError::Unit {
                        variable: name.clone(),
                        message: e.to_string(),
                    }
                })?
            }
            crate::SolveInputValue::Typed(q) => {
                ensure_kind_matches_dimension(q.kind, &constant.dimension).map_err(|msg| {
                    EquationError::Unit {
                        variable: name.clone(),
                        message: msg,
                    }
                })?;
                q.value_si
            }
            crate::SolveInputValue::Expr(q) => {
                ensure_signature_matches_dimension(q.signature, &constant.dimension).map_err(
                    |e| EquationError::Unit {
                        variable: name.clone(),
                        message: format!(
                            "{} (expression input must reduce to {})",
                            e, constant.dimension
                        ),
                    },
                )?;
                q.value_si
            }
        };
        out.insert(constant.key.to_string(), v_si);
    }
    Ok(out)
}

fn resolve_auto_constants_for_equation(
    equation: &EquationDef,
    target: &str,
    givens: &HashMap<String, crate::SolveInputValue>,
    overrides_by_key: &HashMap<String, f64>,
) -> HashMap<String, f64> {
    let mut out = HashMap::new();
    for name in equation.variables.keys() {
        if name == target || givens.contains_key(name) {
            continue;
        }
        if let Some(constant) = constants::get_by_identifier(name) {
            let value = overrides_by_key
                .get(constant.key)
                .copied()
                .unwrap_or(constant.value);
            out.insert(name.clone(), value);
            out.insert(constant.key.to_string(), value);
            out.insert(constant.symbol_ascii.to_string(), value);
            for alias in constant.aliases {
                out.insert(alias.to_string(), value);
            }
        }
    }
    out
}

fn apply_constant_overrides_to_symbol_map(
    values: &mut HashMap<String, f64>,
    overrides_by_key: &HashMap<String, f64>,
) {
    for (key, value) in overrides_by_key {
        if let Some(constant) = constants::get(key) {
            values.insert(constant.key.to_string(), *value);
            values.insert(constant.symbol_ascii.to_string(), *value);
            for alias in constant.aliases {
                values.insert(alias.to_string(), *value);
            }
        }
    }
}

fn ensure_kind_matches_dimension(
    kind: QuantityKind,
    dimension: &str,
) -> std::result::Result<(), String> {
    let dim = dimension.trim().to_ascii_lowercase().replace(' ', "_");
    let ok = match dim.as_str() {
        "pressure" | "stress" => kind == QuantityKind::Pressure,
        "length" | "diameter" | "distance" | "roughness" => kind == QuantityKind::Length,
        "area" => kind == QuantityKind::Area,
        "density" => kind == QuantityKind::Density,
        "viscosity" | "dynamic_viscosity" => kind == QuantityKind::DynamicViscosity,
        "force" => kind == QuantityKind::Force,
        "moment" => kind == QuantityKind::Moment,
        "temperature" => kind == QuantityKind::Temperature,
        "thermal_conductivity" => kind == QuantityKind::ThermalConductivity,
        "heat_transfer_coefficient" => kind == QuantityKind::HeatTransferCoefficient,
        "mass_flow_rate" => kind == QuantityKind::MassFlowRate,
        "volumetric_flow_rate" => kind == QuantityKind::VolumetricFlowRate,
        "dimensionless" | "ratio" | "friction_factor" | "mach" => {
            kind == QuantityKind::Dimensionless
        }
        _ => true,
    };
    if ok {
        Ok(())
    } else {
        Err(format!(
            "typed quantity kind '{:?}' is not valid for variable dimension '{}'; use a matching typed constructor or SI numeric",
            kind, dimension
        ))
    }
}
