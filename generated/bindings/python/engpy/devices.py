from ._runtime import invoke

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

