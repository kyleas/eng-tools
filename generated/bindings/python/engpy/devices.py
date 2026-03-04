from ._runtime import invoke

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

