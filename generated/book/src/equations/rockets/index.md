# Rockets

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
<a class="equation-summary-card-link" href="./cstar_ideal.md" aria-label="Open Ideal Characteristic Velocity"></a>
<div class="equation-summary-header">
<h3 class="equation-summary-title"><a href="./cstar_ideal.md">Ideal Characteristic Velocity</a></h3>
<div class="equation-summary-path"><code>rockets.cstar_ideal</code></div>
</div>
<div class="equation-summary-latex">\(c^* = \sqrt{\frac{R T_c}{\gamma}} \left(\frac{\gamma+1}{2}\right)^{(\gamma+1)/(2(\gamma-1))}\)</div>
<div class="equation-summary-meta">
<div class="equation-summary-chip"><span class="equation-summary-chip-label">Targets</span><span class="equation-summary-chip-value equation-summary-targets"><code>c_star</code></span></div><div class="equation-summary-chip"><span class="equation-summary-chip-label">Default</span><span class="equation-summary-chip-value"><code>-</code></span></div>
</div>
</article>
<article class="equation-summary-card">
<a class="equation-summary-card-link" href="./specific_impulse_ideal.md" aria-label="Open Ideal Specific Impulse"></a>
<div class="equation-summary-header">
<h3 class="equation-summary-title"><a href="./specific_impulse_ideal.md">Ideal Specific Impulse</a></h3>
<div class="equation-summary-path"><code>rockets.specific_impulse_ideal</code></div>
</div>
<div class="equation-summary-latex">\(I_{sp} = \frac{C_f c^*}{g_0}\)</div>
<div class="equation-summary-meta">
<div class="equation-summary-chip"><span class="equation-summary-chip-label">Targets</span><span class="equation-summary-chip-value equation-summary-targets"><code>C_f</code>, <code>I_sp</code>, <code>c_star</code></span></div><div class="equation-summary-chip"><span class="equation-summary-chip-label">Default</span><span class="equation-summary-chip-value"><code>-</code></span></div>
</div>
</article>
<article class="equation-summary-card">
<a class="equation-summary-card-link" href="./thrust_coefficient_ideal.md" aria-label="Open Ideal Thrust Coefficient"></a>
<div class="equation-summary-header">
<h3 class="equation-summary-title"><a href="./thrust_coefficient_ideal.md">Ideal Thrust Coefficient</a></h3>
<div class="equation-summary-path"><code>rockets.thrust_coefficient_ideal</code></div>
</div>
<div class="equation-summary-latex">\(C_f = \sqrt{\frac{2\gamma^2}{\gamma-1}\left(\frac{2}{\gamma+1}\right)^{(\gamma+1)/(\gamma-1)}\left(1-\left(\frac{p_e}{p_c}\right)^{(\gamma-1)/\gamma}\right)} + \left(\frac{p_e}{p_c}-\frac{p_a}{p_c}\right)\frac{A_e}{A_t}\)</div>
<div class="equation-summary-meta">
<div class="equation-summary-chip"><span class="equation-summary-chip-label">Targets</span><span class="equation-summary-chip-value equation-summary-targets"><code>C_f</code></span></div><div class="equation-summary-chip"><span class="equation-summary-chip-label">Default</span><span class="equation-summary-chip-value"><code>-</code></span></div>
</div>
</article>
<article class="equation-summary-card">
<a class="equation-summary-card-link" href="./thrust_from_mass_flow.md" aria-label="Open Thrust From Mass Flow and Effective Exhaust Velocity"></a>
<div class="equation-summary-header">
<h3 class="equation-summary-title"><a href="./thrust_from_mass_flow.md">Thrust From Mass Flow and Effective Exhaust Velocity</a></h3>
<div class="equation-summary-path"><code>rockets.thrust_from_mass_flow</code></div>
</div>
<div class="equation-summary-latex">\(F = \dot{m} c_{eff}\)</div>
<div class="equation-summary-meta">
<div class="equation-summary-chip"><span class="equation-summary-chip-label">Targets</span><span class="equation-summary-chip-value equation-summary-targets"><code>F</code>, <code>c_eff</code>, <code>m_dot</code></span></div><div class="equation-summary-chip"><span class="equation-summary-chip-label">Default</span><span class="equation-summary-chip-value"><code>-</code></span></div>
</div>
</article>
</div>

## Browse

- [Ideal Characteristic Velocity](./cstar_ideal.md)
- [Ideal Specific Impulse](./specific_impulse_ideal.md)
- [Ideal Thrust Coefficient](./thrust_coefficient_ideal.md)
- [Thrust From Mass Flow and Effective Exhaust Velocity](./thrust_from_mass_flow.md)
