from engpy._runtime import invoke

def density_solve_p(rho, r, t):
    """Solve Ideal Gas Law variant Density Form for P

Args:
  rho: Density
  r: Specific gas constant
  t: Absolute temperature
Returns:
  f64
"""
    return invoke("equation.solve", {"path_id": "thermo.ideal_gas.density", "target": "P", "rho": rho, "R": r, "T": t})

def density_solve_r(p, rho, t):
    """Solve Ideal Gas Law variant Density Form for R

Args:
  p: Absolute pressure
  rho: Density
  t: Absolute temperature
Returns:
  f64
"""
    return invoke("equation.solve", {"path_id": "thermo.ideal_gas.density", "target": "R", "P": p, "rho": rho, "T": t})

def density_solve_t(p, rho, r):
    """Solve Ideal Gas Law variant Density Form for T

Args:
  p: Absolute pressure
  rho: Density
  r: Specific gas constant
Returns:
  f64
"""
    return invoke("equation.solve", {"path_id": "thermo.ideal_gas.density", "target": "T", "P": p, "rho": rho, "R": r})

def density_solve_rho(p, r, t):
    """Solve Ideal Gas Law variant Density Form for rho

Args:
  p: Absolute pressure
  r: Specific gas constant
  t: Absolute temperature
Returns:
  f64
"""
    return invoke("equation.solve", {"path_id": "thermo.ideal_gas.density", "target": "rho", "P": p, "R": r, "T": t})

def mass_volume_solve_p(v, m, r, t):
    """Solve Ideal Gas Law variant Mass-Volume Form for P

Args:
  v: Control-volume
  m: Gas mass
  r: Specific gas constant
  t: Absolute temperature
Returns:
  f64
"""
    return invoke("equation.solve", {"path_id": "thermo.ideal_gas.mass_volume", "target": "P", "V": v, "m": m, "R": r, "T": t})

def mass_volume_solve_r(p, v, m, t):
    """Solve Ideal Gas Law variant Mass-Volume Form for R

Args:
  p: Absolute pressure
  v: Control-volume
  m: Gas mass
  t: Absolute temperature
Returns:
  f64
"""
    return invoke("equation.solve", {"path_id": "thermo.ideal_gas.mass_volume", "target": "R", "P": p, "V": v, "m": m, "T": t})

def mass_volume_solve_t(p, v, m, r):
    """Solve Ideal Gas Law variant Mass-Volume Form for T

Args:
  p: Absolute pressure
  v: Control-volume
  m: Gas mass
  r: Specific gas constant
Returns:
  f64
"""
    return invoke("equation.solve", {"path_id": "thermo.ideal_gas.mass_volume", "target": "T", "P": p, "V": v, "m": m, "R": r})

def mass_volume_solve_v(p, m, r, t):
    """Solve Ideal Gas Law variant Mass-Volume Form for V

Args:
  p: Absolute pressure
  m: Gas mass
  r: Specific gas constant
  t: Absolute temperature
Returns:
  f64
"""
    return invoke("equation.solve", {"path_id": "thermo.ideal_gas.mass_volume", "target": "V", "P": p, "m": m, "R": r, "T": t})

def mass_volume_solve_m(p, v, r, t):
    """Solve Ideal Gas Law variant Mass-Volume Form for m

Args:
  p: Absolute pressure
  v: Control-volume
  r: Specific gas constant
  t: Absolute temperature
Returns:
  f64
"""
    return invoke("equation.solve", {"path_id": "thermo.ideal_gas.mass_volume", "target": "m", "P": p, "V": v, "R": r, "T": t})

