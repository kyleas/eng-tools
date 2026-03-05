# Thermo

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
<a class="equation-summary-card-link" href="./ideal_gas/density.md" aria-label="Open Ideal Gas Law (Density Form)"></a>
<div class="equation-summary-header">
<h3 class="equation-summary-title"><a href="./ideal_gas/density.md">Ideal Gas Law (Density Form)</a></h3>
<div class="equation-summary-path"><code>thermo.ideal_gas.density</code></div>
</div>
<div class="equation-summary-latex">\(P = \rho R T\)</div>
<div class="equation-summary-meta">
<div class="equation-summary-chip"><span class="equation-summary-chip-label">Targets</span><span class="equation-summary-chip-value equation-summary-targets"><code>P</code>, <code>R</code>, <code>T</code>, <code>rho</code></span></div><div class="equation-summary-chip"><span class="equation-summary-chip-label">Default</span><span class="equation-summary-chip-value"><code>-</code></span></div><div class="equation-summary-chip"><span class="equation-summary-chip-label">Subcategory</span><span class="equation-summary-chip-value">Ideal Gas</span></div>
</div>
</article>
<article class="equation-summary-card">
<a class="equation-summary-card-link" href="./ideal_gas/mass_volume.md" aria-label="Open Ideal Gas Law (Mass-Volume Form)"></a>
<div class="equation-summary-header">
<h3 class="equation-summary-title"><a href="./ideal_gas/mass_volume.md">Ideal Gas Law (Mass-Volume Form)</a></h3>
<div class="equation-summary-path"><code>thermo.ideal_gas.mass_volume</code></div>
</div>
<div class="equation-summary-latex">\(P V = m R T\)</div>
<div class="equation-summary-meta">
<div class="equation-summary-chip"><span class="equation-summary-chip-label">Targets</span><span class="equation-summary-chip-value equation-summary-targets"><code>P</code>, <code>R</code>, <code>T</code>, <code>V</code>, <code>m</code></span></div><div class="equation-summary-chip"><span class="equation-summary-chip-label">Default</span><span class="equation-summary-chip-value"><code>-</code></span></div><div class="equation-summary-chip"><span class="equation-summary-chip-label">Subcategory</span><span class="equation-summary-chip-value">Ideal Gas</span></div>
</div>
</article>
</div>

## Browse

- [Ideal Gas](./ideal_gas/index.md)
