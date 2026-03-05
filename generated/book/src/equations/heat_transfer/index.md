# Heat Transfer

## Equation Summary

<style>
.equation-summary-cards { display: grid; grid-template-columns: 1fr; gap: 0.9rem; margin: 0.6rem 0 1rem; }
.equation-summary-card { border: 1px solid var(--table-border-color); border-radius: 10px; padding: 0.9rem 1rem; background: rgba(255,255,255,0.02); }
.equation-summary-title { font-size: 1.02rem; font-weight: 600; margin: 0 0 0.35rem; }
.equation-summary-path { font-family: var(--mono-font); font-size: 0.85rem; opacity: 0.9; margin-bottom: 0.45rem; overflow-wrap: anywhere; }
.equation-summary-latex { text-align: center; font-size: 1.2rem; line-height: 1.45; margin: 0.5rem 0 0.6rem; overflow-x: auto; }
.equation-summary-meta { display: grid; grid-template-columns: auto 1fr; column-gap: 0.55rem; row-gap: 0.25rem; font-size: 0.9rem; }
.equation-summary-meta-label { opacity: 0.85; }
.equation-summary-meta-value { overflow-wrap: anywhere; }
.equation-summary-meta-value code { white-space: nowrap; }
@media (max-width: 900px) { .equation-summary-latex { font-size: 1.1rem; } }
</style>
<div class="equation-summary-cards">
<article class="equation-summary-card">
<h3 class="equation-summary-title"><a href="./conduction_plane_wall_heat_rate.md">Plane-Wall Conduction Heat Rate</a></h3>
<div class="equation-summary-path"><code>heat_transfer.conduction_plane_wall_heat_rate</code></div>
<div class="equation-summary-latex">\(\dot{Q} = k A \frac{T_h - T_c}{L}\)</div>
<div class="equation-summary-meta">
<div class="equation-summary-meta-label">Targets</div><div class="equation-summary-meta-value"><code>A</code>, <code>L</code>, <code>Q_dot</code>, <code>T_c</code>, <code>T_h</code>, <code>k</code></div><div class="equation-summary-meta-label">Default</div><div class="equation-summary-meta-value"><code>-</code></div><div class="equation-summary-meta-label">Branches</div><div class="equation-summary-meta-value">-</div><div class="equation-summary-meta-label">Subcategory</div><div class="equation-summary-meta-value">-</div>
</div>
</article>
<article class="equation-summary-card">
<h3 class="equation-summary-title"><a href="./convection_heat_rate.md">Convection Heat Transfer Rate</a></h3>
<div class="equation-summary-path"><code>heat_transfer.convection_heat_rate</code></div>
<div class="equation-summary-latex">\(\dot{Q} = h A (T_s - T_\infty)\)</div>
<div class="equation-summary-meta">
<div class="equation-summary-meta-label">Targets</div><div class="equation-summary-meta-value"><code>A</code>, <code>Q_dot</code>, <code>T_inf</code>, <code>T_s</code>, <code>h</code></div><div class="equation-summary-meta-label">Default</div><div class="equation-summary-meta-value"><code>-</code></div><div class="equation-summary-meta-label">Branches</div><div class="equation-summary-meta-value">-</div><div class="equation-summary-meta-label">Subcategory</div><div class="equation-summary-meta-value">-</div>
</div>
</article>
<article class="equation-summary-card">
<h3 class="equation-summary-title"><a href="./log_mean_temperature_difference.md">Log-Mean Temperature Difference</a></h3>
<div class="equation-summary-path"><code>heat_transfer.log_mean_temperature_difference</code></div>
<div class="equation-summary-latex">\(\Delta T_{lm} = \frac{\Delta T_1 - \Delta T_2}{\ln(\Delta T_1 / \Delta T_2)}\)</div>
<div class="equation-summary-meta">
<div class="equation-summary-meta-label">Targets</div><div class="equation-summary-meta-value"><code>delta_T_lm</code></div><div class="equation-summary-meta-label">Default</div><div class="equation-summary-meta-value"><code>-</code></div><div class="equation-summary-meta-label">Branches</div><div class="equation-summary-meta-value">-</div><div class="equation-summary-meta-label">Subcategory</div><div class="equation-summary-meta-value">-</div>
</div>
</article>
<article class="equation-summary-card">
<h3 class="equation-summary-title"><a href="./thermal_resistance_conduction.md">Conduction Thermal Resistance</a></h3>
<div class="equation-summary-path"><code>heat_transfer.thermal_resistance_conduction</code></div>
<div class="equation-summary-latex">\(R_{th} = \frac{L}{k A}\)</div>
<div class="equation-summary-meta">
<div class="equation-summary-meta-label">Targets</div><div class="equation-summary-meta-value"><code>A</code>, <code>L</code>, <code>R_th</code>, <code>k</code></div><div class="equation-summary-meta-label">Default</div><div class="equation-summary-meta-value"><code>-</code></div><div class="equation-summary-meta-label">Branches</div><div class="equation-summary-meta-value">-</div><div class="equation-summary-meta-label">Subcategory</div><div class="equation-summary-meta-value">-</div>
</div>
</article>
<article class="equation-summary-card">
<h3 class="equation-summary-title"><a href="./thermal_resistance_convection.md">Convection Thermal Resistance</a></h3>
<div class="equation-summary-path"><code>heat_transfer.thermal_resistance_convection</code></div>
<div class="equation-summary-latex">\(R_{th} = \frac{1}{h A}\)</div>
<div class="equation-summary-meta">
<div class="equation-summary-meta-label">Targets</div><div class="equation-summary-meta-value"><code>A</code>, <code>R_th</code>, <code>h</code></div><div class="equation-summary-meta-label">Default</div><div class="equation-summary-meta-value"><code>-</code></div><div class="equation-summary-meta-label">Branches</div><div class="equation-summary-meta-value">-</div><div class="equation-summary-meta-label">Subcategory</div><div class="equation-summary-meta-value">-</div>
</div>
</article>
</div>

## Browse

- [Plane-Wall Conduction Heat Rate](./conduction_plane_wall_heat_rate.md)
- [Convection Heat Transfer Rate](./convection_heat_rate.md)
- [Log-Mean Temperature Difference](./log_mean_temperature_difference.md)
- [Conduction Thermal Resistance](./thermal_resistance_conduction.md)
- [Convection Thermal Resistance](./thermal_resistance_convection.md)
