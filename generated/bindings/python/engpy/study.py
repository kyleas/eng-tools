from ._runtime import invoke

def isentropic_m_to_p_p0_table(gamma=None, start=None, end=None, count=None, branch=None):
    """Study table for isentropic device Mach -> p/p0

Args:
  gamma: Specific heat ratio
  start: Mach start
  end: Mach end
  count: Sample count
  branch: Optional branch
Returns:
  dict(table, spill)
"""
    return invoke("study.device.isentropic_m_to_p_p0.table", {**({"gamma": gamma} if gamma is not None else {}), **({"start": start} if start is not None else {}), **({"end": end} if end is not None else {}), **({"count": count} if count is not None else {}), **({"branch": branch} if branch is not None else {})})

def normal_shock_table(gamma=None, start=None, end=None, count=None):
    """Study table for normal-shock device over M1

Args:
  gamma: Specific heat ratio
  start: M1 start
  end: M1 end
  count: Sample count
Returns:
  dict(table, spill)
"""
    return invoke("study.device.normal_shock.table", {**({"gamma": gamma} if gamma is not None else {}), **({"start": start} if start is not None else {}), **({"end": end} if end is not None else {}), **({"count": count} if count is not None else {})})

def nozzle_flow_table(gamma=None, start=None, end=None, count=None, branch=None):
    """Study table for nozzle-flow device over area ratio

Args:
  gamma: Specific heat ratio
  start: Area-ratio start
  end: Area-ratio end
  count: Sample count
  branch: Branch (subsonic/supersonic)
Returns:
  dict(table, spill)
"""
    return invoke("study.device.nozzle_flow.table", {**({"gamma": gamma} if gamma is not None else {}), **({"start": start} if start is not None else {}), **({"end": end} if end is not None else {}), **({"count": count} if count is not None else {}), **({"branch": branch} if branch is not None else {})})

def equation_sweep_table(path_id=None, target=None, sweep_variable=None, start=None, end=None, count=None, spacing=None, branch=None):
    """Generic equation study sweep table (1D axis)

Args:
  path_id: Equation path id
  target: Solve target key
  sweep_variable: Variable to sweep
  start: Sweep start
  end: Sweep end
  count: Number of samples
  spacing: Spacing mode (linear or logspace)
  branch: Optional branch
Returns:
  dict(table, spill)
"""
    return invoke("study.equation.sweep", {**({"path_id": path_id} if path_id is not None else {}), **({"target": target} if target is not None else {}), **({"sweep_variable": sweep_variable} if sweep_variable is not None else {}), **({"start": start} if start is not None else {}), **({"end": end} if end is not None else {}), **({"count": count} if count is not None else {}), **({"spacing": spacing} if spacing is not None else {}), **({"branch": branch} if branch is not None else {})})

def nozzle_normal_shock_workflow_table(gamma=None, start=None, end=None, count=None, branch=None):
    """Study table for nozzle + normal-shock chained workflow

Args:
  gamma: Specific heat ratio
  start: Area-ratio start
  end: Area-ratio end
  count: Sample count
  branch: Nozzle branch (subsonic/supersonic)
Returns:
  dict(table, spill)
"""
    return invoke("study.workflow.nozzle_normal_shock.table", {**({"gamma": gamma} if gamma is not None else {}), **({"start": start} if start is not None else {}), **({"end": end} if end is not None else {}), **({"count": count} if count is not None else {}), **({"branch": branch} if branch is not None else {})})

