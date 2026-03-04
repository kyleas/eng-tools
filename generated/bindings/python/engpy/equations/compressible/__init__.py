from . import area_mach
from . import choked_mass_flux
from . import isentropic_density_ratio
from . import isentropic_pressure_ratio
from . import isentropic_temperature_ratio
from . import mach_angle
from . import normal_shock_density_ratio
from . import normal_shock_m2
from . import normal_shock_pressure_ratio
from . import normal_shock_stagnation_pressure_ratio
from . import normal_shock_temperature_ratio
from . import oblique_shock_m2
from . import oblique_shock_mn1
from . import oblique_shock_theta_beta_m
from . import prandtl_meyer
from .area_mach import solve_area_ratio as solve_area_ratio
from .choked_mass_flux import solve_g_star as solve_g_star
from .oblique_shock_mn1 import solve_mn1 as solve_mn1
from .oblique_shock_m2 import solve_mn2 as solve_mn2
from .mach_angle import solve_mu as solve_mu
from .prandtl_meyer import solve_nu as solve_nu
from .normal_shock_stagnation_pressure_ratio import solve_p02_p01 as solve_p02_p01
from .normal_shock_pressure_ratio import solve_p2_p1 as solve_p2_p1
from .isentropic_pressure_ratio import solve_p_p0 as solve_p_p0
from .normal_shock_density_ratio import solve_rho2_rho1 as solve_rho2_rho1
from .isentropic_density_ratio import solve_rho_rho0 as solve_rho_rho0
from .normal_shock_temperature_ratio import solve_t2_t1 as solve_t2_t1
from .isentropic_temperature_ratio import solve_t_t0 as solve_t_t0

# Omitted legacy aliases due to collisions:
# - solve_beta
# - solve_m
# - solve_m1
# - solve_m2
# - solve_theta

__all__ = [
    "area_mach",
    "choked_mass_flux",
    "isentropic_density_ratio",
    "isentropic_pressure_ratio",
    "isentropic_temperature_ratio",
    "mach_angle",
    "normal_shock_density_ratio",
    "normal_shock_m2",
    "normal_shock_pressure_ratio",
    "normal_shock_stagnation_pressure_ratio",
    "normal_shock_temperature_ratio",
    "oblique_shock_m2",
    "oblique_shock_mn1",
    "oblique_shock_theta_beta_m",
    "prandtl_meyer",
]
