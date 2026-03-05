# Rockets

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
<h3 class="equation-summary-title"><a href="./cstar_ideal.md">Ideal Characteristic Velocity</a></h3>
<div class="equation-summary-path"><code>rockets.cstar_ideal</code></div>
<div class="equation-summary-latex">\(c^* = \sqrt{\frac{R T_c}{\gamma}} \left(\frac{\gamma+1}{2}\right)^{(\gamma+1)/(2(\gamma-1))}\)</div>
<div class="equation-summary-meta">
<div class="equation-summary-meta-label">Targets</div><div class="equation-summary-meta-value"><code>c_star</code></div><div class="equation-summary-meta-label">Default</div><div class="equation-summary-meta-value"><code>-</code></div><div class="equation-summary-meta-label">Branches</div><div class="equation-summary-meta-value">-</div><div class="equation-summary-meta-label">Subcategory</div><div class="equation-summary-meta-value">-</div>
</div>
</article>
<article class="equation-summary-card">
<h3 class="equation-summary-title"><a href="./specific_impulse_ideal.md">Ideal Specific Impulse</a></h3>
<div class="equation-summary-path"><code>rockets.specific_impulse_ideal</code></div>
<div class="equation-summary-latex">\(I_{sp} = \frac{C_f c^*}{g_0}\)</div>
<div class="equation-summary-meta">
<div class="equation-summary-meta-label">Targets</div><div class="equation-summary-meta-value"><code>C_f</code>, <code>I_sp</code>, <code>c_star</code></div><div class="equation-summary-meta-label">Default</div><div class="equation-summary-meta-value"><code>-</code></div><div class="equation-summary-meta-label">Branches</div><div class="equation-summary-meta-value">-</div><div class="equation-summary-meta-label">Subcategory</div><div class="equation-summary-meta-value">-</div>
</div>
</article>
<article class="equation-summary-card">
<h3 class="equation-summary-title"><a href="./thrust_coefficient_ideal.md">Ideal Thrust Coefficient</a></h3>
<div class="equation-summary-path"><code>rockets.thrust_coefficient_ideal</code></div>
<div class="equation-summary-latex">\(C_f = \sqrt{\frac{2\gamma^2}{\gamma-1}\left(\frac{2}{\gamma+1}\right)^{(\gamma+1)/(\gamma-1)}\left(1-\left(\frac{p_e}{p_c}\right)^{(\gamma-1)/\gamma}\right)} + \left(\frac{p_e}{p_c}-\frac{p_a}{p_c}\right)\frac{A_e}{A_t}\)</div>
<div class="equation-summary-meta">
<div class="equation-summary-meta-label">Targets</div><div class="equation-summary-meta-value"><code>C_f</code></div><div class="equation-summary-meta-label">Default</div><div class="equation-summary-meta-value"><code>-</code></div><div class="equation-summary-meta-label">Branches</div><div class="equation-summary-meta-value">-</div><div class="equation-summary-meta-label">Subcategory</div><div class="equation-summary-meta-value">-</div>
</div>
</article>
<article class="equation-summary-card">
<h3 class="equation-summary-title"><a href="./thrust_from_mass_flow.md">Thrust From Mass Flow and Effective Exhaust Velocity</a></h3>
<div class="equation-summary-path"><code>rockets.thrust_from_mass_flow</code></div>
<div class="equation-summary-latex">\(F = \dot{m} c_{eff}\)</div>
<div class="equation-summary-meta">
<div class="equation-summary-meta-label">Targets</div><div class="equation-summary-meta-value"><code>F</code>, <code>c_eff</code>, <code>m_dot</code></div><div class="equation-summary-meta-label">Default</div><div class="equation-summary-meta-value"><code>-</code></div><div class="equation-summary-meta-label">Branches</div><div class="equation-summary-meta-value">-</div><div class="equation-summary-meta-label">Subcategory</div><div class="equation-summary-meta-value">-</div>
</div>
</article>
</div>

## Browse

- [Ideal Characteristic Velocity](./cstar_ideal.md)
- [Ideal Specific Impulse](./specific_impulse_ideal.md)
- [Ideal Thrust Coefficient](./thrust_coefficient_ideal.md)
- [Thrust From Mass Flow and Effective Exhaust Velocity](./thrust_from_mass_flow.md)
