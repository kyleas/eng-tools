from . import axial_stress
from . import beam_bending_stress
from . import euler_buckling_load
from . import hoop_stress
from . import longitudinal_stress_thin_wall
from . import shaft_torsion_stress
from .axial_stress import solve_a as solve_a
from .beam_bending_stress import solve_c as solve_c
from .euler_buckling_load import solve_e as solve_e
from .axial_stress import solve_f as solve_f
from .shaft_torsion_stress import solve_j as solve_j
from .euler_buckling_load import solve_k as solve_k
from .euler_buckling_load import solve_l as solve_l
from .beam_bending_stress import solve_m as solve_m
from .euler_buckling_load import solve_p_cr as solve_p_cr
from .axial_stress import solve_sigma as solve_sigma
from .beam_bending_stress import solve_sigma_b as solve_sigma_b
from .hoop_stress import solve_sigma_h as solve_sigma_h
from .longitudinal_stress_thin_wall import solve_sigma_l as solve_sigma_l
from .shaft_torsion_stress import solve_tau as solve_tau

# Omitted legacy aliases due to collisions:
# - solve_i
# - solve_p
# - solve_r
# - solve_t

__all__ = [
    "axial_stress",
    "beam_bending_stress",
    "euler_buckling_load",
    "hoop_stress",
    "longitudinal_stress_thin_wall",
    "shaft_torsion_stress",
]
