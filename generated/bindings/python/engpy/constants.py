from ._runtime import invoke

def get_constant(key):
    """Get constant value from registry

Args:
  key: Constant key
Returns:
  f64
"""
    return invoke("constant.get", {"key": key})

