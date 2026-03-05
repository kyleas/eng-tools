try:
    import xloil
except Exception:
    class _X:
        @staticmethod
        def func(*args, **kwargs):
            def _d(f):
                return f
            return _d
    xloil = _X()

from engpy._runtime import invoke

@xloil.func(name="ENG_CONST", help="Get constant value from registry | Arguments: | - key: Constant key | Returns: f64 | Example: =ENG_CONST('g0')")
def e_n_g_c_o_n_s_t(key):
    """Get constant value from registry | Arguments: | - key: Constant key | Returns: f64 | Example: =ENG_CONST('g0')"""
    return invoke("constant.get", {"key": key})

@xloil.func(name="ENG_FANNO_FLOW", help="Fanno-flow calculator: input kind -> target kind through Mach pivot | Arguments: | - input_kind: Input kind (mach, t_tstar, p_pstar, rho_rhostar, p0_p0star, four_flstar_d) | - input_value: Input value | - target_kind: Target kind (mach, t_tstar, p_pstar, rho_rhostar, p0_p0star, v_vstar, four_flstar_d) | - gamma: Specific heat ratio | - branch: Subsonic/supersonic branch for inverse paths | Returns: f64 | Example: =ENG_FANNO_FLOW('mach',2.0,'p_pstar',1.4,'')")
def e_n_g_f_a_n_n_o_f_l_o_w(input_kind, input_value, target_kind, gamma, branch=""):
    """Fanno-flow calculator: input kind -> target kind through Mach pivot | Arguments: | - input_kind: Input kind (mach, t_tstar, p_pstar, rho_rhostar, p0_p0star, four_flstar_d) | - input_value: Input value | - target_kind: Target kind (mach, t_tstar, p_pstar, rho_rhostar, p0_p0star, v_vstar, four_flstar_d) | - gamma: Specific heat ratio | - branch: Subsonic/supersonic branch for inverse paths | Returns: f64 | Example: =ENG_FANNO_FLOW('mach',2.0,'p_pstar',1.4,'')"""
    return invoke("device.fanno_flow_calc.value", {"input_kind": input_kind, "input_value": input_value, "target_kind": target_kind, "gamma": gamma, **({"branch": branch} if branch not in (None, "") else {})})

@xloil.func(name="ENG_FANNO_FLOW_FROM_4FLSTAR_D_TO_M", help="Convenience Fanno path: 4fL*/D -> Mach (branch required) | Arguments: | - input_value: Input ratio value | - gamma: Specific heat ratio | - branch: Subsonic or supersonic branch | Returns: f64 | Example: =ENG_FANNO_FLOW_FROM_4FLSTAR_D_TO_M(0.3049965026,1.4,'supersonic')")
def e_n_g_f_a_n_n_o_f_l_o_w_f_r_o_m_4_f_l_s_t_a_r_d_t_o_m(input_value, gamma, branch=""):
    """Convenience Fanno path: 4fL*/D -> Mach (branch required) | Arguments: | - input_value: Input ratio value | - gamma: Specific heat ratio | - branch: Subsonic or supersonic branch | Returns: f64 | Example: =ENG_FANNO_FLOW_FROM_4FLSTAR_D_TO_M(0.3049965026,1.4,'supersonic')"""
    return invoke("device.fanno_flow_calc.value", {"input_kind": "four_flstar_d", "target_kind": "mach", "input_value": input_value, "gamma": gamma, **({"branch": branch} if branch not in (None, "") else {})})

@xloil.func(name="ENG_FANNO_FLOW_FROM_M_TO_4FLSTAR_D", help="Convenience Fanno path: Mach -> 4fL*/D | Arguments: | - input_value: Mach input value | - gamma: Specific heat ratio | Returns: f64 | Example: =ENG_FANNO_FLOW_FROM_M_TO_4FLSTAR_D(2.0,1.4)")
def e_n_g_f_a_n_n_o_f_l_o_w_f_r_o_m_m_t_o_4_f_l_s_t_a_r_d(input_value, gamma):
    """Convenience Fanno path: Mach -> 4fL*/D | Arguments: | - input_value: Mach input value | - gamma: Specific heat ratio | Returns: f64 | Example: =ENG_FANNO_FLOW_FROM_M_TO_4FLSTAR_D(2.0,1.4)"""
    return invoke("device.fanno_flow_calc.value", {"input_kind": "mach", "target_kind": "four_flstar_d", "input_value": input_value, "gamma": gamma})

@xloil.func(name="ENG_FANNO_FLOW_FROM_M_TO_P0_P0STAR", help="Convenience Fanno path: Mach -> p0/p0* | Arguments: | - input_value: Mach input value | - gamma: Specific heat ratio | Returns: f64 | Example: =ENG_FANNO_FLOW_FROM_M_TO_P0_P0STAR(2.0,1.4)")
def e_n_g_f_a_n_n_o_f_l_o_w_f_r_o_m_m_t_o_p0_p0_s_t_a_r(input_value, gamma):
    """Convenience Fanno path: Mach -> p0/p0* | Arguments: | - input_value: Mach input value | - gamma: Specific heat ratio | Returns: f64 | Example: =ENG_FANNO_FLOW_FROM_M_TO_P0_P0STAR(2.0,1.4)"""
    return invoke("device.fanno_flow_calc.value", {"input_kind": "mach", "target_kind": "p0_p0star", "input_value": input_value, "gamma": gamma})

@xloil.func(name="ENG_FANNO_FLOW_FROM_M_TO_P_PSTAR", help="Convenience Fanno path: Mach -> p/p* | Arguments: | - input_value: Mach input value | - gamma: Specific heat ratio | Returns: f64 | Example: =ENG_FANNO_FLOW_FROM_M_TO_P_PSTAR(2.0,1.4)")
def e_n_g_f_a_n_n_o_f_l_o_w_f_r_o_m_m_t_o_p_p_s_t_a_r(input_value, gamma):
    """Convenience Fanno path: Mach -> p/p* | Arguments: | - input_value: Mach input value | - gamma: Specific heat ratio | Returns: f64 | Example: =ENG_FANNO_FLOW_FROM_M_TO_P_PSTAR(2.0,1.4)"""
    return invoke("device.fanno_flow_calc.value", {"input_kind": "mach", "target_kind": "p_pstar", "input_value": input_value, "gamma": gamma})

@xloil.func(name="ENG_FANNO_FLOW_FROM_M_TO_RHO_RHOSTAR", help="Convenience Fanno path: Mach -> rho/rho* | Arguments: | - input_value: Mach input value | - gamma: Specific heat ratio | Returns: f64 | Example: =ENG_FANNO_FLOW_FROM_M_TO_RHO_RHOSTAR(2.0,1.4)")
def e_n_g_f_a_n_n_o_f_l_o_w_f_r_o_m_m_t_o_r_h_o_r_h_o_s_t_a_r(input_value, gamma):
    """Convenience Fanno path: Mach -> rho/rho* | Arguments: | - input_value: Mach input value | - gamma: Specific heat ratio | Returns: f64 | Example: =ENG_FANNO_FLOW_FROM_M_TO_RHO_RHOSTAR(2.0,1.4)"""
    return invoke("device.fanno_flow_calc.value", {"input_kind": "mach", "target_kind": "rho_rhostar", "input_value": input_value, "gamma": gamma})

@xloil.func(name="ENG_FANNO_FLOW_FROM_M_TO_T_TSTAR", help="Convenience Fanno path: Mach -> T/T* | Arguments: | - input_value: Mach input value | - gamma: Specific heat ratio | Returns: f64 | Example: =ENG_FANNO_FLOW_FROM_M_TO_T_TSTAR(2.0,1.4)")
def e_n_g_f_a_n_n_o_f_l_o_w_f_r_o_m_m_t_o_t_t_s_t_a_r(input_value, gamma):
    """Convenience Fanno path: Mach -> T/T* | Arguments: | - input_value: Mach input value | - gamma: Specific heat ratio | Returns: f64 | Example: =ENG_FANNO_FLOW_FROM_M_TO_T_TSTAR(2.0,1.4)"""
    return invoke("device.fanno_flow_calc.value", {"input_kind": "mach", "target_kind": "t_tstar", "input_value": input_value, "gamma": gamma})

@xloil.func(name="ENG_FANNO_FLOW_FROM_M_TO_V_VSTAR", help="Convenience Fanno path: Mach -> V/V* | Arguments: | - input_value: Mach input value | - gamma: Specific heat ratio | Returns: f64 | Example: =ENG_FANNO_FLOW_FROM_M_TO_V_VSTAR(2.0,1.4)")
def e_n_g_f_a_n_n_o_f_l_o_w_f_r_o_m_m_t_o_v_v_s_t_a_r(input_value, gamma):
    """Convenience Fanno path: Mach -> V/V* | Arguments: | - input_value: Mach input value | - gamma: Specific heat ratio | Returns: f64 | Example: =ENG_FANNO_FLOW_FROM_M_TO_V_VSTAR(2.0,1.4)"""
    return invoke("device.fanno_flow_calc.value", {"input_kind": "mach", "target_kind": "v_vstar", "input_value": input_value, "gamma": gamma})

@xloil.func(name="ENG_FANNO_FLOW_FROM_P0_P0STAR_TO_M", help="Convenience Fanno path: p0/p0* -> Mach (branch required) | Arguments: | - input_value: Input ratio value | - gamma: Specific heat ratio | - branch: Subsonic or supersonic branch | Returns: f64 | Example: =ENG_FANNO_FLOW_FROM_P0_P0STAR_TO_M(1.33984375,1.4,'subsonic')")
def e_n_g_f_a_n_n_o_f_l_o_w_f_r_o_m_p0_p0_s_t_a_r_t_o_m(input_value, gamma, branch=""):
    """Convenience Fanno path: p0/p0* -> Mach (branch required) | Arguments: | - input_value: Input ratio value | - gamma: Specific heat ratio | - branch: Subsonic or supersonic branch | Returns: f64 | Example: =ENG_FANNO_FLOW_FROM_P0_P0STAR_TO_M(1.33984375,1.4,'subsonic')"""
    return invoke("device.fanno_flow_calc.value", {"input_kind": "p0_p0star", "target_kind": "mach", "input_value": input_value, "gamma": gamma, **({"branch": branch} if branch not in (None, "") else {})})

@xloil.func(name="ENG_FANNO_FLOW_PATH_TEXT", help="Fanno-flow calculator helper: compact step trace text | Arguments: | - input_kind: Input kind (mach, t_tstar, p_pstar, rho_rhostar, p0_p0star, four_flstar_d) | - input_value: Input value | - target_kind: Target kind (mach, t_tstar, p_pstar, rho_rhostar, p0_p0star, v_vstar, four_flstar_d) | - gamma: Specific heat ratio | - branch: Subsonic/supersonic branch for inverse paths | Returns: str | Example: =ENG_FANNO_FLOW_PATH_TEXT('p0_p0star',1.33984375,'mach',1.4,'subsonic')")
def e_n_g_f_a_n_n_o_f_l_o_w_p_a_t_h_t_e_x_t(input_kind, input_value, target_kind, gamma, branch=""):
    """Fanno-flow calculator helper: compact step trace text | Arguments: | - input_kind: Input kind (mach, t_tstar, p_pstar, rho_rhostar, p0_p0star, four_flstar_d) | - input_value: Input value | - target_kind: Target kind (mach, t_tstar, p_pstar, rho_rhostar, p0_p0star, v_vstar, four_flstar_d) | - gamma: Specific heat ratio | - branch: Subsonic/supersonic branch for inverse paths | Returns: str | Example: =ENG_FANNO_FLOW_PATH_TEXT('p0_p0star',1.33984375,'mach',1.4,'subsonic')"""
    return invoke("device.fanno_flow_calc.path_text", {"input_kind": input_kind, "input_value": input_value, "target_kind": target_kind, "gamma": gamma, **({"branch": branch} if branch not in (None, "") else {})})

@xloil.func(name="ENG_FANNO_FLOW_PIVOT_MACH", help="Fanno-flow calculator helper: return resolved pivot Mach | Arguments: | - input_kind: Input kind (mach, t_tstar, p_pstar, rho_rhostar, p0_p0star, four_flstar_d) | - input_value: Input value | - target_kind: Target kind (mach, t_tstar, p_pstar, rho_rhostar, p0_p0star, v_vstar, four_flstar_d) | - gamma: Specific heat ratio | - branch: Subsonic/supersonic branch for inverse paths | Returns: f64 | Example: =ENG_FANNO_FLOW_PIVOT_MACH('four_flstar_d',0.3049965026,'mach',1.4,'supersonic')")
def e_n_g_f_a_n_n_o_f_l_o_w_p_i_v_o_t_m_a_c_h(input_kind, input_value, target_kind, gamma, branch=""):
    """Fanno-flow calculator helper: return resolved pivot Mach | Arguments: | - input_kind: Input kind (mach, t_tstar, p_pstar, rho_rhostar, p0_p0star, four_flstar_d) | - input_value: Input value | - target_kind: Target kind (mach, t_tstar, p_pstar, rho_rhostar, p0_p0star, v_vstar, four_flstar_d) | - gamma: Specific heat ratio | - branch: Subsonic/supersonic branch for inverse paths | Returns: f64 | Example: =ENG_FANNO_FLOW_PIVOT_MACH('four_flstar_d',0.3049965026,'mach',1.4,'supersonic')"""
    return invoke("device.fanno_flow_calc.pivot_mach", {"input_kind": input_kind, "input_value": input_value, "target_kind": target_kind, "gamma": gamma, **({"branch": branch} if branch not in (None, "") else {})})

@xloil.func(name="ENG_ISENTROPIC", help="Isentropic calculator: input kind -> target kind through Mach pivot | Arguments: | - value_kind_in: Input kind (mach, mach_angle_deg, prandtl_meyer_angle_deg, pressure_ratio, temperature_ratio, density_ratio, area_ratio) | - value_in: Input value | - target_kind_out: Target kind (mach, mach_angle_deg, prandtl_meyer_angle_deg, pressure_ratio, temperature_ratio, density_ratio, area_ratio) | - gamma: Specific heat ratio | - branch: Optional branch for double-valued inversions (subsonic/supersonic) | Returns: f64 | Example: =ENG_ISENTROPIC('mach',2.0,'pressure_ratio',1.4,'')")
def e_n_g_i_s_e_n_t_r_o_p_i_c(value_kind_in, value_in, target_kind_out, gamma, branch=""):
    """Isentropic calculator: input kind -> target kind through Mach pivot | Arguments: | - value_kind_in: Input kind (mach, mach_angle_deg, prandtl_meyer_angle_deg, pressure_ratio, temperature_ratio, density_ratio, area_ratio) | - value_in: Input value | - target_kind_out: Target kind (mach, mach_angle_deg, prandtl_meyer_angle_deg, pressure_ratio, temperature_ratio, density_ratio, area_ratio) | - gamma: Specific heat ratio | - branch: Optional branch for double-valued inversions (subsonic/supersonic) | Returns: f64 | Example: =ENG_ISENTROPIC('mach',2.0,'pressure_ratio',1.4,'')"""
    return invoke("device.isentropic_calc.value", {"input_kind": value_kind_in, "input_value": value_in, "target_kind": target_kind_out, "gamma": gamma, **({"branch": branch} if branch not in (None, "") else {})})

@xloil.func(name="ENG_ISENTROPIC_FROM_A_ASTAR_TO_M", help="Convenience isentropic path: A/A* -> Mach (branch required) | Arguments: | - value_in: Input value | - gamma: Specific heat ratio | - branch: Optional branch | Returns: f64 | Example: =ENG_ISENTROPIC_FROM_A_ASTAR_TO_M(2.0,1.4,'supersonic')")
def e_n_g_i_s_e_n_t_r_o_p_i_c_f_r_o_m_a_a_s_t_a_r_t_o_m(value_in, gamma, branch=""):
    """Convenience isentropic path: A/A* -> Mach (branch required) | Arguments: | - value_in: Input value | - gamma: Specific heat ratio | - branch: Optional branch | Returns: f64 | Example: =ENG_ISENTROPIC_FROM_A_ASTAR_TO_M(2.0,1.4,'supersonic')"""
    return invoke("device.isentropic_calc.value", {"input_kind": "area_ratio", "target_kind": "mach", "input_value": value_in, "gamma": gamma, **({"branch": branch} if branch not in (None, "") else {})})

@xloil.func(name="ENG_ISENTROPIC_FROM_M_TO_NU_DEG", help="Convenience isentropic path: Mach -> nu(deg) | Arguments: | - value_in: Input value | - gamma: Specific heat ratio | - branch: Optional branch | Returns: f64 | Example: =ENG_ISENTROPIC_FROM_M_TO_NU_DEG(2.0,1.4,'')")
def e_n_g_i_s_e_n_t_r_o_p_i_c_f_r_o_m_m_t_o_n_u_d_e_g(value_in, gamma, branch=""):
    """Convenience isentropic path: Mach -> nu(deg) | Arguments: | - value_in: Input value | - gamma: Specific heat ratio | - branch: Optional branch | Returns: f64 | Example: =ENG_ISENTROPIC_FROM_M_TO_NU_DEG(2.0,1.4,'')"""
    return invoke("device.isentropic_calc.value", {"input_kind": "mach", "target_kind": "prandtl_meyer_angle_deg", "input_value": value_in, "gamma": gamma, **({"branch": branch} if branch not in (None, "") else {})})

@xloil.func(name="ENG_ISENTROPIC_FROM_M_TO_P_P0", help="Convenience isentropic path: Mach -> p/p0 | Arguments: | - value_in: Input value | - gamma: Specific heat ratio | - branch: Optional branch | Returns: f64 | Example: =ENG_ISENTROPIC_FROM_M_TO_P_P0(2.0,1.4,'')")
def e_n_g_i_s_e_n_t_r_o_p_i_c_f_r_o_m_m_t_o_p_p0(value_in, gamma, branch=""):
    """Convenience isentropic path: Mach -> p/p0 | Arguments: | - value_in: Input value | - gamma: Specific heat ratio | - branch: Optional branch | Returns: f64 | Example: =ENG_ISENTROPIC_FROM_M_TO_P_P0(2.0,1.4,'')"""
    return invoke("device.isentropic_calc.value", {"input_kind": "mach", "target_kind": "pressure_ratio", "input_value": value_in, "gamma": gamma, **({"branch": branch} if branch not in (None, "") else {})})

@xloil.func(name="ENG_ISENTROPIC_FROM_MU_DEG_TO_P_P0", help="Convenience isentropic path: mu(deg) -> p/p0 | Arguments: | - value_in: Input value | - gamma: Specific heat ratio | - branch: Optional branch | Returns: f64 | Example: =ENG_ISENTROPIC_FROM_MU_DEG_TO_P_P0(30.0,1.4,'')")
def e_n_g_i_s_e_n_t_r_o_p_i_c_f_r_o_m_m_u_d_e_g_t_o_p_p0(value_in, gamma, branch=""):
    """Convenience isentropic path: mu(deg) -> p/p0 | Arguments: | - value_in: Input value | - gamma: Specific heat ratio | - branch: Optional branch | Returns: f64 | Example: =ENG_ISENTROPIC_FROM_MU_DEG_TO_P_P0(30.0,1.4,'')"""
    return invoke("device.isentropic_calc.value", {"input_kind": "mach_angle_deg", "target_kind": "pressure_ratio", "input_value": value_in, "gamma": gamma, **({"branch": branch} if branch not in (None, "") else {})})

@xloil.func(name="ENG_ISENTROPIC_FROM_NU_DEG_TO_M", help="Convenience isentropic path: nu(deg) -> Mach | Arguments: | - value_in: Input value | - gamma: Specific heat ratio | - branch: Optional branch | Returns: f64 | Example: =ENG_ISENTROPIC_FROM_NU_DEG_TO_M(26.3797608134,1.4,'')")
def e_n_g_i_s_e_n_t_r_o_p_i_c_f_r_o_m_n_u_d_e_g_t_o_m(value_in, gamma, branch=""):
    """Convenience isentropic path: nu(deg) -> Mach | Arguments: | - value_in: Input value | - gamma: Specific heat ratio | - branch: Optional branch | Returns: f64 | Example: =ENG_ISENTROPIC_FROM_NU_DEG_TO_M(26.3797608134,1.4,'')"""
    return invoke("device.isentropic_calc.value", {"input_kind": "prandtl_meyer_angle_deg", "target_kind": "mach", "input_value": value_in, "gamma": gamma, **({"branch": branch} if branch not in (None, "") else {})})

@xloil.func(name="ENG_ISENTROPIC_PATH_TEXT", help="Isentropic calculator helper: compact step trace text | Arguments: | - value_kind_in: Input kind (mach, mach_angle_deg, prandtl_meyer_angle_deg, pressure_ratio, temperature_ratio, density_ratio, area_ratio) | - value_in: Input value | - target_kind_out: Target kind (mach, mach_angle_deg, prandtl_meyer_angle_deg, pressure_ratio, temperature_ratio, density_ratio, area_ratio) | - gamma: Specific heat ratio | - branch: Optional branch for double-valued inversions (subsonic/supersonic) | Returns: str | Example: =ENG_ISENTROPIC_PATH_TEXT('mach_angle_deg',30.0,'pressure_ratio',1.4,'')")
def e_n_g_i_s_e_n_t_r_o_p_i_c_p_a_t_h_t_e_x_t(value_kind_in, value_in, target_kind_out, gamma, branch=""):
    """Isentropic calculator helper: compact step trace text | Arguments: | - value_kind_in: Input kind (mach, mach_angle_deg, prandtl_meyer_angle_deg, pressure_ratio, temperature_ratio, density_ratio, area_ratio) | - value_in: Input value | - target_kind_out: Target kind (mach, mach_angle_deg, prandtl_meyer_angle_deg, pressure_ratio, temperature_ratio, density_ratio, area_ratio) | - gamma: Specific heat ratio | - branch: Optional branch for double-valued inversions (subsonic/supersonic) | Returns: str | Example: =ENG_ISENTROPIC_PATH_TEXT('mach_angle_deg',30.0,'pressure_ratio',1.4,'')"""
    return invoke("device.isentropic_calc.path_text", {"input_kind": value_kind_in, "input_value": value_in, "target_kind": target_kind_out, "gamma": gamma, **({"branch": branch} if branch not in (None, "") else {})})

@xloil.func(name="ENG_ISENTROPIC_PIVOT_MACH", help="Isentropic calculator helper: return resolved pivot Mach | Arguments: | - value_kind_in: Input kind (mach, mach_angle_deg, prandtl_meyer_angle_deg, pressure_ratio, temperature_ratio, density_ratio, area_ratio) | - value_in: Input value | - target_kind_out: Target kind (mach, mach_angle_deg, prandtl_meyer_angle_deg, pressure_ratio, temperature_ratio, density_ratio, area_ratio) | - gamma: Specific heat ratio | - branch: Optional branch for double-valued inversions (subsonic/supersonic) | Returns: f64 | Example: =ENG_ISENTROPIC_PIVOT_MACH('area_ratio',2.0,'mach',1.4,'subsonic')")
def e_n_g_i_s_e_n_t_r_o_p_i_c_p_i_v_o_t_m_a_c_h(value_kind_in, value_in, target_kind_out, gamma, branch=""):
    """Isentropic calculator helper: return resolved pivot Mach | Arguments: | - value_kind_in: Input kind (mach, mach_angle_deg, prandtl_meyer_angle_deg, pressure_ratio, temperature_ratio, density_ratio, area_ratio) | - value_in: Input value | - target_kind_out: Target kind (mach, mach_angle_deg, prandtl_meyer_angle_deg, pressure_ratio, temperature_ratio, density_ratio, area_ratio) | - gamma: Specific heat ratio | - branch: Optional branch for double-valued inversions (subsonic/supersonic) | Returns: f64 | Example: =ENG_ISENTROPIC_PIVOT_MACH('area_ratio',2.0,'mach',1.4,'subsonic')"""
    return invoke("device.isentropic_calc.pivot_mach", {"input_kind": value_kind_in, "input_value": value_in, "target_kind": target_kind_out, "gamma": gamma, **({"branch": branch} if branch not in (None, "") else {})})

@xloil.func(name="ENG_DEVICE_MODES", help="Read supported modes for a device | Arguments: | - key: Device key | Returns: list | Example: =ENG_DEVICE_MODES('pipe_loss')")
def e_n_g_d_e_v_i_c_e_m_o_d_e_s(key):
    """Read supported modes for a device | Arguments: | - key: Device key | Returns: list | Example: =ENG_DEVICE_MODES('pipe_loss')"""
    return invoke("meta.get", {"entity": "device", "field": "supported_modes", "key": key})

@xloil.func(name="ENG_DEVICE_MODE_COUNT", help="Read device mode count | Arguments: | - key: Device key | Returns: u64 | Example: =ENG_DEVICE_MODE_COUNT('pipe_loss')")
def e_n_g_d_e_v_i_c_e_m_o_d_e_c_o_u_n_t(key):
    """Read device mode count | Arguments: | - key: Device key | Returns: u64 | Example: =ENG_DEVICE_MODE_COUNT('pipe_loss')"""
    return invoke("device.mode.count", {"key": key})

@xloil.func(name="ENG_DEVICE_MODES_TEXT", help="Read device modes as delimited text | Arguments: | - key: Device key | Returns: str | Example: =ENG_DEVICE_MODES_TEXT('pipe_loss')")
def e_n_g_d_e_v_i_c_e_m_o_d_e_s_t_e_x_t(key):
    """Read device modes as delimited text | Arguments: | - key: Device key | Returns: str | Example: =ENG_DEVICE_MODES_TEXT('pipe_loss')"""
    return invoke("device.modes.text", {"key": key})

@xloil.func(name="ENG_NORMAL_SHOCK", help="Normal shock calculator: input kind -> target kind through M1 pivot | Arguments: | - input_kind: Input kind (m1, m2, p2_p1, rho2_rho1, t2_t1, p02_p01) | - input_value: Input value | - target_kind: Target kind (m1, m2, p2_p1, rho2_rho1, t2_t1, p02_p01) | - gamma: Specific heat ratio | Returns: f64 | Example: =ENG_NORMAL_SHOCK('m1',2.0,'p2_p1',1.4)")
def e_n_g_n_o_r_m_a_l_s_h_o_c_k(input_kind, input_value, target_kind, gamma):
    """Normal shock calculator: input kind -> target kind through M1 pivot | Arguments: | - input_kind: Input kind (m1, m2, p2_p1, rho2_rho1, t2_t1, p02_p01) | - input_value: Input value | - target_kind: Target kind (m1, m2, p2_p1, rho2_rho1, t2_t1, p02_p01) | - gamma: Specific heat ratio | Returns: f64 | Example: =ENG_NORMAL_SHOCK('m1',2.0,'p2_p1',1.4)"""
    return invoke("device.normal_shock_calc.value", {"input_kind": input_kind, "input_value": input_value, "target_kind": target_kind, "gamma": gamma})

@xloil.func(name="ENG_NORMAL_SHOCK_FROM_M1_TO_M2", help="Convenience normal-shock path: M1 -> M2 | Arguments: | - input_value: Upstream Mach number M1 | - gamma: Specific heat ratio | Returns: f64 | Example: =ENG_NORMAL_SHOCK_FROM_M1_TO_M2(2.0,1.4)")
def e_n_g_n_o_r_m_a_l_s_h_o_c_k_f_r_o_m_m1_t_o_m2(input_value, gamma):
    """Convenience normal-shock path: M1 -> M2 | Arguments: | - input_value: Upstream Mach number M1 | - gamma: Specific heat ratio | Returns: f64 | Example: =ENG_NORMAL_SHOCK_FROM_M1_TO_M2(2.0,1.4)"""
    return invoke("device.normal_shock_calc.value", {"input_kind": "m1", "target_kind": "m2", "input_value": input_value, "gamma": gamma})

@xloil.func(name="ENG_NORMAL_SHOCK_FROM_M1_TO_P02_P01", help="Convenience normal-shock path: M1 -> p02/p01 | Arguments: | - input_value: Upstream Mach number M1 | - gamma: Specific heat ratio | Returns: f64 | Example: =ENG_NORMAL_SHOCK_FROM_M1_TO_P02_P01(2.0,1.4)")
def e_n_g_n_o_r_m_a_l_s_h_o_c_k_f_r_o_m_m1_t_o_p02_p01(input_value, gamma):
    """Convenience normal-shock path: M1 -> p02/p01 | Arguments: | - input_value: Upstream Mach number M1 | - gamma: Specific heat ratio | Returns: f64 | Example: =ENG_NORMAL_SHOCK_FROM_M1_TO_P02_P01(2.0,1.4)"""
    return invoke("device.normal_shock_calc.value", {"input_kind": "m1", "target_kind": "p02_p01", "input_value": input_value, "gamma": gamma})

@xloil.func(name="ENG_NORMAL_SHOCK_FROM_M1_TO_P2_P1", help="Convenience normal-shock path: M1 -> p2/p1 | Arguments: | - input_value: Upstream Mach number M1 | - gamma: Specific heat ratio | Returns: f64 | Example: =ENG_NORMAL_SHOCK_FROM_M1_TO_P2_P1(2.0,1.4)")
def e_n_g_n_o_r_m_a_l_s_h_o_c_k_f_r_o_m_m1_t_o_p2_p1(input_value, gamma):
    """Convenience normal-shock path: M1 -> p2/p1 | Arguments: | - input_value: Upstream Mach number M1 | - gamma: Specific heat ratio | Returns: f64 | Example: =ENG_NORMAL_SHOCK_FROM_M1_TO_P2_P1(2.0,1.4)"""
    return invoke("device.normal_shock_calc.value", {"input_kind": "m1", "target_kind": "p2_p1", "input_value": input_value, "gamma": gamma})

@xloil.func(name="ENG_NORMAL_SHOCK_FROM_M1_TO_RHO2_RHO1", help="Convenience normal-shock path: M1 -> rho2/rho1 | Arguments: | - input_value: Upstream Mach number M1 | - gamma: Specific heat ratio | Returns: f64 | Example: =ENG_NORMAL_SHOCK_FROM_M1_TO_RHO2_RHO1(2.0,1.4)")
def e_n_g_n_o_r_m_a_l_s_h_o_c_k_f_r_o_m_m1_t_o_r_h_o2_r_h_o1(input_value, gamma):
    """Convenience normal-shock path: M1 -> rho2/rho1 | Arguments: | - input_value: Upstream Mach number M1 | - gamma: Specific heat ratio | Returns: f64 | Example: =ENG_NORMAL_SHOCK_FROM_M1_TO_RHO2_RHO1(2.0,1.4)"""
    return invoke("device.normal_shock_calc.value", {"input_kind": "m1", "target_kind": "rho2_rho1", "input_value": input_value, "gamma": gamma})

@xloil.func(name="ENG_NORMAL_SHOCK_FROM_M1_TO_T2_T1", help="Convenience normal-shock path: M1 -> T2/T1 | Arguments: | - input_value: Upstream Mach number M1 | - gamma: Specific heat ratio | Returns: f64 | Example: =ENG_NORMAL_SHOCK_FROM_M1_TO_T2_T1(2.0,1.4)")
def e_n_g_n_o_r_m_a_l_s_h_o_c_k_f_r_o_m_m1_t_o_t2_t1(input_value, gamma):
    """Convenience normal-shock path: M1 -> T2/T1 | Arguments: | - input_value: Upstream Mach number M1 | - gamma: Specific heat ratio | Returns: f64 | Example: =ENG_NORMAL_SHOCK_FROM_M1_TO_T2_T1(2.0,1.4)"""
    return invoke("device.normal_shock_calc.value", {"input_kind": "m1", "target_kind": "t2_t1", "input_value": input_value, "gamma": gamma})

@xloil.func(name="ENG_NORMAL_SHOCK_PATH_TEXT", help="Normal shock calculator helper: compact step trace text | Arguments: | - input_kind: Input kind (m1, m2, p2_p1, rho2_rho1, t2_t1, p02_p01) | - input_value: Input value | - target_kind: Target kind (m1, m2, p2_p1, rho2_rho1, t2_t1, p02_p01) | - gamma: Specific heat ratio | Returns: str | Example: =ENG_NORMAL_SHOCK_PATH_TEXT('p02_p01',0.72,'m2',1.4)")
def e_n_g_n_o_r_m_a_l_s_h_o_c_k_p_a_t_h_t_e_x_t(input_kind, input_value, target_kind, gamma):
    """Normal shock calculator helper: compact step trace text | Arguments: | - input_kind: Input kind (m1, m2, p2_p1, rho2_rho1, t2_t1, p02_p01) | - input_value: Input value | - target_kind: Target kind (m1, m2, p2_p1, rho2_rho1, t2_t1, p02_p01) | - gamma: Specific heat ratio | Returns: str | Example: =ENG_NORMAL_SHOCK_PATH_TEXT('p02_p01',0.72,'m2',1.4)"""
    return invoke("device.normal_shock_calc.path_text", {"input_kind": input_kind, "input_value": input_value, "target_kind": target_kind, "gamma": gamma})

@xloil.func(name="ENG_NORMAL_SHOCK_PIVOT_M1", help="Normal shock calculator helper: return resolved pivot M1 | Arguments: | - input_kind: Input kind (m1, m2, p2_p1, rho2_rho1, t2_t1, p02_p01) | - input_value: Input value | - target_kind: Target kind (m1, m2, p2_p1, rho2_rho1, t2_t1, p02_p01) | - gamma: Specific heat ratio | Returns: f64 | Example: =ENG_NORMAL_SHOCK_PIVOT_M1('p2_p1',4.5,'m2',1.4)")
def e_n_g_n_o_r_m_a_l_s_h_o_c_k_p_i_v_o_t_m1(input_kind, input_value, target_kind, gamma):
    """Normal shock calculator helper: return resolved pivot M1 | Arguments: | - input_kind: Input kind (m1, m2, p2_p1, rho2_rho1, t2_t1, p02_p01) | - input_value: Input value | - target_kind: Target kind (m1, m2, p2_p1, rho2_rho1, t2_t1, p02_p01) | - gamma: Specific heat ratio | Returns: f64 | Example: =ENG_NORMAL_SHOCK_PIVOT_M1('p2_p1',4.5,'m2',1.4)"""
    return invoke("device.normal_shock_calc.pivot_m1", {"input_kind": input_kind, "input_value": input_value, "target_kind": target_kind, "gamma": gamma})

@xloil.func(name="ENG_NOZZLE_FLOW", help="Nozzle-flow calculator: input kind -> target kind through Mach pivot | Arguments: | - input_kind: Input kind (mach, area_ratio, pressure_ratio, temperature_ratio, density_ratio) | - input_value: Input value | - target_kind: Target kind (mach, area_ratio, pressure_ratio, temperature_ratio, density_ratio, p, t, rho) | - gamma: Specific heat ratio | - p0: Optional stagnation pressure reference for static p output | - t0: Optional stagnation temperature reference for static T output | - rho0: Optional stagnation density reference for static rho output | - branch: Subsonic/supersonic branch for area_ratio -> mach inversion | Returns: f64 | Example: =ENG_NOZZLE_FLOW('mach',2.0,'pressure_ratio',1.4,NA(),NA(),NA(),'')")
def e_n_g_n_o_z_z_l_e_f_l_o_w(input_kind, input_value, target_kind, gamma, p0, t0, rho0, branch=""):
    """Nozzle-flow calculator: input kind -> target kind through Mach pivot | Arguments: | - input_kind: Input kind (mach, area_ratio, pressure_ratio, temperature_ratio, density_ratio) | - input_value: Input value | - target_kind: Target kind (mach, area_ratio, pressure_ratio, temperature_ratio, density_ratio, p, t, rho) | - gamma: Specific heat ratio | - p0: Optional stagnation pressure reference for static p output | - t0: Optional stagnation temperature reference for static T output | - rho0: Optional stagnation density reference for static rho output | - branch: Subsonic/supersonic branch for area_ratio -> mach inversion | Returns: f64 | Example: =ENG_NOZZLE_FLOW('mach',2.0,'pressure_ratio',1.4,NA(),NA(),NA(),'')"""
    return invoke("device.nozzle_flow_calc.value", {"input_kind": input_kind, "input_value": input_value, "target_kind": target_kind, "gamma": gamma, "p0": p0, "t0": t0, "rho0": rho0, **({"branch": branch} if branch not in (None, "") else {})})

@xloil.func(name="ENG_NOZZLE_FLOW_FROM_A_ASTAR_TO_M", help="Convenience nozzle-flow path: A/A* -> Mach (branch required) | Arguments: | - input_value: Area ratio input value (A/A*) | - gamma: Specific heat ratio | - branch: Subsonic or supersonic branch | Returns: f64 | Example: =ENG_NOZZLE_FLOW_FROM_A_ASTAR_TO_M(2.0,1.4,'supersonic')")
def e_n_g_n_o_z_z_l_e_f_l_o_w_f_r_o_m_a_a_s_t_a_r_t_o_m(input_value, gamma, branch=""):
    """Convenience nozzle-flow path: A/A* -> Mach (branch required) | Arguments: | - input_value: Area ratio input value (A/A*) | - gamma: Specific heat ratio | - branch: Subsonic or supersonic branch | Returns: f64 | Example: =ENG_NOZZLE_FLOW_FROM_A_ASTAR_TO_M(2.0,1.4,'supersonic')"""
    return invoke("device.nozzle_flow_calc.value", {"input_kind": "area_ratio", "target_kind": "mach", "input_value": input_value, "gamma": gamma, **({"branch": branch} if branch not in (None, "") else {})})

@xloil.func(name="ENG_NOZZLE_FLOW_FROM_M_TO_A_ASTAR", help="Convenience nozzle-flow path: Mach -> A/A* | Arguments: | - input_value: Mach input value | - gamma: Specific heat ratio | Returns: f64 | Example: =ENG_NOZZLE_FLOW_FROM_M_TO_A_ASTAR(2.0,1.4)")
def e_n_g_n_o_z_z_l_e_f_l_o_w_f_r_o_m_m_t_o_a_a_s_t_a_r(input_value, gamma):
    """Convenience nozzle-flow path: Mach -> A/A* | Arguments: | - input_value: Mach input value | - gamma: Specific heat ratio | Returns: f64 | Example: =ENG_NOZZLE_FLOW_FROM_M_TO_A_ASTAR(2.0,1.4)"""
    return invoke("device.nozzle_flow_calc.value", {"input_kind": "mach", "target_kind": "area_ratio", "input_value": input_value, "gamma": gamma})

@xloil.func(name="ENG_NOZZLE_FLOW_FROM_M_TO_P_P0", help="Convenience nozzle-flow path: Mach -> p/p0 | Arguments: | - input_value: Mach input value | - gamma: Specific heat ratio | Returns: f64 | Example: =ENG_NOZZLE_FLOW_FROM_M_TO_P_P0(2.0,1.4)")
def e_n_g_n_o_z_z_l_e_f_l_o_w_f_r_o_m_m_t_o_p_p0(input_value, gamma):
    """Convenience nozzle-flow path: Mach -> p/p0 | Arguments: | - input_value: Mach input value | - gamma: Specific heat ratio | Returns: f64 | Example: =ENG_NOZZLE_FLOW_FROM_M_TO_P_P0(2.0,1.4)"""
    return invoke("device.nozzle_flow_calc.value", {"input_kind": "mach", "target_kind": "pressure_ratio", "input_value": input_value, "gamma": gamma})

@xloil.func(name="ENG_NOZZLE_FLOW_FROM_M_TO_RHO_RHO0", help="Convenience nozzle-flow path: Mach -> rho/rho0 | Arguments: | - input_value: Mach input value | - gamma: Specific heat ratio | Returns: f64 | Example: =ENG_NOZZLE_FLOW_FROM_M_TO_RHO_RHO0(2.0,1.4)")
def e_n_g_n_o_z_z_l_e_f_l_o_w_f_r_o_m_m_t_o_r_h_o_r_h_o0(input_value, gamma):
    """Convenience nozzle-flow path: Mach -> rho/rho0 | Arguments: | - input_value: Mach input value | - gamma: Specific heat ratio | Returns: f64 | Example: =ENG_NOZZLE_FLOW_FROM_M_TO_RHO_RHO0(2.0,1.4)"""
    return invoke("device.nozzle_flow_calc.value", {"input_kind": "mach", "target_kind": "density_ratio", "input_value": input_value, "gamma": gamma})

@xloil.func(name="ENG_NOZZLE_FLOW_FROM_M_TO_T_T0", help="Convenience nozzle-flow path: Mach -> T/T0 | Arguments: | - input_value: Mach input value | - gamma: Specific heat ratio | Returns: f64 | Example: =ENG_NOZZLE_FLOW_FROM_M_TO_T_T0(2.0,1.4)")
def e_n_g_n_o_z_z_l_e_f_l_o_w_f_r_o_m_m_t_o_t_t0(input_value, gamma):
    """Convenience nozzle-flow path: Mach -> T/T0 | Arguments: | - input_value: Mach input value | - gamma: Specific heat ratio | Returns: f64 | Example: =ENG_NOZZLE_FLOW_FROM_M_TO_T_T0(2.0,1.4)"""
    return invoke("device.nozzle_flow_calc.value", {"input_kind": "mach", "target_kind": "temperature_ratio", "input_value": input_value, "gamma": gamma})

@xloil.func(name="ENG_NOZZLE_FLOW_PATH_TEXT", help="Nozzle-flow calculator helper: compact step trace text | Arguments: | - input_kind: Input kind (mach, area_ratio, pressure_ratio, temperature_ratio, density_ratio) | - input_value: Input value | - target_kind: Target kind (mach, area_ratio, pressure_ratio, temperature_ratio, density_ratio, p, t, rho) | - gamma: Specific heat ratio | - p0: Optional stagnation pressure reference for static p output | - t0: Optional stagnation temperature reference for static T output | - rho0: Optional stagnation density reference for static rho output | - branch: Subsonic/supersonic branch for area_ratio -> mach inversion | Returns: str | Example: =ENG_NOZZLE_FLOW_PATH_TEXT('area_ratio',2.0,'mach',1.4,NA(),NA(),NA(),'supersonic')")
def e_n_g_n_o_z_z_l_e_f_l_o_w_p_a_t_h_t_e_x_t(input_kind, input_value, target_kind, gamma, p0, t0, rho0, branch=""):
    """Nozzle-flow calculator helper: compact step trace text | Arguments: | - input_kind: Input kind (mach, area_ratio, pressure_ratio, temperature_ratio, density_ratio) | - input_value: Input value | - target_kind: Target kind (mach, area_ratio, pressure_ratio, temperature_ratio, density_ratio, p, t, rho) | - gamma: Specific heat ratio | - p0: Optional stagnation pressure reference for static p output | - t0: Optional stagnation temperature reference for static T output | - rho0: Optional stagnation density reference for static rho output | - branch: Subsonic/supersonic branch for area_ratio -> mach inversion | Returns: str | Example: =ENG_NOZZLE_FLOW_PATH_TEXT('area_ratio',2.0,'mach',1.4,NA(),NA(),NA(),'supersonic')"""
    return invoke("device.nozzle_flow_calc.path_text", {"input_kind": input_kind, "input_value": input_value, "target_kind": target_kind, "gamma": gamma, "p0": p0, "t0": t0, "rho0": rho0, **({"branch": branch} if branch not in (None, "") else {})})

@xloil.func(name="ENG_NOZZLE_FLOW_PIVOT_MACH", help="Nozzle-flow calculator helper: return resolved pivot Mach | Arguments: | - input_kind: Input kind (mach, area_ratio, pressure_ratio, temperature_ratio, density_ratio) | - input_value: Input value | - target_kind: Target kind (mach, area_ratio, pressure_ratio, temperature_ratio, density_ratio, p, t, rho) | - gamma: Specific heat ratio | - p0: Optional stagnation pressure reference for static p output | - t0: Optional stagnation temperature reference for static T output | - rho0: Optional stagnation density reference for static rho output | - branch: Subsonic/supersonic branch for area_ratio -> mach inversion | Returns: f64 | Example: =ENG_NOZZLE_FLOW_PIVOT_MACH('area_ratio',2.0,'mach',1.4,NA(),NA(),NA(),'subsonic')")
def e_n_g_n_o_z_z_l_e_f_l_o_w_p_i_v_o_t_m_a_c_h(input_kind, input_value, target_kind, gamma, p0, t0, rho0, branch=""):
    """Nozzle-flow calculator helper: return resolved pivot Mach | Arguments: | - input_kind: Input kind (mach, area_ratio, pressure_ratio, temperature_ratio, density_ratio) | - input_value: Input value | - target_kind: Target kind (mach, area_ratio, pressure_ratio, temperature_ratio, density_ratio, p, t, rho) | - gamma: Specific heat ratio | - p0: Optional stagnation pressure reference for static p output | - t0: Optional stagnation temperature reference for static T output | - rho0: Optional stagnation density reference for static rho output | - branch: Subsonic/supersonic branch for area_ratio -> mach inversion | Returns: f64 | Example: =ENG_NOZZLE_FLOW_PIVOT_MACH('area_ratio',2.0,'mach',1.4,NA(),NA(),NA(),'subsonic')"""
    return invoke("device.nozzle_flow_calc.pivot_mach", {"input_kind": input_kind, "input_value": input_value, "target_kind": target_kind, "gamma": gamma, "p0": p0, "t0": t0, "rho0": rho0, **({"branch": branch} if branch not in (None, "") else {})})

@xloil.func(name="ENG_OBLIQUE_SHOCK", help="Oblique shock calculator: (M1 + beta/theta) -> target with weak/strong branch support | Arguments: | - m1: Upstream Mach number M1 | - input_kind: Input kind (beta_deg or theta_deg) | - input_value: Input value in degrees | - target_kind: Target kind (theta_deg, beta_deg, mn1, mn2, m2, p2_p1, rho2_rho1, t2_t1, p02_p01) | - gamma: Specific heat ratio | - branch: Weak/strong branch required for theta->beta inversion paths | Returns: f64 | Example: =ENG_OBLIQUE_SHOCK(2.0,'theta_deg',10.0,'beta_deg',1.4,'weak')")
def e_n_g_o_b_l_i_q_u_e_s_h_o_c_k(m1, input_kind, input_value, target_kind, gamma, branch=""):
    """Oblique shock calculator: (M1 + beta/theta) -> target with weak/strong branch support | Arguments: | - m1: Upstream Mach number M1 | - input_kind: Input kind (beta_deg or theta_deg) | - input_value: Input value in degrees | - target_kind: Target kind (theta_deg, beta_deg, mn1, mn2, m2, p2_p1, rho2_rho1, t2_t1, p02_p01) | - gamma: Specific heat ratio | - branch: Weak/strong branch required for theta->beta inversion paths | Returns: f64 | Example: =ENG_OBLIQUE_SHOCK(2.0,'theta_deg',10.0,'beta_deg',1.4,'weak')"""
    return invoke("device.oblique_shock_calc.value", {"m1": m1, "input_kind": input_kind, "input_value": input_value, "target_kind": target_kind, "gamma": gamma, **({"branch": branch} if branch not in (None, "") else {})})

@xloil.func(name="ENG_OBLIQUE_SHOCK_FROM_M1_BETA_TO_M2", help="Convenience oblique-shock path: (M1, beta_deg) -> M2 | Arguments: | - m1: Upstream Mach number M1 | - input_value: Shock angle beta in degrees | - gamma: Specific heat ratio | Returns: f64 | Example: =ENG_OBLIQUE_SHOCK_FROM_M1_BETA_TO_M2(2.0,40.0,1.4)")
def e_n_g_o_b_l_i_q_u_e_s_h_o_c_k_f_r_o_m_m1_b_e_t_a_t_o_m2(m1, input_value, gamma):
    """Convenience oblique-shock path: (M1, beta_deg) -> M2 | Arguments: | - m1: Upstream Mach number M1 | - input_value: Shock angle beta in degrees | - gamma: Specific heat ratio | Returns: f64 | Example: =ENG_OBLIQUE_SHOCK_FROM_M1_BETA_TO_M2(2.0,40.0,1.4)"""
    return invoke("device.oblique_shock_calc.value", {"input_kind": "beta_deg", "target_kind": "m2", "m1": m1, "input_value": input_value, "gamma": gamma})

@xloil.func(name="ENG_OBLIQUE_SHOCK_FROM_M1_BETA_TO_THETA", help="Convenience oblique-shock path: (M1, beta_deg) -> theta_deg | Arguments: | - m1: Upstream Mach number M1 | - input_value: Shock angle beta in degrees | - gamma: Specific heat ratio | Returns: f64 | Example: =ENG_OBLIQUE_SHOCK_FROM_M1_BETA_TO_THETA(2.0,40.0,1.4)")
def e_n_g_o_b_l_i_q_u_e_s_h_o_c_k_f_r_o_m_m1_b_e_t_a_t_o_t_h_e_t_a(m1, input_value, gamma):
    """Convenience oblique-shock path: (M1, beta_deg) -> theta_deg | Arguments: | - m1: Upstream Mach number M1 | - input_value: Shock angle beta in degrees | - gamma: Specific heat ratio | Returns: f64 | Example: =ENG_OBLIQUE_SHOCK_FROM_M1_BETA_TO_THETA(2.0,40.0,1.4)"""
    return invoke("device.oblique_shock_calc.value", {"input_kind": "beta_deg", "target_kind": "theta_deg", "m1": m1, "input_value": input_value, "gamma": gamma})

@xloil.func(name="ENG_OBLIQUE_SHOCK_FROM_M1_THETA_TO_BETA", help="Convenience oblique-shock path: (M1, theta_deg, branch) -> beta_deg | Arguments: | - m1: Upstream Mach number M1 | - input_value: Flow deflection theta in degrees | - gamma: Specific heat ratio | - branch: Weak or strong branch | Returns: f64 | Example: =ENG_OBLIQUE_SHOCK_FROM_M1_THETA_TO_BETA(2.0,10.0,1.4,'weak')")
def e_n_g_o_b_l_i_q_u_e_s_h_o_c_k_f_r_o_m_m1_t_h_e_t_a_t_o_b_e_t_a(m1, input_value, gamma, branch=""):
    """Convenience oblique-shock path: (M1, theta_deg, branch) -> beta_deg | Arguments: | - m1: Upstream Mach number M1 | - input_value: Flow deflection theta in degrees | - gamma: Specific heat ratio | - branch: Weak or strong branch | Returns: f64 | Example: =ENG_OBLIQUE_SHOCK_FROM_M1_THETA_TO_BETA(2.0,10.0,1.4,'weak')"""
    return invoke("device.oblique_shock_calc.value", {"input_kind": "theta_deg", "target_kind": "beta_deg", "m1": m1, "input_value": input_value, "gamma": gamma, **({"branch": branch} if branch not in (None, "") else {})})

@xloil.func(name="ENG_OBLIQUE_SHOCK_FROM_M1_THETA_TO_P2_P1", help="Convenience oblique-shock path: (M1, theta_deg, branch) -> p2/p1 | Arguments: | - m1: Upstream Mach number M1 | - input_value: Flow deflection theta in degrees | - gamma: Specific heat ratio | - branch: Weak or strong branch | Returns: f64 | Example: =ENG_OBLIQUE_SHOCK_FROM_M1_THETA_TO_P2_P1(2.0,10.0,1.4,'weak')")
def e_n_g_o_b_l_i_q_u_e_s_h_o_c_k_f_r_o_m_m1_t_h_e_t_a_t_o_p2_p1(m1, input_value, gamma, branch=""):
    """Convenience oblique-shock path: (M1, theta_deg, branch) -> p2/p1 | Arguments: | - m1: Upstream Mach number M1 | - input_value: Flow deflection theta in degrees | - gamma: Specific heat ratio | - branch: Weak or strong branch | Returns: f64 | Example: =ENG_OBLIQUE_SHOCK_FROM_M1_THETA_TO_P2_P1(2.0,10.0,1.4,'weak')"""
    return invoke("device.oblique_shock_calc.value", {"input_kind": "theta_deg", "target_kind": "p2_p1", "m1": m1, "input_value": input_value, "gamma": gamma, **({"branch": branch} if branch not in (None, "") else {})})

@xloil.func(name="ENG_OBLIQUE_SHOCK_PATH_TEXT", help="Oblique shock calculator helper: compact step trace text | Arguments: | - m1: Upstream Mach number M1 | - input_kind: Input kind (beta_deg or theta_deg) | - input_value: Input value in degrees | - target_kind: Target kind (theta_deg, beta_deg, mn1, mn2, m2, p2_p1, rho2_rho1, t2_t1, p02_p01) | - gamma: Specific heat ratio | - branch: Weak/strong branch required for theta->beta inversion paths | Returns: str | Example: =ENG_OBLIQUE_SHOCK_PATH_TEXT(2.0,'theta_deg',10.0,'p2_p1',1.4,'weak')")
def e_n_g_o_b_l_i_q_u_e_s_h_o_c_k_p_a_t_h_t_e_x_t(m1, input_kind, input_value, target_kind, gamma, branch=""):
    """Oblique shock calculator helper: compact step trace text | Arguments: | - m1: Upstream Mach number M1 | - input_kind: Input kind (beta_deg or theta_deg) | - input_value: Input value in degrees | - target_kind: Target kind (theta_deg, beta_deg, mn1, mn2, m2, p2_p1, rho2_rho1, t2_t1, p02_p01) | - gamma: Specific heat ratio | - branch: Weak/strong branch required for theta->beta inversion paths | Returns: str | Example: =ENG_OBLIQUE_SHOCK_PATH_TEXT(2.0,'theta_deg',10.0,'p2_p1',1.4,'weak')"""
    return invoke("device.oblique_shock_calc.path_text", {"m1": m1, "input_kind": input_kind, "input_value": input_value, "target_kind": target_kind, "gamma": gamma, **({"branch": branch} if branch not in (None, "") else {})})

@xloil.func(name="ENG_PIPE_LOSS_DELTA_P", help="Solve pipe pressure drop using Fixed/Colebrook friction model | Arguments: | - friction_model: Colebrook or Fixed | - fixed_f: Required when friction_model=Fixed | - density: Density input (optional with fluid context) | - viscosity: Viscosity input (required for Colebrook without fluid context) | - velocity: Velocity | - diameter: Diameter | - length: Length | - roughness: Roughness (Colebrook) | - fluid: Optional fluid key (e.g. H2O) | - in1_key: Fluid state input key 1 | - in1_value: Fluid state input value 1 | - in2_key: Fluid state input key 2 | - in2_value: Fluid state input value 2 | Returns: f64 | Example: =ENG_PIPE_LOSS_DELTA_P(...)")
def e_n_g_p_i_p_e_l_o_s_s_d_e_l_t_a_p(friction_model, fixed_f, density, viscosity, velocity, diameter, length, roughness, fluid, in1_key, in1_value, in2_key, in2_value):
    """Solve pipe pressure drop using Fixed/Colebrook friction model | Arguments: | - friction_model: Colebrook or Fixed | - fixed_f: Required when friction_model=Fixed | - density: Density input (optional with fluid context) | - viscosity: Viscosity input (required for Colebrook without fluid context) | - velocity: Velocity | - diameter: Diameter | - length: Length | - roughness: Roughness (Colebrook) | - fluid: Optional fluid key (e.g. H2O) | - in1_key: Fluid state input key 1 | - in1_value: Fluid state input value 1 | - in2_key: Fluid state input key 2 | - in2_value: Fluid state input value 2 | Returns: f64 | Example: =ENG_PIPE_LOSS_DELTA_P(...)"""
    return invoke("device.pipe_loss.solve_delta_p", {"friction_model": friction_model, "fixed_f": fixed_f, "rho": density, "mu": viscosity, "v": velocity, "d": diameter, "l": length, "eps": roughness, "fluid": fluid, "in1_key": in1_key, "in1_value": in1_value, "in2_key": in2_key, "in2_value": in2_value})

@xloil.func(name="ENG_RAYLEIGH", help="Rayleigh-flow calculator: input kind -> target kind through Mach pivot | Arguments: | - input_kind: Input kind (mach, t_tstar, p_pstar, rho_rhostar, t0_t0star, p0_p0star, v_vstar) | - input_value: Input value | - target_kind: Target kind (mach, t_tstar, p_pstar, rho_rhostar, t0_t0star, p0_p0star, v_vstar) | - gamma: Specific heat ratio | - branch: Subsonic/supersonic branch for branch-sensitive inverse paths | Returns: f64 | Example: =ENG_RAYLEIGH('mach',2.0,'p_pstar',1.4,'')")
def e_n_g_r_a_y_l_e_i_g_h(input_kind, input_value, target_kind, gamma, branch=""):
    """Rayleigh-flow calculator: input kind -> target kind through Mach pivot | Arguments: | - input_kind: Input kind (mach, t_tstar, p_pstar, rho_rhostar, t0_t0star, p0_p0star, v_vstar) | - input_value: Input value | - target_kind: Target kind (mach, t_tstar, p_pstar, rho_rhostar, t0_t0star, p0_p0star, v_vstar) | - gamma: Specific heat ratio | - branch: Subsonic/supersonic branch for branch-sensitive inverse paths | Returns: f64 | Example: =ENG_RAYLEIGH('mach',2.0,'p_pstar',1.4,'')"""
    return invoke("device.rayleigh_calc.value", {"input_kind": input_kind, "input_value": input_value, "target_kind": target_kind, "gamma": gamma, **({"branch": branch} if branch not in (None, "") else {})})

@xloil.func(name="ENG_RAYLEIGH_FROM_M_TO_P0_P0STAR", help="Convenience Rayleigh path: Mach -> p0/p0* | Arguments: | - input_value: Mach input value | - gamma: Specific heat ratio | Returns: f64 | Example: =ENG_RAYLEIGH_FROM_M_TO_P0_P0STAR(2.0,1.4)")
def e_n_g_r_a_y_l_e_i_g_h_f_r_o_m_m_t_o_p0_p0_s_t_a_r(input_value, gamma):
    """Convenience Rayleigh path: Mach -> p0/p0* | Arguments: | - input_value: Mach input value | - gamma: Specific heat ratio | Returns: f64 | Example: =ENG_RAYLEIGH_FROM_M_TO_P0_P0STAR(2.0,1.4)"""
    return invoke("device.rayleigh_calc.value", {"input_kind": "mach", "target_kind": "p0_p0star", "input_value": input_value, "gamma": gamma})

@xloil.func(name="ENG_RAYLEIGH_FROM_M_TO_P_PSTAR", help="Convenience Rayleigh path: Mach -> p/p* | Arguments: | - input_value: Mach input value | - gamma: Specific heat ratio | Returns: f64 | Example: =ENG_RAYLEIGH_FROM_M_TO_P_PSTAR(2.0,1.4)")
def e_n_g_r_a_y_l_e_i_g_h_f_r_o_m_m_t_o_p_p_s_t_a_r(input_value, gamma):
    """Convenience Rayleigh path: Mach -> p/p* | Arguments: | - input_value: Mach input value | - gamma: Specific heat ratio | Returns: f64 | Example: =ENG_RAYLEIGH_FROM_M_TO_P_PSTAR(2.0,1.4)"""
    return invoke("device.rayleigh_calc.value", {"input_kind": "mach", "target_kind": "p_pstar", "input_value": input_value, "gamma": gamma})

@xloil.func(name="ENG_RAYLEIGH_FROM_M_TO_RHO_RHOSTAR", help="Convenience Rayleigh path: Mach -> rho/rho* | Arguments: | - input_value: Mach input value | - gamma: Specific heat ratio | Returns: f64 | Example: =ENG_RAYLEIGH_FROM_M_TO_RHO_RHOSTAR(2.0,1.4)")
def e_n_g_r_a_y_l_e_i_g_h_f_r_o_m_m_t_o_r_h_o_r_h_o_s_t_a_r(input_value, gamma):
    """Convenience Rayleigh path: Mach -> rho/rho* | Arguments: | - input_value: Mach input value | - gamma: Specific heat ratio | Returns: f64 | Example: =ENG_RAYLEIGH_FROM_M_TO_RHO_RHOSTAR(2.0,1.4)"""
    return invoke("device.rayleigh_calc.value", {"input_kind": "mach", "target_kind": "rho_rhostar", "input_value": input_value, "gamma": gamma})

@xloil.func(name="ENG_RAYLEIGH_FROM_M_TO_T0_T0STAR", help="Convenience Rayleigh path: Mach -> T0/T0* | Arguments: | - input_value: Mach input value | - gamma: Specific heat ratio | Returns: f64 | Example: =ENG_RAYLEIGH_FROM_M_TO_T0_T0STAR(2.0,1.4)")
def e_n_g_r_a_y_l_e_i_g_h_f_r_o_m_m_t_o_t0_t0_s_t_a_r(input_value, gamma):
    """Convenience Rayleigh path: Mach -> T0/T0* | Arguments: | - input_value: Mach input value | - gamma: Specific heat ratio | Returns: f64 | Example: =ENG_RAYLEIGH_FROM_M_TO_T0_T0STAR(2.0,1.4)"""
    return invoke("device.rayleigh_calc.value", {"input_kind": "mach", "target_kind": "t0_t0star", "input_value": input_value, "gamma": gamma})

@xloil.func(name="ENG_RAYLEIGH_FROM_M_TO_T_TSTAR", help="Convenience Rayleigh path: Mach -> T/T* | Arguments: | - input_value: Mach input value | - gamma: Specific heat ratio | Returns: f64 | Example: =ENG_RAYLEIGH_FROM_M_TO_T_TSTAR(2.0,1.4)")
def e_n_g_r_a_y_l_e_i_g_h_f_r_o_m_m_t_o_t_t_s_t_a_r(input_value, gamma):
    """Convenience Rayleigh path: Mach -> T/T* | Arguments: | - input_value: Mach input value | - gamma: Specific heat ratio | Returns: f64 | Example: =ENG_RAYLEIGH_FROM_M_TO_T_TSTAR(2.0,1.4)"""
    return invoke("device.rayleigh_calc.value", {"input_kind": "mach", "target_kind": "t_tstar", "input_value": input_value, "gamma": gamma})

@xloil.func(name="ENG_RAYLEIGH_FROM_M_TO_V_VSTAR", help="Convenience Rayleigh path: Mach -> V/V* | Arguments: | - input_value: Mach input value | - gamma: Specific heat ratio | Returns: f64 | Example: =ENG_RAYLEIGH_FROM_M_TO_V_VSTAR(2.0,1.4)")
def e_n_g_r_a_y_l_e_i_g_h_f_r_o_m_m_t_o_v_v_s_t_a_r(input_value, gamma):
    """Convenience Rayleigh path: Mach -> V/V* | Arguments: | - input_value: Mach input value | - gamma: Specific heat ratio | Returns: f64 | Example: =ENG_RAYLEIGH_FROM_M_TO_V_VSTAR(2.0,1.4)"""
    return invoke("device.rayleigh_calc.value", {"input_kind": "mach", "target_kind": "v_vstar", "input_value": input_value, "gamma": gamma})

@xloil.func(name="ENG_RAYLEIGH_FROM_P0_P0STAR_TO_M", help="Convenience Rayleigh path: p0/p0* -> Mach (branch required) | Arguments: | - input_value: Input ratio value | - gamma: Specific heat ratio | - branch: Subsonic or supersonic branch | Returns: f64 | Example: =ENG_RAYLEIGH_FROM_P0_P0STAR_TO_M(1.1140525032,1.4,'subsonic')")
def e_n_g_r_a_y_l_e_i_g_h_f_r_o_m_p0_p0_s_t_a_r_t_o_m(input_value, gamma, branch=""):
    """Convenience Rayleigh path: p0/p0* -> Mach (branch required) | Arguments: | - input_value: Input ratio value | - gamma: Specific heat ratio | - branch: Subsonic or supersonic branch | Returns: f64 | Example: =ENG_RAYLEIGH_FROM_P0_P0STAR_TO_M(1.1140525032,1.4,'subsonic')"""
    return invoke("device.rayleigh_calc.value", {"input_kind": "p0_p0star", "target_kind": "mach", "input_value": input_value, "gamma": gamma, **({"branch": branch} if branch not in (None, "") else {})})

@xloil.func(name="ENG_RAYLEIGH_FROM_T0_T0STAR_TO_M", help="Convenience Rayleigh path: T0/T0* -> Mach (branch required) | Arguments: | - input_value: Input ratio value | - gamma: Specific heat ratio | - branch: Subsonic or supersonic branch | Returns: f64 | Example: =ENG_RAYLEIGH_FROM_T0_T0STAR_TO_M(0.7933884298,1.4,'supersonic')")
def e_n_g_r_a_y_l_e_i_g_h_f_r_o_m_t0_t0_s_t_a_r_t_o_m(input_value, gamma, branch=""):
    """Convenience Rayleigh path: T0/T0* -> Mach (branch required) | Arguments: | - input_value: Input ratio value | - gamma: Specific heat ratio | - branch: Subsonic or supersonic branch | Returns: f64 | Example: =ENG_RAYLEIGH_FROM_T0_T0STAR_TO_M(0.7933884298,1.4,'supersonic')"""
    return invoke("device.rayleigh_calc.value", {"input_kind": "t0_t0star", "target_kind": "mach", "input_value": input_value, "gamma": gamma, **({"branch": branch} if branch not in (None, "") else {})})

@xloil.func(name="ENG_RAYLEIGH_FROM_T_TSTAR_TO_M", help="Convenience Rayleigh path: T/T* -> Mach (branch required) | Arguments: | - input_value: Input ratio value | - gamma: Specific heat ratio | - branch: Subsonic or supersonic branch | Returns: f64 | Example: =ENG_RAYLEIGH_FROM_T_TSTAR_TO_M(0.7901234568,1.4,'subsonic')")
def e_n_g_r_a_y_l_e_i_g_h_f_r_o_m_t_t_s_t_a_r_t_o_m(input_value, gamma, branch=""):
    """Convenience Rayleigh path: T/T* -> Mach (branch required) | Arguments: | - input_value: Input ratio value | - gamma: Specific heat ratio | - branch: Subsonic or supersonic branch | Returns: f64 | Example: =ENG_RAYLEIGH_FROM_T_TSTAR_TO_M(0.7901234568,1.4,'subsonic')"""
    return invoke("device.rayleigh_calc.value", {"input_kind": "t_tstar", "target_kind": "mach", "input_value": input_value, "gamma": gamma, **({"branch": branch} if branch not in (None, "") else {})})

@xloil.func(name="ENG_RAYLEIGH_PATH_TEXT", help="Rayleigh-flow calculator helper: compact step trace text | Arguments: | - input_kind: Input kind (mach, t_tstar, p_pstar, rho_rhostar, t0_t0star, p0_p0star, v_vstar) | - input_value: Input value | - target_kind: Target kind (mach, t_tstar, p_pstar, rho_rhostar, t0_t0star, p0_p0star, v_vstar) | - gamma: Specific heat ratio | - branch: Subsonic/supersonic branch for branch-sensitive inverse paths | Returns: str | Example: =ENG_RAYLEIGH_PATH_TEXT('t_tstar',0.7901234568,'mach',1.4,'subsonic')")
def e_n_g_r_a_y_l_e_i_g_h_p_a_t_h_t_e_x_t(input_kind, input_value, target_kind, gamma, branch=""):
    """Rayleigh-flow calculator helper: compact step trace text | Arguments: | - input_kind: Input kind (mach, t_tstar, p_pstar, rho_rhostar, t0_t0star, p0_p0star, v_vstar) | - input_value: Input value | - target_kind: Target kind (mach, t_tstar, p_pstar, rho_rhostar, t0_t0star, p0_p0star, v_vstar) | - gamma: Specific heat ratio | - branch: Subsonic/supersonic branch for branch-sensitive inverse paths | Returns: str | Example: =ENG_RAYLEIGH_PATH_TEXT('t_tstar',0.7901234568,'mach',1.4,'subsonic')"""
    return invoke("device.rayleigh_calc.path_text", {"input_kind": input_kind, "input_value": input_value, "target_kind": target_kind, "gamma": gamma, **({"branch": branch} if branch not in (None, "") else {})})

@xloil.func(name="ENG_RAYLEIGH_PIVOT_MACH", help="Rayleigh-flow calculator helper: return resolved pivot Mach | Arguments: | - input_kind: Input kind (mach, t_tstar, p_pstar, rho_rhostar, t0_t0star, p0_p0star, v_vstar) | - input_value: Input value | - target_kind: Target kind (mach, t_tstar, p_pstar, rho_rhostar, t0_t0star, p0_p0star, v_vstar) | - gamma: Specific heat ratio | - branch: Subsonic/supersonic branch for branch-sensitive inverse paths | Returns: f64 | Example: =ENG_RAYLEIGH_PIVOT_MACH('t0_t0star',0.7933884298,'mach',1.4,'supersonic')")
def e_n_g_r_a_y_l_e_i_g_h_p_i_v_o_t_m_a_c_h(input_kind, input_value, target_kind, gamma, branch=""):
    """Rayleigh-flow calculator helper: return resolved pivot Mach | Arguments: | - input_kind: Input kind (mach, t_tstar, p_pstar, rho_rhostar, t0_t0star, p0_p0star, v_vstar) | - input_value: Input value | - target_kind: Target kind (mach, t_tstar, p_pstar, rho_rhostar, t0_t0star, p0_p0star, v_vstar) | - gamma: Specific heat ratio | - branch: Subsonic/supersonic branch for branch-sensitive inverse paths | Returns: f64 | Example: =ENG_RAYLEIGH_PIVOT_MACH('t0_t0star',0.7933884298,'mach',1.4,'supersonic')"""
    return invoke("device.rayleigh_calc.pivot_mach", {"input_kind": input_kind, "input_value": input_value, "target_kind": target_kind, "gamma": gamma, **({"branch": branch} if branch not in (None, "") else {})})

@xloil.func(name="ENG_EQUATION_ASCII", help="Read ASCII display form for an equation | Arguments: | - path_id: Equation path id | Returns: str | Example: =ENG_EQUATION_ASCII('fluids.reynolds_number')")
def e_n_g_e_q_u_a_t_i_o_n_a_s_c_i_i(path_id):
    """Read ASCII display form for an equation | Arguments: | - path_id: Equation path id | Returns: str | Example: =ENG_EQUATION_ASCII('fluids.reynolds_number')"""
    return invoke("equation.ascii", {"path_id": path_id})

@xloil.func(name="ENG_EQUATION_BRANCHES_TABLE", help="Read equation branch table rows [branch, is_preferred] | Arguments: | - path_id: Equation path id | Returns: list[list] | Example: =ENG_EQUATION_BRANCHES_TABLE('compressible.area_mach')")
def e_n_g_e_q_u_a_t_i_o_n_b_r_a_n_c_h_e_s_t_a_b_l_e(path_id):
    """Read equation branch table rows [branch, is_preferred] | Arguments: | - path_id: Equation path id | Returns: list[list] | Example: =ENG_EQUATION_BRANCHES_TABLE('compressible.area_mach')"""
    return invoke("equation.branches.table", {"path_id": path_id})

@xloil.func(name="ENG_EQUATION_BRANCHES_TEXT", help="Read equation branch names as delimited text | Arguments: | - path_id: Equation path id | Returns: str | Example: =ENG_EQUATION_BRANCHES_TEXT('compressible.area_mach')")
def e_n_g_e_q_u_a_t_i_o_n_b_r_a_n_c_h_e_s_t_e_x_t(path_id):
    """Read equation branch names as delimited text | Arguments: | - path_id: Equation path id | Returns: str | Example: =ENG_EQUATION_BRANCHES_TEXT('compressible.area_mach')"""
    return invoke("equation.branches.text", {"path_id": path_id})

@xloil.func(name="ENG_COMPRESSIBLE_AREA_MACH_M", help="Solve Isentropic Area-Mach Relation for M | Arguments: | - area_ratio: Area ratio | - gamma: Specific heat ratio | - branch: Optional branch selection. Supported: subsonic, supersonic | Returns: f64 | Example: =ENG_COMPRESSIBLE_AREA_MACH_M('...','...','...')")
def e_n_g_c_o_m_p_r_e_s_s_i_b_l_e_a_r_e_a_m_a_c_h_m(area_ratio, gamma, branch=""):
    """Solve Isentropic Area-Mach Relation for M | Arguments: | - area_ratio: Area ratio | - gamma: Specific heat ratio | - branch: Optional branch selection. Supported: subsonic, supersonic | Returns: f64 | Example: =ENG_COMPRESSIBLE_AREA_MACH_M('...','...','...')"""
    return invoke("equation.solve", {"path_id": "compressible.area_mach", "target": "M", "area_ratio": area_ratio, "gamma": gamma, **({"branch": branch} if branch not in (None, "") else {})})

@xloil.func(name="ENG_COMPRESSIBLE_AREA_MACH_AREA_RATIO", help="Solve Isentropic Area-Mach Relation for area_ratio | Arguments: | - m: Mach number | - gamma: Specific heat ratio | - branch: Optional branch selection. Supported: subsonic, supersonic | Returns: f64 | Example: =ENG_COMPRESSIBLE_AREA_MACH_AREA_RATIO('...','...','...')")
def e_n_g_c_o_m_p_r_e_s_s_i_b_l_e_a_r_e_a_m_a_c_h_a_r_e_a_r_a_t_i_o(m, gamma, branch=""):
    """Solve Isentropic Area-Mach Relation for area_ratio | Arguments: | - m: Mach number | - gamma: Specific heat ratio | - branch: Optional branch selection. Supported: subsonic, supersonic | Returns: f64 | Example: =ENG_COMPRESSIBLE_AREA_MACH_AREA_RATIO('...','...','...')"""
    return invoke("equation.solve", {"path_id": "compressible.area_mach", "target": "area_ratio", "M": m, "gamma": gamma, **({"branch": branch} if branch not in (None, "") else {})})

@xloil.func(name="ENG_COMPRESSIBLE_CHOKED_MASS_FLUX_G_STAR", help="Solve Choked Mass Flux for G_star | Arguments: | - p0: Stagnation pressure | - t0: Stagnation temperature | - gamma: Specific heat ratio | - r: Gas constant | Returns: f64 | Example: =ENG_COMPRESSIBLE_CHOKED_MASS_FLUX_G_STAR('...','...','...','...')")
def e_n_g_c_o_m_p_r_e_s_s_i_b_l_e_c_h_o_k_e_d_m_a_s_s_f_l_u_x_g_s_t_a_r(p0, t0, gamma, r):
    """Solve Choked Mass Flux for G_star | Arguments: | - p0: Stagnation pressure | - t0: Stagnation temperature | - gamma: Specific heat ratio | - r: Gas constant | Returns: f64 | Example: =ENG_COMPRESSIBLE_CHOKED_MASS_FLUX_G_STAR('...','...','...','...')"""
    return invoke("equation.solve", {"path_id": "compressible.choked_mass_flux", "target": "G_star", "p0": p0, "T0": t0, "gamma": gamma, "R": r})

@xloil.func(name="ENG_COMPRESSIBLE_FANNO_DENSITY_RATIO_M", help="Solve Fanno Density Ratio for M | Arguments: | - rho_rhostar: Density ratio to star state | - gamma: Specific heat ratio | - branch: Optional branch selection. Supported: subsonic, supersonic | Returns: f64 | Example: =ENG_COMPRESSIBLE_FANNO_DENSITY_RATIO_M('...','...','...')")
def e_n_g_c_o_m_p_r_e_s_s_i_b_l_e_f_a_n_n_o_d_e_n_s_i_t_y_r_a_t_i_o_m(rho_rhostar, gamma, branch=""):
    """Solve Fanno Density Ratio for M | Arguments: | - rho_rhostar: Density ratio to star state | - gamma: Specific heat ratio | - branch: Optional branch selection. Supported: subsonic, supersonic | Returns: f64 | Example: =ENG_COMPRESSIBLE_FANNO_DENSITY_RATIO_M('...','...','...')"""
    return invoke("equation.solve", {"path_id": "compressible.fanno_density_ratio", "target": "M", "rho_rhostar": rho_rhostar, "gamma": gamma, **({"branch": branch} if branch not in (None, "") else {})})

@xloil.func(name="ENG_COMPRESSIBLE_FANNO_DENSITY_RATIO_RHO_RHOSTAR", help="Solve Fanno Density Ratio for rho_rhostar | Arguments: | - m: Mach number | - gamma: Specific heat ratio | - branch: Optional branch selection. Supported: subsonic, supersonic | Returns: f64 | Example: =ENG_COMPRESSIBLE_FANNO_DENSITY_RATIO_RHO_RHOSTAR('...','...','...')")
def e_n_g_c_o_m_p_r_e_s_s_i_b_l_e_f_a_n_n_o_d_e_n_s_i_t_y_r_a_t_i_o_r_h_o_r_h_o_s_t_a_r(m, gamma, branch=""):
    """Solve Fanno Density Ratio for rho_rhostar | Arguments: | - m: Mach number | - gamma: Specific heat ratio | - branch: Optional branch selection. Supported: subsonic, supersonic | Returns: f64 | Example: =ENG_COMPRESSIBLE_FANNO_DENSITY_RATIO_RHO_RHOSTAR('...','...','...')"""
    return invoke("equation.solve", {"path_id": "compressible.fanno_density_ratio", "target": "rho_rhostar", "M": m, "gamma": gamma, **({"branch": branch} if branch not in (None, "") else {})})

@xloil.func(name="ENG_COMPRESSIBLE_FANNO_FRICTION_PARAMETER_M", help="Solve Fanno Friction Length Parameter for M | Arguments: | - four_flstar_d: Fanno friction length parameter | - gamma: Specific heat ratio | - branch: Optional branch selection. Supported: subsonic, supersonic | Returns: f64 | Example: =ENG_COMPRESSIBLE_FANNO_FRICTION_PARAMETER_M('...','...','...')")
def e_n_g_c_o_m_p_r_e_s_s_i_b_l_e_f_a_n_n_o_f_r_i_c_t_i_o_n_p_a_r_a_m_e_t_e_r_m(four_flstar_d, gamma, branch=""):
    """Solve Fanno Friction Length Parameter for M | Arguments: | - four_flstar_d: Fanno friction length parameter | - gamma: Specific heat ratio | - branch: Optional branch selection. Supported: subsonic, supersonic | Returns: f64 | Example: =ENG_COMPRESSIBLE_FANNO_FRICTION_PARAMETER_M('...','...','...')"""
    return invoke("equation.solve", {"path_id": "compressible.fanno_friction_parameter", "target": "M", "four_flstar_d": four_flstar_d, "gamma": gamma, **({"branch": branch} if branch not in (None, "") else {})})

@xloil.func(name="ENG_COMPRESSIBLE_FANNO_FRICTION_PARAMETER_FOUR_FLSTAR_D", help="Solve Fanno Friction Length Parameter for four_flstar_d | Arguments: | - m: Mach number | - gamma: Specific heat ratio | - branch: Optional branch selection. Supported: subsonic, supersonic | Returns: f64 | Example: =ENG_COMPRESSIBLE_FANNO_FRICTION_PARAMETER_FOUR_FLSTAR_D('...','...','...')")
def e_n_g_c_o_m_p_r_e_s_s_i_b_l_e_f_a_n_n_o_f_r_i_c_t_i_o_n_p_a_r_a_m_e_t_e_r_f_o_u_r_f_l_s_t_a_r_d(m, gamma, branch=""):
    """Solve Fanno Friction Length Parameter for four_flstar_d | Arguments: | - m: Mach number | - gamma: Specific heat ratio | - branch: Optional branch selection. Supported: subsonic, supersonic | Returns: f64 | Example: =ENG_COMPRESSIBLE_FANNO_FRICTION_PARAMETER_FOUR_FLSTAR_D('...','...','...')"""
    return invoke("equation.solve", {"path_id": "compressible.fanno_friction_parameter", "target": "four_flstar_d", "M": m, "gamma": gamma, **({"branch": branch} if branch not in (None, "") else {})})

@xloil.func(name="ENG_COMPRESSIBLE_FANNO_PRESSURE_RATIO_M", help="Solve Fanno Pressure Ratio for M | Arguments: | - p_pstar: Pressure ratio to star state | - gamma: Specific heat ratio | - branch: Optional branch selection. Supported: subsonic, supersonic | Returns: f64 | Example: =ENG_COMPRESSIBLE_FANNO_PRESSURE_RATIO_M('...','...','...')")
def e_n_g_c_o_m_p_r_e_s_s_i_b_l_e_f_a_n_n_o_p_r_e_s_s_u_r_e_r_a_t_i_o_m(p_pstar, gamma, branch=""):
    """Solve Fanno Pressure Ratio for M | Arguments: | - p_pstar: Pressure ratio to star state | - gamma: Specific heat ratio | - branch: Optional branch selection. Supported: subsonic, supersonic | Returns: f64 | Example: =ENG_COMPRESSIBLE_FANNO_PRESSURE_RATIO_M('...','...','...')"""
    return invoke("equation.solve", {"path_id": "compressible.fanno_pressure_ratio", "target": "M", "p_pstar": p_pstar, "gamma": gamma, **({"branch": branch} if branch not in (None, "") else {})})

@xloil.func(name="ENG_COMPRESSIBLE_FANNO_PRESSURE_RATIO_P_PSTAR", help="Solve Fanno Pressure Ratio for p_pstar | Arguments: | - m: Mach number | - gamma: Specific heat ratio | - branch: Optional branch selection. Supported: subsonic, supersonic | Returns: f64 | Example: =ENG_COMPRESSIBLE_FANNO_PRESSURE_RATIO_P_PSTAR('...','...','...')")
def e_n_g_c_o_m_p_r_e_s_s_i_b_l_e_f_a_n_n_o_p_r_e_s_s_u_r_e_r_a_t_i_o_p_p_s_t_a_r(m, gamma, branch=""):
    """Solve Fanno Pressure Ratio for p_pstar | Arguments: | - m: Mach number | - gamma: Specific heat ratio | - branch: Optional branch selection. Supported: subsonic, supersonic | Returns: f64 | Example: =ENG_COMPRESSIBLE_FANNO_PRESSURE_RATIO_P_PSTAR('...','...','...')"""
    return invoke("equation.solve", {"path_id": "compressible.fanno_pressure_ratio", "target": "p_pstar", "M": m, "gamma": gamma, **({"branch": branch} if branch not in (None, "") else {})})

@xloil.func(name="ENG_COMPRESSIBLE_FANNO_STAGNATION_PRESSURE_RATIO_M", help="Solve Fanno Stagnation Pressure Ratio for M | Arguments: | - p0_p0star: Stagnation pressure ratio to star state | - gamma: Specific heat ratio | - branch: Optional branch selection. Supported: subsonic, supersonic | Returns: f64 | Example: =ENG_COMPRESSIBLE_FANNO_STAGNATION_PRESSURE_RATIO_M('...','...','...')")
def e_n_g_c_o_m_p_r_e_s_s_i_b_l_e_f_a_n_n_o_s_t_a_g_n_a_t_i_o_n_p_r_e_s_s_u_r_e_r_a_t_i_o_m(p0_p0star, gamma, branch=""):
    """Solve Fanno Stagnation Pressure Ratio for M | Arguments: | - p0_p0star: Stagnation pressure ratio to star state | - gamma: Specific heat ratio | - branch: Optional branch selection. Supported: subsonic, supersonic | Returns: f64 | Example: =ENG_COMPRESSIBLE_FANNO_STAGNATION_PRESSURE_RATIO_M('...','...','...')"""
    return invoke("equation.solve", {"path_id": "compressible.fanno_stagnation_pressure_ratio", "target": "M", "p0_p0star": p0_p0star, "gamma": gamma, **({"branch": branch} if branch not in (None, "") else {})})

@xloil.func(name="ENG_COMPRESSIBLE_FANNO_STAGNATION_PRESSURE_RATIO_P0_P0STAR", help="Solve Fanno Stagnation Pressure Ratio for p0_p0star | Arguments: | - m: Mach number | - gamma: Specific heat ratio | - branch: Optional branch selection. Supported: subsonic, supersonic | Returns: f64 | Example: =ENG_COMPRESSIBLE_FANNO_STAGNATION_PRESSURE_RATIO_P0_P0STAR('...','...','...')")
def e_n_g_c_o_m_p_r_e_s_s_i_b_l_e_f_a_n_n_o_s_t_a_g_n_a_t_i_o_n_p_r_e_s_s_u_r_e_r_a_t_i_o_p0_p0_s_t_a_r(m, gamma, branch=""):
    """Solve Fanno Stagnation Pressure Ratio for p0_p0star | Arguments: | - m: Mach number | - gamma: Specific heat ratio | - branch: Optional branch selection. Supported: subsonic, supersonic | Returns: f64 | Example: =ENG_COMPRESSIBLE_FANNO_STAGNATION_PRESSURE_RATIO_P0_P0STAR('...','...','...')"""
    return invoke("equation.solve", {"path_id": "compressible.fanno_stagnation_pressure_ratio", "target": "p0_p0star", "M": m, "gamma": gamma, **({"branch": branch} if branch not in (None, "") else {})})

@xloil.func(name="ENG_COMPRESSIBLE_FANNO_TEMPERATURE_RATIO_M", help="Solve Fanno Temperature Ratio for M | Arguments: | - t_tstar: Temperature ratio to star state | - gamma: Specific heat ratio | - branch: Optional branch selection. Supported: subsonic, supersonic | Returns: f64 | Example: =ENG_COMPRESSIBLE_FANNO_TEMPERATURE_RATIO_M('...','...','...')")
def e_n_g_c_o_m_p_r_e_s_s_i_b_l_e_f_a_n_n_o_t_e_m_p_e_r_a_t_u_r_e_r_a_t_i_o_m(t_tstar, gamma, branch=""):
    """Solve Fanno Temperature Ratio for M | Arguments: | - t_tstar: Temperature ratio to star state | - gamma: Specific heat ratio | - branch: Optional branch selection. Supported: subsonic, supersonic | Returns: f64 | Example: =ENG_COMPRESSIBLE_FANNO_TEMPERATURE_RATIO_M('...','...','...')"""
    return invoke("equation.solve", {"path_id": "compressible.fanno_temperature_ratio", "target": "M", "t_tstar": t_tstar, "gamma": gamma, **({"branch": branch} if branch not in (None, "") else {})})

@xloil.func(name="ENG_COMPRESSIBLE_FANNO_TEMPERATURE_RATIO_T_TSTAR", help="Solve Fanno Temperature Ratio for t_tstar | Arguments: | - m: Mach number | - gamma: Specific heat ratio | - branch: Optional branch selection. Supported: subsonic, supersonic | Returns: f64 | Example: =ENG_COMPRESSIBLE_FANNO_TEMPERATURE_RATIO_T_TSTAR('...','...','...')")
def e_n_g_c_o_m_p_r_e_s_s_i_b_l_e_f_a_n_n_o_t_e_m_p_e_r_a_t_u_r_e_r_a_t_i_o_t_t_s_t_a_r(m, gamma, branch=""):
    """Solve Fanno Temperature Ratio for t_tstar | Arguments: | - m: Mach number | - gamma: Specific heat ratio | - branch: Optional branch selection. Supported: subsonic, supersonic | Returns: f64 | Example: =ENG_COMPRESSIBLE_FANNO_TEMPERATURE_RATIO_T_TSTAR('...','...','...')"""
    return invoke("equation.solve", {"path_id": "compressible.fanno_temperature_ratio", "target": "t_tstar", "M": m, "gamma": gamma, **({"branch": branch} if branch not in (None, "") else {})})

@xloil.func(name="ENG_COMPRESSIBLE_FANNO_VELOCITY_RATIO_M", help="Solve Fanno Velocity Ratio for M | Arguments: | - v_vstar: Velocity ratio to star state | - gamma: Specific heat ratio | - branch: Optional branch selection. Supported: subsonic, supersonic | Returns: f64 | Example: =ENG_COMPRESSIBLE_FANNO_VELOCITY_RATIO_M('...','...','...')")
def e_n_g_c_o_m_p_r_e_s_s_i_b_l_e_f_a_n_n_o_v_e_l_o_c_i_t_y_r_a_t_i_o_m(v_vstar, gamma, branch=""):
    """Solve Fanno Velocity Ratio for M | Arguments: | - v_vstar: Velocity ratio to star state | - gamma: Specific heat ratio | - branch: Optional branch selection. Supported: subsonic, supersonic | Returns: f64 | Example: =ENG_COMPRESSIBLE_FANNO_VELOCITY_RATIO_M('...','...','...')"""
    return invoke("equation.solve", {"path_id": "compressible.fanno_velocity_ratio", "target": "M", "v_vstar": v_vstar, "gamma": gamma, **({"branch": branch} if branch not in (None, "") else {})})

@xloil.func(name="ENG_COMPRESSIBLE_FANNO_VELOCITY_RATIO_V_VSTAR", help="Solve Fanno Velocity Ratio for v_vstar | Arguments: | - m: Mach number | - gamma: Specific heat ratio | - branch: Optional branch selection. Supported: subsonic, supersonic | Returns: f64 | Example: =ENG_COMPRESSIBLE_FANNO_VELOCITY_RATIO_V_VSTAR('...','...','...')")
def e_n_g_c_o_m_p_r_e_s_s_i_b_l_e_f_a_n_n_o_v_e_l_o_c_i_t_y_r_a_t_i_o_v_v_s_t_a_r(m, gamma, branch=""):
    """Solve Fanno Velocity Ratio for v_vstar | Arguments: | - m: Mach number | - gamma: Specific heat ratio | - branch: Optional branch selection. Supported: subsonic, supersonic | Returns: f64 | Example: =ENG_COMPRESSIBLE_FANNO_VELOCITY_RATIO_V_VSTAR('...','...','...')"""
    return invoke("equation.solve", {"path_id": "compressible.fanno_velocity_ratio", "target": "v_vstar", "M": m, "gamma": gamma, **({"branch": branch} if branch not in (None, "") else {})})

@xloil.func(name="ENG_COMPRESSIBLE_ISENTROPIC_DENSITY_RATIO_M", help="Solve Isentropic Density Ratio for M | Arguments: | - rho_rho0: Static-to-stagnation density ratio | - gamma: Specific heat ratio | Returns: f64 | Example: =ENG_COMPRESSIBLE_ISENTROPIC_DENSITY_RATIO_M('...','...')")
def e_n_g_c_o_m_p_r_e_s_s_i_b_l_e_i_s_e_n_t_r_o_p_i_c_d_e_n_s_i_t_y_r_a_t_i_o_m(rho_rho0, gamma):
    """Solve Isentropic Density Ratio for M | Arguments: | - rho_rho0: Static-to-stagnation density ratio | - gamma: Specific heat ratio | Returns: f64 | Example: =ENG_COMPRESSIBLE_ISENTROPIC_DENSITY_RATIO_M('...','...')"""
    return invoke("equation.solve", {"path_id": "compressible.isentropic_density_ratio", "target": "M", "rho_rho0": rho_rho0, "gamma": gamma})

@xloil.func(name="ENG_COMPRESSIBLE_ISENTROPIC_DENSITY_RATIO_RHO_RHO0", help="Solve Isentropic Density Ratio for rho_rho0 | Arguments: | - m: Mach number | - gamma: Specific heat ratio | Returns: f64 | Example: =ENG_COMPRESSIBLE_ISENTROPIC_DENSITY_RATIO_RHO_RHO0('...','...')")
def e_n_g_c_o_m_p_r_e_s_s_i_b_l_e_i_s_e_n_t_r_o_p_i_c_d_e_n_s_i_t_y_r_a_t_i_o_r_h_o_r_h_o0(m, gamma):
    """Solve Isentropic Density Ratio for rho_rho0 | Arguments: | - m: Mach number | - gamma: Specific heat ratio | Returns: f64 | Example: =ENG_COMPRESSIBLE_ISENTROPIC_DENSITY_RATIO_RHO_RHO0('...','...')"""
    return invoke("equation.solve", {"path_id": "compressible.isentropic_density_ratio", "target": "rho_rho0", "M": m, "gamma": gamma})

@xloil.func(name="ENG_COMPRESSIBLE_ISENTROPIC_PRESSURE_RATIO_M", help="Solve Isentropic Pressure Ratio for M | Arguments: | - p_p0: Static-to-stagnation pressure ratio | - gamma: Specific heat ratio | Returns: f64 | Example: =ENG_COMPRESSIBLE_ISENTROPIC_PRESSURE_RATIO_M('...','...')")
def e_n_g_c_o_m_p_r_e_s_s_i_b_l_e_i_s_e_n_t_r_o_p_i_c_p_r_e_s_s_u_r_e_r_a_t_i_o_m(p_p0, gamma):
    """Solve Isentropic Pressure Ratio for M | Arguments: | - p_p0: Static-to-stagnation pressure ratio | - gamma: Specific heat ratio | Returns: f64 | Example: =ENG_COMPRESSIBLE_ISENTROPIC_PRESSURE_RATIO_M('...','...')"""
    return invoke("equation.solve", {"path_id": "compressible.isentropic_pressure_ratio", "target": "M", "p_p0": p_p0, "gamma": gamma})

@xloil.func(name="ENG_COMPRESSIBLE_ISENTROPIC_PRESSURE_RATIO_P_P0", help="Solve Isentropic Pressure Ratio for p_p0 | Arguments: | - m: Mach number | - gamma: Specific heat ratio | Returns: f64 | Example: =ENG_COMPRESSIBLE_ISENTROPIC_PRESSURE_RATIO_P_P0('...','...')")
def e_n_g_c_o_m_p_r_e_s_s_i_b_l_e_i_s_e_n_t_r_o_p_i_c_p_r_e_s_s_u_r_e_r_a_t_i_o_p_p0(m, gamma):
    """Solve Isentropic Pressure Ratio for p_p0 | Arguments: | - m: Mach number | - gamma: Specific heat ratio | Returns: f64 | Example: =ENG_COMPRESSIBLE_ISENTROPIC_PRESSURE_RATIO_P_P0('...','...')"""
    return invoke("equation.solve", {"path_id": "compressible.isentropic_pressure_ratio", "target": "p_p0", "M": m, "gamma": gamma})

@xloil.func(name="ENG_COMPRESSIBLE_ISENTROPIC_TEMPERATURE_RATIO_M", help="Solve Isentropic Temperature Ratio for M | Arguments: | - t_t0: Static-to-stagnation temperature ratio | - gamma: Specific heat ratio | Returns: f64 | Example: =ENG_COMPRESSIBLE_ISENTROPIC_TEMPERATURE_RATIO_M('...','...')")
def e_n_g_c_o_m_p_r_e_s_s_i_b_l_e_i_s_e_n_t_r_o_p_i_c_t_e_m_p_e_r_a_t_u_r_e_r_a_t_i_o_m(t_t0, gamma):
    """Solve Isentropic Temperature Ratio for M | Arguments: | - t_t0: Static-to-stagnation temperature ratio | - gamma: Specific heat ratio | Returns: f64 | Example: =ENG_COMPRESSIBLE_ISENTROPIC_TEMPERATURE_RATIO_M('...','...')"""
    return invoke("equation.solve", {"path_id": "compressible.isentropic_temperature_ratio", "target": "M", "T_T0": t_t0, "gamma": gamma})

@xloil.func(name="ENG_COMPRESSIBLE_ISENTROPIC_TEMPERATURE_RATIO_T_T0", help="Solve Isentropic Temperature Ratio for T_T0 | Arguments: | - m: Mach number | - gamma: Specific heat ratio | Returns: f64 | Example: =ENG_COMPRESSIBLE_ISENTROPIC_TEMPERATURE_RATIO_T_T0('...','...')")
def e_n_g_c_o_m_p_r_e_s_s_i_b_l_e_i_s_e_n_t_r_o_p_i_c_t_e_m_p_e_r_a_t_u_r_e_r_a_t_i_o_t_t0(m, gamma):
    """Solve Isentropic Temperature Ratio for T_T0 | Arguments: | - m: Mach number | - gamma: Specific heat ratio | Returns: f64 | Example: =ENG_COMPRESSIBLE_ISENTROPIC_TEMPERATURE_RATIO_T_T0('...','...')"""
    return invoke("equation.solve", {"path_id": "compressible.isentropic_temperature_ratio", "target": "T_T0", "M": m, "gamma": gamma})

@xloil.func(name="ENG_COMPRESSIBLE_MACH_ANGLE_M", help="Solve Mach Angle for M | Arguments: | - mu: Mach angle | Returns: f64 | Example: =ENG_COMPRESSIBLE_MACH_ANGLE_M('...')")
def e_n_g_c_o_m_p_r_e_s_s_i_b_l_e_m_a_c_h_a_n_g_l_e_m(mu):
    """Solve Mach Angle for M | Arguments: | - mu: Mach angle | Returns: f64 | Example: =ENG_COMPRESSIBLE_MACH_ANGLE_M('...')"""
    return invoke("equation.solve", {"path_id": "compressible.mach_angle", "target": "M", "mu": mu})

@xloil.func(name="ENG_COMPRESSIBLE_MACH_ANGLE_MU", help="Solve Mach Angle for mu | Arguments: | - m: Mach number | Returns: f64 | Example: =ENG_COMPRESSIBLE_MACH_ANGLE_MU('...')")
def e_n_g_c_o_m_p_r_e_s_s_i_b_l_e_m_a_c_h_a_n_g_l_e_m_u(m):
    """Solve Mach Angle for mu | Arguments: | - m: Mach number | Returns: f64 | Example: =ENG_COMPRESSIBLE_MACH_ANGLE_MU('...')"""
    return invoke("equation.solve", {"path_id": "compressible.mach_angle", "target": "mu", "M": m})

@xloil.func(name="ENG_COMPRESSIBLE_NORMAL_SHOCK_DENSITY_RATIO_M1", help="Solve Normal Shock Density Ratio for M1 | Arguments: | - rho2_rho1: Density ratio | - gamma: Specific heat ratio | Returns: f64 | Example: =ENG_COMPRESSIBLE_NORMAL_SHOCK_DENSITY_RATIO_M1('...','...')")
def e_n_g_c_o_m_p_r_e_s_s_i_b_l_e_n_o_r_m_a_l_s_h_o_c_k_d_e_n_s_i_t_y_r_a_t_i_o_m1(rho2_rho1, gamma):
    """Solve Normal Shock Density Ratio for M1 | Arguments: | - rho2_rho1: Density ratio | - gamma: Specific heat ratio | Returns: f64 | Example: =ENG_COMPRESSIBLE_NORMAL_SHOCK_DENSITY_RATIO_M1('...','...')"""
    return invoke("equation.solve", {"path_id": "compressible.normal_shock_density_ratio", "target": "M1", "rho2_rho1": rho2_rho1, "gamma": gamma})

@xloil.func(name="ENG_COMPRESSIBLE_NORMAL_SHOCK_DENSITY_RATIO_RHO2_RHO1", help="Solve Normal Shock Density Ratio for rho2_rho1 | Arguments: | - m1: Upstream Mach number | - gamma: Specific heat ratio | Returns: f64 | Example: =ENG_COMPRESSIBLE_NORMAL_SHOCK_DENSITY_RATIO_RHO2_RHO1('...','...')")
def e_n_g_c_o_m_p_r_e_s_s_i_b_l_e_n_o_r_m_a_l_s_h_o_c_k_d_e_n_s_i_t_y_r_a_t_i_o_r_h_o2_r_h_o1(m1, gamma):
    """Solve Normal Shock Density Ratio for rho2_rho1 | Arguments: | - m1: Upstream Mach number | - gamma: Specific heat ratio | Returns: f64 | Example: =ENG_COMPRESSIBLE_NORMAL_SHOCK_DENSITY_RATIO_RHO2_RHO1('...','...')"""
    return invoke("equation.solve", {"path_id": "compressible.normal_shock_density_ratio", "target": "rho2_rho1", "M1": m1, "gamma": gamma})

@xloil.func(name="ENG_COMPRESSIBLE_NORMAL_SHOCK_M2_M1", help="Solve Normal Shock Downstream Mach Number for M1 | Arguments: | - m2: Downstream Mach number | - gamma: Specific heat ratio | Returns: f64 | Example: =ENG_COMPRESSIBLE_NORMAL_SHOCK_M2_M1('...','...')")
def e_n_g_c_o_m_p_r_e_s_s_i_b_l_e_n_o_r_m_a_l_s_h_o_c_k_m2_m1(m2, gamma):
    """Solve Normal Shock Downstream Mach Number for M1 | Arguments: | - m2: Downstream Mach number | - gamma: Specific heat ratio | Returns: f64 | Example: =ENG_COMPRESSIBLE_NORMAL_SHOCK_M2_M1('...','...')"""
    return invoke("equation.solve", {"path_id": "compressible.normal_shock_m2", "target": "M1", "M2": m2, "gamma": gamma})

@xloil.func(name="ENG_COMPRESSIBLE_NORMAL_SHOCK_M2_M2", help="Solve Normal Shock Downstream Mach Number for M2 | Arguments: | - m1: Upstream Mach number | - gamma: Specific heat ratio | Returns: f64 | Example: =ENG_COMPRESSIBLE_NORMAL_SHOCK_M2_M2('...','...')")
def e_n_g_c_o_m_p_r_e_s_s_i_b_l_e_n_o_r_m_a_l_s_h_o_c_k_m2_m2(m1, gamma):
    """Solve Normal Shock Downstream Mach Number for M2 | Arguments: | - m1: Upstream Mach number | - gamma: Specific heat ratio | Returns: f64 | Example: =ENG_COMPRESSIBLE_NORMAL_SHOCK_M2_M2('...','...')"""
    return invoke("equation.solve", {"path_id": "compressible.normal_shock_m2", "target": "M2", "M1": m1, "gamma": gamma})

@xloil.func(name="ENG_COMPRESSIBLE_NORMAL_SHOCK_PRESSURE_RATIO_M1", help="Solve Normal Shock Static Pressure Ratio for M1 | Arguments: | - p2_p1: Static pressure ratio | - gamma: Specific heat ratio | Returns: f64 | Example: =ENG_COMPRESSIBLE_NORMAL_SHOCK_PRESSURE_RATIO_M1('...','...')")
def e_n_g_c_o_m_p_r_e_s_s_i_b_l_e_n_o_r_m_a_l_s_h_o_c_k_p_r_e_s_s_u_r_e_r_a_t_i_o_m1(p2_p1, gamma):
    """Solve Normal Shock Static Pressure Ratio for M1 | Arguments: | - p2_p1: Static pressure ratio | - gamma: Specific heat ratio | Returns: f64 | Example: =ENG_COMPRESSIBLE_NORMAL_SHOCK_PRESSURE_RATIO_M1('...','...')"""
    return invoke("equation.solve", {"path_id": "compressible.normal_shock_pressure_ratio", "target": "M1", "p2_p1": p2_p1, "gamma": gamma})

@xloil.func(name="ENG_COMPRESSIBLE_NORMAL_SHOCK_PRESSURE_RATIO_P2_P1", help="Solve Normal Shock Static Pressure Ratio for p2_p1 | Arguments: | - m1: Upstream Mach number | - gamma: Specific heat ratio | Returns: f64 | Example: =ENG_COMPRESSIBLE_NORMAL_SHOCK_PRESSURE_RATIO_P2_P1('...','...')")
def e_n_g_c_o_m_p_r_e_s_s_i_b_l_e_n_o_r_m_a_l_s_h_o_c_k_p_r_e_s_s_u_r_e_r_a_t_i_o_p2_p1(m1, gamma):
    """Solve Normal Shock Static Pressure Ratio for p2_p1 | Arguments: | - m1: Upstream Mach number | - gamma: Specific heat ratio | Returns: f64 | Example: =ENG_COMPRESSIBLE_NORMAL_SHOCK_PRESSURE_RATIO_P2_P1('...','...')"""
    return invoke("equation.solve", {"path_id": "compressible.normal_shock_pressure_ratio", "target": "p2_p1", "M1": m1, "gamma": gamma})

@xloil.func(name="ENG_COMPRESSIBLE_NORMAL_SHOCK_STAGNATION_PRESSURE_RATIO_M1", help="Solve Normal Shock Stagnation Pressure Ratio for M1 | Arguments: | - p02_p01: Stagnation pressure ratio | - gamma: Specific heat ratio | Returns: f64 | Example: =ENG_COMPRESSIBLE_NORMAL_SHOCK_STAGNATION_PRESSURE_RATIO_M1('...','...')")
def e_n_g_c_o_m_p_r_e_s_s_i_b_l_e_n_o_r_m_a_l_s_h_o_c_k_s_t_a_g_n_a_t_i_o_n_p_r_e_s_s_u_r_e_r_a_t_i_o_m1(p02_p01, gamma):
    """Solve Normal Shock Stagnation Pressure Ratio for M1 | Arguments: | - p02_p01: Stagnation pressure ratio | - gamma: Specific heat ratio | Returns: f64 | Example: =ENG_COMPRESSIBLE_NORMAL_SHOCK_STAGNATION_PRESSURE_RATIO_M1('...','...')"""
    return invoke("equation.solve", {"path_id": "compressible.normal_shock_stagnation_pressure_ratio", "target": "M1", "p02_p01": p02_p01, "gamma": gamma})

@xloil.func(name="ENG_COMPRESSIBLE_NORMAL_SHOCK_STAGNATION_PRESSURE_RATIO_P02_P01", help="Solve Normal Shock Stagnation Pressure Ratio for p02_p01 | Arguments: | - m1: Upstream Mach number | - gamma: Specific heat ratio | Returns: f64 | Example: =ENG_COMPRESSIBLE_NORMAL_SHOCK_STAGNATION_PRESSURE_RATIO_P02_P01('...','...')")
def e_n_g_c_o_m_p_r_e_s_s_i_b_l_e_n_o_r_m_a_l_s_h_o_c_k_s_t_a_g_n_a_t_i_o_n_p_r_e_s_s_u_r_e_r_a_t_i_o_p02_p01(m1, gamma):
    """Solve Normal Shock Stagnation Pressure Ratio for p02_p01 | Arguments: | - m1: Upstream Mach number | - gamma: Specific heat ratio | Returns: f64 | Example: =ENG_COMPRESSIBLE_NORMAL_SHOCK_STAGNATION_PRESSURE_RATIO_P02_P01('...','...')"""
    return invoke("equation.solve", {"path_id": "compressible.normal_shock_stagnation_pressure_ratio", "target": "p02_p01", "M1": m1, "gamma": gamma})

@xloil.func(name="ENG_COMPRESSIBLE_NORMAL_SHOCK_TEMPERATURE_RATIO_M1", help="Solve Normal Shock Temperature Ratio for M1 | Arguments: | - t2_t1: Static temperature ratio | - gamma: Specific heat ratio | Returns: f64 | Example: =ENG_COMPRESSIBLE_NORMAL_SHOCK_TEMPERATURE_RATIO_M1('...','...')")
def e_n_g_c_o_m_p_r_e_s_s_i_b_l_e_n_o_r_m_a_l_s_h_o_c_k_t_e_m_p_e_r_a_t_u_r_e_r_a_t_i_o_m1(t2_t1, gamma):
    """Solve Normal Shock Temperature Ratio for M1 | Arguments: | - t2_t1: Static temperature ratio | - gamma: Specific heat ratio | Returns: f64 | Example: =ENG_COMPRESSIBLE_NORMAL_SHOCK_TEMPERATURE_RATIO_M1('...','...')"""
    return invoke("equation.solve", {"path_id": "compressible.normal_shock_temperature_ratio", "target": "M1", "T2_T1": t2_t1, "gamma": gamma})

@xloil.func(name="ENG_COMPRESSIBLE_NORMAL_SHOCK_TEMPERATURE_RATIO_T2_T1", help="Solve Normal Shock Temperature Ratio for T2_T1 | Arguments: | - m1: Upstream Mach number | - gamma: Specific heat ratio | Returns: f64 | Example: =ENG_COMPRESSIBLE_NORMAL_SHOCK_TEMPERATURE_RATIO_T2_T1('...','...')")
def e_n_g_c_o_m_p_r_e_s_s_i_b_l_e_n_o_r_m_a_l_s_h_o_c_k_t_e_m_p_e_r_a_t_u_r_e_r_a_t_i_o_t2_t1(m1, gamma):
    """Solve Normal Shock Temperature Ratio for T2_T1 | Arguments: | - m1: Upstream Mach number | - gamma: Specific heat ratio | Returns: f64 | Example: =ENG_COMPRESSIBLE_NORMAL_SHOCK_TEMPERATURE_RATIO_T2_T1('...','...')"""
    return invoke("equation.solve", {"path_id": "compressible.normal_shock_temperature_ratio", "target": "T2_T1", "M1": m1, "gamma": gamma})

@xloil.func(name="ENG_COMPRESSIBLE_OBLIQUE_SHOCK_M2_BETA", help="Solve Oblique Shock Downstream Mach for beta | Arguments: | - m2: Downstream Mach number | - mn2: Downstream normal Mach | - theta: Flow deflection angle | Returns: f64 | Example: =ENG_COMPRESSIBLE_OBLIQUE_SHOCK_M2_BETA('...','...','...')")
def e_n_g_c_o_m_p_r_e_s_s_i_b_l_e_o_b_l_i_q_u_e_s_h_o_c_k_m2_b_e_t_a(m2, mn2, theta):
    """Solve Oblique Shock Downstream Mach for beta | Arguments: | - m2: Downstream Mach number | - mn2: Downstream normal Mach | - theta: Flow deflection angle | Returns: f64 | Example: =ENG_COMPRESSIBLE_OBLIQUE_SHOCK_M2_BETA('...','...','...')"""
    return invoke("equation.solve", {"path_id": "compressible.oblique_shock_m2", "target": "beta", "m2": m2, "mn2": mn2, "theta": theta})

@xloil.func(name="ENG_COMPRESSIBLE_OBLIQUE_SHOCK_M2_M2", help="Solve Oblique Shock Downstream Mach for m2 | Arguments: | - mn2: Downstream normal Mach | - beta: Shock angle | - theta: Flow deflection angle | Returns: f64 | Example: =ENG_COMPRESSIBLE_OBLIQUE_SHOCK_M2_M2('...','...','...')")
def e_n_g_c_o_m_p_r_e_s_s_i_b_l_e_o_b_l_i_q_u_e_s_h_o_c_k_m2_m2(mn2, beta, theta):
    """Solve Oblique Shock Downstream Mach for m2 | Arguments: | - mn2: Downstream normal Mach | - beta: Shock angle | - theta: Flow deflection angle | Returns: f64 | Example: =ENG_COMPRESSIBLE_OBLIQUE_SHOCK_M2_M2('...','...','...')"""
    return invoke("equation.solve", {"path_id": "compressible.oblique_shock_m2", "target": "m2", "mn2": mn2, "beta": beta, "theta": theta})

@xloil.func(name="ENG_COMPRESSIBLE_OBLIQUE_SHOCK_M2_MN2", help="Solve Oblique Shock Downstream Mach for mn2 | Arguments: | - m2: Downstream Mach number | - beta: Shock angle | - theta: Flow deflection angle | Returns: f64 | Example: =ENG_COMPRESSIBLE_OBLIQUE_SHOCK_M2_MN2('...','...','...')")
def e_n_g_c_o_m_p_r_e_s_s_i_b_l_e_o_b_l_i_q_u_e_s_h_o_c_k_m2_m_n2(m2, beta, theta):
    """Solve Oblique Shock Downstream Mach for mn2 | Arguments: | - m2: Downstream Mach number | - beta: Shock angle | - theta: Flow deflection angle | Returns: f64 | Example: =ENG_COMPRESSIBLE_OBLIQUE_SHOCK_M2_MN2('...','...','...')"""
    return invoke("equation.solve", {"path_id": "compressible.oblique_shock_m2", "target": "mn2", "m2": m2, "beta": beta, "theta": theta})

@xloil.func(name="ENG_COMPRESSIBLE_OBLIQUE_SHOCK_M2_THETA", help="Solve Oblique Shock Downstream Mach for theta | Arguments: | - m2: Downstream Mach number | - mn2: Downstream normal Mach | - beta: Shock angle | Returns: f64 | Example: =ENG_COMPRESSIBLE_OBLIQUE_SHOCK_M2_THETA('...','...','...')")
def e_n_g_c_o_m_p_r_e_s_s_i_b_l_e_o_b_l_i_q_u_e_s_h_o_c_k_m2_t_h_e_t_a(m2, mn2, beta):
    """Solve Oblique Shock Downstream Mach for theta | Arguments: | - m2: Downstream Mach number | - mn2: Downstream normal Mach | - beta: Shock angle | Returns: f64 | Example: =ENG_COMPRESSIBLE_OBLIQUE_SHOCK_M2_THETA('...','...','...')"""
    return invoke("equation.solve", {"path_id": "compressible.oblique_shock_m2", "target": "theta", "m2": m2, "mn2": mn2, "beta": beta})

@xloil.func(name="ENG_COMPRESSIBLE_OBLIQUE_SHOCK_MN1_BETA", help="Solve Oblique Shock Normal Upstream Mach for beta | Arguments: | - mn1: Upstream normal Mach | - m1: Upstream Mach number | Returns: f64 | Example: =ENG_COMPRESSIBLE_OBLIQUE_SHOCK_MN1_BETA('...','...')")
def e_n_g_c_o_m_p_r_e_s_s_i_b_l_e_o_b_l_i_q_u_e_s_h_o_c_k_m_n1_b_e_t_a(mn1, m1):
    """Solve Oblique Shock Normal Upstream Mach for beta | Arguments: | - mn1: Upstream normal Mach | - m1: Upstream Mach number | Returns: f64 | Example: =ENG_COMPRESSIBLE_OBLIQUE_SHOCK_MN1_BETA('...','...')"""
    return invoke("equation.solve", {"path_id": "compressible.oblique_shock_mn1", "target": "beta", "mn1": mn1, "m1": m1})

@xloil.func(name="ENG_COMPRESSIBLE_OBLIQUE_SHOCK_MN1_M1", help="Solve Oblique Shock Normal Upstream Mach for m1 | Arguments: | - mn1: Upstream normal Mach | - beta: Shock angle | Returns: f64 | Example: =ENG_COMPRESSIBLE_OBLIQUE_SHOCK_MN1_M1('...','...')")
def e_n_g_c_o_m_p_r_e_s_s_i_b_l_e_o_b_l_i_q_u_e_s_h_o_c_k_m_n1_m1(mn1, beta):
    """Solve Oblique Shock Normal Upstream Mach for m1 | Arguments: | - mn1: Upstream normal Mach | - beta: Shock angle | Returns: f64 | Example: =ENG_COMPRESSIBLE_OBLIQUE_SHOCK_MN1_M1('...','...')"""
    return invoke("equation.solve", {"path_id": "compressible.oblique_shock_mn1", "target": "m1", "mn1": mn1, "beta": beta})

@xloil.func(name="ENG_COMPRESSIBLE_OBLIQUE_SHOCK_MN1_MN1", help="Solve Oblique Shock Normal Upstream Mach for mn1 | Arguments: | - m1: Upstream Mach number | - beta: Shock angle | Returns: f64 | Example: =ENG_COMPRESSIBLE_OBLIQUE_SHOCK_MN1_MN1('...','...')")
def e_n_g_c_o_m_p_r_e_s_s_i_b_l_e_o_b_l_i_q_u_e_s_h_o_c_k_m_n1_m_n1(m1, beta):
    """Solve Oblique Shock Normal Upstream Mach for mn1 | Arguments: | - m1: Upstream Mach number | - beta: Shock angle | Returns: f64 | Example: =ENG_COMPRESSIBLE_OBLIQUE_SHOCK_MN1_MN1('...','...')"""
    return invoke("equation.solve", {"path_id": "compressible.oblique_shock_mn1", "target": "mn1", "m1": m1, "beta": beta})

@xloil.func(name="ENG_COMPRESSIBLE_OBLIQUE_SHOCK_THETA_BETA_M_BETA", help="Solve Oblique Shock Theta-Beta-M Relation for beta | Arguments: | - theta: Flow deflection angle | - m1: Upstream Mach number | - gamma: Specific heat ratio | - branch: Optional branch selection. Supported: weak, strong | Returns: f64 | Example: =ENG_COMPRESSIBLE_OBLIQUE_SHOCK_THETA_BETA_M_BETA('...','...','...','...')")
def e_n_g_c_o_m_p_r_e_s_s_i_b_l_e_o_b_l_i_q_u_e_s_h_o_c_k_t_h_e_t_a_b_e_t_a_m_b_e_t_a(theta, m1, gamma, branch=""):
    """Solve Oblique Shock Theta-Beta-M Relation for beta | Arguments: | - theta: Flow deflection angle | - m1: Upstream Mach number | - gamma: Specific heat ratio | - branch: Optional branch selection. Supported: weak, strong | Returns: f64 | Example: =ENG_COMPRESSIBLE_OBLIQUE_SHOCK_THETA_BETA_M_BETA('...','...','...','...')"""
    return invoke("equation.solve", {"path_id": "compressible.oblique_shock_theta_beta_m", "target": "beta", "theta": theta, "m1": m1, "gamma": gamma, **({"branch": branch} if branch not in (None, "") else {})})

@xloil.func(name="ENG_COMPRESSIBLE_OBLIQUE_SHOCK_THETA_BETA_M_THETA", help="Solve Oblique Shock Theta-Beta-M Relation for theta | Arguments: | - beta: Shock angle | - m1: Upstream Mach number | - gamma: Specific heat ratio | - branch: Optional branch selection. Supported: weak, strong | Returns: f64 | Example: =ENG_COMPRESSIBLE_OBLIQUE_SHOCK_THETA_BETA_M_THETA('...','...','...','...')")
def e_n_g_c_o_m_p_r_e_s_s_i_b_l_e_o_b_l_i_q_u_e_s_h_o_c_k_t_h_e_t_a_b_e_t_a_m_t_h_e_t_a(beta, m1, gamma, branch=""):
    """Solve Oblique Shock Theta-Beta-M Relation for theta | Arguments: | - beta: Shock angle | - m1: Upstream Mach number | - gamma: Specific heat ratio | - branch: Optional branch selection. Supported: weak, strong | Returns: f64 | Example: =ENG_COMPRESSIBLE_OBLIQUE_SHOCK_THETA_BETA_M_THETA('...','...','...','...')"""
    return invoke("equation.solve", {"path_id": "compressible.oblique_shock_theta_beta_m", "target": "theta", "beta": beta, "m1": m1, "gamma": gamma, **({"branch": branch} if branch not in (None, "") else {})})

@xloil.func(name="ENG_COMPRESSIBLE_PRANDTL_MEYER_M", help="Solve Prandtl-Meyer Expansion Angle for M | Arguments: | - nu: Prandtl-Meyer angle | - gamma: Specific heat ratio | Returns: f64 | Example: =ENG_COMPRESSIBLE_PRANDTL_MEYER_M('...','...')")
def e_n_g_c_o_m_p_r_e_s_s_i_b_l_e_p_r_a_n_d_t_l_m_e_y_e_r_m(nu, gamma):
    """Solve Prandtl-Meyer Expansion Angle for M | Arguments: | - nu: Prandtl-Meyer angle | - gamma: Specific heat ratio | Returns: f64 | Example: =ENG_COMPRESSIBLE_PRANDTL_MEYER_M('...','...')"""
    return invoke("equation.solve", {"path_id": "compressible.prandtl_meyer", "target": "M", "nu": nu, "gamma": gamma})

@xloil.func(name="ENG_COMPRESSIBLE_PRANDTL_MEYER_NU", help="Solve Prandtl-Meyer Expansion Angle for nu | Arguments: | - m: Mach number | - gamma: Specific heat ratio | Returns: f64 | Example: =ENG_COMPRESSIBLE_PRANDTL_MEYER_NU('...','...')")
def e_n_g_c_o_m_p_r_e_s_s_i_b_l_e_p_r_a_n_d_t_l_m_e_y_e_r_n_u(m, gamma):
    """Solve Prandtl-Meyer Expansion Angle for nu | Arguments: | - m: Mach number | - gamma: Specific heat ratio | Returns: f64 | Example: =ENG_COMPRESSIBLE_PRANDTL_MEYER_NU('...','...')"""
    return invoke("equation.solve", {"path_id": "compressible.prandtl_meyer", "target": "nu", "M": m, "gamma": gamma})

@xloil.func(name="ENG_COMPRESSIBLE_RAYLEIGH_DENSITY_RATIO_M", help="Solve Rayleigh Density Ratio for M | Arguments: | - rho_rhostar: Density ratio to star state | - gamma: Specific heat ratio | Returns: f64 | Example: =ENG_COMPRESSIBLE_RAYLEIGH_DENSITY_RATIO_M('...','...')")
def e_n_g_c_o_m_p_r_e_s_s_i_b_l_e_r_a_y_l_e_i_g_h_d_e_n_s_i_t_y_r_a_t_i_o_m(rho_rhostar, gamma):
    """Solve Rayleigh Density Ratio for M | Arguments: | - rho_rhostar: Density ratio to star state | - gamma: Specific heat ratio | Returns: f64 | Example: =ENG_COMPRESSIBLE_RAYLEIGH_DENSITY_RATIO_M('...','...')"""
    return invoke("equation.solve", {"path_id": "compressible.rayleigh_density_ratio", "target": "M", "rho_rhostar": rho_rhostar, "gamma": gamma})

@xloil.func(name="ENG_COMPRESSIBLE_RAYLEIGH_DENSITY_RATIO_RHO_RHOSTAR", help="Solve Rayleigh Density Ratio for rho_rhostar | Arguments: | - m: Mach number | - gamma: Specific heat ratio | Returns: f64 | Example: =ENG_COMPRESSIBLE_RAYLEIGH_DENSITY_RATIO_RHO_RHOSTAR('...','...')")
def e_n_g_c_o_m_p_r_e_s_s_i_b_l_e_r_a_y_l_e_i_g_h_d_e_n_s_i_t_y_r_a_t_i_o_r_h_o_r_h_o_s_t_a_r(m, gamma):
    """Solve Rayleigh Density Ratio for rho_rhostar | Arguments: | - m: Mach number | - gamma: Specific heat ratio | Returns: f64 | Example: =ENG_COMPRESSIBLE_RAYLEIGH_DENSITY_RATIO_RHO_RHOSTAR('...','...')"""
    return invoke("equation.solve", {"path_id": "compressible.rayleigh_density_ratio", "target": "rho_rhostar", "M": m, "gamma": gamma})

@xloil.func(name="ENG_COMPRESSIBLE_RAYLEIGH_PRESSURE_RATIO_M", help="Solve Rayleigh Pressure Ratio for M | Arguments: | - p_pstar: Pressure ratio to star state | - gamma: Specific heat ratio | Returns: f64 | Example: =ENG_COMPRESSIBLE_RAYLEIGH_PRESSURE_RATIO_M('...','...')")
def e_n_g_c_o_m_p_r_e_s_s_i_b_l_e_r_a_y_l_e_i_g_h_p_r_e_s_s_u_r_e_r_a_t_i_o_m(p_pstar, gamma):
    """Solve Rayleigh Pressure Ratio for M | Arguments: | - p_pstar: Pressure ratio to star state | - gamma: Specific heat ratio | Returns: f64 | Example: =ENG_COMPRESSIBLE_RAYLEIGH_PRESSURE_RATIO_M('...','...')"""
    return invoke("equation.solve", {"path_id": "compressible.rayleigh_pressure_ratio", "target": "M", "p_pstar": p_pstar, "gamma": gamma})

@xloil.func(name="ENG_COMPRESSIBLE_RAYLEIGH_PRESSURE_RATIO_P_PSTAR", help="Solve Rayleigh Pressure Ratio for p_pstar | Arguments: | - m: Mach number | - gamma: Specific heat ratio | Returns: f64 | Example: =ENG_COMPRESSIBLE_RAYLEIGH_PRESSURE_RATIO_P_PSTAR('...','...')")
def e_n_g_c_o_m_p_r_e_s_s_i_b_l_e_r_a_y_l_e_i_g_h_p_r_e_s_s_u_r_e_r_a_t_i_o_p_p_s_t_a_r(m, gamma):
    """Solve Rayleigh Pressure Ratio for p_pstar | Arguments: | - m: Mach number | - gamma: Specific heat ratio | Returns: f64 | Example: =ENG_COMPRESSIBLE_RAYLEIGH_PRESSURE_RATIO_P_PSTAR('...','...')"""
    return invoke("equation.solve", {"path_id": "compressible.rayleigh_pressure_ratio", "target": "p_pstar", "M": m, "gamma": gamma})

@xloil.func(name="ENG_COMPRESSIBLE_RAYLEIGH_STAGNATION_PRESSURE_RATIO_M", help="Solve Rayleigh Stagnation Pressure Ratio for M | Arguments: | - p0_p0star: Stagnation pressure ratio to star state | - gamma: Specific heat ratio | - branch: Optional branch selection. Supported: subsonic, supersonic | Returns: f64 | Example: =ENG_COMPRESSIBLE_RAYLEIGH_STAGNATION_PRESSURE_RATIO_M('...','...','...')")
def e_n_g_c_o_m_p_r_e_s_s_i_b_l_e_r_a_y_l_e_i_g_h_s_t_a_g_n_a_t_i_o_n_p_r_e_s_s_u_r_e_r_a_t_i_o_m(p0_p0star, gamma, branch=""):
    """Solve Rayleigh Stagnation Pressure Ratio for M | Arguments: | - p0_p0star: Stagnation pressure ratio to star state | - gamma: Specific heat ratio | - branch: Optional branch selection. Supported: subsonic, supersonic | Returns: f64 | Example: =ENG_COMPRESSIBLE_RAYLEIGH_STAGNATION_PRESSURE_RATIO_M('...','...','...')"""
    return invoke("equation.solve", {"path_id": "compressible.rayleigh_stagnation_pressure_ratio", "target": "M", "p0_p0star": p0_p0star, "gamma": gamma, **({"branch": branch} if branch not in (None, "") else {})})

@xloil.func(name="ENG_COMPRESSIBLE_RAYLEIGH_STAGNATION_PRESSURE_RATIO_P0_P0STAR", help="Solve Rayleigh Stagnation Pressure Ratio for p0_p0star | Arguments: | - m: Mach number | - gamma: Specific heat ratio | - branch: Optional branch selection. Supported: subsonic, supersonic | Returns: f64 | Example: =ENG_COMPRESSIBLE_RAYLEIGH_STAGNATION_PRESSURE_RATIO_P0_P0STAR('...','...','...')")
def e_n_g_c_o_m_p_r_e_s_s_i_b_l_e_r_a_y_l_e_i_g_h_s_t_a_g_n_a_t_i_o_n_p_r_e_s_s_u_r_e_r_a_t_i_o_p0_p0_s_t_a_r(m, gamma, branch=""):
    """Solve Rayleigh Stagnation Pressure Ratio for p0_p0star | Arguments: | - m: Mach number | - gamma: Specific heat ratio | - branch: Optional branch selection. Supported: subsonic, supersonic | Returns: f64 | Example: =ENG_COMPRESSIBLE_RAYLEIGH_STAGNATION_PRESSURE_RATIO_P0_P0STAR('...','...','...')"""
    return invoke("equation.solve", {"path_id": "compressible.rayleigh_stagnation_pressure_ratio", "target": "p0_p0star", "M": m, "gamma": gamma, **({"branch": branch} if branch not in (None, "") else {})})

@xloil.func(name="ENG_COMPRESSIBLE_RAYLEIGH_STAGNATION_TEMPERATURE_RATIO_M", help="Solve Rayleigh Stagnation Temperature Ratio for M | Arguments: | - t0_t0star: Stagnation temperature ratio to star state | - gamma: Specific heat ratio | - branch: Optional branch selection. Supported: subsonic, supersonic | Returns: f64 | Example: =ENG_COMPRESSIBLE_RAYLEIGH_STAGNATION_TEMPERATURE_RATIO_M('...','...','...')")
def e_n_g_c_o_m_p_r_e_s_s_i_b_l_e_r_a_y_l_e_i_g_h_s_t_a_g_n_a_t_i_o_n_t_e_m_p_e_r_a_t_u_r_e_r_a_t_i_o_m(t0_t0star, gamma, branch=""):
    """Solve Rayleigh Stagnation Temperature Ratio for M | Arguments: | - t0_t0star: Stagnation temperature ratio to star state | - gamma: Specific heat ratio | - branch: Optional branch selection. Supported: subsonic, supersonic | Returns: f64 | Example: =ENG_COMPRESSIBLE_RAYLEIGH_STAGNATION_TEMPERATURE_RATIO_M('...','...','...')"""
    return invoke("equation.solve", {"path_id": "compressible.rayleigh_stagnation_temperature_ratio", "target": "M", "t0_t0star": t0_t0star, "gamma": gamma, **({"branch": branch} if branch not in (None, "") else {})})

@xloil.func(name="ENG_COMPRESSIBLE_RAYLEIGH_STAGNATION_TEMPERATURE_RATIO_T0_T0STAR", help="Solve Rayleigh Stagnation Temperature Ratio for t0_t0star | Arguments: | - m: Mach number | - gamma: Specific heat ratio | - branch: Optional branch selection. Supported: subsonic, supersonic | Returns: f64 | Example: =ENG_COMPRESSIBLE_RAYLEIGH_STAGNATION_TEMPERATURE_RATIO_T0_T0STAR('...','...','...')")
def e_n_g_c_o_m_p_r_e_s_s_i_b_l_e_r_a_y_l_e_i_g_h_s_t_a_g_n_a_t_i_o_n_t_e_m_p_e_r_a_t_u_r_e_r_a_t_i_o_t0_t0_s_t_a_r(m, gamma, branch=""):
    """Solve Rayleigh Stagnation Temperature Ratio for t0_t0star | Arguments: | - m: Mach number | - gamma: Specific heat ratio | - branch: Optional branch selection. Supported: subsonic, supersonic | Returns: f64 | Example: =ENG_COMPRESSIBLE_RAYLEIGH_STAGNATION_TEMPERATURE_RATIO_T0_T0STAR('...','...','...')"""
    return invoke("equation.solve", {"path_id": "compressible.rayleigh_stagnation_temperature_ratio", "target": "t0_t0star", "M": m, "gamma": gamma, **({"branch": branch} if branch not in (None, "") else {})})

@xloil.func(name="ENG_COMPRESSIBLE_RAYLEIGH_TEMPERATURE_RATIO_M", help="Solve Rayleigh Temperature Ratio for M | Arguments: | - t_tstar: Temperature ratio to star state | - gamma: Specific heat ratio | - branch: Optional branch selection. Supported: subsonic, supersonic | Returns: f64 | Example: =ENG_COMPRESSIBLE_RAYLEIGH_TEMPERATURE_RATIO_M('...','...','...')")
def e_n_g_c_o_m_p_r_e_s_s_i_b_l_e_r_a_y_l_e_i_g_h_t_e_m_p_e_r_a_t_u_r_e_r_a_t_i_o_m(t_tstar, gamma, branch=""):
    """Solve Rayleigh Temperature Ratio for M | Arguments: | - t_tstar: Temperature ratio to star state | - gamma: Specific heat ratio | - branch: Optional branch selection. Supported: subsonic, supersonic | Returns: f64 | Example: =ENG_COMPRESSIBLE_RAYLEIGH_TEMPERATURE_RATIO_M('...','...','...')"""
    return invoke("equation.solve", {"path_id": "compressible.rayleigh_temperature_ratio", "target": "M", "t_tstar": t_tstar, "gamma": gamma, **({"branch": branch} if branch not in (None, "") else {})})

@xloil.func(name="ENG_COMPRESSIBLE_RAYLEIGH_TEMPERATURE_RATIO_T_TSTAR", help="Solve Rayleigh Temperature Ratio for t_tstar | Arguments: | - m: Mach number | - gamma: Specific heat ratio | - branch: Optional branch selection. Supported: subsonic, supersonic | Returns: f64 | Example: =ENG_COMPRESSIBLE_RAYLEIGH_TEMPERATURE_RATIO_T_TSTAR('...','...','...')")
def e_n_g_c_o_m_p_r_e_s_s_i_b_l_e_r_a_y_l_e_i_g_h_t_e_m_p_e_r_a_t_u_r_e_r_a_t_i_o_t_t_s_t_a_r(m, gamma, branch=""):
    """Solve Rayleigh Temperature Ratio for t_tstar | Arguments: | - m: Mach number | - gamma: Specific heat ratio | - branch: Optional branch selection. Supported: subsonic, supersonic | Returns: f64 | Example: =ENG_COMPRESSIBLE_RAYLEIGH_TEMPERATURE_RATIO_T_TSTAR('...','...','...')"""
    return invoke("equation.solve", {"path_id": "compressible.rayleigh_temperature_ratio", "target": "t_tstar", "M": m, "gamma": gamma, **({"branch": branch} if branch not in (None, "") else {})})

@xloil.func(name="ENG_COMPRESSIBLE_RAYLEIGH_VELOCITY_RATIO_M", help="Solve Rayleigh Velocity Ratio for M | Arguments: | - v_vstar: Velocity ratio to star state | - gamma: Specific heat ratio | Returns: f64 | Example: =ENG_COMPRESSIBLE_RAYLEIGH_VELOCITY_RATIO_M('...','...')")
def e_n_g_c_o_m_p_r_e_s_s_i_b_l_e_r_a_y_l_e_i_g_h_v_e_l_o_c_i_t_y_r_a_t_i_o_m(v_vstar, gamma):
    """Solve Rayleigh Velocity Ratio for M | Arguments: | - v_vstar: Velocity ratio to star state | - gamma: Specific heat ratio | Returns: f64 | Example: =ENG_COMPRESSIBLE_RAYLEIGH_VELOCITY_RATIO_M('...','...')"""
    return invoke("equation.solve", {"path_id": "compressible.rayleigh_velocity_ratio", "target": "M", "v_vstar": v_vstar, "gamma": gamma})

@xloil.func(name="ENG_COMPRESSIBLE_RAYLEIGH_VELOCITY_RATIO_V_VSTAR", help="Solve Rayleigh Velocity Ratio for v_vstar | Arguments: | - m: Mach number | - gamma: Specific heat ratio | Returns: f64 | Example: =ENG_COMPRESSIBLE_RAYLEIGH_VELOCITY_RATIO_V_VSTAR('...','...')")
def e_n_g_c_o_m_p_r_e_s_s_i_b_l_e_r_a_y_l_e_i_g_h_v_e_l_o_c_i_t_y_r_a_t_i_o_v_v_s_t_a_r(m, gamma):
    """Solve Rayleigh Velocity Ratio for v_vstar | Arguments: | - m: Mach number | - gamma: Specific heat ratio | Returns: f64 | Example: =ENG_COMPRESSIBLE_RAYLEIGH_VELOCITY_RATIO_V_VSTAR('...','...')"""
    return invoke("equation.solve", {"path_id": "compressible.rayleigh_velocity_ratio", "target": "v_vstar", "M": m, "gamma": gamma})

@xloil.func(name="ENG_EQUATION_DEFAULT_UNIT", help="Read canonical default unit for one equation variable | Arguments: | - path_id: Equation path id | - variable: Variable key (case-insensitive) | Returns: str | Example: =ENG_EQUATION_DEFAULT_UNIT('fluids.reynolds_number','mu')")
def e_n_g_e_q_u_a_t_i_o_n_d_e_f_a_u_l_t_u_n_i_t(path_id, variable):
    """Read canonical default unit for one equation variable | Arguments: | - path_id: Equation path id | - variable: Variable key (case-insensitive) | Returns: str | Example: =ENG_EQUATION_DEFAULT_UNIT('fluids.reynolds_number','mu')"""
    return invoke("equation.default_unit", {"path_id": path_id, "variable": variable})

@xloil.func(name="ENG_EQUATION_DESCRIPTION", help="Read equation description | Arguments: | - path_id: Equation path id | Returns: str | Example: =ENG_EQUATION_DESCRIPTION('fluids.reynolds_number')")
def e_n_g_e_q_u_a_t_i_o_n_d_e_s_c_r_i_p_t_i_o_n(path_id):
    """Read equation description | Arguments: | - path_id: Equation path id | Returns: str | Example: =ENG_EQUATION_DESCRIPTION('fluids.reynolds_number')"""
    return invoke("equation.description", {"path_id": path_id})

@xloil.func(name="ENG_EQUATION_FAMILY", help="Read parent family/variant metadata for an equation | Arguments: | - path_id: Equation path id | Returns: dict|null | Example: =ENG_EQUATION_FAMILY('thermo.ideal_gas.density')")
def e_n_g_e_q_u_a_t_i_o_n_f_a_m_i_l_y(path_id):
    """Read parent family/variant metadata for an equation | Arguments: | - path_id: Equation path id | Returns: dict|null | Example: =ENG_EQUATION_FAMILY('thermo.ideal_gas.density')"""
    return invoke("equation.family", {"path_id": path_id})

@xloil.func(name="ENG_FLUIDS_CIRCULAR_PIPE_AREA_A", help="Solve Circular Pipe Flow Area for A | Arguments: | - d: Diameter | Returns: f64 | Example: =ENG_FLUIDS_CIRCULAR_PIPE_AREA_A('...')")
def e_n_g_f_l_u_i_d_s_c_i_r_c_u_l_a_r_p_i_p_e_a_r_e_a_a(d):
    """Solve Circular Pipe Flow Area for A | Arguments: | - d: Diameter | Returns: f64 | Example: =ENG_FLUIDS_CIRCULAR_PIPE_AREA_A('...')"""
    return invoke("equation.solve", {"path_id": "fluids.circular_pipe_area", "target": "A", "D": d})

@xloil.func(name="ENG_FLUIDS_CIRCULAR_PIPE_AREA_D", help="Solve Circular Pipe Flow Area for D | Arguments: | - a: Area | Returns: f64 | Example: =ENG_FLUIDS_CIRCULAR_PIPE_AREA_D('...')")
def e_n_g_f_l_u_i_d_s_c_i_r_c_u_l_a_r_p_i_p_e_a_r_e_a_d(a):
    """Solve Circular Pipe Flow Area for D | Arguments: | - a: Area | Returns: f64 | Example: =ENG_FLUIDS_CIRCULAR_PIPE_AREA_D('...')"""
    return invoke("equation.solve", {"path_id": "fluids.circular_pipe_area", "target": "D", "A": a})

@xloil.func(name="ENG_FLUIDS_COLEBROOK_F", help="Solve Colebrook-White Friction Factor for f | Arguments: | - eps_d: Relative roughness | - re: Reynolds number | Returns: f64 | Example: =ENG_FLUIDS_COLEBROOK_F('...','...')")
def e_n_g_f_l_u_i_d_s_c_o_l_e_b_r_o_o_k_f(eps_d, re):
    """Solve Colebrook-White Friction Factor for f | Arguments: | - eps_d: Relative roughness | - re: Reynolds number | Returns: f64 | Example: =ENG_FLUIDS_COLEBROOK_F('...','...')"""
    return invoke("equation.solve", {"path_id": "fluids.colebrook", "target": "f", "eps_D": eps_d, "Re": re})

@xloil.func(name="ENG_FLUIDS_CONTINUITY_MASS_FLOW_A", help="Solve Continuity Mass Flow for A | Arguments: | - m_dot: Mass flow rate | - rho: Fluid density | - v: Mean velocity | Returns: f64 | Example: =ENG_FLUIDS_CONTINUITY_MASS_FLOW_A('...','...','...')")
def e_n_g_f_l_u_i_d_s_c_o_n_t_i_n_u_i_t_y_m_a_s_s_f_l_o_w_a(m_dot, rho, v):
    """Solve Continuity Mass Flow for A | Arguments: | - m_dot: Mass flow rate | - rho: Fluid density | - v: Mean velocity | Returns: f64 | Example: =ENG_FLUIDS_CONTINUITY_MASS_FLOW_A('...','...','...')"""
    return invoke("equation.solve", {"path_id": "fluids.continuity_mass_flow", "target": "A", "m_dot": m_dot, "rho": rho, "V": v})

@xloil.func(name="ENG_FLUIDS_CONTINUITY_MASS_FLOW_V", help="Solve Continuity Mass Flow for V | Arguments: | - m_dot: Mass flow rate | - rho: Fluid density | - a: Flow area | Returns: f64 | Example: =ENG_FLUIDS_CONTINUITY_MASS_FLOW_V('...','...','...')")
def e_n_g_f_l_u_i_d_s_c_o_n_t_i_n_u_i_t_y_m_a_s_s_f_l_o_w_v(m_dot, rho, a):
    """Solve Continuity Mass Flow for V | Arguments: | - m_dot: Mass flow rate | - rho: Fluid density | - a: Flow area | Returns: f64 | Example: =ENG_FLUIDS_CONTINUITY_MASS_FLOW_V('...','...','...')"""
    return invoke("equation.solve", {"path_id": "fluids.continuity_mass_flow", "target": "V", "m_dot": m_dot, "rho": rho, "A": a})

@xloil.func(name="ENG_FLUIDS_CONTINUITY_MASS_FLOW_M_DOT", help="Solve Continuity Mass Flow for m_dot | Arguments: | - rho: Fluid density | - a: Flow area | - v: Mean velocity | Returns: f64 | Example: =ENG_FLUIDS_CONTINUITY_MASS_FLOW_M_DOT('...','...','...')")
def e_n_g_f_l_u_i_d_s_c_o_n_t_i_n_u_i_t_y_m_a_s_s_f_l_o_w_m_d_o_t(rho, a, v):
    """Solve Continuity Mass Flow for m_dot | Arguments: | - rho: Fluid density | - a: Flow area | - v: Mean velocity | Returns: f64 | Example: =ENG_FLUIDS_CONTINUITY_MASS_FLOW_M_DOT('...','...','...')"""
    return invoke("equation.solve", {"path_id": "fluids.continuity_mass_flow", "target": "m_dot", "rho": rho, "A": a, "V": v})

@xloil.func(name="ENG_FLUIDS_CONTINUITY_MASS_FLOW_RHO", help="Solve Continuity Mass Flow for rho | Arguments: | - m_dot: Mass flow rate | - a: Flow area | - v: Mean velocity | Returns: f64 | Example: =ENG_FLUIDS_CONTINUITY_MASS_FLOW_RHO('...','...','...')")
def e_n_g_f_l_u_i_d_s_c_o_n_t_i_n_u_i_t_y_m_a_s_s_f_l_o_w_r_h_o(m_dot, a, v):
    """Solve Continuity Mass Flow for rho | Arguments: | - m_dot: Mass flow rate | - a: Flow area | - v: Mean velocity | Returns: f64 | Example: =ENG_FLUIDS_CONTINUITY_MASS_FLOW_RHO('...','...','...')"""
    return invoke("equation.solve", {"path_id": "fluids.continuity_mass_flow", "target": "rho", "m_dot": m_dot, "A": a, "V": v})

@xloil.func(name="ENG_FLUIDS_DARCY_WEISBACH_PRESSURE_DROP_D", help="Solve Darcy-Weisbach Pressure Drop for D | Arguments: | - delta_p: Pressure drop | - f: Darcy friction factor | - l: Pipe length | - rho: Fluid density | - v: Mean velocity | Returns: f64 | Example: =ENG_FLUIDS_DARCY_WEISBACH_PRESSURE_DROP_D('...','...','...','...','...')")
def e_n_g_f_l_u_i_d_s_d_a_r_c_y_w_e_i_s_b_a_c_h_p_r_e_s_s_u_r_e_d_r_o_p_d(delta_p, f, l, rho, v):
    """Solve Darcy-Weisbach Pressure Drop for D | Arguments: | - delta_p: Pressure drop | - f: Darcy friction factor | - l: Pipe length | - rho: Fluid density | - v: Mean velocity | Returns: f64 | Example: =ENG_FLUIDS_DARCY_WEISBACH_PRESSURE_DROP_D('...','...','...','...','...')"""
    return invoke("equation.solve", {"path_id": "fluids.darcy_weisbach_pressure_drop", "target": "D", "delta_p": delta_p, "f": f, "L": l, "rho": rho, "V": v})

@xloil.func(name="ENG_FLUIDS_DARCY_WEISBACH_PRESSURE_DROP_L", help="Solve Darcy-Weisbach Pressure Drop for L | Arguments: | - delta_p: Pressure drop | - f: Darcy friction factor | - d: Pipe diameter | - rho: Fluid density | - v: Mean velocity | Returns: f64 | Example: =ENG_FLUIDS_DARCY_WEISBACH_PRESSURE_DROP_L('...','...','...','...','...')")
def e_n_g_f_l_u_i_d_s_d_a_r_c_y_w_e_i_s_b_a_c_h_p_r_e_s_s_u_r_e_d_r_o_p_l(delta_p, f, d, rho, v):
    """Solve Darcy-Weisbach Pressure Drop for L | Arguments: | - delta_p: Pressure drop | - f: Darcy friction factor | - d: Pipe diameter | - rho: Fluid density | - v: Mean velocity | Returns: f64 | Example: =ENG_FLUIDS_DARCY_WEISBACH_PRESSURE_DROP_L('...','...','...','...','...')"""
    return invoke("equation.solve", {"path_id": "fluids.darcy_weisbach_pressure_drop", "target": "L", "delta_p": delta_p, "f": f, "D": d, "rho": rho, "V": v})

@xloil.func(name="ENG_FLUIDS_DARCY_WEISBACH_PRESSURE_DROP_V", help="Solve Darcy-Weisbach Pressure Drop for V | Arguments: | - delta_p: Pressure drop | - f: Darcy friction factor | - l: Pipe length | - d: Pipe diameter | - rho: Fluid density | Returns: f64 | Example: =ENG_FLUIDS_DARCY_WEISBACH_PRESSURE_DROP_V('...','...','...','...','...')")
def e_n_g_f_l_u_i_d_s_d_a_r_c_y_w_e_i_s_b_a_c_h_p_r_e_s_s_u_r_e_d_r_o_p_v(delta_p, f, l, d, rho):
    """Solve Darcy-Weisbach Pressure Drop for V | Arguments: | - delta_p: Pressure drop | - f: Darcy friction factor | - l: Pipe length | - d: Pipe diameter | - rho: Fluid density | Returns: f64 | Example: =ENG_FLUIDS_DARCY_WEISBACH_PRESSURE_DROP_V('...','...','...','...','...')"""
    return invoke("equation.solve", {"path_id": "fluids.darcy_weisbach_pressure_drop", "target": "V", "delta_p": delta_p, "f": f, "L": l, "D": d, "rho": rho})

@xloil.func(name="ENG_FLUIDS_DARCY_WEISBACH_PRESSURE_DROP_DELTA_P", help="Solve Darcy-Weisbach Pressure Drop for delta_p | Arguments: | - f: Darcy friction factor | - l: Pipe length | - d: Pipe diameter | - rho: Fluid density | - v: Mean velocity | Returns: f64 | Example: =ENG_FLUIDS_DARCY_WEISBACH_PRESSURE_DROP_DELTA_P('...','...','...','...','...')")
def e_n_g_f_l_u_i_d_s_d_a_r_c_y_w_e_i_s_b_a_c_h_p_r_e_s_s_u_r_e_d_r_o_p_d_e_l_t_a_p(f, l, d, rho, v):
    """Solve Darcy-Weisbach Pressure Drop for delta_p | Arguments: | - f: Darcy friction factor | - l: Pipe length | - d: Pipe diameter | - rho: Fluid density | - v: Mean velocity | Returns: f64 | Example: =ENG_FLUIDS_DARCY_WEISBACH_PRESSURE_DROP_DELTA_P('...','...','...','...','...')"""
    return invoke("equation.solve", {"path_id": "fluids.darcy_weisbach_pressure_drop", "target": "delta_p", "f": f, "L": l, "D": d, "rho": rho, "V": v})

@xloil.func(name="ENG_FLUIDS_DARCY_WEISBACH_PRESSURE_DROP_F", help="Solve Darcy-Weisbach Pressure Drop for f | Arguments: | - delta_p: Pressure drop | - l: Pipe length | - d: Pipe diameter | - rho: Fluid density | - v: Mean velocity | Returns: f64 | Example: =ENG_FLUIDS_DARCY_WEISBACH_PRESSURE_DROP_F('...','...','...','...','...')")
def e_n_g_f_l_u_i_d_s_d_a_r_c_y_w_e_i_s_b_a_c_h_p_r_e_s_s_u_r_e_d_r_o_p_f(delta_p, l, d, rho, v):
    """Solve Darcy-Weisbach Pressure Drop for f | Arguments: | - delta_p: Pressure drop | - l: Pipe length | - d: Pipe diameter | - rho: Fluid density | - v: Mean velocity | Returns: f64 | Example: =ENG_FLUIDS_DARCY_WEISBACH_PRESSURE_DROP_F('...','...','...','...','...')"""
    return invoke("equation.solve", {"path_id": "fluids.darcy_weisbach_pressure_drop", "target": "f", "delta_p": delta_p, "L": l, "D": d, "rho": rho, "V": v})

@xloil.func(name="ENG_FLUIDS_DARCY_WEISBACH_PRESSURE_DROP_RHO", help="Solve Darcy-Weisbach Pressure Drop for rho | Arguments: | - delta_p: Pressure drop | - f: Darcy friction factor | - l: Pipe length | - d: Pipe diameter | - v: Mean velocity | Returns: f64 | Example: =ENG_FLUIDS_DARCY_WEISBACH_PRESSURE_DROP_RHO('...','...','...','...','...')")
def e_n_g_f_l_u_i_d_s_d_a_r_c_y_w_e_i_s_b_a_c_h_p_r_e_s_s_u_r_e_d_r_o_p_r_h_o(delta_p, f, l, d, v):
    """Solve Darcy-Weisbach Pressure Drop for rho | Arguments: | - delta_p: Pressure drop | - f: Darcy friction factor | - l: Pipe length | - d: Pipe diameter | - v: Mean velocity | Returns: f64 | Example: =ENG_FLUIDS_DARCY_WEISBACH_PRESSURE_DROP_RHO('...','...','...','...','...')"""
    return invoke("equation.solve", {"path_id": "fluids.darcy_weisbach_pressure_drop", "target": "rho", "delta_p": delta_p, "f": f, "L": l, "D": d, "V": v})

@xloil.func(name="ENG_FLUIDS_ORIFICE_MASS_FLOW_INCOMPRESSIBLE_A", help="Solve Incompressible Orifice Mass Flow for A | Arguments: | - m_dot: Mass flow rate | - c_d: Discharge coefficient | - rho: Fluid density | - delta_p: Pressure drop | Returns: f64 | Example: =ENG_FLUIDS_ORIFICE_MASS_FLOW_INCOMPRESSIBLE_A('...','...','...','...')")
def e_n_g_f_l_u_i_d_s_o_r_i_f_i_c_e_m_a_s_s_f_l_o_w_i_n_c_o_m_p_r_e_s_s_i_b_l_e_a(m_dot, c_d, rho, delta_p):
    """Solve Incompressible Orifice Mass Flow for A | Arguments: | - m_dot: Mass flow rate | - c_d: Discharge coefficient | - rho: Fluid density | - delta_p: Pressure drop | Returns: f64 | Example: =ENG_FLUIDS_ORIFICE_MASS_FLOW_INCOMPRESSIBLE_A('...','...','...','...')"""
    return invoke("equation.solve", {"path_id": "fluids.orifice_mass_flow_incompressible", "target": "A", "m_dot": m_dot, "C_d": c_d, "rho": rho, "delta_p": delta_p})

@xloil.func(name="ENG_FLUIDS_ORIFICE_MASS_FLOW_INCOMPRESSIBLE_C_D", help="Solve Incompressible Orifice Mass Flow for C_d | Arguments: | - m_dot: Mass flow rate | - a: Orifice area | - rho: Fluid density | - delta_p: Pressure drop | Returns: f64 | Example: =ENG_FLUIDS_ORIFICE_MASS_FLOW_INCOMPRESSIBLE_C_D('...','...','...','...')")
def e_n_g_f_l_u_i_d_s_o_r_i_f_i_c_e_m_a_s_s_f_l_o_w_i_n_c_o_m_p_r_e_s_s_i_b_l_e_c_d(m_dot, a, rho, delta_p):
    """Solve Incompressible Orifice Mass Flow for C_d | Arguments: | - m_dot: Mass flow rate | - a: Orifice area | - rho: Fluid density | - delta_p: Pressure drop | Returns: f64 | Example: =ENG_FLUIDS_ORIFICE_MASS_FLOW_INCOMPRESSIBLE_C_D('...','...','...','...')"""
    return invoke("equation.solve", {"path_id": "fluids.orifice_mass_flow_incompressible", "target": "C_d", "m_dot": m_dot, "A": a, "rho": rho, "delta_p": delta_p})

@xloil.func(name="ENG_FLUIDS_ORIFICE_MASS_FLOW_INCOMPRESSIBLE_DELTA_P", help="Solve Incompressible Orifice Mass Flow for delta_p | Arguments: | - m_dot: Mass flow rate | - c_d: Discharge coefficient | - a: Orifice area | - rho: Fluid density | Returns: f64 | Example: =ENG_FLUIDS_ORIFICE_MASS_FLOW_INCOMPRESSIBLE_DELTA_P('...','...','...','...')")
def e_n_g_f_l_u_i_d_s_o_r_i_f_i_c_e_m_a_s_s_f_l_o_w_i_n_c_o_m_p_r_e_s_s_i_b_l_e_d_e_l_t_a_p(m_dot, c_d, a, rho):
    """Solve Incompressible Orifice Mass Flow for delta_p | Arguments: | - m_dot: Mass flow rate | - c_d: Discharge coefficient | - a: Orifice area | - rho: Fluid density | Returns: f64 | Example: =ENG_FLUIDS_ORIFICE_MASS_FLOW_INCOMPRESSIBLE_DELTA_P('...','...','...','...')"""
    return invoke("equation.solve", {"path_id": "fluids.orifice_mass_flow_incompressible", "target": "delta_p", "m_dot": m_dot, "C_d": c_d, "A": a, "rho": rho})

@xloil.func(name="ENG_FLUIDS_ORIFICE_MASS_FLOW_INCOMPRESSIBLE_M_DOT", help="Solve Incompressible Orifice Mass Flow for m_dot | Arguments: | - c_d: Discharge coefficient | - a: Orifice area | - rho: Fluid density | - delta_p: Pressure drop | Returns: f64 | Example: =ENG_FLUIDS_ORIFICE_MASS_FLOW_INCOMPRESSIBLE_M_DOT('...','...','...','...')")
def e_n_g_f_l_u_i_d_s_o_r_i_f_i_c_e_m_a_s_s_f_l_o_w_i_n_c_o_m_p_r_e_s_s_i_b_l_e_m_d_o_t(c_d, a, rho, delta_p):
    """Solve Incompressible Orifice Mass Flow for m_dot | Arguments: | - c_d: Discharge coefficient | - a: Orifice area | - rho: Fluid density | - delta_p: Pressure drop | Returns: f64 | Example: =ENG_FLUIDS_ORIFICE_MASS_FLOW_INCOMPRESSIBLE_M_DOT('...','...','...','...')"""
    return invoke("equation.solve", {"path_id": "fluids.orifice_mass_flow_incompressible", "target": "m_dot", "C_d": c_d, "A": a, "rho": rho, "delta_p": delta_p})

@xloil.func(name="ENG_FLUIDS_ORIFICE_MASS_FLOW_INCOMPRESSIBLE_RHO", help="Solve Incompressible Orifice Mass Flow for rho | Arguments: | - m_dot: Mass flow rate | - c_d: Discharge coefficient | - a: Orifice area | - delta_p: Pressure drop | Returns: f64 | Example: =ENG_FLUIDS_ORIFICE_MASS_FLOW_INCOMPRESSIBLE_RHO('...','...','...','...')")
def e_n_g_f_l_u_i_d_s_o_r_i_f_i_c_e_m_a_s_s_f_l_o_w_i_n_c_o_m_p_r_e_s_s_i_b_l_e_r_h_o(m_dot, c_d, a, delta_p):
    """Solve Incompressible Orifice Mass Flow for rho | Arguments: | - m_dot: Mass flow rate | - c_d: Discharge coefficient | - a: Orifice area | - delta_p: Pressure drop | Returns: f64 | Example: =ENG_FLUIDS_ORIFICE_MASS_FLOW_INCOMPRESSIBLE_RHO('...','...','...','...')"""
    return invoke("equation.solve", {"path_id": "fluids.orifice_mass_flow_incompressible", "target": "rho", "m_dot": m_dot, "C_d": c_d, "A": a, "delta_p": delta_p})

@xloil.func(name="ENG_FLUIDS_REYNOLDS_NUMBER_D", help="Solve Reynolds Number for D | Arguments: | - re: Reynolds number | - rho: Fluid density | - v: Mean velocity | - mu: Dynamic viscosity | Returns: f64 | Example: =ENG_FLUIDS_REYNOLDS_NUMBER_D('...','...','...','...')")
def e_n_g_f_l_u_i_d_s_r_e_y_n_o_l_d_s_n_u_m_b_e_r_d(re, rho, v, mu):
    """Solve Reynolds Number for D | Arguments: | - re: Reynolds number | - rho: Fluid density | - v: Mean velocity | - mu: Dynamic viscosity | Returns: f64 | Example: =ENG_FLUIDS_REYNOLDS_NUMBER_D('...','...','...','...')"""
    return invoke("equation.solve", {"path_id": "fluids.reynolds_number", "target": "D", "Re": re, "rho": rho, "V": v, "mu": mu})

@xloil.func(name="ENG_FLUIDS_REYNOLDS_NUMBER_RE", help="Solve Reynolds Number for Re | Arguments: | - rho: Fluid density | - v: Mean velocity | - d: Hydraulic diameter | - mu: Dynamic viscosity | Returns: f64 | Example: =ENG_FLUIDS_REYNOLDS_NUMBER_RE('...','...','...','...')")
def e_n_g_f_l_u_i_d_s_r_e_y_n_o_l_d_s_n_u_m_b_e_r_r_e(rho, v, d, mu):
    """Solve Reynolds Number for Re | Arguments: | - rho: Fluid density | - v: Mean velocity | - d: Hydraulic diameter | - mu: Dynamic viscosity | Returns: f64 | Example: =ENG_FLUIDS_REYNOLDS_NUMBER_RE('...','...','...','...')"""
    return invoke("equation.solve", {"path_id": "fluids.reynolds_number", "target": "Re", "rho": rho, "V": v, "D": d, "mu": mu})

@xloil.func(name="ENG_FLUIDS_REYNOLDS_NUMBER_V", help="Solve Reynolds Number for V | Arguments: | - re: Reynolds number | - rho: Fluid density | - d: Hydraulic diameter | - mu: Dynamic viscosity | Returns: f64 | Example: =ENG_FLUIDS_REYNOLDS_NUMBER_V('...','...','...','...')")
def e_n_g_f_l_u_i_d_s_r_e_y_n_o_l_d_s_n_u_m_b_e_r_v(re, rho, d, mu):
    """Solve Reynolds Number for V | Arguments: | - re: Reynolds number | - rho: Fluid density | - d: Hydraulic diameter | - mu: Dynamic viscosity | Returns: f64 | Example: =ENG_FLUIDS_REYNOLDS_NUMBER_V('...','...','...','...')"""
    return invoke("equation.solve", {"path_id": "fluids.reynolds_number", "target": "V", "Re": re, "rho": rho, "D": d, "mu": mu})

@xloil.func(name="ENG_FLUIDS_REYNOLDS_NUMBER_MU", help="Solve Reynolds Number for mu | Arguments: | - re: Reynolds number | - rho: Fluid density | - v: Mean velocity | - d: Hydraulic diameter | Returns: f64 | Example: =ENG_FLUIDS_REYNOLDS_NUMBER_MU('...','...','...','...')")
def e_n_g_f_l_u_i_d_s_r_e_y_n_o_l_d_s_n_u_m_b_e_r_m_u(re, rho, v, d):
    """Solve Reynolds Number for mu | Arguments: | - re: Reynolds number | - rho: Fluid density | - v: Mean velocity | - d: Hydraulic diameter | Returns: f64 | Example: =ENG_FLUIDS_REYNOLDS_NUMBER_MU('...','...','...','...')"""
    return invoke("equation.solve", {"path_id": "fluids.reynolds_number", "target": "mu", "Re": re, "rho": rho, "V": v, "D": d})

@xloil.func(name="ENG_FLUIDS_REYNOLDS_NUMBER_RHO", help="Solve Reynolds Number for rho | Arguments: | - re: Reynolds number | - v: Mean velocity | - d: Hydraulic diameter | - mu: Dynamic viscosity | Returns: f64 | Example: =ENG_FLUIDS_REYNOLDS_NUMBER_RHO('...','...','...','...')")
def e_n_g_f_l_u_i_d_s_r_e_y_n_o_l_d_s_n_u_m_b_e_r_r_h_o(re, v, d, mu):
    """Solve Reynolds Number for rho | Arguments: | - re: Reynolds number | - v: Mean velocity | - d: Hydraulic diameter | - mu: Dynamic viscosity | Returns: f64 | Example: =ENG_FLUIDS_REYNOLDS_NUMBER_RHO('...','...','...','...')"""
    return invoke("equation.solve", {"path_id": "fluids.reynolds_number", "target": "rho", "Re": re, "V": v, "D": d, "mu": mu})

@xloil.func(name="ENG_HEAT_TRANSFER_CONDUCTION_PLANE_WALL_HEAT_RATE_A", help="Solve Plane-Wall Conduction Heat Rate for A | Arguments: | - q_dot: Heat transfer rate | - k: Thermal conductivity | - t_h: Hot-side temperature | - t_c: Cold-side temperature | - l: Wall thickness | Returns: f64 | Example: =ENG_HEAT_TRANSFER_CONDUCTION_PLANE_WALL_HEAT_RATE_A('...','...','...','...','...')")
def e_n_g_h_e_a_t_t_r_a_n_s_f_e_r_c_o_n_d_u_c_t_i_o_n_p_l_a_n_e_w_a_l_l_h_e_a_t_r_a_t_e_a(q_dot, k, t_h, t_c, l):
    """Solve Plane-Wall Conduction Heat Rate for A | Arguments: | - q_dot: Heat transfer rate | - k: Thermal conductivity | - t_h: Hot-side temperature | - t_c: Cold-side temperature | - l: Wall thickness | Returns: f64 | Example: =ENG_HEAT_TRANSFER_CONDUCTION_PLANE_WALL_HEAT_RATE_A('...','...','...','...','...')"""
    return invoke("equation.solve", {"path_id": "heat_transfer.conduction_plane_wall_heat_rate", "target": "A", "Q_dot": q_dot, "k": k, "T_h": t_h, "T_c": t_c, "L": l})

@xloil.func(name="ENG_HEAT_TRANSFER_CONDUCTION_PLANE_WALL_HEAT_RATE_L", help="Solve Plane-Wall Conduction Heat Rate for L | Arguments: | - q_dot: Heat transfer rate | - k: Thermal conductivity | - a: Area normal to heat flow | - t_h: Hot-side temperature | - t_c: Cold-side temperature | Returns: f64 | Example: =ENG_HEAT_TRANSFER_CONDUCTION_PLANE_WALL_HEAT_RATE_L('...','...','...','...','...')")
def e_n_g_h_e_a_t_t_r_a_n_s_f_e_r_c_o_n_d_u_c_t_i_o_n_p_l_a_n_e_w_a_l_l_h_e_a_t_r_a_t_e_l(q_dot, k, a, t_h, t_c):
    """Solve Plane-Wall Conduction Heat Rate for L | Arguments: | - q_dot: Heat transfer rate | - k: Thermal conductivity | - a: Area normal to heat flow | - t_h: Hot-side temperature | - t_c: Cold-side temperature | Returns: f64 | Example: =ENG_HEAT_TRANSFER_CONDUCTION_PLANE_WALL_HEAT_RATE_L('...','...','...','...','...')"""
    return invoke("equation.solve", {"path_id": "heat_transfer.conduction_plane_wall_heat_rate", "target": "L", "Q_dot": q_dot, "k": k, "A": a, "T_h": t_h, "T_c": t_c})

@xloil.func(name="ENG_HEAT_TRANSFER_CONDUCTION_PLANE_WALL_HEAT_RATE_Q_DOT", help="Solve Plane-Wall Conduction Heat Rate for Q_dot | Arguments: | - k: Thermal conductivity | - a: Area normal to heat flow | - t_h: Hot-side temperature | - t_c: Cold-side temperature | - l: Wall thickness | Returns: f64 | Example: =ENG_HEAT_TRANSFER_CONDUCTION_PLANE_WALL_HEAT_RATE_Q_DOT('...','...','...','...','...')")
def e_n_g_h_e_a_t_t_r_a_n_s_f_e_r_c_o_n_d_u_c_t_i_o_n_p_l_a_n_e_w_a_l_l_h_e_a_t_r_a_t_e_q_d_o_t(k, a, t_h, t_c, l):
    """Solve Plane-Wall Conduction Heat Rate for Q_dot | Arguments: | - k: Thermal conductivity | - a: Area normal to heat flow | - t_h: Hot-side temperature | - t_c: Cold-side temperature | - l: Wall thickness | Returns: f64 | Example: =ENG_HEAT_TRANSFER_CONDUCTION_PLANE_WALL_HEAT_RATE_Q_DOT('...','...','...','...','...')"""
    return invoke("equation.solve", {"path_id": "heat_transfer.conduction_plane_wall_heat_rate", "target": "Q_dot", "k": k, "A": a, "T_h": t_h, "T_c": t_c, "L": l})

@xloil.func(name="ENG_HEAT_TRANSFER_CONDUCTION_PLANE_WALL_HEAT_RATE_T_C", help="Solve Plane-Wall Conduction Heat Rate for T_c | Arguments: | - q_dot: Heat transfer rate | - k: Thermal conductivity | - a: Area normal to heat flow | - t_h: Hot-side temperature | - l: Wall thickness | Returns: f64 | Example: =ENG_HEAT_TRANSFER_CONDUCTION_PLANE_WALL_HEAT_RATE_T_C('...','...','...','...','...')")
def e_n_g_h_e_a_t_t_r_a_n_s_f_e_r_c_o_n_d_u_c_t_i_o_n_p_l_a_n_e_w_a_l_l_h_e_a_t_r_a_t_e_t_c(q_dot, k, a, t_h, l):
    """Solve Plane-Wall Conduction Heat Rate for T_c | Arguments: | - q_dot: Heat transfer rate | - k: Thermal conductivity | - a: Area normal to heat flow | - t_h: Hot-side temperature | - l: Wall thickness | Returns: f64 | Example: =ENG_HEAT_TRANSFER_CONDUCTION_PLANE_WALL_HEAT_RATE_T_C('...','...','...','...','...')"""
    return invoke("equation.solve", {"path_id": "heat_transfer.conduction_plane_wall_heat_rate", "target": "T_c", "Q_dot": q_dot, "k": k, "A": a, "T_h": t_h, "L": l})

@xloil.func(name="ENG_HEAT_TRANSFER_CONDUCTION_PLANE_WALL_HEAT_RATE_T_H", help="Solve Plane-Wall Conduction Heat Rate for T_h | Arguments: | - q_dot: Heat transfer rate | - k: Thermal conductivity | - a: Area normal to heat flow | - t_c: Cold-side temperature | - l: Wall thickness | Returns: f64 | Example: =ENG_HEAT_TRANSFER_CONDUCTION_PLANE_WALL_HEAT_RATE_T_H('...','...','...','...','...')")
def e_n_g_h_e_a_t_t_r_a_n_s_f_e_r_c_o_n_d_u_c_t_i_o_n_p_l_a_n_e_w_a_l_l_h_e_a_t_r_a_t_e_t_h(q_dot, k, a, t_c, l):
    """Solve Plane-Wall Conduction Heat Rate for T_h | Arguments: | - q_dot: Heat transfer rate | - k: Thermal conductivity | - a: Area normal to heat flow | - t_c: Cold-side temperature | - l: Wall thickness | Returns: f64 | Example: =ENG_HEAT_TRANSFER_CONDUCTION_PLANE_WALL_HEAT_RATE_T_H('...','...','...','...','...')"""
    return invoke("equation.solve", {"path_id": "heat_transfer.conduction_plane_wall_heat_rate", "target": "T_h", "Q_dot": q_dot, "k": k, "A": a, "T_c": t_c, "L": l})

@xloil.func(name="ENG_HEAT_TRANSFER_CONDUCTION_PLANE_WALL_HEAT_RATE_K", help="Solve Plane-Wall Conduction Heat Rate for k | Arguments: | - q_dot: Heat transfer rate | - a: Area normal to heat flow | - t_h: Hot-side temperature | - t_c: Cold-side temperature | - l: Wall thickness | Returns: f64 | Example: =ENG_HEAT_TRANSFER_CONDUCTION_PLANE_WALL_HEAT_RATE_K('...','...','...','...','...')")
def e_n_g_h_e_a_t_t_r_a_n_s_f_e_r_c_o_n_d_u_c_t_i_o_n_p_l_a_n_e_w_a_l_l_h_e_a_t_r_a_t_e_k(q_dot, a, t_h, t_c, l):
    """Solve Plane-Wall Conduction Heat Rate for k | Arguments: | - q_dot: Heat transfer rate | - a: Area normal to heat flow | - t_h: Hot-side temperature | - t_c: Cold-side temperature | - l: Wall thickness | Returns: f64 | Example: =ENG_HEAT_TRANSFER_CONDUCTION_PLANE_WALL_HEAT_RATE_K('...','...','...','...','...')"""
    return invoke("equation.solve", {"path_id": "heat_transfer.conduction_plane_wall_heat_rate", "target": "k", "Q_dot": q_dot, "A": a, "T_h": t_h, "T_c": t_c, "L": l})

@xloil.func(name="ENG_HEAT_TRANSFER_CONVECTION_HEAT_RATE_A", help="Solve Convection Heat Transfer Rate for A | Arguments: | - q_dot: Heat transfer rate | - h: Convective heat transfer coefficient | - t_s: Surface temperature | - t_inf: Free-stream temperature | Returns: f64 | Example: =ENG_HEAT_TRANSFER_CONVECTION_HEAT_RATE_A('...','...','...','...')")
def e_n_g_h_e_a_t_t_r_a_n_s_f_e_r_c_o_n_v_e_c_t_i_o_n_h_e_a_t_r_a_t_e_a(q_dot, h, t_s, t_inf):
    """Solve Convection Heat Transfer Rate for A | Arguments: | - q_dot: Heat transfer rate | - h: Convective heat transfer coefficient | - t_s: Surface temperature | - t_inf: Free-stream temperature | Returns: f64 | Example: =ENG_HEAT_TRANSFER_CONVECTION_HEAT_RATE_A('...','...','...','...')"""
    return invoke("equation.solve", {"path_id": "heat_transfer.convection_heat_rate", "target": "A", "Q_dot": q_dot, "h": h, "T_s": t_s, "T_inf": t_inf})

@xloil.func(name="ENG_HEAT_TRANSFER_CONVECTION_HEAT_RATE_Q_DOT", help="Solve Convection Heat Transfer Rate for Q_dot | Arguments: | - h: Convective heat transfer coefficient | - a: Surface area | - t_s: Surface temperature | - t_inf: Free-stream temperature | Returns: f64 | Example: =ENG_HEAT_TRANSFER_CONVECTION_HEAT_RATE_Q_DOT('...','...','...','...')")
def e_n_g_h_e_a_t_t_r_a_n_s_f_e_r_c_o_n_v_e_c_t_i_o_n_h_e_a_t_r_a_t_e_q_d_o_t(h, a, t_s, t_inf):
    """Solve Convection Heat Transfer Rate for Q_dot | Arguments: | - h: Convective heat transfer coefficient | - a: Surface area | - t_s: Surface temperature | - t_inf: Free-stream temperature | Returns: f64 | Example: =ENG_HEAT_TRANSFER_CONVECTION_HEAT_RATE_Q_DOT('...','...','...','...')"""
    return invoke("equation.solve", {"path_id": "heat_transfer.convection_heat_rate", "target": "Q_dot", "h": h, "A": a, "T_s": t_s, "T_inf": t_inf})

@xloil.func(name="ENG_HEAT_TRANSFER_CONVECTION_HEAT_RATE_T_INF", help="Solve Convection Heat Transfer Rate for T_inf | Arguments: | - q_dot: Heat transfer rate | - h: Convective heat transfer coefficient | - a: Surface area | - t_s: Surface temperature | Returns: f64 | Example: =ENG_HEAT_TRANSFER_CONVECTION_HEAT_RATE_T_INF('...','...','...','...')")
def e_n_g_h_e_a_t_t_r_a_n_s_f_e_r_c_o_n_v_e_c_t_i_o_n_h_e_a_t_r_a_t_e_t_i_n_f(q_dot, h, a, t_s):
    """Solve Convection Heat Transfer Rate for T_inf | Arguments: | - q_dot: Heat transfer rate | - h: Convective heat transfer coefficient | - a: Surface area | - t_s: Surface temperature | Returns: f64 | Example: =ENG_HEAT_TRANSFER_CONVECTION_HEAT_RATE_T_INF('...','...','...','...')"""
    return invoke("equation.solve", {"path_id": "heat_transfer.convection_heat_rate", "target": "T_inf", "Q_dot": q_dot, "h": h, "A": a, "T_s": t_s})

@xloil.func(name="ENG_HEAT_TRANSFER_CONVECTION_HEAT_RATE_T_S", help="Solve Convection Heat Transfer Rate for T_s | Arguments: | - q_dot: Heat transfer rate | - h: Convective heat transfer coefficient | - a: Surface area | - t_inf: Free-stream temperature | Returns: f64 | Example: =ENG_HEAT_TRANSFER_CONVECTION_HEAT_RATE_T_S('...','...','...','...')")
def e_n_g_h_e_a_t_t_r_a_n_s_f_e_r_c_o_n_v_e_c_t_i_o_n_h_e_a_t_r_a_t_e_t_s(q_dot, h, a, t_inf):
    """Solve Convection Heat Transfer Rate for T_s | Arguments: | - q_dot: Heat transfer rate | - h: Convective heat transfer coefficient | - a: Surface area | - t_inf: Free-stream temperature | Returns: f64 | Example: =ENG_HEAT_TRANSFER_CONVECTION_HEAT_RATE_T_S('...','...','...','...')"""
    return invoke("equation.solve", {"path_id": "heat_transfer.convection_heat_rate", "target": "T_s", "Q_dot": q_dot, "h": h, "A": a, "T_inf": t_inf})

@xloil.func(name="ENG_HEAT_TRANSFER_CONVECTION_HEAT_RATE_H", help="Solve Convection Heat Transfer Rate for h | Arguments: | - q_dot: Heat transfer rate | - a: Surface area | - t_s: Surface temperature | - t_inf: Free-stream temperature | Returns: f64 | Example: =ENG_HEAT_TRANSFER_CONVECTION_HEAT_RATE_H('...','...','...','...')")
def e_n_g_h_e_a_t_t_r_a_n_s_f_e_r_c_o_n_v_e_c_t_i_o_n_h_e_a_t_r_a_t_e_h(q_dot, a, t_s, t_inf):
    """Solve Convection Heat Transfer Rate for h | Arguments: | - q_dot: Heat transfer rate | - a: Surface area | - t_s: Surface temperature | - t_inf: Free-stream temperature | Returns: f64 | Example: =ENG_HEAT_TRANSFER_CONVECTION_HEAT_RATE_H('...','...','...','...')"""
    return invoke("equation.solve", {"path_id": "heat_transfer.convection_heat_rate", "target": "h", "Q_dot": q_dot, "A": a, "T_s": t_s, "T_inf": t_inf})

@xloil.func(name="ENG_HEAT_TRANSFER_LOG_MEAN_TEMPERATURE_DIFFERENCE_DELTA_T_LM", help="Solve Log-Mean Temperature Difference for delta_T_lm | Arguments: | - delta_t_1: End temperature difference 1 | - delta_t_2: End temperature difference 2 | Returns: f64 | Example: =ENG_HEAT_TRANSFER_LOG_MEAN_TEMPERATURE_DIFFERENCE_DELTA_T_LM('...','...')")
def e_n_g_h_e_a_t_t_r_a_n_s_f_e_r_l_o_g_m_e_a_n_t_e_m_p_e_r_a_t_u_r_e_d_i_f_f_e_r_e_n_c_e_d_e_l_t_a_t_l_m(delta_t_1, delta_t_2):
    """Solve Log-Mean Temperature Difference for delta_T_lm | Arguments: | - delta_t_1: End temperature difference 1 | - delta_t_2: End temperature difference 2 | Returns: f64 | Example: =ENG_HEAT_TRANSFER_LOG_MEAN_TEMPERATURE_DIFFERENCE_DELTA_T_LM('...','...')"""
    return invoke("equation.solve", {"path_id": "heat_transfer.log_mean_temperature_difference", "target": "delta_T_lm", "delta_T_1": delta_t_1, "delta_T_2": delta_t_2})

@xloil.func(name="ENG_HEAT_TRANSFER_THERMAL_RESISTANCE_CONDUCTION_A", help="Solve Conduction Thermal Resistance for A | Arguments: | - r_th: Thermal resistance | - l: Wall thickness | - k: Thermal conductivity | Returns: f64 | Example: =ENG_HEAT_TRANSFER_THERMAL_RESISTANCE_CONDUCTION_A('...','...','...')")
def e_n_g_h_e_a_t_t_r_a_n_s_f_e_r_t_h_e_r_m_a_l_r_e_s_i_s_t_a_n_c_e_c_o_n_d_u_c_t_i_o_n_a(r_th, l, k):
    """Solve Conduction Thermal Resistance for A | Arguments: | - r_th: Thermal resistance | - l: Wall thickness | - k: Thermal conductivity | Returns: f64 | Example: =ENG_HEAT_TRANSFER_THERMAL_RESISTANCE_CONDUCTION_A('...','...','...')"""
    return invoke("equation.solve", {"path_id": "heat_transfer.thermal_resistance_conduction", "target": "A", "R_th": r_th, "L": l, "k": k})

@xloil.func(name="ENG_HEAT_TRANSFER_THERMAL_RESISTANCE_CONDUCTION_L", help="Solve Conduction Thermal Resistance for L | Arguments: | - r_th: Thermal resistance | - k: Thermal conductivity | - a: Area | Returns: f64 | Example: =ENG_HEAT_TRANSFER_THERMAL_RESISTANCE_CONDUCTION_L('...','...','...')")
def e_n_g_h_e_a_t_t_r_a_n_s_f_e_r_t_h_e_r_m_a_l_r_e_s_i_s_t_a_n_c_e_c_o_n_d_u_c_t_i_o_n_l(r_th, k, a):
    """Solve Conduction Thermal Resistance for L | Arguments: | - r_th: Thermal resistance | - k: Thermal conductivity | - a: Area | Returns: f64 | Example: =ENG_HEAT_TRANSFER_THERMAL_RESISTANCE_CONDUCTION_L('...','...','...')"""
    return invoke("equation.solve", {"path_id": "heat_transfer.thermal_resistance_conduction", "target": "L", "R_th": r_th, "k": k, "A": a})

@xloil.func(name="ENG_HEAT_TRANSFER_THERMAL_RESISTANCE_CONDUCTION_R_TH", help="Solve Conduction Thermal Resistance for R_th | Arguments: | - l: Wall thickness | - k: Thermal conductivity | - a: Area | Returns: f64 | Example: =ENG_HEAT_TRANSFER_THERMAL_RESISTANCE_CONDUCTION_R_TH('...','...','...')")
def e_n_g_h_e_a_t_t_r_a_n_s_f_e_r_t_h_e_r_m_a_l_r_e_s_i_s_t_a_n_c_e_c_o_n_d_u_c_t_i_o_n_r_t_h(l, k, a):
    """Solve Conduction Thermal Resistance for R_th | Arguments: | - l: Wall thickness | - k: Thermal conductivity | - a: Area | Returns: f64 | Example: =ENG_HEAT_TRANSFER_THERMAL_RESISTANCE_CONDUCTION_R_TH('...','...','...')"""
    return invoke("equation.solve", {"path_id": "heat_transfer.thermal_resistance_conduction", "target": "R_th", "L": l, "k": k, "A": a})

@xloil.func(name="ENG_HEAT_TRANSFER_THERMAL_RESISTANCE_CONDUCTION_K", help="Solve Conduction Thermal Resistance for k | Arguments: | - r_th: Thermal resistance | - l: Wall thickness | - a: Area | Returns: f64 | Example: =ENG_HEAT_TRANSFER_THERMAL_RESISTANCE_CONDUCTION_K('...','...','...')")
def e_n_g_h_e_a_t_t_r_a_n_s_f_e_r_t_h_e_r_m_a_l_r_e_s_i_s_t_a_n_c_e_c_o_n_d_u_c_t_i_o_n_k(r_th, l, a):
    """Solve Conduction Thermal Resistance for k | Arguments: | - r_th: Thermal resistance | - l: Wall thickness | - a: Area | Returns: f64 | Example: =ENG_HEAT_TRANSFER_THERMAL_RESISTANCE_CONDUCTION_K('...','...','...')"""
    return invoke("equation.solve", {"path_id": "heat_transfer.thermal_resistance_conduction", "target": "k", "R_th": r_th, "L": l, "A": a})

@xloil.func(name="ENG_HEAT_TRANSFER_THERMAL_RESISTANCE_CONVECTION_A", help="Solve Convection Thermal Resistance for A | Arguments: | - r_th: Thermal resistance | - h: Convective heat transfer coefficient | Returns: f64 | Example: =ENG_HEAT_TRANSFER_THERMAL_RESISTANCE_CONVECTION_A('...','...')")
def e_n_g_h_e_a_t_t_r_a_n_s_f_e_r_t_h_e_r_m_a_l_r_e_s_i_s_t_a_n_c_e_c_o_n_v_e_c_t_i_o_n_a(r_th, h):
    """Solve Convection Thermal Resistance for A | Arguments: | - r_th: Thermal resistance | - h: Convective heat transfer coefficient | Returns: f64 | Example: =ENG_HEAT_TRANSFER_THERMAL_RESISTANCE_CONVECTION_A('...','...')"""
    return invoke("equation.solve", {"path_id": "heat_transfer.thermal_resistance_convection", "target": "A", "R_th": r_th, "h": h})

@xloil.func(name="ENG_HEAT_TRANSFER_THERMAL_RESISTANCE_CONVECTION_R_TH", help="Solve Convection Thermal Resistance for R_th | Arguments: | - h: Convective heat transfer coefficient | - a: Surface area | Returns: f64 | Example: =ENG_HEAT_TRANSFER_THERMAL_RESISTANCE_CONVECTION_R_TH('...','...')")
def e_n_g_h_e_a_t_t_r_a_n_s_f_e_r_t_h_e_r_m_a_l_r_e_s_i_s_t_a_n_c_e_c_o_n_v_e_c_t_i_o_n_r_t_h(h, a):
    """Solve Convection Thermal Resistance for R_th | Arguments: | - h: Convective heat transfer coefficient | - a: Surface area | Returns: f64 | Example: =ENG_HEAT_TRANSFER_THERMAL_RESISTANCE_CONVECTION_R_TH('...','...')"""
    return invoke("equation.solve", {"path_id": "heat_transfer.thermal_resistance_convection", "target": "R_th", "h": h, "A": a})

@xloil.func(name="ENG_HEAT_TRANSFER_THERMAL_RESISTANCE_CONVECTION_H", help="Solve Convection Thermal Resistance for h | Arguments: | - r_th: Thermal resistance | - a: Surface area | Returns: f64 | Example: =ENG_HEAT_TRANSFER_THERMAL_RESISTANCE_CONVECTION_H('...','...')")
def e_n_g_h_e_a_t_t_r_a_n_s_f_e_r_t_h_e_r_m_a_l_r_e_s_i_s_t_a_n_c_e_c_o_n_v_e_c_t_i_o_n_h(r_th, a):
    """Solve Convection Thermal Resistance for h | Arguments: | - r_th: Thermal resistance | - a: Surface area | Returns: f64 | Example: =ENG_HEAT_TRANSFER_THERMAL_RESISTANCE_CONVECTION_H('...','...')"""
    return invoke("equation.solve", {"path_id": "heat_transfer.thermal_resistance_convection", "target": "h", "R_th": r_th, "A": a})

@xloil.func(name="ENG_EQUATION_LATEX", help="Read LaTeX display form for an equation | Arguments: | - path_id: Equation path id | Returns: str | Example: =ENG_EQUATION_LATEX('fluids.reynolds_number')")
def e_n_g_e_q_u_a_t_i_o_n_l_a_t_e_x(path_id):
    """Read LaTeX display form for an equation | Arguments: | - path_id: Equation path id | Returns: str | Example: =ENG_EQUATION_LATEX('fluids.reynolds_number')"""
    return invoke("equation.latex", {"path_id": path_id})

@xloil.func(name="ENG_EQUATION_META", help="Read equation metadata (display forms, variables, dimensions, units, targets) | Arguments: | - path_id: Equation path id (for example `fluids.reynolds_number`) | Returns: dict | Example: =ENG_EQUATION_META('fluids.reynolds_number')")
def e_n_g_e_q_u_a_t_i_o_n_m_e_t_a(path_id):
    """Read equation metadata (display forms, variables, dimensions, units, targets) | Arguments: | - path_id: Equation path id (for example `fluids.reynolds_number`) | Returns: dict | Example: =ENG_EQUATION_META('fluids.reynolds_number')"""
    return invoke("equation.meta", {"path_id": path_id})

@xloil.func(name="ENG_EQUATION_NAME", help="Read equation name | Arguments: | - path_id: Equation path id | Returns: str | Example: =ENG_EQUATION_NAME('fluids.reynolds_number')")
def e_n_g_e_q_u_a_t_i_o_n_n_a_m_e(path_id):
    """Read equation name | Arguments: | - path_id: Equation path id | Returns: str | Example: =ENG_EQUATION_NAME('fluids.reynolds_number')"""
    return invoke("equation.name", {"path_id": path_id})

@xloil.func(name="ENG_ROCKETS_CSTAR_IDEAL_C_STAR", help="Solve Ideal Characteristic Velocity for c_star | Arguments: | - r: Gas constant | - t_c: Chamber temperature | - gamma: Specific heat ratio | Returns: f64 | Example: =ENG_ROCKETS_CSTAR_IDEAL_C_STAR('...','...','...')")
def e_n_g_r_o_c_k_e_t_s_c_s_t_a_r_i_d_e_a_l_c_s_t_a_r(r, t_c, gamma):
    """Solve Ideal Characteristic Velocity for c_star | Arguments: | - r: Gas constant | - t_c: Chamber temperature | - gamma: Specific heat ratio | Returns: f64 | Example: =ENG_ROCKETS_CSTAR_IDEAL_C_STAR('...','...','...')"""
    return invoke("equation.solve", {"path_id": "rockets.cstar_ideal", "target": "c_star", "R": r, "T_c": t_c, "gamma": gamma})

@xloil.func(name="ENG_ROCKETS_SPECIFIC_IMPULSE_IDEAL_C_F", help="Solve Ideal Specific Impulse for C_f | Arguments: | - i_sp: Specific impulse | - c_star: Characteristic velocity | Returns: f64 | Example: =ENG_ROCKETS_SPECIFIC_IMPULSE_IDEAL_C_F('...','...')")
def e_n_g_r_o_c_k_e_t_s_s_p_e_c_i_f_i_c_i_m_p_u_l_s_e_i_d_e_a_l_c_f(i_sp, c_star):
    """Solve Ideal Specific Impulse for C_f | Arguments: | - i_sp: Specific impulse | - c_star: Characteristic velocity | Returns: f64 | Example: =ENG_ROCKETS_SPECIFIC_IMPULSE_IDEAL_C_F('...','...')"""
    return invoke("equation.solve", {"path_id": "rockets.specific_impulse_ideal", "target": "C_f", "I_sp": i_sp, "c_star": c_star})

@xloil.func(name="ENG_ROCKETS_SPECIFIC_IMPULSE_IDEAL_I_SP", help="Solve Ideal Specific Impulse for I_sp | Arguments: | - c_f: Thrust coefficient | - c_star: Characteristic velocity | Returns: f64 | Example: =ENG_ROCKETS_SPECIFIC_IMPULSE_IDEAL_I_SP('...','...')")
def e_n_g_r_o_c_k_e_t_s_s_p_e_c_i_f_i_c_i_m_p_u_l_s_e_i_d_e_a_l_i_s_p(c_f, c_star):
    """Solve Ideal Specific Impulse for I_sp | Arguments: | - c_f: Thrust coefficient | - c_star: Characteristic velocity | Returns: f64 | Example: =ENG_ROCKETS_SPECIFIC_IMPULSE_IDEAL_I_SP('...','...')"""
    return invoke("equation.solve", {"path_id": "rockets.specific_impulse_ideal", "target": "I_sp", "C_f": c_f, "c_star": c_star})

@xloil.func(name="ENG_ROCKETS_SPECIFIC_IMPULSE_IDEAL_C_STAR", help="Solve Ideal Specific Impulse for c_star | Arguments: | - i_sp: Specific impulse | - c_f: Thrust coefficient | Returns: f64 | Example: =ENG_ROCKETS_SPECIFIC_IMPULSE_IDEAL_C_STAR('...','...')")
def e_n_g_r_o_c_k_e_t_s_s_p_e_c_i_f_i_c_i_m_p_u_l_s_e_i_d_e_a_l_c_s_t_a_r(i_sp, c_f):
    """Solve Ideal Specific Impulse for c_star | Arguments: | - i_sp: Specific impulse | - c_f: Thrust coefficient | Returns: f64 | Example: =ENG_ROCKETS_SPECIFIC_IMPULSE_IDEAL_C_STAR('...','...')"""
    return invoke("equation.solve", {"path_id": "rockets.specific_impulse_ideal", "target": "c_star", "I_sp": i_sp, "C_f": c_f})

@xloil.func(name="ENG_ROCKETS_THRUST_COEFFICIENT_IDEAL_C_F", help="Solve Ideal Thrust Coefficient for C_f | Arguments: | - gamma: Specific heat ratio | - p_e_p_c: Exit-to-chamber pressure ratio | - p_a_p_c: Ambient-to-chamber pressure ratio | - a_e_a_t: Area expansion ratio | Returns: f64 | Example: =ENG_ROCKETS_THRUST_COEFFICIENT_IDEAL_C_F('...','...','...','...')")
def e_n_g_r_o_c_k_e_t_s_t_h_r_u_s_t_c_o_e_f_f_i_c_i_e_n_t_i_d_e_a_l_c_f(gamma, p_e_p_c, p_a_p_c, a_e_a_t):
    """Solve Ideal Thrust Coefficient for C_f | Arguments: | - gamma: Specific heat ratio | - p_e_p_c: Exit-to-chamber pressure ratio | - p_a_p_c: Ambient-to-chamber pressure ratio | - a_e_a_t: Area expansion ratio | Returns: f64 | Example: =ENG_ROCKETS_THRUST_COEFFICIENT_IDEAL_C_F('...','...','...','...')"""
    return invoke("equation.solve", {"path_id": "rockets.thrust_coefficient_ideal", "target": "C_f", "gamma": gamma, "p_e_p_c": p_e_p_c, "p_a_p_c": p_a_p_c, "A_e_A_t": a_e_a_t})

@xloil.func(name="ENG_ROCKETS_THRUST_FROM_MASS_FLOW_F", help="Solve Thrust From Mass Flow and Effective Exhaust Velocity for F | Arguments: | - m_dot: Mass flow rate | - c_eff: Effective exhaust velocity | Returns: f64 | Example: =ENG_ROCKETS_THRUST_FROM_MASS_FLOW_F('...','...')")
def e_n_g_r_o_c_k_e_t_s_t_h_r_u_s_t_f_r_o_m_m_a_s_s_f_l_o_w_f(m_dot, c_eff):
    """Solve Thrust From Mass Flow and Effective Exhaust Velocity for F | Arguments: | - m_dot: Mass flow rate | - c_eff: Effective exhaust velocity | Returns: f64 | Example: =ENG_ROCKETS_THRUST_FROM_MASS_FLOW_F('...','...')"""
    return invoke("equation.solve", {"path_id": "rockets.thrust_from_mass_flow", "target": "F", "m_dot": m_dot, "c_eff": c_eff})

@xloil.func(name="ENG_ROCKETS_THRUST_FROM_MASS_FLOW_C_EFF", help="Solve Thrust From Mass Flow and Effective Exhaust Velocity for c_eff | Arguments: | - f: Thrust | - m_dot: Mass flow rate | Returns: f64 | Example: =ENG_ROCKETS_THRUST_FROM_MASS_FLOW_C_EFF('...','...')")
def e_n_g_r_o_c_k_e_t_s_t_h_r_u_s_t_f_r_o_m_m_a_s_s_f_l_o_w_c_e_f_f(f, m_dot):
    """Solve Thrust From Mass Flow and Effective Exhaust Velocity for c_eff | Arguments: | - f: Thrust | - m_dot: Mass flow rate | Returns: f64 | Example: =ENG_ROCKETS_THRUST_FROM_MASS_FLOW_C_EFF('...','...')"""
    return invoke("equation.solve", {"path_id": "rockets.thrust_from_mass_flow", "target": "c_eff", "F": f, "m_dot": m_dot})

@xloil.func(name="ENG_ROCKETS_THRUST_FROM_MASS_FLOW_M_DOT", help="Solve Thrust From Mass Flow and Effective Exhaust Velocity for m_dot | Arguments: | - f: Thrust | - c_eff: Effective exhaust velocity | Returns: f64 | Example: =ENG_ROCKETS_THRUST_FROM_MASS_FLOW_M_DOT('...','...')")
def e_n_g_r_o_c_k_e_t_s_t_h_r_u_s_t_f_r_o_m_m_a_s_s_f_l_o_w_m_d_o_t(f, c_eff):
    """Solve Thrust From Mass Flow and Effective Exhaust Velocity for m_dot | Arguments: | - f: Thrust | - c_eff: Effective exhaust velocity | Returns: f64 | Example: =ENG_ROCKETS_THRUST_FROM_MASS_FLOW_M_DOT('...','...')"""
    return invoke("equation.solve", {"path_id": "rockets.thrust_from_mass_flow", "target": "m_dot", "F": f, "c_eff": c_eff})

@xloil.func(name="ENG_STRUCTURES_AXIAL_STRESS_A", help="Solve Axial Normal Stress for A | Arguments: | - sigma: Axial stress | - f: Axial force | Returns: f64 | Example: =ENG_STRUCTURES_AXIAL_STRESS_A('...','...')")
def e_n_g_s_t_r_u_c_t_u_r_e_s_a_x_i_a_l_s_t_r_e_s_s_a(sigma, f):
    """Solve Axial Normal Stress for A | Arguments: | - sigma: Axial stress | - f: Axial force | Returns: f64 | Example: =ENG_STRUCTURES_AXIAL_STRESS_A('...','...')"""
    return invoke("equation.solve", {"path_id": "structures.axial_stress", "target": "A", "sigma": sigma, "F": f})

@xloil.func(name="ENG_STRUCTURES_AXIAL_STRESS_F", help="Solve Axial Normal Stress for F | Arguments: | - sigma: Axial stress | - a: Cross-sectional area | Returns: f64 | Example: =ENG_STRUCTURES_AXIAL_STRESS_F('...','...')")
def e_n_g_s_t_r_u_c_t_u_r_e_s_a_x_i_a_l_s_t_r_e_s_s_f(sigma, a):
    """Solve Axial Normal Stress for F | Arguments: | - sigma: Axial stress | - a: Cross-sectional area | Returns: f64 | Example: =ENG_STRUCTURES_AXIAL_STRESS_F('...','...')"""
    return invoke("equation.solve", {"path_id": "structures.axial_stress", "target": "F", "sigma": sigma, "A": a})

@xloil.func(name="ENG_STRUCTURES_AXIAL_STRESS_SIGMA", help="Solve Axial Normal Stress for sigma | Arguments: | - f: Axial force | - a: Cross-sectional area | Returns: f64 | Example: =ENG_STRUCTURES_AXIAL_STRESS_SIGMA('...','...')")
def e_n_g_s_t_r_u_c_t_u_r_e_s_a_x_i_a_l_s_t_r_e_s_s_s_i_g_m_a(f, a):
    """Solve Axial Normal Stress for sigma | Arguments: | - f: Axial force | - a: Cross-sectional area | Returns: f64 | Example: =ENG_STRUCTURES_AXIAL_STRESS_SIGMA('...','...')"""
    return invoke("equation.solve", {"path_id": "structures.axial_stress", "target": "sigma", "F": f, "A": a})

@xloil.func(name="ENG_STRUCTURES_BEAM_BENDING_STRESS_I", help="Solve Beam Bending Stress for I | Arguments: | - sigma_b: Bending stress | - m: Bending moment | - c: Distance to outer fiber | Returns: f64 | Example: =ENG_STRUCTURES_BEAM_BENDING_STRESS_I('...','...','...')")
def e_n_g_s_t_r_u_c_t_u_r_e_s_b_e_a_m_b_e_n_d_i_n_g_s_t_r_e_s_s_i(sigma_b, m, c):
    """Solve Beam Bending Stress for I | Arguments: | - sigma_b: Bending stress | - m: Bending moment | - c: Distance to outer fiber | Returns: f64 | Example: =ENG_STRUCTURES_BEAM_BENDING_STRESS_I('...','...','...')"""
    return invoke("equation.solve", {"path_id": "structures.beam_bending_stress", "target": "I", "sigma_b": sigma_b, "M": m, "c": c})

@xloil.func(name="ENG_STRUCTURES_BEAM_BENDING_STRESS_M", help="Solve Beam Bending Stress for M | Arguments: | - sigma_b: Bending stress | - c: Distance to outer fiber | - i: Area moment of inertia | Returns: f64 | Example: =ENG_STRUCTURES_BEAM_BENDING_STRESS_M('...','...','...')")
def e_n_g_s_t_r_u_c_t_u_r_e_s_b_e_a_m_b_e_n_d_i_n_g_s_t_r_e_s_s_m(sigma_b, c, i):
    """Solve Beam Bending Stress for M | Arguments: | - sigma_b: Bending stress | - c: Distance to outer fiber | - i: Area moment of inertia | Returns: f64 | Example: =ENG_STRUCTURES_BEAM_BENDING_STRESS_M('...','...','...')"""
    return invoke("equation.solve", {"path_id": "structures.beam_bending_stress", "target": "M", "sigma_b": sigma_b, "c": c, "I": i})

@xloil.func(name="ENG_STRUCTURES_BEAM_BENDING_STRESS_C", help="Solve Beam Bending Stress for c | Arguments: | - sigma_b: Bending stress | - m: Bending moment | - i: Area moment of inertia | Returns: f64 | Example: =ENG_STRUCTURES_BEAM_BENDING_STRESS_C('...','...','...')")
def e_n_g_s_t_r_u_c_t_u_r_e_s_b_e_a_m_b_e_n_d_i_n_g_s_t_r_e_s_s_c(sigma_b, m, i):
    """Solve Beam Bending Stress for c | Arguments: | - sigma_b: Bending stress | - m: Bending moment | - i: Area moment of inertia | Returns: f64 | Example: =ENG_STRUCTURES_BEAM_BENDING_STRESS_C('...','...','...')"""
    return invoke("equation.solve", {"path_id": "structures.beam_bending_stress", "target": "c", "sigma_b": sigma_b, "M": m, "I": i})

@xloil.func(name="ENG_STRUCTURES_BEAM_BENDING_STRESS_SIGMA_B", help="Solve Beam Bending Stress for sigma_b | Arguments: | - m: Bending moment | - c: Distance to outer fiber | - i: Area moment of inertia | Returns: f64 | Example: =ENG_STRUCTURES_BEAM_BENDING_STRESS_SIGMA_B('...','...','...')")
def e_n_g_s_t_r_u_c_t_u_r_e_s_b_e_a_m_b_e_n_d_i_n_g_s_t_r_e_s_s_s_i_g_m_a_b(m, c, i):
    """Solve Beam Bending Stress for sigma_b | Arguments: | - m: Bending moment | - c: Distance to outer fiber | - i: Area moment of inertia | Returns: f64 | Example: =ENG_STRUCTURES_BEAM_BENDING_STRESS_SIGMA_B('...','...','...')"""
    return invoke("equation.solve", {"path_id": "structures.beam_bending_stress", "target": "sigma_b", "M": m, "c": c, "I": i})

@xloil.func(name="ENG_STRUCTURES_EULER_BUCKLING_LOAD_E", help="Solve Euler Buckling Critical Load for E | Arguments: | - p_cr: Critical buckling load | - i: Area moment of inertia | - k: Effective length factor | - l: Unbraced length | Returns: f64 | Example: =ENG_STRUCTURES_EULER_BUCKLING_LOAD_E('...','...','...','...')")
def e_n_g_s_t_r_u_c_t_u_r_e_s_e_u_l_e_r_b_u_c_k_l_i_n_g_l_o_a_d_e(p_cr, i, k, l):
    """Solve Euler Buckling Critical Load for E | Arguments: | - p_cr: Critical buckling load | - i: Area moment of inertia | - k: Effective length factor | - l: Unbraced length | Returns: f64 | Example: =ENG_STRUCTURES_EULER_BUCKLING_LOAD_E('...','...','...','...')"""
    return invoke("equation.solve", {"path_id": "structures.euler_buckling_load", "target": "E", "P_cr": p_cr, "I": i, "K": k, "L": l})

@xloil.func(name="ENG_STRUCTURES_EULER_BUCKLING_LOAD_I", help="Solve Euler Buckling Critical Load for I | Arguments: | - p_cr: Critical buckling load | - e: Elastic modulus | - k: Effective length factor | - l: Unbraced length | Returns: f64 | Example: =ENG_STRUCTURES_EULER_BUCKLING_LOAD_I('...','...','...','...')")
def e_n_g_s_t_r_u_c_t_u_r_e_s_e_u_l_e_r_b_u_c_k_l_i_n_g_l_o_a_d_i(p_cr, e, k, l):
    """Solve Euler Buckling Critical Load for I | Arguments: | - p_cr: Critical buckling load | - e: Elastic modulus | - k: Effective length factor | - l: Unbraced length | Returns: f64 | Example: =ENG_STRUCTURES_EULER_BUCKLING_LOAD_I('...','...','...','...')"""
    return invoke("equation.solve", {"path_id": "structures.euler_buckling_load", "target": "I", "P_cr": p_cr, "E": e, "K": k, "L": l})

@xloil.func(name="ENG_STRUCTURES_EULER_BUCKLING_LOAD_K", help="Solve Euler Buckling Critical Load for K | Arguments: | - p_cr: Critical buckling load | - e: Elastic modulus | - i: Area moment of inertia | - l: Unbraced length | Returns: f64 | Example: =ENG_STRUCTURES_EULER_BUCKLING_LOAD_K('...','...','...','...')")
def e_n_g_s_t_r_u_c_t_u_r_e_s_e_u_l_e_r_b_u_c_k_l_i_n_g_l_o_a_d_k(p_cr, e, i, l):
    """Solve Euler Buckling Critical Load for K | Arguments: | - p_cr: Critical buckling load | - e: Elastic modulus | - i: Area moment of inertia | - l: Unbraced length | Returns: f64 | Example: =ENG_STRUCTURES_EULER_BUCKLING_LOAD_K('...','...','...','...')"""
    return invoke("equation.solve", {"path_id": "structures.euler_buckling_load", "target": "K", "P_cr": p_cr, "E": e, "I": i, "L": l})

@xloil.func(name="ENG_STRUCTURES_EULER_BUCKLING_LOAD_L", help="Solve Euler Buckling Critical Load for L | Arguments: | - p_cr: Critical buckling load | - e: Elastic modulus | - i: Area moment of inertia | - k: Effective length factor | Returns: f64 | Example: =ENG_STRUCTURES_EULER_BUCKLING_LOAD_L('...','...','...','...')")
def e_n_g_s_t_r_u_c_t_u_r_e_s_e_u_l_e_r_b_u_c_k_l_i_n_g_l_o_a_d_l(p_cr, e, i, k):
    """Solve Euler Buckling Critical Load for L | Arguments: | - p_cr: Critical buckling load | - e: Elastic modulus | - i: Area moment of inertia | - k: Effective length factor | Returns: f64 | Example: =ENG_STRUCTURES_EULER_BUCKLING_LOAD_L('...','...','...','...')"""
    return invoke("equation.solve", {"path_id": "structures.euler_buckling_load", "target": "L", "P_cr": p_cr, "E": e, "I": i, "K": k})

@xloil.func(name="ENG_STRUCTURES_EULER_BUCKLING_LOAD_P_CR", help="Solve Euler Buckling Critical Load for P_cr | Arguments: | - e: Elastic modulus | - i: Area moment of inertia | - k: Effective length factor | - l: Unbraced length | Returns: f64 | Example: =ENG_STRUCTURES_EULER_BUCKLING_LOAD_P_CR('...','...','...','...')")
def e_n_g_s_t_r_u_c_t_u_r_e_s_e_u_l_e_r_b_u_c_k_l_i_n_g_l_o_a_d_p_c_r(e, i, k, l):
    """Solve Euler Buckling Critical Load for P_cr | Arguments: | - e: Elastic modulus | - i: Area moment of inertia | - k: Effective length factor | - l: Unbraced length | Returns: f64 | Example: =ENG_STRUCTURES_EULER_BUCKLING_LOAD_P_CR('...','...','...','...')"""
    return invoke("equation.solve", {"path_id": "structures.euler_buckling_load", "target": "P_cr", "E": e, "I": i, "K": k, "L": l})

@xloil.func(name="ENG_STRUCTURES_HOOP_STRESS_P", help="Solve Thin-Wall Hoop Stress for P | Arguments: | - sigma_h: Hoop stress | - r: Mean radius | - t: Wall thickness | Returns: f64 | Example: =ENG_STRUCTURES_HOOP_STRESS_P('...','...','...')")
def e_n_g_s_t_r_u_c_t_u_r_e_s_h_o_o_p_s_t_r_e_s_s_p(sigma_h, r, t):
    """Solve Thin-Wall Hoop Stress for P | Arguments: | - sigma_h: Hoop stress | - r: Mean radius | - t: Wall thickness | Returns: f64 | Example: =ENG_STRUCTURES_HOOP_STRESS_P('...','...','...')"""
    return invoke("equation.solve", {"path_id": "structures.hoop_stress", "target": "P", "sigma_h": sigma_h, "r": r, "t": t})

@xloil.func(name="ENG_STRUCTURES_HOOP_STRESS_R", help="Solve Thin-Wall Hoop Stress for r | Arguments: | - sigma_h: Hoop stress | - p: Internal pressure | - t: Wall thickness | Returns: f64 | Example: =ENG_STRUCTURES_HOOP_STRESS_R('...','...','...')")
def e_n_g_s_t_r_u_c_t_u_r_e_s_h_o_o_p_s_t_r_e_s_s_r(sigma_h, p, t):
    """Solve Thin-Wall Hoop Stress for r | Arguments: | - sigma_h: Hoop stress | - p: Internal pressure | - t: Wall thickness | Returns: f64 | Example: =ENG_STRUCTURES_HOOP_STRESS_R('...','...','...')"""
    return invoke("equation.solve", {"path_id": "structures.hoop_stress", "target": "r", "sigma_h": sigma_h, "P": p, "t": t})

@xloil.func(name="ENG_STRUCTURES_HOOP_STRESS_SIGMA_H", help="Solve Thin-Wall Hoop Stress for sigma_h | Arguments: | - p: Internal pressure | - r: Mean radius | - t: Wall thickness | Returns: f64 | Example: =ENG_STRUCTURES_HOOP_STRESS_SIGMA_H('...','...','...')")
def e_n_g_s_t_r_u_c_t_u_r_e_s_h_o_o_p_s_t_r_e_s_s_s_i_g_m_a_h(p, r, t):
    """Solve Thin-Wall Hoop Stress for sigma_h | Arguments: | - p: Internal pressure | - r: Mean radius | - t: Wall thickness | Returns: f64 | Example: =ENG_STRUCTURES_HOOP_STRESS_SIGMA_H('...','...','...')"""
    return invoke("equation.solve", {"path_id": "structures.hoop_stress", "target": "sigma_h", "P": p, "r": r, "t": t})

@xloil.func(name="ENG_STRUCTURES_HOOP_STRESS_T", help="Solve Thin-Wall Hoop Stress for t | Arguments: | - sigma_h: Hoop stress | - p: Internal pressure | - r: Mean radius | Returns: f64 | Example: =ENG_STRUCTURES_HOOP_STRESS_T('...','...','...')")
def e_n_g_s_t_r_u_c_t_u_r_e_s_h_o_o_p_s_t_r_e_s_s_t(sigma_h, p, r):
    """Solve Thin-Wall Hoop Stress for t | Arguments: | - sigma_h: Hoop stress | - p: Internal pressure | - r: Mean radius | Returns: f64 | Example: =ENG_STRUCTURES_HOOP_STRESS_T('...','...','...')"""
    return invoke("equation.solve", {"path_id": "structures.hoop_stress", "target": "t", "sigma_h": sigma_h, "P": p, "r": r})

@xloil.func(name="ENG_STRUCTURES_LONGITUDINAL_STRESS_THIN_WALL_P", help="Solve Thin-Wall Longitudinal Stress for P | Arguments: | - sigma_l: Longitudinal stress | - r: Mean radius | - t: Wall thickness | Returns: f64 | Example: =ENG_STRUCTURES_LONGITUDINAL_STRESS_THIN_WALL_P('...','...','...')")
def e_n_g_s_t_r_u_c_t_u_r_e_s_l_o_n_g_i_t_u_d_i_n_a_l_s_t_r_e_s_s_t_h_i_n_w_a_l_l_p(sigma_l, r, t):
    """Solve Thin-Wall Longitudinal Stress for P | Arguments: | - sigma_l: Longitudinal stress | - r: Mean radius | - t: Wall thickness | Returns: f64 | Example: =ENG_STRUCTURES_LONGITUDINAL_STRESS_THIN_WALL_P('...','...','...')"""
    return invoke("equation.solve", {"path_id": "structures.longitudinal_stress_thin_wall", "target": "P", "sigma_l": sigma_l, "r": r, "t": t})

@xloil.func(name="ENG_STRUCTURES_LONGITUDINAL_STRESS_THIN_WALL_R", help="Solve Thin-Wall Longitudinal Stress for r | Arguments: | - sigma_l: Longitudinal stress | - p: Internal pressure | - t: Wall thickness | Returns: f64 | Example: =ENG_STRUCTURES_LONGITUDINAL_STRESS_THIN_WALL_R('...','...','...')")
def e_n_g_s_t_r_u_c_t_u_r_e_s_l_o_n_g_i_t_u_d_i_n_a_l_s_t_r_e_s_s_t_h_i_n_w_a_l_l_r(sigma_l, p, t):
    """Solve Thin-Wall Longitudinal Stress for r | Arguments: | - sigma_l: Longitudinal stress | - p: Internal pressure | - t: Wall thickness | Returns: f64 | Example: =ENG_STRUCTURES_LONGITUDINAL_STRESS_THIN_WALL_R('...','...','...')"""
    return invoke("equation.solve", {"path_id": "structures.longitudinal_stress_thin_wall", "target": "r", "sigma_l": sigma_l, "P": p, "t": t})

@xloil.func(name="ENG_STRUCTURES_LONGITUDINAL_STRESS_THIN_WALL_SIGMA_L", help="Solve Thin-Wall Longitudinal Stress for sigma_l | Arguments: | - p: Internal pressure | - r: Mean radius | - t: Wall thickness | Returns: f64 | Example: =ENG_STRUCTURES_LONGITUDINAL_STRESS_THIN_WALL_SIGMA_L('...','...','...')")
def e_n_g_s_t_r_u_c_t_u_r_e_s_l_o_n_g_i_t_u_d_i_n_a_l_s_t_r_e_s_s_t_h_i_n_w_a_l_l_s_i_g_m_a_l(p, r, t):
    """Solve Thin-Wall Longitudinal Stress for sigma_l | Arguments: | - p: Internal pressure | - r: Mean radius | - t: Wall thickness | Returns: f64 | Example: =ENG_STRUCTURES_LONGITUDINAL_STRESS_THIN_WALL_SIGMA_L('...','...','...')"""
    return invoke("equation.solve", {"path_id": "structures.longitudinal_stress_thin_wall", "target": "sigma_l", "P": p, "r": r, "t": t})

@xloil.func(name="ENG_STRUCTURES_LONGITUDINAL_STRESS_THIN_WALL_T", help="Solve Thin-Wall Longitudinal Stress for t | Arguments: | - sigma_l: Longitudinal stress | - p: Internal pressure | - r: Mean radius | Returns: f64 | Example: =ENG_STRUCTURES_LONGITUDINAL_STRESS_THIN_WALL_T('...','...','...')")
def e_n_g_s_t_r_u_c_t_u_r_e_s_l_o_n_g_i_t_u_d_i_n_a_l_s_t_r_e_s_s_t_h_i_n_w_a_l_l_t(sigma_l, p, r):
    """Solve Thin-Wall Longitudinal Stress for t | Arguments: | - sigma_l: Longitudinal stress | - p: Internal pressure | - r: Mean radius | Returns: f64 | Example: =ENG_STRUCTURES_LONGITUDINAL_STRESS_THIN_WALL_T('...','...','...')"""
    return invoke("equation.solve", {"path_id": "structures.longitudinal_stress_thin_wall", "target": "t", "sigma_l": sigma_l, "P": p, "r": r})

@xloil.func(name="ENG_STRUCTURES_SHAFT_TORSION_STRESS_J", help="Solve Circular Shaft Torsion Stress for J | Arguments: | - tau: Shear stress | - t: Torque | - r: Radius | Returns: f64 | Example: =ENG_STRUCTURES_SHAFT_TORSION_STRESS_J('...','...','...')")
def e_n_g_s_t_r_u_c_t_u_r_e_s_s_h_a_f_t_t_o_r_s_i_o_n_s_t_r_e_s_s_j(tau, t, r):
    """Solve Circular Shaft Torsion Stress for J | Arguments: | - tau: Shear stress | - t: Torque | - r: Radius | Returns: f64 | Example: =ENG_STRUCTURES_SHAFT_TORSION_STRESS_J('...','...','...')"""
    return invoke("equation.solve", {"path_id": "structures.shaft_torsion_stress", "target": "J", "tau": tau, "T": t, "r": r})

@xloil.func(name="ENG_STRUCTURES_SHAFT_TORSION_STRESS_T", help="Solve Circular Shaft Torsion Stress for T | Arguments: | - tau: Shear stress | - r: Radius | - j: Polar moment of inertia | Returns: f64 | Example: =ENG_STRUCTURES_SHAFT_TORSION_STRESS_T('...','...','...')")
def e_n_g_s_t_r_u_c_t_u_r_e_s_s_h_a_f_t_t_o_r_s_i_o_n_s_t_r_e_s_s_t(tau, r, j):
    """Solve Circular Shaft Torsion Stress for T | Arguments: | - tau: Shear stress | - r: Radius | - j: Polar moment of inertia | Returns: f64 | Example: =ENG_STRUCTURES_SHAFT_TORSION_STRESS_T('...','...','...')"""
    return invoke("equation.solve", {"path_id": "structures.shaft_torsion_stress", "target": "T", "tau": tau, "r": r, "J": j})

@xloil.func(name="ENG_STRUCTURES_SHAFT_TORSION_STRESS_R", help="Solve Circular Shaft Torsion Stress for r | Arguments: | - tau: Shear stress | - t: Torque | - j: Polar moment of inertia | Returns: f64 | Example: =ENG_STRUCTURES_SHAFT_TORSION_STRESS_R('...','...','...')")
def e_n_g_s_t_r_u_c_t_u_r_e_s_s_h_a_f_t_t_o_r_s_i_o_n_s_t_r_e_s_s_r(tau, t, j):
    """Solve Circular Shaft Torsion Stress for r | Arguments: | - tau: Shear stress | - t: Torque | - j: Polar moment of inertia | Returns: f64 | Example: =ENG_STRUCTURES_SHAFT_TORSION_STRESS_R('...','...','...')"""
    return invoke("equation.solve", {"path_id": "structures.shaft_torsion_stress", "target": "r", "tau": tau, "T": t, "J": j})

@xloil.func(name="ENG_STRUCTURES_SHAFT_TORSION_STRESS_TAU", help="Solve Circular Shaft Torsion Stress for tau | Arguments: | - t: Torque | - r: Radius | - j: Polar moment of inertia | Returns: f64 | Example: =ENG_STRUCTURES_SHAFT_TORSION_STRESS_TAU('...','...','...')")
def e_n_g_s_t_r_u_c_t_u_r_e_s_s_h_a_f_t_t_o_r_s_i_o_n_s_t_r_e_s_s_t_a_u(t, r, j):
    """Solve Circular Shaft Torsion Stress for tau | Arguments: | - t: Torque | - r: Radius | - j: Polar moment of inertia | Returns: f64 | Example: =ENG_STRUCTURES_SHAFT_TORSION_STRESS_TAU('...','...','...')"""
    return invoke("equation.solve", {"path_id": "structures.shaft_torsion_stress", "target": "tau", "T": t, "r": r, "J": j})

@xloil.func(name="ENG_EQUATION_TARGET_COUNT", help="Read equation target count | Arguments: | - path_id: Equation path id | Returns: u64 | Example: =ENG_EQUATION_TARGET_COUNT('structures.hoop_stress')")
def e_n_g_e_q_u_a_t_i_o_n_t_a_r_g_e_t_c_o_u_n_t(path_id):
    """Read equation target count | Arguments: | - path_id: Equation path id | Returns: u64 | Example: =ENG_EQUATION_TARGET_COUNT('structures.hoop_stress')"""
    return invoke("equation.target.count", {"path_id": path_id})

@xloil.func(name="ENG_EQUATION_TARGETS", help="Read solve targets for an equation | Arguments: | - path_id: Equation path id | Returns: list | Example: =ENG_EQUATION_TARGETS('fluids.reynolds_number')")
def e_n_g_e_q_u_a_t_i_o_n_t_a_r_g_e_t_s(path_id):
    """Read solve targets for an equation | Arguments: | - path_id: Equation path id | Returns: list | Example: =ENG_EQUATION_TARGETS('fluids.reynolds_number')"""
    return invoke("equation.targets", {"path_id": path_id})

@xloil.func(name="ENG_EQUATION_TARGETS_TABLE", help="Read equation targets table rows [target, is_default] | Arguments: | - path_id: Equation path id | Returns: list[list] | Example: =ENG_EQUATION_TARGETS_TABLE('structures.hoop_stress')")
def e_n_g_e_q_u_a_t_i_o_n_t_a_r_g_e_t_s_t_a_b_l_e(path_id):
    """Read equation targets table rows [target, is_default] | Arguments: | - path_id: Equation path id | Returns: list[list] | Example: =ENG_EQUATION_TARGETS_TABLE('structures.hoop_stress')"""
    return invoke("equation.targets.table", {"path_id": path_id})

@xloil.func(name="ENG_EQUATION_TARGETS_TEXT", help="Read equation targets as delimited text | Arguments: | - path_id: Equation path id | Returns: str | Example: =ENG_EQUATION_TARGETS_TEXT('structures.hoop_stress')")
def e_n_g_e_q_u_a_t_i_o_n_t_a_r_g_e_t_s_t_e_x_t(path_id):
    """Read equation targets as delimited text | Arguments: | - path_id: Equation path id | Returns: str | Example: =ENG_EQUATION_TARGETS_TEXT('structures.hoop_stress')"""
    return invoke("equation.targets.text", {"path_id": path_id})

@xloil.func(name="ENG_THERMO_IDEAL_GAS_DENSITY_P", help="Solve Ideal Gas Law (Density Form) for P | Arguments: | - rho: Density | - r: Specific gas constant | - t: Absolute temperature | Returns: f64 | Example: =ENG_THERMO_IDEAL_GAS_DENSITY_P('...','...','...')")
def e_n_g_t_h_e_r_m_o_i_d_e_a_l_g_a_s_d_e_n_s_i_t_y_p(rho, r, t):
    """Solve Ideal Gas Law (Density Form) for P | Arguments: | - rho: Density | - r: Specific gas constant | - t: Absolute temperature | Returns: f64 | Example: =ENG_THERMO_IDEAL_GAS_DENSITY_P('...','...','...')"""
    return invoke("equation.solve", {"path_id": "thermo.ideal_gas.density", "target": "P", "rho": rho, "R": r, "T": t})

@xloil.func(name="ENG_THERMO_IDEAL_GAS_DENSITY_R", help="Solve Ideal Gas Law (Density Form) for R | Arguments: | - p: Absolute pressure | - rho: Density | - t: Absolute temperature | Returns: f64 | Example: =ENG_THERMO_IDEAL_GAS_DENSITY_R('...','...','...')")
def e_n_g_t_h_e_r_m_o_i_d_e_a_l_g_a_s_d_e_n_s_i_t_y_r(p, rho, t):
    """Solve Ideal Gas Law (Density Form) for R | Arguments: | - p: Absolute pressure | - rho: Density | - t: Absolute temperature | Returns: f64 | Example: =ENG_THERMO_IDEAL_GAS_DENSITY_R('...','...','...')"""
    return invoke("equation.solve", {"path_id": "thermo.ideal_gas.density", "target": "R", "P": p, "rho": rho, "T": t})

@xloil.func(name="ENG_THERMO_IDEAL_GAS_DENSITY_T", help="Solve Ideal Gas Law (Density Form) for T | Arguments: | - p: Absolute pressure | - rho: Density | - r: Specific gas constant | Returns: f64 | Example: =ENG_THERMO_IDEAL_GAS_DENSITY_T('...','...','...')")
def e_n_g_t_h_e_r_m_o_i_d_e_a_l_g_a_s_d_e_n_s_i_t_y_t(p, rho, r):
    """Solve Ideal Gas Law (Density Form) for T | Arguments: | - p: Absolute pressure | - rho: Density | - r: Specific gas constant | Returns: f64 | Example: =ENG_THERMO_IDEAL_GAS_DENSITY_T('...','...','...')"""
    return invoke("equation.solve", {"path_id": "thermo.ideal_gas.density", "target": "T", "P": p, "rho": rho, "R": r})

@xloil.func(name="ENG_THERMO_IDEAL_GAS_DENSITY_RHO", help="Solve Ideal Gas Law (Density Form) for rho | Arguments: | - p: Absolute pressure | - r: Specific gas constant | - t: Absolute temperature | Returns: f64 | Example: =ENG_THERMO_IDEAL_GAS_DENSITY_RHO('...','...','...')")
def e_n_g_t_h_e_r_m_o_i_d_e_a_l_g_a_s_d_e_n_s_i_t_y_r_h_o(p, r, t):
    """Solve Ideal Gas Law (Density Form) for rho | Arguments: | - p: Absolute pressure | - r: Specific gas constant | - t: Absolute temperature | Returns: f64 | Example: =ENG_THERMO_IDEAL_GAS_DENSITY_RHO('...','...','...')"""
    return invoke("equation.solve", {"path_id": "thermo.ideal_gas.density", "target": "rho", "P": p, "R": r, "T": t})

@xloil.func(name="ENG_THERMO_IDEAL_GAS_MASS_VOLUME_P", help="Solve Ideal Gas Law (Mass-Volume Form) for P | Arguments: | - v: Control-volume | - m: Gas mass | - r: Specific gas constant | - t: Absolute temperature | Returns: f64 | Example: =ENG_THERMO_IDEAL_GAS_MASS_VOLUME_P('...','...','...','...')")
def e_n_g_t_h_e_r_m_o_i_d_e_a_l_g_a_s_m_a_s_s_v_o_l_u_m_e_p(v, m, r, t):
    """Solve Ideal Gas Law (Mass-Volume Form) for P | Arguments: | - v: Control-volume | - m: Gas mass | - r: Specific gas constant | - t: Absolute temperature | Returns: f64 | Example: =ENG_THERMO_IDEAL_GAS_MASS_VOLUME_P('...','...','...','...')"""
    return invoke("equation.solve", {"path_id": "thermo.ideal_gas.mass_volume", "target": "P", "V": v, "m": m, "R": r, "T": t})

@xloil.func(name="ENG_THERMO_IDEAL_GAS_MASS_VOLUME_R", help="Solve Ideal Gas Law (Mass-Volume Form) for R | Arguments: | - p: Absolute pressure | - v: Control-volume | - m: Gas mass | - t: Absolute temperature | Returns: f64 | Example: =ENG_THERMO_IDEAL_GAS_MASS_VOLUME_R('...','...','...','...')")
def e_n_g_t_h_e_r_m_o_i_d_e_a_l_g_a_s_m_a_s_s_v_o_l_u_m_e_r(p, v, m, t):
    """Solve Ideal Gas Law (Mass-Volume Form) for R | Arguments: | - p: Absolute pressure | - v: Control-volume | - m: Gas mass | - t: Absolute temperature | Returns: f64 | Example: =ENG_THERMO_IDEAL_GAS_MASS_VOLUME_R('...','...','...','...')"""
    return invoke("equation.solve", {"path_id": "thermo.ideal_gas.mass_volume", "target": "R", "P": p, "V": v, "m": m, "T": t})

@xloil.func(name="ENG_THERMO_IDEAL_GAS_MASS_VOLUME_T", help="Solve Ideal Gas Law (Mass-Volume Form) for T | Arguments: | - p: Absolute pressure | - v: Control-volume | - m: Gas mass | - r: Specific gas constant | Returns: f64 | Example: =ENG_THERMO_IDEAL_GAS_MASS_VOLUME_T('...','...','...','...')")
def e_n_g_t_h_e_r_m_o_i_d_e_a_l_g_a_s_m_a_s_s_v_o_l_u_m_e_t(p, v, m, r):
    """Solve Ideal Gas Law (Mass-Volume Form) for T | Arguments: | - p: Absolute pressure | - v: Control-volume | - m: Gas mass | - r: Specific gas constant | Returns: f64 | Example: =ENG_THERMO_IDEAL_GAS_MASS_VOLUME_T('...','...','...','...')"""
    return invoke("equation.solve", {"path_id": "thermo.ideal_gas.mass_volume", "target": "T", "P": p, "V": v, "m": m, "R": r})

@xloil.func(name="ENG_THERMO_IDEAL_GAS_MASS_VOLUME_V", help="Solve Ideal Gas Law (Mass-Volume Form) for V | Arguments: | - p: Absolute pressure | - m: Gas mass | - r: Specific gas constant | - t: Absolute temperature | Returns: f64 | Example: =ENG_THERMO_IDEAL_GAS_MASS_VOLUME_V('...','...','...','...')")
def e_n_g_t_h_e_r_m_o_i_d_e_a_l_g_a_s_m_a_s_s_v_o_l_u_m_e_v(p, m, r, t):
    """Solve Ideal Gas Law (Mass-Volume Form) for V | Arguments: | - p: Absolute pressure | - m: Gas mass | - r: Specific gas constant | - t: Absolute temperature | Returns: f64 | Example: =ENG_THERMO_IDEAL_GAS_MASS_VOLUME_V('...','...','...','...')"""
    return invoke("equation.solve", {"path_id": "thermo.ideal_gas.mass_volume", "target": "V", "P": p, "m": m, "R": r, "T": t})

@xloil.func(name="ENG_THERMO_IDEAL_GAS_MASS_VOLUME_M", help="Solve Ideal Gas Law (Mass-Volume Form) for m | Arguments: | - p: Absolute pressure | - v: Control-volume | - r: Specific gas constant | - t: Absolute temperature | Returns: f64 | Example: =ENG_THERMO_IDEAL_GAS_MASS_VOLUME_M('...','...','...','...')")
def e_n_g_t_h_e_r_m_o_i_d_e_a_l_g_a_s_m_a_s_s_v_o_l_u_m_e_m(p, v, r, t):
    """Solve Ideal Gas Law (Mass-Volume Form) for m | Arguments: | - p: Absolute pressure | - v: Control-volume | - r: Specific gas constant | - t: Absolute temperature | Returns: f64 | Example: =ENG_THERMO_IDEAL_GAS_MASS_VOLUME_M('...','...','...','...')"""
    return invoke("equation.solve", {"path_id": "thermo.ideal_gas.mass_volume", "target": "m", "P": p, "V": v, "R": r, "T": t})

@xloil.func(name="ENG_EQUATION_UNICODE", help="Read Unicode display form for an equation | Arguments: | - path_id: Equation path id | Returns: str | Example: =ENG_EQUATION_UNICODE('fluids.reynolds_number')")
def e_n_g_e_q_u_a_t_i_o_n_u_n_i_c_o_d_e(path_id):
    """Read Unicode display form for an equation | Arguments: | - path_id: Equation path id | Returns: str | Example: =ENG_EQUATION_UNICODE('fluids.reynolds_number')"""
    return invoke("equation.unicode", {"path_id": path_id})

@xloil.func(name="ENG_EQUATION_VARIABLE_COUNT", help="Read equation variable count | Arguments: | - path_id: Equation path id | Returns: u64 | Example: =ENG_EQUATION_VARIABLE_COUNT('structures.hoop_stress')")
def e_n_g_e_q_u_a_t_i_o_n_v_a_r_i_a_b_l_e_c_o_u_n_t(path_id):
    """Read equation variable count | Arguments: | - path_id: Equation path id | Returns: u64 | Example: =ENG_EQUATION_VARIABLE_COUNT('structures.hoop_stress')"""
    return invoke("equation.variable.count", {"path_id": path_id})

@xloil.func(name="ENG_EQUATION_VARIABLES", help="Read variable metadata for an equation | Arguments: | - path_id: Equation path id | Returns: list | Example: =ENG_EQUATION_VARIABLES('fluids.reynolds_number')")
def e_n_g_e_q_u_a_t_i_o_n_v_a_r_i_a_b_l_e_s(path_id):
    """Read variable metadata for an equation | Arguments: | - path_id: Equation path id | Returns: list | Example: =ENG_EQUATION_VARIABLES('fluids.reynolds_number')"""
    return invoke("equation.variables", {"path_id": path_id})

@xloil.func(name="ENG_EQUATION_VARIABLES_TABLE", help="Read equation variable table rows [variable, default_unit] | Arguments: | - path_id: Equation path id | Returns: list[list] | Example: =ENG_EQUATION_VARIABLES_TABLE('structures.hoop_stress')")
def e_n_g_e_q_u_a_t_i_o_n_v_a_r_i_a_b_l_e_s_t_a_b_l_e(path_id):
    """Read equation variable table rows [variable, default_unit] | Arguments: | - path_id: Equation path id | Returns: list[list] | Example: =ENG_EQUATION_VARIABLES_TABLE('structures.hoop_stress')"""
    return invoke("equation.variables.table", {"path_id": path_id})

@xloil.func(name="ENG_EQUATION_VARIABLES_TEXT", help="Read equation variables as delimited text | Arguments: | - path_id: Equation path id | Returns: str | Example: =ENG_EQUATION_VARIABLES_TEXT('structures.hoop_stress')")
def e_n_g_e_q_u_a_t_i_o_n_v_a_r_i_a_b_l_e_s_t_e_x_t(path_id):
    """Read equation variables as delimited text | Arguments: | - path_id: Equation path id | Returns: str | Example: =ENG_EQUATION_VARIABLES_TEXT('structures.hoop_stress')"""
    return invoke("equation.variables.text", {"path_id": path_id})

@xloil.func(name="ENG_FAMILY_IDEAL_GAS_DENSITY_P", help="Solve Ideal Gas Law variant Density Form for P | Arguments: | - rho: Density | - r: Specific gas constant | - t: Absolute temperature | Returns: f64 | Example: =ENG_FAMILY_IDEAL_GAS_DENSITY_P('...','...','...')")
def e_n_g_f_a_m_i_l_y_i_d_e_a_l_g_a_s_d_e_n_s_i_t_y_p(rho, r, t):
    """Solve Ideal Gas Law variant Density Form for P | Arguments: | - rho: Density | - r: Specific gas constant | - t: Absolute temperature | Returns: f64 | Example: =ENG_FAMILY_IDEAL_GAS_DENSITY_P('...','...','...')"""
    return invoke("equation.solve", {"path_id": "thermo.ideal_gas.density", "target": "P", "rho": rho, "R": r, "T": t})

@xloil.func(name="ENG_FAMILY_IDEAL_GAS_DENSITY_R", help="Solve Ideal Gas Law variant Density Form for R | Arguments: | - p: Absolute pressure | - rho: Density | - t: Absolute temperature | Returns: f64 | Example: =ENG_FAMILY_IDEAL_GAS_DENSITY_R('...','...','...')")
def e_n_g_f_a_m_i_l_y_i_d_e_a_l_g_a_s_d_e_n_s_i_t_y_r(p, rho, t):
    """Solve Ideal Gas Law variant Density Form for R | Arguments: | - p: Absolute pressure | - rho: Density | - t: Absolute temperature | Returns: f64 | Example: =ENG_FAMILY_IDEAL_GAS_DENSITY_R('...','...','...')"""
    return invoke("equation.solve", {"path_id": "thermo.ideal_gas.density", "target": "R", "P": p, "rho": rho, "T": t})

@xloil.func(name="ENG_FAMILY_IDEAL_GAS_DENSITY_T", help="Solve Ideal Gas Law variant Density Form for T | Arguments: | - p: Absolute pressure | - rho: Density | - r: Specific gas constant | Returns: f64 | Example: =ENG_FAMILY_IDEAL_GAS_DENSITY_T('...','...','...')")
def e_n_g_f_a_m_i_l_y_i_d_e_a_l_g_a_s_d_e_n_s_i_t_y_t(p, rho, r):
    """Solve Ideal Gas Law variant Density Form for T | Arguments: | - p: Absolute pressure | - rho: Density | - r: Specific gas constant | Returns: f64 | Example: =ENG_FAMILY_IDEAL_GAS_DENSITY_T('...','...','...')"""
    return invoke("equation.solve", {"path_id": "thermo.ideal_gas.density", "target": "T", "P": p, "rho": rho, "R": r})

@xloil.func(name="ENG_FAMILY_IDEAL_GAS_DENSITY_RHO", help="Solve Ideal Gas Law variant Density Form for rho | Arguments: | - p: Absolute pressure | - r: Specific gas constant | - t: Absolute temperature | Returns: f64 | Example: =ENG_FAMILY_IDEAL_GAS_DENSITY_RHO('...','...','...')")
def e_n_g_f_a_m_i_l_y_i_d_e_a_l_g_a_s_d_e_n_s_i_t_y_r_h_o(p, r, t):
    """Solve Ideal Gas Law variant Density Form for rho | Arguments: | - p: Absolute pressure | - r: Specific gas constant | - t: Absolute temperature | Returns: f64 | Example: =ENG_FAMILY_IDEAL_GAS_DENSITY_RHO('...','...','...')"""
    return invoke("equation.solve", {"path_id": "thermo.ideal_gas.density", "target": "rho", "P": p, "R": r, "T": t})

@xloil.func(name="ENG_FAMILY_IDEAL_GAS_MASS_VOLUME_P", help="Solve Ideal Gas Law variant Mass-Volume Form for P | Arguments: | - v: Control-volume | - m: Gas mass | - r: Specific gas constant | - t: Absolute temperature | Returns: f64 | Example: =ENG_FAMILY_IDEAL_GAS_MASS_VOLUME_P('...','...','...','...')")
def e_n_g_f_a_m_i_l_y_i_d_e_a_l_g_a_s_m_a_s_s_v_o_l_u_m_e_p(v, m, r, t):
    """Solve Ideal Gas Law variant Mass-Volume Form for P | Arguments: | - v: Control-volume | - m: Gas mass | - r: Specific gas constant | - t: Absolute temperature | Returns: f64 | Example: =ENG_FAMILY_IDEAL_GAS_MASS_VOLUME_P('...','...','...','...')"""
    return invoke("equation.solve", {"path_id": "thermo.ideal_gas.mass_volume", "target": "P", "V": v, "m": m, "R": r, "T": t})

@xloil.func(name="ENG_FAMILY_IDEAL_GAS_MASS_VOLUME_R", help="Solve Ideal Gas Law variant Mass-Volume Form for R | Arguments: | - p: Absolute pressure | - v: Control-volume | - m: Gas mass | - t: Absolute temperature | Returns: f64 | Example: =ENG_FAMILY_IDEAL_GAS_MASS_VOLUME_R('...','...','...','...')")
def e_n_g_f_a_m_i_l_y_i_d_e_a_l_g_a_s_m_a_s_s_v_o_l_u_m_e_r(p, v, m, t):
    """Solve Ideal Gas Law variant Mass-Volume Form for R | Arguments: | - p: Absolute pressure | - v: Control-volume | - m: Gas mass | - t: Absolute temperature | Returns: f64 | Example: =ENG_FAMILY_IDEAL_GAS_MASS_VOLUME_R('...','...','...','...')"""
    return invoke("equation.solve", {"path_id": "thermo.ideal_gas.mass_volume", "target": "R", "P": p, "V": v, "m": m, "T": t})

@xloil.func(name="ENG_FAMILY_IDEAL_GAS_MASS_VOLUME_T", help="Solve Ideal Gas Law variant Mass-Volume Form for T | Arguments: | - p: Absolute pressure | - v: Control-volume | - m: Gas mass | - r: Specific gas constant | Returns: f64 | Example: =ENG_FAMILY_IDEAL_GAS_MASS_VOLUME_T('...','...','...','...')")
def e_n_g_f_a_m_i_l_y_i_d_e_a_l_g_a_s_m_a_s_s_v_o_l_u_m_e_t(p, v, m, r):
    """Solve Ideal Gas Law variant Mass-Volume Form for T | Arguments: | - p: Absolute pressure | - v: Control-volume | - m: Gas mass | - r: Specific gas constant | Returns: f64 | Example: =ENG_FAMILY_IDEAL_GAS_MASS_VOLUME_T('...','...','...','...')"""
    return invoke("equation.solve", {"path_id": "thermo.ideal_gas.mass_volume", "target": "T", "P": p, "V": v, "m": m, "R": r})

@xloil.func(name="ENG_FAMILY_IDEAL_GAS_MASS_VOLUME_V", help="Solve Ideal Gas Law variant Mass-Volume Form for V | Arguments: | - p: Absolute pressure | - m: Gas mass | - r: Specific gas constant | - t: Absolute temperature | Returns: f64 | Example: =ENG_FAMILY_IDEAL_GAS_MASS_VOLUME_V('...','...','...','...')")
def e_n_g_f_a_m_i_l_y_i_d_e_a_l_g_a_s_m_a_s_s_v_o_l_u_m_e_v(p, m, r, t):
    """Solve Ideal Gas Law variant Mass-Volume Form for V | Arguments: | - p: Absolute pressure | - m: Gas mass | - r: Specific gas constant | - t: Absolute temperature | Returns: f64 | Example: =ENG_FAMILY_IDEAL_GAS_MASS_VOLUME_V('...','...','...','...')"""
    return invoke("equation.solve", {"path_id": "thermo.ideal_gas.mass_volume", "target": "V", "P": p, "m": m, "R": r, "T": t})

@xloil.func(name="ENG_FAMILY_IDEAL_GAS_MASS_VOLUME_M", help="Solve Ideal Gas Law variant Mass-Volume Form for m | Arguments: | - p: Absolute pressure | - v: Control-volume | - r: Specific gas constant | - t: Absolute temperature | Returns: f64 | Example: =ENG_FAMILY_IDEAL_GAS_MASS_VOLUME_M('...','...','...','...')")
def e_n_g_f_a_m_i_l_y_i_d_e_a_l_g_a_s_m_a_s_s_v_o_l_u_m_e_m(p, v, r, t):
    """Solve Ideal Gas Law variant Mass-Volume Form for m | Arguments: | - p: Absolute pressure | - v: Control-volume | - r: Specific gas constant | - t: Absolute temperature | Returns: f64 | Example: =ENG_FAMILY_IDEAL_GAS_MASS_VOLUME_M('...','...','...','...')"""
    return invoke("equation.solve", {"path_id": "thermo.ideal_gas.mass_volume", "target": "m", "P": p, "V": v, "R": r, "T": t})

@xloil.func(name="ENG_FLUID_PROPERTIES", help="Read supported properties for a fluid | Arguments: | - key: Fluid key/alias | Returns: list | Example: =ENG_FLUID_PROPERTIES('H2O')")
def e_n_g_f_l_u_i_d_p_r_o_p_e_r_t_i_e_s(key):
    """Read supported properties for a fluid | Arguments: | - key: Fluid key/alias | Returns: list | Example: =ENG_FLUID_PROPERTIES('H2O')"""
    return invoke("meta.get", {"entity": "fluid", "field": "supported_properties", "key": key})

@xloil.func(name="ENG_FLUID_PROP", help="Binding-friendly fluid property lookup | Arguments: | - fluid: Fluid key/name | - state_prop_1: State input key 1 | - state_value_1: State input value 1 | - state_prop_2: State input key 2 | - state_value_2: State input value 2 | - out_prop: Output property key | Returns: f64 | Example: =ENG_FLUID_PROP('H2O','T','300 K','P','1 bar','rho')")
def e_n_g_f_l_u_i_d_p_r_o_p(fluid, state_prop_1, state_value_1, state_prop_2, state_value_2, out_prop):
    """Binding-friendly fluid property lookup | Arguments: | - fluid: Fluid key/name | - state_prop_1: State input key 1 | - state_value_1: State input value 1 | - state_prop_2: State input key 2 | - state_value_2: State input value 2 | - out_prop: Output property key | Returns: f64 | Example: =ENG_FLUID_PROP('H2O','T','300 K','P','1 bar','rho')"""
    return invoke("fluid.prop", {"fluid": fluid, "in1_key": state_prop_1, "in1_value": state_value_1, "in2_key": state_prop_2, "in2_value": state_value_2, "out_prop": out_prop})

@xloil.func(name="ENG_FLUID_PROPERTIES_TABLE", help="Read fluid property table rows [property, default_unit] | Arguments: | - key: Fluid key/alias | Returns: list[list] | Example: =ENG_FLUID_PROPERTIES_TABLE('H2O')")
def e_n_g_f_l_u_i_d_p_r_o_p_e_r_t_i_e_s_t_a_b_l_e(key):
    """Read fluid property table rows [property, default_unit] | Arguments: | - key: Fluid key/alias | Returns: list[list] | Example: =ENG_FLUID_PROPERTIES_TABLE('H2O')"""
    return invoke("fluid.properties.table", {"key": key})

@xloil.func(name="ENG_FLUID_PROPERTIES_TEXT", help="Read fluid properties as delimited text | Arguments: | - key: Fluid key/alias | Returns: str | Example: =ENG_FLUID_PROPERTIES_TEXT('H2O')")
def e_n_g_f_l_u_i_d_p_r_o_p_e_r_t_i_e_s_t_e_x_t(key):
    """Read fluid properties as delimited text | Arguments: | - key: Fluid key/alias | Returns: str | Example: =ENG_FLUID_PROPERTIES_TEXT('H2O')"""
    return invoke("fluid.properties.text", {"key": key})

@xloil.func(name="ENG_FLUID_PROPERTY_COUNT", help="Read fluid property count | Arguments: | - key: Fluid key/alias | Returns: u64 | Example: =ENG_FLUID_PROPERTY_COUNT('H2O')")
def e_n_g_f_l_u_i_d_p_r_o_p_e_r_t_y_c_o_u_n_t(key):
    """Read fluid property count | Arguments: | - key: Fluid key/alias | Returns: u64 | Example: =ENG_FLUID_PROPERTY_COUNT('H2O')"""
    return invoke("fluid.property.count", {"key": key})

@xloil.func(name="ENG_FORMAT", help="Convert a numeric value from input units to output units (with dimensional checks) | Arguments: | - value: Input value in `in_unit` | - in_unit: Input unit expression (for example Pa, m, psia, kg/(m*s)) | - out_unit: Requested output unit expression | Returns: f64 | Example: =ENG_FORMAT(2500000,'Pa','psia')")
def e_n_g_f_o_r_m_a_t(value, in_unit, out_unit):
    """Convert a numeric value from input units to output units (with dimensional checks) | Arguments: | - value: Input value in `in_unit` | - in_unit: Input unit expression (for example Pa, m, psia, kg/(m*s)) | - out_unit: Requested output unit expression | Returns: f64 | Example: =ENG_FORMAT(2500000,'Pa','psia')"""
    return invoke("format.value", {"value": value, "in_unit": in_unit, "out_unit": out_unit})

@xloil.func(name="ENG_MATERIAL_PROPERTIES", help="Read available properties for a material | Arguments: | - key: Material key/alias | Returns: list | Example: =ENG_MATERIAL_PROPERTIES('stainless_304')")
def e_n_g_m_a_t_e_r_i_a_l_p_r_o_p_e_r_t_i_e_s(key):
    """Read available properties for a material | Arguments: | - key: Material key/alias | Returns: list | Example: =ENG_MATERIAL_PROPERTIES('stainless_304')"""
    return invoke("meta.get", {"entity": "material", "field": "properties", "key": key})

@xloil.func(name="ENG_MAT_PROP", help="Binding-friendly material property lookup | Arguments: | - material: Material key/name | - property_key: Property key | - temperature: Temperature input | Returns: f64 | Example: =ENG_MAT_PROP('stainless_304','elastic_modulus','350 K')")
def e_n_g_m_a_t_p_r_o_p(material, property_key, temperature):
    """Binding-friendly material property lookup | Arguments: | - material: Material key/name | - property_key: Property key | - temperature: Temperature input | Returns: f64 | Example: =ENG_MAT_PROP('stainless_304','elastic_modulus','350 K')"""
    return invoke("material.prop", {"material": material, "property": property_key, "temperature": temperature})

@xloil.func(name="ENG_MATERIAL_PROPERTIES_TABLE", help="Read material property table rows [property, unit] | Arguments: | - key: Material key/alias | Returns: list[list] | Example: =ENG_MATERIAL_PROPERTIES_TABLE('stainless_304')")
def e_n_g_m_a_t_e_r_i_a_l_p_r_o_p_e_r_t_i_e_s_t_a_b_l_e(key):
    """Read material property table rows [property, unit] | Arguments: | - key: Material key/alias | Returns: list[list] | Example: =ENG_MATERIAL_PROPERTIES_TABLE('stainless_304')"""
    return invoke("material.properties.table", {"key": key})

@xloil.func(name="ENG_MATERIAL_PROPERTIES_TEXT", help="Read material properties as delimited text | Arguments: | - key: Material key/alias | Returns: str | Example: =ENG_MATERIAL_PROPERTIES_TEXT('stainless_304')")
def e_n_g_m_a_t_e_r_i_a_l_p_r_o_p_e_r_t_i_e_s_t_e_x_t(key):
    """Read material properties as delimited text | Arguments: | - key: Material key/alias | Returns: str | Example: =ENG_MATERIAL_PROPERTIES_TEXT('stainless_304')"""
    return invoke("material.properties.text", {"key": key})

@xloil.func(name="ENG_MATERIAL_PROPERTY_COUNT", help="Read material property count | Arguments: | - key: Material key/alias | Returns: u64 | Example: =ENG_MATERIAL_PROPERTY_COUNT('stainless_304')")
def e_n_g_m_a_t_e_r_i_a_l_p_r_o_p_e_r_t_y_c_o_u_n_t(key):
    """Read material property count | Arguments: | - key: Material key/alias | Returns: u64 | Example: =ENG_MATERIAL_PROPERTY_COUNT('stainless_304')"""
    return invoke("material.property.count", {"key": key})

@xloil.func(name="ENG_META", help="General metadata helper for bindings | Arguments: | - entity: equation | device | fluid | material | constant | - key: Entity id/key | - field: Metadata field to read | Returns: scalar|list|dict | Example: =ENG_META('equation','structures.hoop_stress','ascii')")
def e_n_g_m_e_t_a(entity, key, field):
    """General metadata helper for bindings | Arguments: | - entity: equation | device | fluid | material | constant | - key: Entity id/key | - field: Metadata field to read | Returns: scalar|list|dict | Example: =ENG_META('equation','structures.hoop_stress','ascii')"""
    return invoke("meta.get", {"entity": entity, "key": key, "field": field})

