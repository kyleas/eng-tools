from ._runtime import invoke

def fanno_flow_calc(input_kind, input_value, target_kind, gamma, branch=None):
    """Fanno-flow calculator: input kind -> target kind through Mach pivot

Args:
  input_kind: Input kind (mach, t_tstar, p_pstar, rho_rhostar, p0_p0star, four_flstar_d)
  input_value: Input value
  target_kind: Target kind (mach, t_tstar, p_pstar, rho_rhostar, p0_p0star, v_vstar, four_flstar_d)
  gamma: Specific heat ratio
  branch: Subsonic/supersonic branch for inverse paths
Returns:
  f64
"""
    return invoke("device.fanno_flow_calc.value", {"input_kind": input_kind, "input_value": input_value, "target_kind": target_kind, "gamma": gamma, **({"branch": branch} if branch not in (None, "") else {})})

def fanno_flow_from_4flstar_d_to_m(input_value, gamma, branch=None):
    """Convenience Fanno path: 4fL*/D -> Mach (branch required)

Args:
  input_value: Input ratio value
  gamma: Specific heat ratio
  branch: Subsonic or supersonic branch
Returns:
  f64
"""
    return invoke("device.fanno_flow_calc.value", {"input_kind": "four_flstar_d", "target_kind": "mach", "input_value": input_value, "gamma": gamma, **({"branch": branch} if branch not in (None, "") else {})})

def fanno_flow_from_m_to_4flstar_d(input_value, gamma):
    """Convenience Fanno path: Mach -> 4fL*/D

Args:
  input_value: Mach input value
  gamma: Specific heat ratio
Returns:
  f64
"""
    return invoke("device.fanno_flow_calc.value", {"input_kind": "mach", "target_kind": "four_flstar_d", "input_value": input_value, "gamma": gamma})

def fanno_flow_from_m_to_p0_p0star(input_value, gamma):
    """Convenience Fanno path: Mach -> p0/p0*

Args:
  input_value: Mach input value
  gamma: Specific heat ratio
Returns:
  f64
"""
    return invoke("device.fanno_flow_calc.value", {"input_kind": "mach", "target_kind": "p0_p0star", "input_value": input_value, "gamma": gamma})

def fanno_flow_from_m_to_p_pstar(input_value, gamma):
    """Convenience Fanno path: Mach -> p/p*

Args:
  input_value: Mach input value
  gamma: Specific heat ratio
Returns:
  f64
"""
    return invoke("device.fanno_flow_calc.value", {"input_kind": "mach", "target_kind": "p_pstar", "input_value": input_value, "gamma": gamma})

def fanno_flow_from_m_to_rho_rhostar(input_value, gamma):
    """Convenience Fanno path: Mach -> rho/rho*

Args:
  input_value: Mach input value
  gamma: Specific heat ratio
Returns:
  f64
"""
    return invoke("device.fanno_flow_calc.value", {"input_kind": "mach", "target_kind": "rho_rhostar", "input_value": input_value, "gamma": gamma})

def fanno_flow_from_m_to_t_tstar(input_value, gamma):
    """Convenience Fanno path: Mach -> T/T*

Args:
  input_value: Mach input value
  gamma: Specific heat ratio
Returns:
  f64
"""
    return invoke("device.fanno_flow_calc.value", {"input_kind": "mach", "target_kind": "t_tstar", "input_value": input_value, "gamma": gamma})

def fanno_flow_from_m_to_v_vstar(input_value, gamma):
    """Convenience Fanno path: Mach -> V/V*

Args:
  input_value: Mach input value
  gamma: Specific heat ratio
Returns:
  f64
"""
    return invoke("device.fanno_flow_calc.value", {"input_kind": "mach", "target_kind": "v_vstar", "input_value": input_value, "gamma": gamma})

def fanno_flow_from_p0_p0star_to_m(input_value, gamma, branch=None):
    """Convenience Fanno path: p0/p0* -> Mach (branch required)

Args:
  input_value: Input ratio value
  gamma: Specific heat ratio
  branch: Subsonic or supersonic branch
Returns:
  f64
"""
    return invoke("device.fanno_flow_calc.value", {"input_kind": "p0_p0star", "target_kind": "mach", "input_value": input_value, "gamma": gamma, **({"branch": branch} if branch not in (None, "") else {})})

def fanno_flow_path_text(input_kind, input_value, target_kind, gamma, branch=None):
    """Fanno-flow calculator helper: compact step trace text

Args:
  input_kind: Input kind (mach, t_tstar, p_pstar, rho_rhostar, p0_p0star, four_flstar_d)
  input_value: Input value
  target_kind: Target kind (mach, t_tstar, p_pstar, rho_rhostar, p0_p0star, v_vstar, four_flstar_d)
  gamma: Specific heat ratio
  branch: Subsonic/supersonic branch for inverse paths
Returns:
  str
"""
    return invoke("device.fanno_flow_calc.path_text", {"input_kind": input_kind, "input_value": input_value, "target_kind": target_kind, "gamma": gamma, **({"branch": branch} if branch not in (None, "") else {})})

def fanno_flow_pivot_mach(input_kind, input_value, target_kind, gamma, branch=None):
    """Fanno-flow calculator helper: return resolved pivot Mach

Args:
  input_kind: Input kind (mach, t_tstar, p_pstar, rho_rhostar, p0_p0star, four_flstar_d)
  input_value: Input value
  target_kind: Target kind (mach, t_tstar, p_pstar, rho_rhostar, p0_p0star, v_vstar, four_flstar_d)
  gamma: Specific heat ratio
  branch: Subsonic/supersonic branch for inverse paths
Returns:
  f64
"""
    return invoke("device.fanno_flow_calc.pivot_mach", {"input_kind": input_kind, "input_value": input_value, "target_kind": target_kind, "gamma": gamma, **({"branch": branch} if branch not in (None, "") else {})})

def isentropic_calc(input_kind, input_value, target_kind, gamma, branch=None):
    """Isentropic calculator: input kind -> target kind through Mach pivot

Args:
  input_kind: Input kind (mach, mach_angle_deg, prandtl_meyer_angle_deg, pressure_ratio, temperature_ratio, density_ratio, area_ratio)
  input_value: Input value
  target_kind: Target kind (mach, mach_angle_deg, prandtl_meyer_angle_deg, pressure_ratio, temperature_ratio, density_ratio, area_ratio)
  gamma: Specific heat ratio
  branch: Optional branch for double-valued inversions (subsonic/supersonic)
Returns:
  f64
"""
    return invoke("device.isentropic_calc.value", {"input_kind": input_kind, "input_value": input_value, "target_kind": target_kind, "gamma": gamma, **({"branch": branch} if branch not in (None, "") else {})})

def isentropic_from_area_ratio_to_m(input_value, gamma, branch=None):
    """Convenience isentropic path: A/A* -> Mach (branch required)

Args:
  input_value: Input value
  gamma: Specific heat ratio
  branch: Optional branch
Returns:
  f64
"""
    return invoke("device.isentropic_calc.value", {"input_kind": "area_ratio", "target_kind": "mach", "input_value": input_value, "gamma": gamma, **({"branch": branch} if branch not in (None, "") else {})})

def isentropic_from_m_to_nu_deg(input_value, gamma, branch=None):
    """Convenience isentropic path: Mach -> nu(deg)

Args:
  input_value: Input value
  gamma: Specific heat ratio
  branch: Optional branch
Returns:
  f64
"""
    return invoke("device.isentropic_calc.value", {"input_kind": "mach", "target_kind": "prandtl_meyer_angle_deg", "input_value": input_value, "gamma": gamma, **({"branch": branch} if branch not in (None, "") else {})})

def isentropic_from_m_to_p_p0(input_value, gamma, branch=None):
    """Convenience isentropic path: Mach -> p/p0

Args:
  input_value: Input value
  gamma: Specific heat ratio
  branch: Optional branch
Returns:
  f64
"""
    return invoke("device.isentropic_calc.value", {"input_kind": "mach", "target_kind": "pressure_ratio", "input_value": input_value, "gamma": gamma, **({"branch": branch} if branch not in (None, "") else {})})

def isentropic_from_mu_deg_to_p_p0(input_value, gamma, branch=None):
    """Convenience isentropic path: mu(deg) -> p/p0

Args:
  input_value: Input value
  gamma: Specific heat ratio
  branch: Optional branch
Returns:
  f64
"""
    return invoke("device.isentropic_calc.value", {"input_kind": "mach_angle_deg", "target_kind": "pressure_ratio", "input_value": input_value, "gamma": gamma, **({"branch": branch} if branch not in (None, "") else {})})

def isentropic_from_nu_deg_to_m(input_value, gamma, branch=None):
    """Convenience isentropic path: nu(deg) -> Mach

Args:
  input_value: Input value
  gamma: Specific heat ratio
  branch: Optional branch
Returns:
  f64
"""
    return invoke("device.isentropic_calc.value", {"input_kind": "prandtl_meyer_angle_deg", "target_kind": "mach", "input_value": input_value, "gamma": gamma, **({"branch": branch} if branch not in (None, "") else {})})

def isentropic_path_text(input_kind, input_value, target_kind, gamma, branch=None):
    """Isentropic calculator helper: compact step trace text

Args:
  input_kind: Input kind (mach, mach_angle_deg, prandtl_meyer_angle_deg, pressure_ratio, temperature_ratio, density_ratio, area_ratio)
  input_value: Input value
  target_kind: Target kind (mach, mach_angle_deg, prandtl_meyer_angle_deg, pressure_ratio, temperature_ratio, density_ratio, area_ratio)
  gamma: Specific heat ratio
  branch: Optional branch for double-valued inversions (subsonic/supersonic)
Returns:
  str
"""
    return invoke("device.isentropic_calc.path_text", {"input_kind": input_kind, "input_value": input_value, "target_kind": target_kind, "gamma": gamma, **({"branch": branch} if branch not in (None, "") else {})})

def isentropic_pivot_mach(input_kind, input_value, target_kind, gamma, branch=None):
    """Isentropic calculator helper: return resolved pivot Mach

Args:
  input_kind: Input kind (mach, mach_angle_deg, prandtl_meyer_angle_deg, pressure_ratio, temperature_ratio, density_ratio, area_ratio)
  input_value: Input value
  target_kind: Target kind (mach, mach_angle_deg, prandtl_meyer_angle_deg, pressure_ratio, temperature_ratio, density_ratio, area_ratio)
  gamma: Specific heat ratio
  branch: Optional branch for double-valued inversions (subsonic/supersonic)
Returns:
  f64
"""
    return invoke("device.isentropic_calc.pivot_mach", {"input_kind": input_kind, "input_value": input_value, "target_kind": target_kind, "gamma": gamma, **({"branch": branch} if branch not in (None, "") else {})})

def normal_shock_calc(input_kind, input_value, target_kind, gamma):
    """Normal shock calculator: input kind -> target kind through M1 pivot

Args:
  input_kind: Input kind (m1, m2, p2_p1, rho2_rho1, t2_t1, p02_p01)
  input_value: Input value
  target_kind: Target kind (m1, m2, p2_p1, rho2_rho1, t2_t1, p02_p01)
  gamma: Specific heat ratio
Returns:
  f64
"""
    return invoke("device.normal_shock_calc.value", {"input_kind": input_kind, "input_value": input_value, "target_kind": target_kind, "gamma": gamma})

def normal_shock_from_m1_to_m2(input_value, gamma):
    """Convenience normal-shock path: M1 -> M2

Args:
  input_value: Upstream Mach number M1
  gamma: Specific heat ratio
Returns:
  f64
"""
    return invoke("device.normal_shock_calc.value", {"input_kind": "m1", "target_kind": "m2", "input_value": input_value, "gamma": gamma})

def normal_shock_from_m1_to_p02_p01(input_value, gamma):
    """Convenience normal-shock path: M1 -> p02/p01

Args:
  input_value: Upstream Mach number M1
  gamma: Specific heat ratio
Returns:
  f64
"""
    return invoke("device.normal_shock_calc.value", {"input_kind": "m1", "target_kind": "p02_p01", "input_value": input_value, "gamma": gamma})

def normal_shock_from_m1_to_p2_p1(input_value, gamma):
    """Convenience normal-shock path: M1 -> p2/p1

Args:
  input_value: Upstream Mach number M1
  gamma: Specific heat ratio
Returns:
  f64
"""
    return invoke("device.normal_shock_calc.value", {"input_kind": "m1", "target_kind": "p2_p1", "input_value": input_value, "gamma": gamma})

def normal_shock_from_m1_to_rho2_rho1(input_value, gamma):
    """Convenience normal-shock path: M1 -> rho2/rho1

Args:
  input_value: Upstream Mach number M1
  gamma: Specific heat ratio
Returns:
  f64
"""
    return invoke("device.normal_shock_calc.value", {"input_kind": "m1", "target_kind": "rho2_rho1", "input_value": input_value, "gamma": gamma})

def normal_shock_from_m1_to_t2_t1(input_value, gamma):
    """Convenience normal-shock path: M1 -> T2/T1

Args:
  input_value: Upstream Mach number M1
  gamma: Specific heat ratio
Returns:
  f64
"""
    return invoke("device.normal_shock_calc.value", {"input_kind": "m1", "target_kind": "t2_t1", "input_value": input_value, "gamma": gamma})

def normal_shock_path_text(input_kind, input_value, target_kind, gamma):
    """Normal shock calculator helper: compact step trace text

Args:
  input_kind: Input kind (m1, m2, p2_p1, rho2_rho1, t2_t1, p02_p01)
  input_value: Input value
  target_kind: Target kind (m1, m2, p2_p1, rho2_rho1, t2_t1, p02_p01)
  gamma: Specific heat ratio
Returns:
  str
"""
    return invoke("device.normal_shock_calc.path_text", {"input_kind": input_kind, "input_value": input_value, "target_kind": target_kind, "gamma": gamma})

def normal_shock_pivot_m1(input_kind, input_value, target_kind, gamma):
    """Normal shock calculator helper: return resolved pivot M1

Args:
  input_kind: Input kind (m1, m2, p2_p1, rho2_rho1, t2_t1, p02_p01)
  input_value: Input value
  target_kind: Target kind (m1, m2, p2_p1, rho2_rho1, t2_t1, p02_p01)
  gamma: Specific heat ratio
Returns:
  f64
"""
    return invoke("device.normal_shock_calc.pivot_m1", {"input_kind": input_kind, "input_value": input_value, "target_kind": target_kind, "gamma": gamma})

def nozzle_flow_calc(input_kind, input_value, target_kind, gamma, p0, t0, rho0, branch=None):
    """Nozzle-flow calculator: input kind -> target kind through Mach pivot

Args:
  input_kind: Input kind (mach, area_ratio, pressure_ratio, temperature_ratio, density_ratio)
  input_value: Input value
  target_kind: Target kind (mach, area_ratio, pressure_ratio, temperature_ratio, density_ratio, p, t, rho)
  gamma: Specific heat ratio
  p0: Optional stagnation pressure reference for static p output
  t0: Optional stagnation temperature reference for static T output
  rho0: Optional stagnation density reference for static rho output
  branch: Subsonic/supersonic branch for area_ratio -> mach inversion
Returns:
  f64
"""
    return invoke("device.nozzle_flow_calc.value", {"input_kind": input_kind, "input_value": input_value, "target_kind": target_kind, "gamma": gamma, "p0": p0, "t0": t0, "rho0": rho0, **({"branch": branch} if branch not in (None, "") else {})})

def nozzle_flow_from_a_astar_to_m(input_value, gamma, branch=None):
    """Convenience nozzle-flow path: A/A* -> Mach (branch required)

Args:
  input_value: Area ratio input value (A/A*)
  gamma: Specific heat ratio
  branch: Subsonic or supersonic branch
Returns:
  f64
"""
    return invoke("device.nozzle_flow_calc.value", {"input_kind": "area_ratio", "target_kind": "mach", "input_value": input_value, "gamma": gamma, **({"branch": branch} if branch not in (None, "") else {})})

def nozzle_flow_from_m_to_a_astar(input_value, gamma):
    """Convenience nozzle-flow path: Mach -> A/A*

Args:
  input_value: Mach input value
  gamma: Specific heat ratio
Returns:
  f64
"""
    return invoke("device.nozzle_flow_calc.value", {"input_kind": "mach", "target_kind": "area_ratio", "input_value": input_value, "gamma": gamma})

def nozzle_flow_from_m_to_p_p0(input_value, gamma):
    """Convenience nozzle-flow path: Mach -> p/p0

Args:
  input_value: Mach input value
  gamma: Specific heat ratio
Returns:
  f64
"""
    return invoke("device.nozzle_flow_calc.value", {"input_kind": "mach", "target_kind": "pressure_ratio", "input_value": input_value, "gamma": gamma})

def nozzle_flow_from_m_to_rho_rho0(input_value, gamma):
    """Convenience nozzle-flow path: Mach -> rho/rho0

Args:
  input_value: Mach input value
  gamma: Specific heat ratio
Returns:
  f64
"""
    return invoke("device.nozzle_flow_calc.value", {"input_kind": "mach", "target_kind": "density_ratio", "input_value": input_value, "gamma": gamma})

def nozzle_flow_from_m_to_t_t0(input_value, gamma):
    """Convenience nozzle-flow path: Mach -> T/T0

Args:
  input_value: Mach input value
  gamma: Specific heat ratio
Returns:
  f64
"""
    return invoke("device.nozzle_flow_calc.value", {"input_kind": "mach", "target_kind": "temperature_ratio", "input_value": input_value, "gamma": gamma})

def nozzle_flow_path_text(input_kind, input_value, target_kind, gamma, p0, t0, rho0, branch=None):
    """Nozzle-flow calculator helper: compact step trace text

Args:
  input_kind: Input kind (mach, area_ratio, pressure_ratio, temperature_ratio, density_ratio)
  input_value: Input value
  target_kind: Target kind (mach, area_ratio, pressure_ratio, temperature_ratio, density_ratio, p, t, rho)
  gamma: Specific heat ratio
  p0: Optional stagnation pressure reference for static p output
  t0: Optional stagnation temperature reference for static T output
  rho0: Optional stagnation density reference for static rho output
  branch: Subsonic/supersonic branch for area_ratio -> mach inversion
Returns:
  str
"""
    return invoke("device.nozzle_flow_calc.path_text", {"input_kind": input_kind, "input_value": input_value, "target_kind": target_kind, "gamma": gamma, "p0": p0, "t0": t0, "rho0": rho0, **({"branch": branch} if branch not in (None, "") else {})})

def nozzle_flow_pivot_mach(input_kind, input_value, target_kind, gamma, p0, t0, rho0, branch=None):
    """Nozzle-flow calculator helper: return resolved pivot Mach

Args:
  input_kind: Input kind (mach, area_ratio, pressure_ratio, temperature_ratio, density_ratio)
  input_value: Input value
  target_kind: Target kind (mach, area_ratio, pressure_ratio, temperature_ratio, density_ratio, p, t, rho)
  gamma: Specific heat ratio
  p0: Optional stagnation pressure reference for static p output
  t0: Optional stagnation temperature reference for static T output
  rho0: Optional stagnation density reference for static rho output
  branch: Subsonic/supersonic branch for area_ratio -> mach inversion
Returns:
  f64
"""
    return invoke("device.nozzle_flow_calc.pivot_mach", {"input_kind": input_kind, "input_value": input_value, "target_kind": target_kind, "gamma": gamma, "p0": p0, "t0": t0, "rho0": rho0, **({"branch": branch} if branch not in (None, "") else {})})

def oblique_shock_calc(m1, input_kind, input_value, target_kind, gamma, branch=None):
    """Oblique shock calculator: (M1 + beta/theta) -> target with weak/strong branch support

Args:
  m1: Upstream Mach number M1
  input_kind: Input kind (beta_deg or theta_deg)
  input_value: Input value in degrees
  target_kind: Target kind (theta_deg, beta_deg, mn1, mn2, m2, p2_p1, rho2_rho1, t2_t1, p02_p01)
  gamma: Specific heat ratio
  branch: Weak/strong branch required for theta->beta inversion paths
Returns:
  f64
"""
    return invoke("device.oblique_shock_calc.value", {"m1": m1, "input_kind": input_kind, "input_value": input_value, "target_kind": target_kind, "gamma": gamma, **({"branch": branch} if branch not in (None, "") else {})})

def oblique_shock_from_m1_beta_to_m2(m1, input_value, gamma):
    """Convenience oblique-shock path: (M1, beta_deg) -> M2

Args:
  m1: Upstream Mach number M1
  input_value: Shock angle beta in degrees
  gamma: Specific heat ratio
Returns:
  f64
"""
    return invoke("device.oblique_shock_calc.value", {"input_kind": "beta_deg", "target_kind": "m2", "m1": m1, "input_value": input_value, "gamma": gamma})

def oblique_shock_from_m1_beta_to_theta(m1, input_value, gamma):
    """Convenience oblique-shock path: (M1, beta_deg) -> theta_deg

Args:
  m1: Upstream Mach number M1
  input_value: Shock angle beta in degrees
  gamma: Specific heat ratio
Returns:
  f64
"""
    return invoke("device.oblique_shock_calc.value", {"input_kind": "beta_deg", "target_kind": "theta_deg", "m1": m1, "input_value": input_value, "gamma": gamma})

def oblique_shock_from_m1_theta_to_beta(m1, input_value, gamma, branch=None):
    """Convenience oblique-shock path: (M1, theta_deg, branch) -> beta_deg

Args:
  m1: Upstream Mach number M1
  input_value: Flow deflection theta in degrees
  gamma: Specific heat ratio
  branch: Weak or strong branch
Returns:
  f64
"""
    return invoke("device.oblique_shock_calc.value", {"input_kind": "theta_deg", "target_kind": "beta_deg", "m1": m1, "input_value": input_value, "gamma": gamma, **({"branch": branch} if branch not in (None, "") else {})})

def oblique_shock_from_m1_theta_to_p2_p1(m1, input_value, gamma, branch=None):
    """Convenience oblique-shock path: (M1, theta_deg, branch) -> p2/p1

Args:
  m1: Upstream Mach number M1
  input_value: Flow deflection theta in degrees
  gamma: Specific heat ratio
  branch: Weak or strong branch
Returns:
  f64
"""
    return invoke("device.oblique_shock_calc.value", {"input_kind": "theta_deg", "target_kind": "p2_p1", "m1": m1, "input_value": input_value, "gamma": gamma, **({"branch": branch} if branch not in (None, "") else {})})

def oblique_shock_path_text(m1, input_kind, input_value, target_kind, gamma, branch=None):
    """Oblique shock calculator helper: compact step trace text

Args:
  m1: Upstream Mach number M1
  input_kind: Input kind (beta_deg or theta_deg)
  input_value: Input value in degrees
  target_kind: Target kind (theta_deg, beta_deg, mn1, mn2, m2, p2_p1, rho2_rho1, t2_t1, p02_p01)
  gamma: Specific heat ratio
  branch: Weak/strong branch required for theta->beta inversion paths
Returns:
  str
"""
    return invoke("device.oblique_shock_calc.path_text", {"m1": m1, "input_kind": input_kind, "input_value": input_value, "target_kind": target_kind, "gamma": gamma, **({"branch": branch} if branch not in (None, "") else {})})

def pipe_loss_solve_delta_p(friction_model="Colebrook", fixed_f=None, rho=None, mu=None, v=None, d=None, l=None, eps=None, fluid=None, in1_key=None, in1_value=None, in2_key=None, in2_value=None):
    """Solve pipe pressure drop using composed Reynolds/Colebrook/Darcy behavior.

Args:
  friction_model: 'Colebrook' or 'Fixed'
  fixed_f: fixed Darcy friction factor (required when friction_model='Fixed')
  rho, mu, v, d, l, eps: direct inputs
  fluid, in1_key, in1_value, in2_key, in2_value: optional fluid-state context inputs
Returns:
  dict with delta_p, friction_factor, reynolds_number
"""
    args = {
        "friction_model": friction_model,
        "fixed_f": fixed_f,
        "rho": rho,
        "mu": mu,
        "v": v,
        "d": d,
        "l": l,
        "eps": eps,
        "fluid": fluid,
        "in1_key": in1_key,
        "in1_value": in1_value,
        "in2_key": in2_key,
        "in2_value": in2_value,
    }
    return invoke("device.pipe_loss.solve_delta_p", {k: v for k, v in args.items() if v is not None})

def rayleigh_calc(input_kind, input_value, target_kind, gamma, branch=None):
    """Rayleigh-flow calculator: input kind -> target kind through Mach pivot

Args:
  input_kind: Input kind (mach, t_tstar, p_pstar, rho_rhostar, t0_t0star, p0_p0star, v_vstar)
  input_value: Input value
  target_kind: Target kind (mach, t_tstar, p_pstar, rho_rhostar, t0_t0star, p0_p0star, v_vstar)
  gamma: Specific heat ratio
  branch: Subsonic/supersonic branch for branch-sensitive inverse paths
Returns:
  f64
"""
    return invoke("device.rayleigh_calc.value", {"input_kind": input_kind, "input_value": input_value, "target_kind": target_kind, "gamma": gamma, **({"branch": branch} if branch not in (None, "") else {})})

def rayleigh_from_m_to_p0_p0star(input_value, gamma):
    """Convenience Rayleigh path: Mach -> p0/p0*

Args:
  input_value: Mach input value
  gamma: Specific heat ratio
Returns:
  f64
"""
    return invoke("device.rayleigh_calc.value", {"input_kind": "mach", "target_kind": "p0_p0star", "input_value": input_value, "gamma": gamma})

def rayleigh_from_m_to_p_pstar(input_value, gamma):
    """Convenience Rayleigh path: Mach -> p/p*

Args:
  input_value: Mach input value
  gamma: Specific heat ratio
Returns:
  f64
"""
    return invoke("device.rayleigh_calc.value", {"input_kind": "mach", "target_kind": "p_pstar", "input_value": input_value, "gamma": gamma})

def rayleigh_from_m_to_rho_rhostar(input_value, gamma):
    """Convenience Rayleigh path: Mach -> rho/rho*

Args:
  input_value: Mach input value
  gamma: Specific heat ratio
Returns:
  f64
"""
    return invoke("device.rayleigh_calc.value", {"input_kind": "mach", "target_kind": "rho_rhostar", "input_value": input_value, "gamma": gamma})

def rayleigh_from_m_to_t0_t0star(input_value, gamma):
    """Convenience Rayleigh path: Mach -> T0/T0*

Args:
  input_value: Mach input value
  gamma: Specific heat ratio
Returns:
  f64
"""
    return invoke("device.rayleigh_calc.value", {"input_kind": "mach", "target_kind": "t0_t0star", "input_value": input_value, "gamma": gamma})

def rayleigh_from_m_to_t_tstar(input_value, gamma):
    """Convenience Rayleigh path: Mach -> T/T*

Args:
  input_value: Mach input value
  gamma: Specific heat ratio
Returns:
  f64
"""
    return invoke("device.rayleigh_calc.value", {"input_kind": "mach", "target_kind": "t_tstar", "input_value": input_value, "gamma": gamma})

def rayleigh_from_m_to_v_vstar(input_value, gamma):
    """Convenience Rayleigh path: Mach -> V/V*

Args:
  input_value: Mach input value
  gamma: Specific heat ratio
Returns:
  f64
"""
    return invoke("device.rayleigh_calc.value", {"input_kind": "mach", "target_kind": "v_vstar", "input_value": input_value, "gamma": gamma})

def rayleigh_from_p0_p0star_to_m(input_value, gamma, branch=None):
    """Convenience Rayleigh path: p0/p0* -> Mach (branch required)

Args:
  input_value: Input ratio value
  gamma: Specific heat ratio
  branch: Subsonic or supersonic branch
Returns:
  f64
"""
    return invoke("device.rayleigh_calc.value", {"input_kind": "p0_p0star", "target_kind": "mach", "input_value": input_value, "gamma": gamma, **({"branch": branch} if branch not in (None, "") else {})})

def rayleigh_from_t0_t0star_to_m(input_value, gamma, branch=None):
    """Convenience Rayleigh path: T0/T0* -> Mach (branch required)

Args:
  input_value: Input ratio value
  gamma: Specific heat ratio
  branch: Subsonic or supersonic branch
Returns:
  f64
"""
    return invoke("device.rayleigh_calc.value", {"input_kind": "t0_t0star", "target_kind": "mach", "input_value": input_value, "gamma": gamma, **({"branch": branch} if branch not in (None, "") else {})})

def rayleigh_from_t_tstar_to_m(input_value, gamma, branch=None):
    """Convenience Rayleigh path: T/T* -> Mach (branch required)

Args:
  input_value: Input ratio value
  gamma: Specific heat ratio
  branch: Subsonic or supersonic branch
Returns:
  f64
"""
    return invoke("device.rayleigh_calc.value", {"input_kind": "t_tstar", "target_kind": "mach", "input_value": input_value, "gamma": gamma, **({"branch": branch} if branch not in (None, "") else {})})

def rayleigh_path_text(input_kind, input_value, target_kind, gamma, branch=None):
    """Rayleigh-flow calculator helper: compact step trace text

Args:
  input_kind: Input kind (mach, t_tstar, p_pstar, rho_rhostar, t0_t0star, p0_p0star, v_vstar)
  input_value: Input value
  target_kind: Target kind (mach, t_tstar, p_pstar, rho_rhostar, t0_t0star, p0_p0star, v_vstar)
  gamma: Specific heat ratio
  branch: Subsonic/supersonic branch for branch-sensitive inverse paths
Returns:
  str
"""
    return invoke("device.rayleigh_calc.path_text", {"input_kind": input_kind, "input_value": input_value, "target_kind": target_kind, "gamma": gamma, **({"branch": branch} if branch not in (None, "") else {})})

def rayleigh_pivot_mach(input_kind, input_value, target_kind, gamma, branch=None):
    """Rayleigh-flow calculator helper: return resolved pivot Mach

Args:
  input_kind: Input kind (mach, t_tstar, p_pstar, rho_rhostar, t0_t0star, p0_p0star, v_vstar)
  input_value: Input value
  target_kind: Target kind (mach, t_tstar, p_pstar, rho_rhostar, t0_t0star, p0_p0star, v_vstar)
  gamma: Specific heat ratio
  branch: Subsonic/supersonic branch for branch-sensitive inverse paths
Returns:
  f64
"""
    return invoke("device.rayleigh_calc.pivot_mach", {"input_kind": input_kind, "input_value": input_value, "target_kind": target_kind, "gamma": gamma, **({"branch": branch} if branch not in (None, "") else {})})

