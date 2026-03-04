from engpy._runtime import invoke

def solve_delta_t_lm(delta_t_1, delta_t_2):
    """Solve Log-Mean Temperature Difference for delta_T_lm

Args:
  delta_t_1: End temperature difference 1
  delta_t_2: End temperature difference 2
Returns:
  f64
"""
    return invoke("equation.solve", {"delta_T_1": delta_t_1, "delta_T_2": delta_t_2, "path_id": "heat_transfer.log_mean_temperature_difference", "target": "delta_T_lm"})

__all__ = [
    "solve_delta_t_lm",
]
