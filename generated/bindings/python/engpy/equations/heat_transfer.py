from engpy._runtime import invoke

def solve_a(q_dot, k, t_h, t_c, l):
    """Solve Plane-Wall Conduction Heat Rate for A

Args:
  q_dot: Heat transfer rate
  k: Thermal conductivity
  t_h: Hot-side temperature
  t_c: Cold-side temperature
  l: Wall thickness
Returns:
  f64
"""
    return invoke("equation.solve", {"path_id": "heat_transfer.conduction_plane_wall_heat_rate", "target": "A", "Q_dot": q_dot, "k": k, "T_h": t_h, "T_c": t_c, "L": l})

def solve_l(q_dot, k, a, t_h, t_c):
    """Solve Plane-Wall Conduction Heat Rate for L

Args:
  q_dot: Heat transfer rate
  k: Thermal conductivity
  a: Area normal to heat flow
  t_h: Hot-side temperature
  t_c: Cold-side temperature
Returns:
  f64
"""
    return invoke("equation.solve", {"path_id": "heat_transfer.conduction_plane_wall_heat_rate", "target": "L", "Q_dot": q_dot, "k": k, "A": a, "T_h": t_h, "T_c": t_c})

def solve_q_dot(k, a, t_h, t_c, l):
    """Solve Plane-Wall Conduction Heat Rate for Q_dot

Args:
  k: Thermal conductivity
  a: Area normal to heat flow
  t_h: Hot-side temperature
  t_c: Cold-side temperature
  l: Wall thickness
Returns:
  f64
"""
    return invoke("equation.solve", {"path_id": "heat_transfer.conduction_plane_wall_heat_rate", "target": "Q_dot", "k": k, "A": a, "T_h": t_h, "T_c": t_c, "L": l})

def solve_t_c(q_dot, k, a, t_h, l):
    """Solve Plane-Wall Conduction Heat Rate for T_c

Args:
  q_dot: Heat transfer rate
  k: Thermal conductivity
  a: Area normal to heat flow
  t_h: Hot-side temperature
  l: Wall thickness
Returns:
  f64
"""
    return invoke("equation.solve", {"path_id": "heat_transfer.conduction_plane_wall_heat_rate", "target": "T_c", "Q_dot": q_dot, "k": k, "A": a, "T_h": t_h, "L": l})

def solve_t_h(q_dot, k, a, t_c, l):
    """Solve Plane-Wall Conduction Heat Rate for T_h

Args:
  q_dot: Heat transfer rate
  k: Thermal conductivity
  a: Area normal to heat flow
  t_c: Cold-side temperature
  l: Wall thickness
Returns:
  f64
"""
    return invoke("equation.solve", {"path_id": "heat_transfer.conduction_plane_wall_heat_rate", "target": "T_h", "Q_dot": q_dot, "k": k, "A": a, "T_c": t_c, "L": l})

def solve_k(q_dot, a, t_h, t_c, l):
    """Solve Plane-Wall Conduction Heat Rate for k

Args:
  q_dot: Heat transfer rate
  a: Area normal to heat flow
  t_h: Hot-side temperature
  t_c: Cold-side temperature
  l: Wall thickness
Returns:
  f64
"""
    return invoke("equation.solve", {"path_id": "heat_transfer.conduction_plane_wall_heat_rate", "target": "k", "Q_dot": q_dot, "A": a, "T_h": t_h, "T_c": t_c, "L": l})

def solve_a(q_dot, h, t_s, t_inf):
    """Solve Convection Heat Transfer Rate for A

Args:
  q_dot: Heat transfer rate
  h: Convective heat transfer coefficient
  t_s: Surface temperature
  t_inf: Free-stream temperature
Returns:
  f64
"""
    return invoke("equation.solve", {"path_id": "heat_transfer.convection_heat_rate", "target": "A", "Q_dot": q_dot, "h": h, "T_s": t_s, "T_inf": t_inf})

def solve_q_dot(h, a, t_s, t_inf):
    """Solve Convection Heat Transfer Rate for Q_dot

Args:
  h: Convective heat transfer coefficient
  a: Surface area
  t_s: Surface temperature
  t_inf: Free-stream temperature
Returns:
  f64
"""
    return invoke("equation.solve", {"path_id": "heat_transfer.convection_heat_rate", "target": "Q_dot", "h": h, "A": a, "T_s": t_s, "T_inf": t_inf})

def solve_t_inf(q_dot, h, a, t_s):
    """Solve Convection Heat Transfer Rate for T_inf

Args:
  q_dot: Heat transfer rate
  h: Convective heat transfer coefficient
  a: Surface area
  t_s: Surface temperature
Returns:
  f64
"""
    return invoke("equation.solve", {"path_id": "heat_transfer.convection_heat_rate", "target": "T_inf", "Q_dot": q_dot, "h": h, "A": a, "T_s": t_s})

def solve_t_s(q_dot, h, a, t_inf):
    """Solve Convection Heat Transfer Rate for T_s

Args:
  q_dot: Heat transfer rate
  h: Convective heat transfer coefficient
  a: Surface area
  t_inf: Free-stream temperature
Returns:
  f64
"""
    return invoke("equation.solve", {"path_id": "heat_transfer.convection_heat_rate", "target": "T_s", "Q_dot": q_dot, "h": h, "A": a, "T_inf": t_inf})

def solve_h(q_dot, a, t_s, t_inf):
    """Solve Convection Heat Transfer Rate for h

Args:
  q_dot: Heat transfer rate
  a: Surface area
  t_s: Surface temperature
  t_inf: Free-stream temperature
Returns:
  f64
"""
    return invoke("equation.solve", {"path_id": "heat_transfer.convection_heat_rate", "target": "h", "Q_dot": q_dot, "A": a, "T_s": t_s, "T_inf": t_inf})

def solve_delta_t_lm(delta_t_1, delta_t_2):
    """Solve Log-Mean Temperature Difference for delta_T_lm

Args:
  delta_t_1: End temperature difference 1
  delta_t_2: End temperature difference 2
Returns:
  f64
"""
    return invoke("equation.solve", {"path_id": "heat_transfer.log_mean_temperature_difference", "target": "delta_T_lm", "delta_T_1": delta_t_1, "delta_T_2": delta_t_2})

def solve_a(r_th, l, k):
    """Solve Conduction Thermal Resistance for A

Args:
  r_th: Thermal resistance
  l: Wall thickness
  k: Thermal conductivity
Returns:
  f64
"""
    return invoke("equation.solve", {"path_id": "heat_transfer.thermal_resistance_conduction", "target": "A", "R_th": r_th, "L": l, "k": k})

def solve_l(r_th, k, a):
    """Solve Conduction Thermal Resistance for L

Args:
  r_th: Thermal resistance
  k: Thermal conductivity
  a: Area
Returns:
  f64
"""
    return invoke("equation.solve", {"path_id": "heat_transfer.thermal_resistance_conduction", "target": "L", "R_th": r_th, "k": k, "A": a})

def solve_r_th(l, k, a):
    """Solve Conduction Thermal Resistance for R_th

Args:
  l: Wall thickness
  k: Thermal conductivity
  a: Area
Returns:
  f64
"""
    return invoke("equation.solve", {"path_id": "heat_transfer.thermal_resistance_conduction", "target": "R_th", "L": l, "k": k, "A": a})

def solve_k(r_th, l, a):
    """Solve Conduction Thermal Resistance for k

Args:
  r_th: Thermal resistance
  l: Wall thickness
  a: Area
Returns:
  f64
"""
    return invoke("equation.solve", {"path_id": "heat_transfer.thermal_resistance_conduction", "target": "k", "R_th": r_th, "L": l, "A": a})

def solve_a(r_th, h):
    """Solve Convection Thermal Resistance for A

Args:
  r_th: Thermal resistance
  h: Convective heat transfer coefficient
Returns:
  f64
"""
    return invoke("equation.solve", {"path_id": "heat_transfer.thermal_resistance_convection", "target": "A", "R_th": r_th, "h": h})

def solve_r_th(h, a):
    """Solve Convection Thermal Resistance for R_th

Args:
  h: Convective heat transfer coefficient
  a: Surface area
Returns:
  f64
"""
    return invoke("equation.solve", {"path_id": "heat_transfer.thermal_resistance_convection", "target": "R_th", "h": h, "A": a})

def solve_h(r_th, a):
    """Solve Convection Thermal Resistance for h

Args:
  r_th: Thermal resistance
  a: Surface area
Returns:
  f64
"""
    return invoke("equation.solve", {"path_id": "heat_transfer.thermal_resistance_convection", "target": "h", "R_th": r_th, "A": a})

