from engpy._runtime import invoke

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
    return invoke("equation.solve", {"Q_dot": q_dot, "h": h, "T_s": t_s, "T_inf": t_inf, "path_id": "heat_transfer.convection_heat_rate", "target": "A"})

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
    return invoke("equation.solve", {"h": h, "A": a, "T_s": t_s, "T_inf": t_inf, "path_id": "heat_transfer.convection_heat_rate", "target": "Q_dot"})

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
    return invoke("equation.solve", {"Q_dot": q_dot, "h": h, "A": a, "T_s": t_s, "path_id": "heat_transfer.convection_heat_rate", "target": "T_inf"})

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
    return invoke("equation.solve", {"Q_dot": q_dot, "h": h, "A": a, "T_inf": t_inf, "path_id": "heat_transfer.convection_heat_rate", "target": "T_s"})

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
    return invoke("equation.solve", {"Q_dot": q_dot, "A": a, "T_s": t_s, "T_inf": t_inf, "path_id": "heat_transfer.convection_heat_rate", "target": "h"})

__all__ = [
    "solve_a",
    "solve_q_dot",
    "solve_t_inf",
    "solve_t_s",
    "solve_h",
]
