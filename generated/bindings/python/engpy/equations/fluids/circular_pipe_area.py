from engpy._runtime import invoke

def solve_a(d):
    """Solve Circular Pipe Flow Area for A

Args:
  d: Diameter
Returns:
  f64
"""
    return invoke("equation.solve", {"D": d, "path_id": "fluids.circular_pipe_area", "target": "A"})

def solve_d(a):
    """Solve Circular Pipe Flow Area for D

Args:
  a: Area
Returns:
  f64
"""
    return invoke("equation.solve", {"A": a, "path_id": "fluids.circular_pipe_area", "target": "D"})

__all__ = [
    "solve_a",
    "solve_d",
]
