# Structures

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
<a class="equation-summary-card-link" href="./axial_stress.md" aria-label="Open Axial Normal Stress"></a>
<div class="equation-summary-header">
<h3 class="equation-summary-title"><a href="./axial_stress.md">Axial Normal Stress</a></h3>
<div class="equation-summary-path"><code>structures.axial_stress</code></div>
</div>
<div class="equation-summary-latex">\(\sigma = \frac{F}{A}\)</div>
<div class="equation-summary-meta">
<div class="equation-summary-chip"><span class="equation-summary-chip-label">Targets</span><span class="equation-summary-chip-value equation-summary-targets"><code>A</code>, <code>F</code>, <code>sigma</code></span></div><div class="equation-summary-chip"><span class="equation-summary-chip-label">Default</span><span class="equation-summary-chip-value"><code>-</code></span></div>
</div>
</article>
<article class="equation-summary-card">
<a class="equation-summary-card-link" href="./beam_bending_stress.md" aria-label="Open Beam Bending Stress"></a>
<div class="equation-summary-header">
<h3 class="equation-summary-title"><a href="./beam_bending_stress.md">Beam Bending Stress</a></h3>
<div class="equation-summary-path"><code>structures.beam_bending_stress</code></div>
</div>
<div class="equation-summary-latex">\(\sigma_b = \frac{M c}{I}\)</div>
<div class="equation-summary-meta">
<div class="equation-summary-chip"><span class="equation-summary-chip-label">Targets</span><span class="equation-summary-chip-value equation-summary-targets"><code>I</code>, <code>M</code>, <code>c</code>, <code>sigma_b</code></span></div><div class="equation-summary-chip"><span class="equation-summary-chip-label">Default</span><span class="equation-summary-chip-value"><code>-</code></span></div>
</div>
</article>
<article class="equation-summary-card">
<a class="equation-summary-card-link" href="./euler_buckling_load.md" aria-label="Open Euler Buckling Critical Load"></a>
<div class="equation-summary-header">
<h3 class="equation-summary-title"><a href="./euler_buckling_load.md">Euler Buckling Critical Load</a></h3>
<div class="equation-summary-path"><code>structures.euler_buckling_load</code></div>
</div>
<div class="equation-summary-latex">\(P_{cr} = \frac{\pi^2 E I}{(K L)^2}\)</div>
<div class="equation-summary-meta">
<div class="equation-summary-chip"><span class="equation-summary-chip-label">Targets</span><span class="equation-summary-chip-value equation-summary-targets"><code>E</code>, <code>I</code>, <code>K</code>, <code>L</code>, <code>P_cr</code></span></div><div class="equation-summary-chip"><span class="equation-summary-chip-label">Default</span><span class="equation-summary-chip-value"><code>-</code></span></div>
</div>
</article>
<article class="equation-summary-card">
<a class="equation-summary-card-link" href="./hoop_stress.md" aria-label="Open Thin-Wall Hoop Stress"></a>
<div class="equation-summary-header">
<h3 class="equation-summary-title"><a href="./hoop_stress.md">Thin-Wall Hoop Stress</a></h3>
<div class="equation-summary-path"><code>structures.hoop_stress</code></div>
</div>
<div class="equation-summary-latex">\(\sigma_h = \frac{P r}{t}\)</div>
<div class="equation-summary-meta">
<div class="equation-summary-chip"><span class="equation-summary-chip-label">Targets</span><span class="equation-summary-chip-value equation-summary-targets"><code>P</code>, <code>r</code>, <code>sigma_h</code>, <code>t</code></span></div><div class="equation-summary-chip"><span class="equation-summary-chip-label">Default</span><span class="equation-summary-chip-value"><code>-</code></span></div>
</div>
</article>
<article class="equation-summary-card">
<a class="equation-summary-card-link" href="./longitudinal_stress_thin_wall.md" aria-label="Open Thin-Wall Longitudinal Stress"></a>
<div class="equation-summary-header">
<h3 class="equation-summary-title"><a href="./longitudinal_stress_thin_wall.md">Thin-Wall Longitudinal Stress</a></h3>
<div class="equation-summary-path"><code>structures.longitudinal_stress_thin_wall</code></div>
</div>
<div class="equation-summary-latex">\(\sigma_l = \frac{P r}{2 t}\)</div>
<div class="equation-summary-meta">
<div class="equation-summary-chip"><span class="equation-summary-chip-label">Targets</span><span class="equation-summary-chip-value equation-summary-targets"><code>P</code>, <code>r</code>, <code>sigma_l</code>, <code>t</code></span></div><div class="equation-summary-chip"><span class="equation-summary-chip-label">Default</span><span class="equation-summary-chip-value"><code>-</code></span></div>
</div>
</article>
<article class="equation-summary-card">
<a class="equation-summary-card-link" href="./shaft_torsion_stress.md" aria-label="Open Circular Shaft Torsion Stress"></a>
<div class="equation-summary-header">
<h3 class="equation-summary-title"><a href="./shaft_torsion_stress.md">Circular Shaft Torsion Stress</a></h3>
<div class="equation-summary-path"><code>structures.shaft_torsion_stress</code></div>
</div>
<div class="equation-summary-latex">\(\tau = \frac{T r}{J}\)</div>
<div class="equation-summary-meta">
<div class="equation-summary-chip"><span class="equation-summary-chip-label">Targets</span><span class="equation-summary-chip-value equation-summary-targets"><code>J</code>, <code>T</code>, <code>r</code>, <code>tau</code></span></div><div class="equation-summary-chip"><span class="equation-summary-chip-label">Default</span><span class="equation-summary-chip-value"><code>-</code></span></div>
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
