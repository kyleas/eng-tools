# Heat Transfer

## Equation Summary

<style>
.equation-summary-cards { display: grid; grid-template-columns: 1fr; gap: 1.2rem; margin: 0.75rem 0 1.25rem; }
.equation-summary-card { position: relative; border: 1px solid var(--table-border-color); border-radius: 12px; padding: 1.15rem 1.25rem 1rem; background: rgba(255,255,255,0.03); box-shadow: 0 1px 0 rgba(255,255,255,0.02) inset; }
.equation-summary-card:hover { border-color: rgba(255,255,255,0.35); background: rgba(255,255,255,0.04); }
.equation-summary-card-link { position: absolute; inset: 0; z-index: 1; border-radius: 12px; }
.equation-summary-header { position: relative; z-index: 2; margin-bottom: 0.35rem; }
.equation-summary-title { font-size: 1.24rem; line-height: 1.3; font-weight: 650; margin: 0 0 0.35rem; }
.equation-summary-title a { position: relative; z-index: 2; }
.equation-summary-path { font-family: var(--mono-font); font-size: 0.88rem; opacity: 0.75; margin: 0 0 0.7rem; overflow-wrap: anywhere; }
.equation-summary-latex { position: relative; z-index: 2; text-align: center; font-size: 1.55rem; line-height: 1.5; margin: 0.5rem 0 0.75rem; padding: 0.35rem 0.5rem; overflow-x: auto; }
.equation-summary-meta { position: relative; z-index: 2; display: flex; flex-wrap: wrap; gap: 0.35rem 0.45rem; align-items: center; margin-top: 0.35rem; }
.equation-summary-chip { display: inline-flex; align-items: center; gap: 0.25rem; padding: 0.2rem 0.45rem; border: 1px solid var(--table-border-color); border-radius: 999px; font-size: 0.82rem; line-height: 1.2; background: rgba(255,255,255,0.02); }
.equation-summary-chip-label { opacity: 0.75; font-weight: 500; }
.equation-summary-chip-value { overflow-wrap: anywhere; }
.equation-summary-chip-value code { white-space: nowrap; }
.equation-summary-targets { display: flex; flex-wrap: wrap; gap: 0.25rem; }
.equation-summary-targets code { display: inline-block; padding: 0.05rem 0.35rem; border-radius: 8px; background: rgba(255,255,255,0.07); }
@media (min-width: 1200px) { .equation-summary-cards { grid-template-columns: repeat(2, minmax(0, 1fr)); gap: 1.35rem; } }
@media (max-width: 900px) { .equation-summary-card { padding: 1rem; } .equation-summary-title { font-size: 1.12rem; } .equation-summary-latex { font-size: 1.3rem; } }
</style>
<div class="equation-summary-cards">
<article class="equation-summary-card">
<a class="equation-summary-card-link" href="./conduction_plane_wall_heat_rate.md" aria-label="Open Plane-Wall Conduction Heat Rate"></a>
<div class="equation-summary-header">
<h3 class="equation-summary-title"><a href="./conduction_plane_wall_heat_rate.md">Plane-Wall Conduction Heat Rate</a></h3>
<div class="equation-summary-path"><code>heat_transfer.conduction_plane_wall_heat_rate</code></div>
</div>
<div class="equation-summary-latex">\(\dot{Q} = k A \frac{T_h - T_c}{L}\)</div>
<div class="equation-summary-meta">
<div class="equation-summary-chip"><span class="equation-summary-chip-label">Targets</span><span class="equation-summary-chip-value equation-summary-targets"><code>A</code>, <code>L</code>, <code>Q_dot</code>, <code>T_c</code>, <code>T_h</code>, <code>k</code></span></div><div class="equation-summary-chip"><span class="equation-summary-chip-label">Default</span><span class="equation-summary-chip-value"><code>-</code></span></div>
</div>
</article>
<article class="equation-summary-card">
<a class="equation-summary-card-link" href="./convection_heat_rate.md" aria-label="Open Convection Heat Transfer Rate"></a>
<div class="equation-summary-header">
<h3 class="equation-summary-title"><a href="./convection_heat_rate.md">Convection Heat Transfer Rate</a></h3>
<div class="equation-summary-path"><code>heat_transfer.convection_heat_rate</code></div>
</div>
<div class="equation-summary-latex">\(\dot{Q} = h A (T_s - T_\infty)\)</div>
<div class="equation-summary-meta">
<div class="equation-summary-chip"><span class="equation-summary-chip-label">Targets</span><span class="equation-summary-chip-value equation-summary-targets"><code>A</code>, <code>Q_dot</code>, <code>T_inf</code>, <code>T_s</code>, <code>h</code></span></div><div class="equation-summary-chip"><span class="equation-summary-chip-label">Default</span><span class="equation-summary-chip-value"><code>-</code></span></div>
</div>
</article>
<article class="equation-summary-card">
<a class="equation-summary-card-link" href="./log_mean_temperature_difference.md" aria-label="Open Log-Mean Temperature Difference"></a>
<div class="equation-summary-header">
<h3 class="equation-summary-title"><a href="./log_mean_temperature_difference.md">Log-Mean Temperature Difference</a></h3>
<div class="equation-summary-path"><code>heat_transfer.log_mean_temperature_difference</code></div>
</div>
<div class="equation-summary-latex">\(\Delta T_{lm} = \frac{\Delta T_1 - \Delta T_2}{\ln(\Delta T_1 / \Delta T_2)}\)</div>
<div class="equation-summary-meta">
<div class="equation-summary-chip"><span class="equation-summary-chip-label">Targets</span><span class="equation-summary-chip-value equation-summary-targets"><code>delta_T_lm</code></span></div><div class="equation-summary-chip"><span class="equation-summary-chip-label">Default</span><span class="equation-summary-chip-value"><code>-</code></span></div>
</div>
</article>
<article class="equation-summary-card">
<a class="equation-summary-card-link" href="./thermal_resistance_conduction.md" aria-label="Open Conduction Thermal Resistance"></a>
<div class="equation-summary-header">
<h3 class="equation-summary-title"><a href="./thermal_resistance_conduction.md">Conduction Thermal Resistance</a></h3>
<div class="equation-summary-path"><code>heat_transfer.thermal_resistance_conduction</code></div>
</div>
<div class="equation-summary-latex">\(R_{th} = \frac{L}{k A}\)</div>
<div class="equation-summary-meta">
<div class="equation-summary-chip"><span class="equation-summary-chip-label">Targets</span><span class="equation-summary-chip-value equation-summary-targets"><code>A</code>, <code>L</code>, <code>R_th</code>, <code>k</code></span></div><div class="equation-summary-chip"><span class="equation-summary-chip-label">Default</span><span class="equation-summary-chip-value"><code>-</code></span></div>
</div>
</article>
<article class="equation-summary-card">
<a class="equation-summary-card-link" href="./thermal_resistance_convection.md" aria-label="Open Convection Thermal Resistance"></a>
<div class="equation-summary-header">
<h3 class="equation-summary-title"><a href="./thermal_resistance_convection.md">Convection Thermal Resistance</a></h3>
<div class="equation-summary-path"><code>heat_transfer.thermal_resistance_convection</code></div>
</div>
<div class="equation-summary-latex">\(R_{th} = \frac{1}{h A}\)</div>
<div class="equation-summary-meta">
<div class="equation-summary-chip"><span class="equation-summary-chip-label">Targets</span><span class="equation-summary-chip-value equation-summary-targets"><code>A</code>, <code>R_th</code>, <code>h</code></span></div><div class="equation-summary-chip"><span class="equation-summary-chip-label">Default</span><span class="equation-summary-chip-value"><code>-</code></span></div>
</div>
</article>
</div>

## Browse

- [Plane-Wall Conduction Heat Rate](./conduction_plane_wall_heat_rate.md)
- [Convection Heat Transfer Rate](./convection_heat_rate.md)
- [Log-Mean Temperature Difference](./log_mean_temperature_difference.md)
- [Conduction Thermal Resistance](./thermal_resistance_conduction.md)
- [Convection Thermal Resistance](./thermal_resistance_convection.md)
