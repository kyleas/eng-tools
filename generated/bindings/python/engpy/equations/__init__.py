from engpy._runtime import invoke

from . import compressible
from . import families
from . import fluids
from . import heat_transfer
from . import meta
from . import rockets
from . import structures
from . import thermo

def solve(path_id, target, **inputs):
    """Generic equation solve fallback.

Args:
  path_id: equation id (for example 'fluids.reynolds_number')
  target: solve target variable key
  **inputs: named solve inputs
Returns:
  f64
"""
    args = {"path_id": path_id, "target": target}
    args.update(inputs)
    return invoke("equation.solve", args)

__all__ = [
    "compressible",
    "families",
    "fluids",
    "heat_transfer",
    "meta",
    "rockets",
    "structures",
    "thermo",
    "solve",
]
