from engpy._runtime import invoke

def solve_a(sigma, f):
    """Solve Axial Normal Stress for A

Args:
  sigma: Axial stress
  f: Axial force
Returns:
  f64
"""
    return invoke("equation.solve", {"path_id": "structures.axial_stress", "target": "A", "sigma": sigma, "F": f})

def solve_f(sigma, a):
    """Solve Axial Normal Stress for F

Args:
  sigma: Axial stress
  a: Cross-sectional area
Returns:
  f64
"""
    return invoke("equation.solve", {"path_id": "structures.axial_stress", "target": "F", "sigma": sigma, "A": a})

def solve_sigma(f, a):
    """Solve Axial Normal Stress for sigma

Args:
  f: Axial force
  a: Cross-sectional area
Returns:
  f64
"""
    return invoke("equation.solve", {"path_id": "structures.axial_stress", "target": "sigma", "F": f, "A": a})

def solve_i(sigma_b, m, c):
    """Solve Beam Bending Stress for I

Args:
  sigma_b: Bending stress
  m: Bending moment
  c: Distance to outer fiber
Returns:
  f64
"""
    return invoke("equation.solve", {"path_id": "structures.beam_bending_stress", "target": "I", "sigma_b": sigma_b, "M": m, "c": c})

def solve_m(sigma_b, c, i):
    """Solve Beam Bending Stress for M

Args:
  sigma_b: Bending stress
  c: Distance to outer fiber
  i: Area moment of inertia
Returns:
  f64
"""
    return invoke("equation.solve", {"path_id": "structures.beam_bending_stress", "target": "M", "sigma_b": sigma_b, "c": c, "I": i})

def solve_c(sigma_b, m, i):
    """Solve Beam Bending Stress for c

Args:
  sigma_b: Bending stress
  m: Bending moment
  i: Area moment of inertia
Returns:
  f64
"""
    return invoke("equation.solve", {"path_id": "structures.beam_bending_stress", "target": "c", "sigma_b": sigma_b, "M": m, "I": i})

def solve_sigma_b(m, c, i):
    """Solve Beam Bending Stress for sigma_b

Args:
  m: Bending moment
  c: Distance to outer fiber
  i: Area moment of inertia
Returns:
  f64
"""
    return invoke("equation.solve", {"path_id": "structures.beam_bending_stress", "target": "sigma_b", "M": m, "c": c, "I": i})

def solve_e(p_cr, i, k, l):
    """Solve Euler Buckling Critical Load for E

Args:
  p_cr: Critical buckling load
  i: Area moment of inertia
  k: Effective length factor
  l: Unbraced length
Returns:
  f64
"""
    return invoke("equation.solve", {"path_id": "structures.euler_buckling_load", "target": "E", "P_cr": p_cr, "I": i, "K": k, "L": l})

def solve_i(p_cr, e, k, l):
    """Solve Euler Buckling Critical Load for I

Args:
  p_cr: Critical buckling load
  e: Elastic modulus
  k: Effective length factor
  l: Unbraced length
Returns:
  f64
"""
    return invoke("equation.solve", {"path_id": "structures.euler_buckling_load", "target": "I", "P_cr": p_cr, "E": e, "K": k, "L": l})

def solve_k(p_cr, e, i, l):
    """Solve Euler Buckling Critical Load for K

Args:
  p_cr: Critical buckling load
  e: Elastic modulus
  i: Area moment of inertia
  l: Unbraced length
Returns:
  f64
"""
    return invoke("equation.solve", {"path_id": "structures.euler_buckling_load", "target": "K", "P_cr": p_cr, "E": e, "I": i, "L": l})

def solve_l(p_cr, e, i, k):
    """Solve Euler Buckling Critical Load for L

Args:
  p_cr: Critical buckling load
  e: Elastic modulus
  i: Area moment of inertia
  k: Effective length factor
Returns:
  f64
"""
    return invoke("equation.solve", {"path_id": "structures.euler_buckling_load", "target": "L", "P_cr": p_cr, "E": e, "I": i, "K": k})

def solve_p_cr(e, i, k, l):
    """Solve Euler Buckling Critical Load for P_cr

Args:
  e: Elastic modulus
  i: Area moment of inertia
  k: Effective length factor
  l: Unbraced length
Returns:
  f64
"""
    return invoke("equation.solve", {"path_id": "structures.euler_buckling_load", "target": "P_cr", "E": e, "I": i, "K": k, "L": l})

def solve_p(sigma_h, r, t):
    """Solve Thin-Wall Hoop Stress for P

Args:
  sigma_h: Hoop stress
  r: Mean radius
  t: Wall thickness
Returns:
  f64
"""
    return invoke("equation.solve", {"path_id": "structures.hoop_stress", "target": "P", "sigma_h": sigma_h, "r": r, "t": t})

def solve_r(sigma_h, p, t):
    """Solve Thin-Wall Hoop Stress for r

Args:
  sigma_h: Hoop stress
  p: Internal pressure
  t: Wall thickness
Returns:
  f64
"""
    return invoke("equation.solve", {"path_id": "structures.hoop_stress", "target": "r", "sigma_h": sigma_h, "P": p, "t": t})

def solve_sigma_h(p, r, t):
    """Solve Thin-Wall Hoop Stress for sigma_h

Args:
  p: Internal pressure
  r: Mean radius
  t: Wall thickness
Returns:
  f64
"""
    return invoke("equation.solve", {"path_id": "structures.hoop_stress", "target": "sigma_h", "P": p, "r": r, "t": t})

def solve_t(sigma_h, p, r):
    """Solve Thin-Wall Hoop Stress for t

Args:
  sigma_h: Hoop stress
  p: Internal pressure
  r: Mean radius
Returns:
  f64
"""
    return invoke("equation.solve", {"path_id": "structures.hoop_stress", "target": "t", "sigma_h": sigma_h, "P": p, "r": r})

def solve_p(sigma_l, r, t):
    """Solve Thin-Wall Longitudinal Stress for P

Args:
  sigma_l: Longitudinal stress
  r: Mean radius
  t: Wall thickness
Returns:
  f64
"""
    return invoke("equation.solve", {"path_id": "structures.longitudinal_stress_thin_wall", "target": "P", "sigma_l": sigma_l, "r": r, "t": t})

def solve_r(sigma_l, p, t):
    """Solve Thin-Wall Longitudinal Stress for r

Args:
  sigma_l: Longitudinal stress
  p: Internal pressure
  t: Wall thickness
Returns:
  f64
"""
    return invoke("equation.solve", {"path_id": "structures.longitudinal_stress_thin_wall", "target": "r", "sigma_l": sigma_l, "P": p, "t": t})

def solve_sigma_l(p, r, t):
    """Solve Thin-Wall Longitudinal Stress for sigma_l

Args:
  p: Internal pressure
  r: Mean radius
  t: Wall thickness
Returns:
  f64
"""
    return invoke("equation.solve", {"path_id": "structures.longitudinal_stress_thin_wall", "target": "sigma_l", "P": p, "r": r, "t": t})

def solve_t(sigma_l, p, r):
    """Solve Thin-Wall Longitudinal Stress for t

Args:
  sigma_l: Longitudinal stress
  p: Internal pressure
  r: Mean radius
Returns:
  f64
"""
    return invoke("equation.solve", {"path_id": "structures.longitudinal_stress_thin_wall", "target": "t", "sigma_l": sigma_l, "P": p, "r": r})

def solve_j(tau, t, r):
    """Solve Circular Shaft Torsion Stress for J

Args:
  tau: Shear stress
  t: Torque
  r: Radius
Returns:
  f64
"""
    return invoke("equation.solve", {"path_id": "structures.shaft_torsion_stress", "target": "J", "tau": tau, "T": t, "r": r})

def solve_t(tau, r, j):
    """Solve Circular Shaft Torsion Stress for T

Args:
  tau: Shear stress
  r: Radius
  j: Polar moment of inertia
Returns:
  f64
"""
    return invoke("equation.solve", {"path_id": "structures.shaft_torsion_stress", "target": "T", "tau": tau, "r": r, "J": j})

def solve_r(tau, t, j):
    """Solve Circular Shaft Torsion Stress for r

Args:
  tau: Shear stress
  t: Torque
  j: Polar moment of inertia
Returns:
  f64
"""
    return invoke("equation.solve", {"path_id": "structures.shaft_torsion_stress", "target": "r", "tau": tau, "T": t, "J": j})

def solve_tau(t, r, j):
    """Solve Circular Shaft Torsion Stress for tau

Args:
  t: Torque
  r: Radius
  j: Polar moment of inertia
Returns:
  f64
"""
    return invoke("equation.solve", {"path_id": "structures.shaft_torsion_stress", "target": "tau", "T": t, "r": r, "J": j})

