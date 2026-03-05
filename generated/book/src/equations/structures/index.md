# Structures

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
<h3 class="equation-summary-title"><a href="./axial_stress.md">Axial Normal Stress</a></h3>
<div class="equation-summary-path"><code>structures.axial_stress</code></div>
<div class="equation-summary-latex">\(\sigma = \frac{F}{A}\)</div>
<div class="equation-summary-meta">
<div class="equation-summary-meta-label">Targets</div><div class="equation-summary-meta-value"><code>A</code>, <code>F</code>, <code>sigma</code></div><div class="equation-summary-meta-label">Default</div><div class="equation-summary-meta-value"><code>-</code></div><div class="equation-summary-meta-label">Branches</div><div class="equation-summary-meta-value">-</div><div class="equation-summary-meta-label">Subcategory</div><div class="equation-summary-meta-value">-</div>
</div>
</article>
<article class="equation-summary-card">
<h3 class="equation-summary-title"><a href="./beam_bending_stress.md">Beam Bending Stress</a></h3>
<div class="equation-summary-path"><code>structures.beam_bending_stress</code></div>
<div class="equation-summary-latex">\(\sigma_b = \frac{M c}{I}\)</div>
<div class="equation-summary-meta">
<div class="equation-summary-meta-label">Targets</div><div class="equation-summary-meta-value"><code>I</code>, <code>M</code>, <code>c</code>, <code>sigma_b</code></div><div class="equation-summary-meta-label">Default</div><div class="equation-summary-meta-value"><code>-</code></div><div class="equation-summary-meta-label">Branches</div><div class="equation-summary-meta-value">-</div><div class="equation-summary-meta-label">Subcategory</div><div class="equation-summary-meta-value">-</div>
</div>
</article>
<article class="equation-summary-card">
<h3 class="equation-summary-title"><a href="./euler_buckling_load.md">Euler Buckling Critical Load</a></h3>
<div class="equation-summary-path"><code>structures.euler_buckling_load</code></div>
<div class="equation-summary-latex">\(P_{cr} = \frac{\pi^2 E I}{(K L)^2}\)</div>
<div class="equation-summary-meta">
<div class="equation-summary-meta-label">Targets</div><div class="equation-summary-meta-value"><code>E</code>, <code>I</code>, <code>K</code>, <code>L</code>, <code>P_cr</code></div><div class="equation-summary-meta-label">Default</div><div class="equation-summary-meta-value"><code>-</code></div><div class="equation-summary-meta-label">Branches</div><div class="equation-summary-meta-value">-</div><div class="equation-summary-meta-label">Subcategory</div><div class="equation-summary-meta-value">-</div>
</div>
</article>
<article class="equation-summary-card">
<h3 class="equation-summary-title"><a href="./hoop_stress.md">Thin-Wall Hoop Stress</a></h3>
<div class="equation-summary-path"><code>structures.hoop_stress</code></div>
<div class="equation-summary-latex">\(\sigma_h = \frac{P r}{t}\)</div>
<div class="equation-summary-meta">
<div class="equation-summary-meta-label">Targets</div><div class="equation-summary-meta-value"><code>P</code>, <code>r</code>, <code>sigma_h</code>, <code>t</code></div><div class="equation-summary-meta-label">Default</div><div class="equation-summary-meta-value"><code>-</code></div><div class="equation-summary-meta-label">Branches</div><div class="equation-summary-meta-value">-</div><div class="equation-summary-meta-label">Subcategory</div><div class="equation-summary-meta-value">-</div>
</div>
</article>
<article class="equation-summary-card">
<h3 class="equation-summary-title"><a href="./longitudinal_stress_thin_wall.md">Thin-Wall Longitudinal Stress</a></h3>
<div class="equation-summary-path"><code>structures.longitudinal_stress_thin_wall</code></div>
<div class="equation-summary-latex">\(\sigma_l = \frac{P r}{2 t}\)</div>
<div class="equation-summary-meta">
<div class="equation-summary-meta-label">Targets</div><div class="equation-summary-meta-value"><code>P</code>, <code>r</code>, <code>sigma_l</code>, <code>t</code></div><div class="equation-summary-meta-label">Default</div><div class="equation-summary-meta-value"><code>-</code></div><div class="equation-summary-meta-label">Branches</div><div class="equation-summary-meta-value">-</div><div class="equation-summary-meta-label">Subcategory</div><div class="equation-summary-meta-value">-</div>
</div>
</article>
<article class="equation-summary-card">
<h3 class="equation-summary-title"><a href="./shaft_torsion_stress.md">Circular Shaft Torsion Stress</a></h3>
<div class="equation-summary-path"><code>structures.shaft_torsion_stress</code></div>
<div class="equation-summary-latex">\(\tau = \frac{T r}{J}\)</div>
<div class="equation-summary-meta">
<div class="equation-summary-meta-label">Targets</div><div class="equation-summary-meta-value"><code>J</code>, <code>T</code>, <code>r</code>, <code>tau</code></div><div class="equation-summary-meta-label">Default</div><div class="equation-summary-meta-value"><code>-</code></div><div class="equation-summary-meta-label">Branches</div><div class="equation-summary-meta-value">-</div><div class="equation-summary-meta-label">Subcategory</div><div class="equation-summary-meta-value">-</div>
</div>
</article>
</div>

## Browse

- [Axial Normal Stress](./axial_stress.md)
- [Beam Bending Stress](./beam_bending_stress.md)
- [Euler Buckling Critical Load](./euler_buckling_load.md)
- [Thin-Wall Hoop Stress](./hoop_stress.md)
- [Thin-Wall Longitudinal Stress](./longitudinal_stress_thin_wall.md)
- [Circular Shaft Torsion Stress](./shaft_torsion_stress.md)
