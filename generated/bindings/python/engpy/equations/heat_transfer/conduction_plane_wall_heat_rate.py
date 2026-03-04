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
    return invoke("equation.solve", {"Q_dot": q_dot, "k": k, "T_h": t_h, "T_c": t_c, "L": l, "path_id": "heat_transfer.conduction_plane_wall_heat_rate", "target": "A"})

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
    return invoke("equation.solve", {"Q_dot": q_dot, "k": k, "A": a, "T_h": t_h, "T_c": t_c, "path_id": "heat_transfer.conduction_plane_wall_heat_rate", "target": "L"})

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
    return invoke("equation.solve", {"k": k, "A": a, "T_h": t_h, "T_c": t_c, "L": l, "path_id": "heat_transfer.conduction_plane_wall_heat_rate", "target": "Q_dot"})

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
    return invoke("equation.solve", {"Q_dot": q_dot, "k": k, "A": a, "T_h": t_h, "L": l, "path_id": "heat_transfer.conduction_plane_wall_heat_rate", "target": "T_c"})

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
    return invoke("equation.solve", {"Q_dot": q_dot, "k": k, "A": a, "T_c": t_c, "L": l, "path_id": "heat_transfer.conduction_plane_wall_heat_rate", "target": "T_h"})

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
    return invoke("equation.solve", {"Q_dot": q_dot, "A": a, "T_h": t_h, "T_c": t_c, "L": l, "path_id": "heat_transfer.conduction_plane_wall_heat_rate", "target": "k"})

__all__ = [
    "solve_a",
    "solve_l",
    "solve_q_dot",
    "solve_t_c",
    "solve_t_h",
    "solve_k",
]
