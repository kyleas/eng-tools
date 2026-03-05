# Compressible

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
<h3 class="equation-summary-title"><a href="./area_mach.md">Isentropic Area-Mach Relation</a></h3>
<div class="equation-summary-path"><code>compressible.area_mach</code></div>
<div class="equation-summary-latex">\(\frac{A}{A^*} = \frac{1}{M}\left(\frac{2}{\gamma+1}\left(1+\frac{\gamma-1}{2}M^2\right)\right)^{\frac{\gamma+1}{2(\gamma-1)}}\)</div>
<div class="equation-summary-meta">
<div class="equation-summary-meta-label">Targets</div><div class="equation-summary-meta-value"><code>M</code>, <code>area_ratio</code></div><div class="equation-summary-meta-label">Default</div><div class="equation-summary-meta-value"><code>-</code></div><div class="equation-summary-meta-label">Branches</div><div class="equation-summary-meta-value"><code>subsonic</code>, <code>supersonic</code></div><div class="equation-summary-meta-label">Subcategory</div><div class="equation-summary-meta-value">-</div>
</div>
</article>
<article class="equation-summary-card">
<h3 class="equation-summary-title"><a href="./choked_mass_flux.md">Choked Mass Flux</a></h3>
<div class="equation-summary-path"><code>compressible.choked_mass_flux</code></div>
<div class="equation-summary-latex">\(G^* = \frac{p_0}{\sqrt{T_0}} \sqrt{\frac{\gamma}{R}} \left(\frac{2}{\gamma+1}\right)^{(\gamma+1)/(2(\gamma-1))}\)</div>
<div class="equation-summary-meta">
<div class="equation-summary-meta-label">Targets</div><div class="equation-summary-meta-value"><code>G_star</code></div><div class="equation-summary-meta-label">Default</div><div class="equation-summary-meta-value"><code>-</code></div><div class="equation-summary-meta-label">Branches</div><div class="equation-summary-meta-value">-</div><div class="equation-summary-meta-label">Subcategory</div><div class="equation-summary-meta-value">-</div>
</div>
</article>
<article class="equation-summary-card">
<h3 class="equation-summary-title"><a href="./fanno_density_ratio.md">Fanno Density Ratio</a></h3>
<div class="equation-summary-path"><code>compressible.fanno_density_ratio</code></div>
<div class="equation-summary-latex">\(\frac{\rho}{\rho^*} = \frac{1}{M}\sqrt{\frac{1+\frac{\gamma-1}{2}M^2}{\frac{\gamma+1}{2}}}\)</div>
<div class="equation-summary-meta">
<div class="equation-summary-meta-label">Targets</div><div class="equation-summary-meta-value"><code>M</code>, <code>rho_rhostar</code></div><div class="equation-summary-meta-label">Default</div><div class="equation-summary-meta-value"><code>-</code></div><div class="equation-summary-meta-label">Branches</div><div class="equation-summary-meta-value"><code>subsonic</code>, <code>supersonic</code></div><div class="equation-summary-meta-label">Subcategory</div><div class="equation-summary-meta-value">-</div>
</div>
</article>
<article class="equation-summary-card">
<h3 class="equation-summary-title"><a href="./fanno_friction_parameter.md">Fanno Friction Length Parameter</a></h3>
<div class="equation-summary-path"><code>compressible.fanno_friction_parameter</code></div>
<div class="equation-summary-latex">\(\frac{4 f L^*}{D} = \frac{1-M^2}{\gamma M^2} + \frac{\gamma+1}{2\gamma}\ln\!\left(\frac{\frac{\gamma+1}{2}M^2}{1+\frac{\gamma-1}{2}M^2}\right)\)</div>
<div class="equation-summary-meta">
<div class="equation-summary-meta-label">Targets</div><div class="equation-summary-meta-value"><code>M</code>, <code>four_flstar_d</code></div><div class="equation-summary-meta-label">Default</div><div class="equation-summary-meta-value"><code>-</code></div><div class="equation-summary-meta-label">Branches</div><div class="equation-summary-meta-value"><code>subsonic</code>, <code>supersonic</code></div><div class="equation-summary-meta-label">Subcategory</div><div class="equation-summary-meta-value">-</div>
</div>
</article>
<article class="equation-summary-card">
<h3 class="equation-summary-title"><a href="./fanno_pressure_ratio.md">Fanno Pressure Ratio</a></h3>
<div class="equation-summary-path"><code>compressible.fanno_pressure_ratio</code></div>
<div class="equation-summary-latex">\(\frac{p}{p^*} = \frac{1}{M}\sqrt{\frac{\frac{\gamma+1}{2}}{1+\frac{\gamma-1}{2}M^2}}\)</div>
<div class="equation-summary-meta">
<div class="equation-summary-meta-label">Targets</div><div class="equation-summary-meta-value"><code>M</code>, <code>p_pstar</code></div><div class="equation-summary-meta-label">Default</div><div class="equation-summary-meta-value"><code>-</code></div><div class="equation-summary-meta-label">Branches</div><div class="equation-summary-meta-value"><code>subsonic</code>, <code>supersonic</code></div><div class="equation-summary-meta-label">Subcategory</div><div class="equation-summary-meta-value">-</div>
</div>
</article>
<article class="equation-summary-card">
<h3 class="equation-summary-title"><a href="./fanno_stagnation_pressure_ratio.md">Fanno Stagnation Pressure Ratio</a></h3>
<div class="equation-summary-path"><code>compressible.fanno_stagnation_pressure_ratio</code></div>
<div class="equation-summary-latex">\(\frac{p_0}{p_0^*} = \frac{1}{M}\left(\frac{1+\frac{\gamma-1}{2}M^2}{\frac{\gamma+1}{2}}\right)^{\frac{\gamma+1}{2(\gamma-1)}}\)</div>
<div class="equation-summary-meta">
<div class="equation-summary-meta-label">Targets</div><div class="equation-summary-meta-value"><code>M</code>, <code>p0_p0star</code></div><div class="equation-summary-meta-label">Default</div><div class="equation-summary-meta-value"><code>-</code></div><div class="equation-summary-meta-label">Branches</div><div class="equation-summary-meta-value"><code>subsonic</code>, <code>supersonic</code></div><div class="equation-summary-meta-label">Subcategory</div><div class="equation-summary-meta-value">-</div>
</div>
</article>
<article class="equation-summary-card">
<h3 class="equation-summary-title"><a href="./fanno_temperature_ratio.md">Fanno Temperature Ratio</a></h3>
<div class="equation-summary-path"><code>compressible.fanno_temperature_ratio</code></div>
<div class="equation-summary-latex">\(\frac{T}{T^*} = \frac{\frac{\gamma+1}{2}}{1+\frac{\gamma-1}{2}M^2}\)</div>
<div class="equation-summary-meta">
<div class="equation-summary-meta-label">Targets</div><div class="equation-summary-meta-value"><code>M</code>, <code>t_tstar</code></div><div class="equation-summary-meta-label">Default</div><div class="equation-summary-meta-value"><code>-</code></div><div class="equation-summary-meta-label">Branches</div><div class="equation-summary-meta-value"><code>subsonic</code>, <code>supersonic</code></div><div class="equation-summary-meta-label">Subcategory</div><div class="equation-summary-meta-value">-</div>
</div>
</article>
<article class="equation-summary-card">
<h3 class="equation-summary-title"><a href="./fanno_velocity_ratio.md">Fanno Velocity Ratio</a></h3>
<div class="equation-summary-path"><code>compressible.fanno_velocity_ratio</code></div>
<div class="equation-summary-latex">\(\frac{V}{V^*} = M\sqrt{\frac{\frac{\gamma+1}{2}}{1+\frac{\gamma-1}{2}M^2}}\)</div>
<div class="equation-summary-meta">
<div class="equation-summary-meta-label">Targets</div><div class="equation-summary-meta-value"><code>M</code>, <code>v_vstar</code></div><div class="equation-summary-meta-label">Default</div><div class="equation-summary-meta-value"><code>-</code></div><div class="equation-summary-meta-label">Branches</div><div class="equation-summary-meta-value"><code>subsonic</code>, <code>supersonic</code></div><div class="equation-summary-meta-label">Subcategory</div><div class="equation-summary-meta-value">-</div>
</div>
</article>
<article class="equation-summary-card">
<h3 class="equation-summary-title"><a href="./isentropic_density_ratio.md">Isentropic Density Ratio</a></h3>
<div class="equation-summary-path"><code>compressible.isentropic_density_ratio</code></div>
<div class="equation-summary-latex">\(\frac{\rho}{\rho_0} = \left(1+\frac{\gamma-1}{2}M^2\right)^{-1/(\gamma-1)}\)</div>
<div class="equation-summary-meta">
<div class="equation-summary-meta-label">Targets</div><div class="equation-summary-meta-value"><code>M</code>, <code>rho_rho0</code></div><div class="equation-summary-meta-label">Default</div><div class="equation-summary-meta-value"><code>-</code></div><div class="equation-summary-meta-label">Branches</div><div class="equation-summary-meta-value">-</div><div class="equation-summary-meta-label">Subcategory</div><div class="equation-summary-meta-value">-</div>
</div>
</article>
<article class="equation-summary-card">
<h3 class="equation-summary-title"><a href="./isentropic_pressure_ratio.md">Isentropic Pressure Ratio</a></h3>
<div class="equation-summary-path"><code>compressible.isentropic_pressure_ratio</code></div>
<div class="equation-summary-latex">\(\frac{p}{p_0} = \left(1+\frac{\gamma-1}{2}M^2\right)^{-\gamma/(\gamma-1)}\)</div>
<div class="equation-summary-meta">
<div class="equation-summary-meta-label">Targets</div><div class="equation-summary-meta-value"><code>M</code>, <code>p_p0</code></div><div class="equation-summary-meta-label">Default</div><div class="equation-summary-meta-value"><code>-</code></div><div class="equation-summary-meta-label">Branches</div><div class="equation-summary-meta-value">-</div><div class="equation-summary-meta-label">Subcategory</div><div class="equation-summary-meta-value">-</div>
</div>
</article>
<article class="equation-summary-card">
<h3 class="equation-summary-title"><a href="./isentropic_temperature_ratio.md">Isentropic Temperature Ratio</a></h3>
<div class="equation-summary-path"><code>compressible.isentropic_temperature_ratio</code></div>
<div class="equation-summary-latex">\(\frac{T}{T_0} = \left(1+\frac{\gamma-1}{2}M^2\right)^{-1}\)</div>
<div class="equation-summary-meta">
<div class="equation-summary-meta-label">Targets</div><div class="equation-summary-meta-value"><code>M</code>, <code>T_T0</code></div><div class="equation-summary-meta-label">Default</div><div class="equation-summary-meta-value"><code>-</code></div><div class="equation-summary-meta-label">Branches</div><div class="equation-summary-meta-value">-</div><div class="equation-summary-meta-label">Subcategory</div><div class="equation-summary-meta-value">-</div>
</div>
</article>
<article class="equation-summary-card">
<h3 class="equation-summary-title"><a href="./mach_angle.md">Mach Angle</a></h3>
<div class="equation-summary-path"><code>compressible.mach_angle</code></div>
<div class="equation-summary-latex">\(\mu = \arcsin\left(\frac{1}{M}\right)\)</div>
<div class="equation-summary-meta">
<div class="equation-summary-meta-label">Targets</div><div class="equation-summary-meta-value"><code>M</code>, <code>mu</code></div><div class="equation-summary-meta-label">Default</div><div class="equation-summary-meta-value"><code>-</code></div><div class="equation-summary-meta-label">Branches</div><div class="equation-summary-meta-value">-</div><div class="equation-summary-meta-label">Subcategory</div><div class="equation-summary-meta-value">-</div>
</div>
</article>
<article class="equation-summary-card">
<h3 class="equation-summary-title"><a href="./normal_shock_density_ratio.md">Normal Shock Density Ratio</a></h3>
<div class="equation-summary-path"><code>compressible.normal_shock_density_ratio</code></div>
<div class="equation-summary-latex">\(\frac{\rho_2}{\rho_1} = \frac{(\gamma+1)M_1^2}{(\gamma-1)M_1^2+2}\)</div>
<div class="equation-summary-meta">
<div class="equation-summary-meta-label">Targets</div><div class="equation-summary-meta-value"><code>M1</code>, <code>rho2_rho1</code></div><div class="equation-summary-meta-label">Default</div><div class="equation-summary-meta-value"><code>-</code></div><div class="equation-summary-meta-label">Branches</div><div class="equation-summary-meta-value">-</div><div class="equation-summary-meta-label">Subcategory</div><div class="equation-summary-meta-value">-</div>
</div>
</article>
<article class="equation-summary-card">
<h3 class="equation-summary-title"><a href="./normal_shock_m2.md">Normal Shock Downstream Mach Number</a></h3>
<div class="equation-summary-path"><code>compressible.normal_shock_m2</code></div>
<div class="equation-summary-latex">\(M_2 = \sqrt{\frac{1 + \frac{\gamma-1}{2}M_1^2}{\gamma M_1^2 - \frac{\gamma-1}{2}}}\)</div>
<div class="equation-summary-meta">
<div class="equation-summary-meta-label">Targets</div><div class="equation-summary-meta-value"><code>M1</code>, <code>M2</code></div><div class="equation-summary-meta-label">Default</div><div class="equation-summary-meta-value"><code>-</code></div><div class="equation-summary-meta-label">Branches</div><div class="equation-summary-meta-value">-</div><div class="equation-summary-meta-label">Subcategory</div><div class="equation-summary-meta-value">-</div>
</div>
</article>
<article class="equation-summary-card">
<h3 class="equation-summary-title"><a href="./normal_shock_pressure_ratio.md">Normal Shock Static Pressure Ratio</a></h3>
<div class="equation-summary-path"><code>compressible.normal_shock_pressure_ratio</code></div>
<div class="equation-summary-latex">\(\frac{p_2}{p_1} = 1 + \frac{2\gamma}{\gamma+1}(M_1^2-1)\)</div>
<div class="equation-summary-meta">
<div class="equation-summary-meta-label">Targets</div><div class="equation-summary-meta-value"><code>M1</code>, <code>p2_p1</code></div><div class="equation-summary-meta-label">Default</div><div class="equation-summary-meta-value"><code>-</code></div><div class="equation-summary-meta-label">Branches</div><div class="equation-summary-meta-value">-</div><div class="equation-summary-meta-label">Subcategory</div><div class="equation-summary-meta-value">-</div>
</div>
</article>
<article class="equation-summary-card">
<h3 class="equation-summary-title"><a href="./normal_shock_stagnation_pressure_ratio.md">Normal Shock Stagnation Pressure Ratio</a></h3>
<div class="equation-summary-path"><code>compressible.normal_shock_stagnation_pressure_ratio</code></div>
<div class="equation-summary-latex">\(\frac{p_{02}}{p_{01}} = \left(\frac{(\gamma+1)M_1^2}{(\gamma-1)M_1^2+2}\right)^{\gamma/(\gamma-1)}\left(\frac{\gamma+1}{2\gamma M_1^2-(\gamma-1)}\right)^{1/(\gamma-1)}\)</div>
<div class="equation-summary-meta">
<div class="equation-summary-meta-label">Targets</div><div class="equation-summary-meta-value"><code>M1</code>, <code>p02_p01</code></div><div class="equation-summary-meta-label">Default</div><div class="equation-summary-meta-value"><code>-</code></div><div class="equation-summary-meta-label">Branches</div><div class="equation-summary-meta-value">-</div><div class="equation-summary-meta-label">Subcategory</div><div class="equation-summary-meta-value">-</div>
</div>
</article>
<article class="equation-summary-card">
<h3 class="equation-summary-title"><a href="./normal_shock_temperature_ratio.md">Normal Shock Temperature Ratio</a></h3>
<div class="equation-summary-path"><code>compressible.normal_shock_temperature_ratio</code></div>
<div class="equation-summary-latex">\(\frac{T_2}{T_1} = \frac{p_2/p_1}{\rho_2/\rho_1} = \left(1+\frac{2\gamma}{\gamma+1}(M_1^2-1)\right)\frac{(\gamma-1)M_1^2+2}{(\gamma+1)M_1^2}\)</div>
<div class="equation-summary-meta">
<div class="equation-summary-meta-label">Targets</div><div class="equation-summary-meta-value"><code>M1</code>, <code>T2_T1</code></div><div class="equation-summary-meta-label">Default</div><div class="equation-summary-meta-value"><code>-</code></div><div class="equation-summary-meta-label">Branches</div><div class="equation-summary-meta-value">-</div><div class="equation-summary-meta-label">Subcategory</div><div class="equation-summary-meta-value">-</div>
</div>
</article>
<article class="equation-summary-card">
<h3 class="equation-summary-title"><a href="./oblique_shock_m2.md">Oblique Shock Downstream Mach</a></h3>
<div class="equation-summary-path"><code>compressible.oblique_shock_m2</code></div>
<div class="equation-summary-latex">\(M_2 = \frac{M_{n2}}{\sin(\beta-\theta)}\)</div>
<div class="equation-summary-meta">
<div class="equation-summary-meta-label">Targets</div><div class="equation-summary-meta-value"><code>beta</code>, <code>m2</code>, <code>mn2</code>, <code>theta</code></div><div class="equation-summary-meta-label">Default</div><div class="equation-summary-meta-value"><code>-</code></div><div class="equation-summary-meta-label">Branches</div><div class="equation-summary-meta-value">-</div><div class="equation-summary-meta-label">Subcategory</div><div class="equation-summary-meta-value">-</div>
</div>
</article>
<article class="equation-summary-card">
<h3 class="equation-summary-title"><a href="./oblique_shock_mn1.md">Oblique Shock Normal Upstream Mach</a></h3>
<div class="equation-summary-path"><code>compressible.oblique_shock_mn1</code></div>
<div class="equation-summary-latex">\(M_{n1} = M_1 \sin\beta\)</div>
<div class="equation-summary-meta">
<div class="equation-summary-meta-label">Targets</div><div class="equation-summary-meta-value"><code>beta</code>, <code>m1</code>, <code>mn1</code></div><div class="equation-summary-meta-label">Default</div><div class="equation-summary-meta-value"><code>-</code></div><div class="equation-summary-meta-label">Branches</div><div class="equation-summary-meta-value">-</div><div class="equation-summary-meta-label">Subcategory</div><div class="equation-summary-meta-value">-</div>
</div>
</article>
<article class="equation-summary-card">
<h3 class="equation-summary-title"><a href="./oblique_shock_theta_beta_m.md">Oblique Shock Theta-Beta-M Relation</a></h3>
<div class="equation-summary-path"><code>compressible.oblique_shock_theta_beta_m</code></div>
<div class="equation-summary-latex">\(\tan\theta = \frac{2\cot\beta\left(M_1^2\sin^2\beta - 1\right)}{M_1^2\left(\gamma + \cos 2\beta\right)+2}\)</div>
<div class="equation-summary-meta">
<div class="equation-summary-meta-label">Targets</div><div class="equation-summary-meta-value"><code>beta</code>, <code>theta</code></div><div class="equation-summary-meta-label">Default</div><div class="equation-summary-meta-value"><code>-</code></div><div class="equation-summary-meta-label">Branches</div><div class="equation-summary-meta-value"><code>weak</code>, <code>strong</code></div><div class="equation-summary-meta-label">Subcategory</div><div class="equation-summary-meta-value">-</div>
</div>
</article>
<article class="equation-summary-card">
<h3 class="equation-summary-title"><a href="./prandtl_meyer.md">Prandtl-Meyer Expansion Angle</a></h3>
<div class="equation-summary-path"><code>compressible.prandtl_meyer</code></div>
<div class="equation-summary-latex">\(\nu = \sqrt{\frac{\gamma+1}{\gamma-1}} \tan^{-1}\!\left(\sqrt{\frac{\gamma-1}{\gamma+1}(M^2-1)}\right) - \tan^{-1}\!\left(\sqrt{M^2-1}\right)\)</div>
<div class="equation-summary-meta">
<div class="equation-summary-meta-label">Targets</div><div class="equation-summary-meta-value"><code>M</code>, <code>nu</code></div><div class="equation-summary-meta-label">Default</div><div class="equation-summary-meta-value"><code>-</code></div><div class="equation-summary-meta-label">Branches</div><div class="equation-summary-meta-value">-</div><div class="equation-summary-meta-label">Subcategory</div><div class="equation-summary-meta-value">-</div>
</div>
</article>
<article class="equation-summary-card">
<h3 class="equation-summary-title"><a href="./rayleigh_density_ratio.md">Rayleigh Density Ratio</a></h3>
<div class="equation-summary-path"><code>compressible.rayleigh_density_ratio</code></div>
<div class="equation-summary-latex">\(\frac{\rho}{\rho^*} = \frac{1+\gamma M^2}{(\gamma+1)M^2}\)</div>
<div class="equation-summary-meta">
<div class="equation-summary-meta-label">Targets</div><div class="equation-summary-meta-value"><code>M</code>, <code>rho_rhostar</code></div><div class="equation-summary-meta-label">Default</div><div class="equation-summary-meta-value"><code>-</code></div><div class="equation-summary-meta-label">Branches</div><div class="equation-summary-meta-value">-</div><div class="equation-summary-meta-label">Subcategory</div><div class="equation-summary-meta-value">-</div>
</div>
</article>
<article class="equation-summary-card">
<h3 class="equation-summary-title"><a href="./rayleigh_pressure_ratio.md">Rayleigh Pressure Ratio</a></h3>
<div class="equation-summary-path"><code>compressible.rayleigh_pressure_ratio</code></div>
<div class="equation-summary-latex">\(\frac{p}{p^*} = \frac{\gamma+1}{1+\gamma M^2}\)</div>
<div class="equation-summary-meta">
<div class="equation-summary-meta-label">Targets</div><div class="equation-summary-meta-value"><code>M</code>, <code>p_pstar</code></div><div class="equation-summary-meta-label">Default</div><div class="equation-summary-meta-value"><code>-</code></div><div class="equation-summary-meta-label">Branches</div><div class="equation-summary-meta-value">-</div><div class="equation-summary-meta-label">Subcategory</div><div class="equation-summary-meta-value">-</div>
</div>
</article>
<article class="equation-summary-card">
<h3 class="equation-summary-title"><a href="./rayleigh_stagnation_pressure_ratio.md">Rayleigh Stagnation Pressure Ratio</a></h3>
<div class="equation-summary-path"><code>compressible.rayleigh_stagnation_pressure_ratio</code></div>
<div class="equation-summary-latex">\(\frac{p_0}{p_0^*} = \frac{\gamma+1}{1+\gamma M^2}\left(\frac{1+\frac{\gamma-1}{2}M^2}{\frac{\gamma+1}{2}}\right)^{\frac{\gamma}{\gamma-1}}\)</div>
<div class="equation-summary-meta">
<div class="equation-summary-meta-label">Targets</div><div class="equation-summary-meta-value"><code>M</code>, <code>p0_p0star</code></div><div class="equation-summary-meta-label">Default</div><div class="equation-summary-meta-value"><code>-</code></div><div class="equation-summary-meta-label">Branches</div><div class="equation-summary-meta-value"><code>subsonic</code>, <code>supersonic</code></div><div class="equation-summary-meta-label">Subcategory</div><div class="equation-summary-meta-value">-</div>
</div>
</article>
<article class="equation-summary-card">
<h3 class="equation-summary-title"><a href="./rayleigh_stagnation_temperature_ratio.md">Rayleigh Stagnation Temperature Ratio</a></h3>
<div class="equation-summary-path"><code>compressible.rayleigh_stagnation_temperature_ratio</code></div>
<div class="equation-summary-latex">\(\frac{T_0}{T_0^*} = \frac{2(\gamma+1)M^2\left(1+\frac{\gamma-1}{2}M^2\right)}{(1+\gamma M^2)^2}\)</div>
<div class="equation-summary-meta">
<div class="equation-summary-meta-label">Targets</div><div class="equation-summary-meta-value"><code>M</code>, <code>t0_t0star</code></div><div class="equation-summary-meta-label">Default</div><div class="equation-summary-meta-value"><code>-</code></div><div class="equation-summary-meta-label">Branches</div><div class="equation-summary-meta-value"><code>subsonic</code>, <code>supersonic</code></div><div class="equation-summary-meta-label">Subcategory</div><div class="equation-summary-meta-value">-</div>
</div>
</article>
<article class="equation-summary-card">
<h3 class="equation-summary-title"><a href="./rayleigh_temperature_ratio.md">Rayleigh Temperature Ratio</a></h3>
<div class="equation-summary-path"><code>compressible.rayleigh_temperature_ratio</code></div>
<div class="equation-summary-latex">\(\frac{T}{T^*} = \frac{(\gamma+1)^2 M^2}{(1+\gamma M^2)^2}\)</div>
<div class="equation-summary-meta">
<div class="equation-summary-meta-label">Targets</div><div class="equation-summary-meta-value"><code>M</code>, <code>t_tstar</code></div><div class="equation-summary-meta-label">Default</div><div class="equation-summary-meta-value"><code>-</code></div><div class="equation-summary-meta-label">Branches</div><div class="equation-summary-meta-value"><code>subsonic</code>, <code>supersonic</code></div><div class="equation-summary-meta-label">Subcategory</div><div class="equation-summary-meta-value">-</div>
</div>
</article>
<article class="equation-summary-card">
<h3 class="equation-summary-title"><a href="./rayleigh_velocity_ratio.md">Rayleigh Velocity Ratio</a></h3>
<div class="equation-summary-path"><code>compressible.rayleigh_velocity_ratio</code></div>
<div class="equation-summary-latex">\(\frac{V}{V^*} = \frac{(\gamma+1)M^2}{1+\gamma M^2}\)</div>
<div class="equation-summary-meta">
<div class="equation-summary-meta-label">Targets</div><div class="equation-summary-meta-value"><code>M</code>, <code>v_vstar</code></div><div class="equation-summary-meta-label">Default</div><div class="equation-summary-meta-value"><code>-</code></div><div class="equation-summary-meta-label">Branches</div><div class="equation-summary-meta-value">-</div><div class="equation-summary-meta-label">Subcategory</div><div class="equation-summary-meta-value">-</div>
</div>
</article>
</div>

## Browse

- [Isentropic Area-Mach Relation](./area_mach.md)
- [Choked Mass Flux](./choked_mass_flux.md)
- [Fanno Density Ratio](./fanno_density_ratio.md)
- [Fanno Friction Length Parameter](./fanno_friction_parameter.md)
- [Fanno Pressure Ratio](./fanno_pressure_ratio.md)
- [Fanno Stagnation Pressure Ratio](./fanno_stagnation_pressure_ratio.md)
- [Fanno Temperature Ratio](./fanno_temperature_ratio.md)
- [Fanno Velocity Ratio](./fanno_velocity_ratio.md)
- [Isentropic Density Ratio](./isentropic_density_ratio.md)
- [Isentropic Pressure Ratio](./isentropic_pressure_ratio.md)
- [Isentropic Temperature Ratio](./isentropic_temperature_ratio.md)
- [Mach Angle](./mach_angle.md)
- [Normal Shock Density Ratio](./normal_shock_density_ratio.md)
- [Normal Shock Downstream Mach Number](./normal_shock_m2.md)
- [Normal Shock Static Pressure Ratio](./normal_shock_pressure_ratio.md)
- [Normal Shock Stagnation Pressure Ratio](./normal_shock_stagnation_pressure_ratio.md)
- [Normal Shock Temperature Ratio](./normal_shock_temperature_ratio.md)
- [Oblique Shock Downstream Mach](./oblique_shock_m2.md)
- [Oblique Shock Normal Upstream Mach](./oblique_shock_mn1.md)
- [Oblique Shock Theta-Beta-M Relation](./oblique_shock_theta_beta_m.md)
- [Prandtl-Meyer Expansion Angle](./prandtl_meyer.md)
- [Rayleigh Density Ratio](./rayleigh_density_ratio.md)
- [Rayleigh Pressure Ratio](./rayleigh_pressure_ratio.md)
- [Rayleigh Stagnation Pressure Ratio](./rayleigh_stagnation_pressure_ratio.md)
- [Rayleigh Stagnation Temperature Ratio](./rayleigh_stagnation_temperature_ratio.md)
- [Rayleigh Temperature Ratio](./rayleigh_temperature_ratio.md)
- [Rayleigh Velocity Ratio](./rayleigh_velocity_ratio.md)
