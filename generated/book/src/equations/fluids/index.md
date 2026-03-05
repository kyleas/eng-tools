# Fluids

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
<h3 class="equation-summary-title"><a href="./circular_pipe_area.md">Circular Pipe Flow Area</a></h3>
<div class="equation-summary-path"><code>fluids.circular_pipe_area</code></div>
<div class="equation-summary-latex">\(A = \frac{\pi D^2}{4}\)</div>
<div class="equation-summary-meta">
<div class="equation-summary-meta-label">Targets</div><div class="equation-summary-meta-value"><code>A</code>, <code>D</code></div><div class="equation-summary-meta-label">Default</div><div class="equation-summary-meta-value"><code>-</code></div><div class="equation-summary-meta-label">Branches</div><div class="equation-summary-meta-value">-</div><div class="equation-summary-meta-label">Subcategory</div><div class="equation-summary-meta-value">-</div>
</div>
</article>
<article class="equation-summary-card">
<h3 class="equation-summary-title"><a href="./colebrook.md">Colebrook-White Friction Factor</a></h3>
<div class="equation-summary-path"><code>fluids.colebrook</code></div>
<div class="equation-summary-latex">\(\frac{1}{\sqrt{f}} + 2\log_{10}\left(\frac{\varepsilon_D}{3.7} + \frac{2.51}{Re\sqrt{f}}\right) = 0\)</div>
<div class="equation-summary-meta">
<div class="equation-summary-meta-label">Targets</div><div class="equation-summary-meta-value"><code>f</code></div><div class="equation-summary-meta-label">Default</div><div class="equation-summary-meta-value"><code>-</code></div><div class="equation-summary-meta-label">Branches</div><div class="equation-summary-meta-value">-</div><div class="equation-summary-meta-label">Subcategory</div><div class="equation-summary-meta-value">-</div>
</div>
</article>
<article class="equation-summary-card">
<h3 class="equation-summary-title"><a href="./continuity_mass_flow.md">Continuity Mass Flow</a></h3>
<div class="equation-summary-path"><code>fluids.continuity_mass_flow</code></div>
<div class="equation-summary-latex">\(\dot{m} = \rho A V\)</div>
<div class="equation-summary-meta">
<div class="equation-summary-meta-label">Targets</div><div class="equation-summary-meta-value"><code>A</code>, <code>V</code>, <code>m_dot</code>, <code>rho</code></div><div class="equation-summary-meta-label">Default</div><div class="equation-summary-meta-value"><code>-</code></div><div class="equation-summary-meta-label">Branches</div><div class="equation-summary-meta-value">-</div><div class="equation-summary-meta-label">Subcategory</div><div class="equation-summary-meta-value">-</div>
</div>
</article>
<article class="equation-summary-card">
<h3 class="equation-summary-title"><a href="./darcy_weisbach_pressure_drop.md">Darcy-Weisbach Pressure Drop</a></h3>
<div class="equation-summary-path"><code>fluids.darcy_weisbach_pressure_drop</code></div>
<div class="equation-summary-latex">\(\Delta p = f \frac{L}{D} \frac{\rho V^2}{2}\)</div>
<div class="equation-summary-meta">
<div class="equation-summary-meta-label">Targets</div><div class="equation-summary-meta-value"><code>D</code>, <code>L</code>, <code>V</code>, <code>delta_p</code>, <code>f</code>, <code>rho</code></div><div class="equation-summary-meta-label">Default</div><div class="equation-summary-meta-value"><code>-</code></div><div class="equation-summary-meta-label">Branches</div><div class="equation-summary-meta-value">-</div><div class="equation-summary-meta-label">Subcategory</div><div class="equation-summary-meta-value">-</div>
</div>
</article>
<article class="equation-summary-card">
<h3 class="equation-summary-title"><a href="./orifice_mass_flow_incompressible.md">Incompressible Orifice Mass Flow</a></h3>
<div class="equation-summary-path"><code>fluids.orifice_mass_flow_incompressible</code></div>
<div class="equation-summary-latex">\(\dot{m} = C_d A \sqrt{2 \rho \Delta p}\)</div>
<div class="equation-summary-meta">
<div class="equation-summary-meta-label">Targets</div><div class="equation-summary-meta-value"><code>A</code>, <code>C_d</code>, <code>delta_p</code>, <code>m_dot</code>, <code>rho</code></div><div class="equation-summary-meta-label">Default</div><div class="equation-summary-meta-value"><code>-</code></div><div class="equation-summary-meta-label">Branches</div><div class="equation-summary-meta-value">-</div><div class="equation-summary-meta-label">Subcategory</div><div class="equation-summary-meta-value">-</div>
</div>
</article>
<article class="equation-summary-card">
<h3 class="equation-summary-title"><a href="./reynolds_number.md">Reynolds Number</a></h3>
<div class="equation-summary-path"><code>fluids.reynolds_number</code></div>
<div class="equation-summary-latex">\(Re = \frac{\rho V D}{\mu}\)</div>
<div class="equation-summary-meta">
<div class="equation-summary-meta-label">Targets</div><div class="equation-summary-meta-value"><code>D</code>, <code>Re</code>, <code>V</code>, <code>mu</code>, <code>rho</code></div><div class="equation-summary-meta-label">Default</div><div class="equation-summary-meta-value"><code>-</code></div><div class="equation-summary-meta-label">Branches</div><div class="equation-summary-meta-value">-</div><div class="equation-summary-meta-label">Subcategory</div><div class="equation-summary-meta-value">-</div>
</div>
</article>
</div>

## Browse

- [Circular Pipe Flow Area](./circular_pipe_area.md)
- [Colebrook-White Friction Factor](./colebrook.md)
- [Continuity Mass Flow](./continuity_mass_flow.md)
- [Darcy-Weisbach Pressure Drop](./darcy_weisbach_pressure_drop.md)
- [Incompressible Orifice Mass Flow](./orifice_mass_flow_incompressible.md)
- [Reynolds Number](./reynolds_number.md)
