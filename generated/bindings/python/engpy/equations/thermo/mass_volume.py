from engpy._runtime import invoke

def solve_p(v, m, r, t):
    """Solve Ideal Gas Law (Mass-Volume Form) for P

Args:
  v: Control-volume
  m: Gas mass
  r: Specific gas constant
  t: Absolute temperature
Returns:
  f64
"""
    return invoke("equation.solve", {"V": v, "m": m, "R": r, "T": t, "path_id": "thermo.ideal_gas.mass_volume", "target": "P"})

def solve_r(p, v, m, t):
    """Solve Ideal Gas Law (Mass-Volume Form) for R

Args:
  p: Absolute pressure
  v: Control-volume
  m: Gas mass
  t: Absolute temperature
Returns:
  f64
"""
    return invoke("equation.solve", {"P": p, "V": v, "m": m, "T": t, "path_id": "thermo.ideal_gas.mass_volume", "target": "R"})

def solve_t(p, v, m, r):
    """Solve Ideal Gas Law (Mass-Volume Form) for T

Args:
  p: Absolute pressure
  v: Control-volume
  m: Gas mass
  r: Specific gas constant
Returns:
  f64
"""
    return invoke("equation.solve", {"P": p, "V": v, "m": m, "R": r, "path_id": "thermo.ideal_gas.mass_volume", "target": "T"})

def solve_v(p, m, r, t):
    """Solve Ideal Gas Law (Mass-Volume Form) for V

Args:
  p: Absolute pressure
  m: Gas mass
  r: Specific gas constant
  t: Absolute temperature
Returns:
  f64
"""
    return invoke("equation.solve", {"P": p, "m": m, "R": r, "T": t, "path_id": "thermo.ideal_gas.mass_volume", "target": "V"})

def solve_m(p, v, r, t):
    """Solve Ideal Gas Law (Mass-Volume Form) for m

Args:
  p: Absolute pressure
  v: Control-volume
  r: Specific gas constant
  t: Absolute temperature
Returns:
  f64
"""
    return invoke("equation.solve", {"P": p, "V": v, "R": r, "T": t, "path_id": "thermo.ideal_gas.mass_volume", "target": "m"})

__all__ = [
    "solve_p",
    "solve_r",
    "solve_t",
    "solve_v",
    "solve_m",
]
