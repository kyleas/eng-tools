from ._runtime import invoke

def fluid_prop(fluid, in1_key, in1_value, in2_key, in2_value, out_prop):
    """Binding-friendly fluid property lookup

Args:
  fluid: Fluid key/name
  in1_key: State input key 1
  in1_value: State input value 1
  in2_key: State input key 2
  in2_value: State input value 2
  out_prop: Output property key
Returns:
  f64
"""
    return invoke("fluid.prop", {"fluid": fluid, "in1_key": in1_key, "in1_value": in1_value, "in2_key": in2_key, "in2_value": in2_value, "out_prop": out_prop})

