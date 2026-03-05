# Compressible

## Equation Summary

<table><thead><tr><th>Equation</th><th>Path ID</th><th>LaTeX</th><th>Targets</th><th>Default</th><th>Branches</th><th>Subcategory</th></tr></thead><tbody>
<tr><td><a href="./area_mach.md">Isentropic Area-Mach Relation</a></td><td><code>compressible.area_mach</code></td><td>\(\frac{A}{A^*} = \frac{1}{M}\left(\frac{2}{\gamma+1}\left(1+\frac{\gamma-1}{2}M^2\right)\right)^{\frac{\gamma+1}{2(\gamma-1)}}\)</td><td><code>M</code>, <code>area_ratio</code></td><td><code>-</code></td><td><code>subsonic</code>, <code>supersonic</code></td><td>-</td></tr>
<tr><td><a href="./choked_mass_flux.md">Choked Mass Flux</a></td><td><code>compressible.choked_mass_flux</code></td><td>\(G^* = \frac{p_0}{\sqrt{T_0}} \sqrt{\frac{\gamma}{R}} \left(\frac{2}{\gamma+1}\right)^{(\gamma+1)/(2(\gamma-1))}\)</td><td><code>G_star</code></td><td><code>-</code></td><td>-</td><td>-</td></tr>
<tr><td><a href="./fanno_density_ratio.md">Fanno Density Ratio</a></td><td><code>compressible.fanno_density_ratio</code></td><td>\(\frac{\rho}{\rho^*} = \frac{1}{M}\sqrt{\frac{1+\frac{\gamma-1}{2}M^2}{\frac{\gamma+1}{2}}}\)</td><td><code>M</code>, <code>rho_rhostar</code></td><td><code>-</code></td><td><code>subsonic</code>, <code>supersonic</code></td><td>-</td></tr>
<tr><td><a href="./fanno_friction_parameter.md">Fanno Friction Length Parameter</a></td><td><code>compressible.fanno_friction_parameter</code></td><td>\(\frac{4 f L^*}{D} = \frac{1-M^2}{\gamma M^2} + \frac{\gamma+1}{2\gamma}\ln\!\left(\frac{\frac{\gamma+1}{2}M^2}{1+\frac{\gamma-1}{2}M^2}\right)\)</td><td><code>M</code>, <code>four_flstar_d</code></td><td><code>-</code></td><td><code>subsonic</code>, <code>supersonic</code></td><td>-</td></tr>
<tr><td><a href="./fanno_pressure_ratio.md">Fanno Pressure Ratio</a></td><td><code>compressible.fanno_pressure_ratio</code></td><td>\(\frac{p}{p^*} = \frac{1}{M}\sqrt{\frac{\frac{\gamma+1}{2}}{1+\frac{\gamma-1}{2}M^2}}\)</td><td><code>M</code>, <code>p_pstar</code></td><td><code>-</code></td><td><code>subsonic</code>, <code>supersonic</code></td><td>-</td></tr>
<tr><td><a href="./fanno_stagnation_pressure_ratio.md">Fanno Stagnation Pressure Ratio</a></td><td><code>compressible.fanno_stagnation_pressure_ratio</code></td><td>\(\frac{p_0}{p_0^*} = \frac{1}{M}\left(\frac{1+\frac{\gamma-1}{2}M^2}{\frac{\gamma+1}{2}}\right)^{\frac{\gamma+1}{2(\gamma-1)}}\)</td><td><code>M</code>, <code>p0_p0star</code></td><td><code>-</code></td><td><code>subsonic</code>, <code>supersonic</code></td><td>-</td></tr>
<tr><td><a href="./fanno_temperature_ratio.md">Fanno Temperature Ratio</a></td><td><code>compressible.fanno_temperature_ratio</code></td><td>\(\frac{T}{T^*} = \frac{\frac{\gamma+1}{2}}{1+\frac{\gamma-1}{2}M^2}\)</td><td><code>M</code>, <code>t_tstar</code></td><td><code>-</code></td><td><code>subsonic</code>, <code>supersonic</code></td><td>-</td></tr>
<tr><td><a href="./fanno_velocity_ratio.md">Fanno Velocity Ratio</a></td><td><code>compressible.fanno_velocity_ratio</code></td><td>\(\frac{V}{V^*} = M\sqrt{\frac{\frac{\gamma+1}{2}}{1+\frac{\gamma-1}{2}M^2}}\)</td><td><code>M</code>, <code>v_vstar</code></td><td><code>-</code></td><td><code>subsonic</code>, <code>supersonic</code></td><td>-</td></tr>
<tr><td><a href="./isentropic_density_ratio.md">Isentropic Density Ratio</a></td><td><code>compressible.isentropic_density_ratio</code></td><td>\(\frac{\rho}{\rho_0} = \left(1+\frac{\gamma-1}{2}M^2\right)^{-1/(\gamma-1)}\)</td><td><code>M</code>, <code>rho_rho0</code></td><td><code>-</code></td><td>-</td><td>-</td></tr>
<tr><td><a href="./isentropic_pressure_ratio.md">Isentropic Pressure Ratio</a></td><td><code>compressible.isentropic_pressure_ratio</code></td><td>\(\frac{p}{p_0} = \left(1+\frac{\gamma-1}{2}M^2\right)^{-\gamma/(\gamma-1)}\)</td><td><code>M</code>, <code>p_p0</code></td><td><code>-</code></td><td>-</td><td>-</td></tr>
<tr><td><a href="./isentropic_temperature_ratio.md">Isentropic Temperature Ratio</a></td><td><code>compressible.isentropic_temperature_ratio</code></td><td>\(\frac{T}{T_0} = \left(1+\frac{\gamma-1}{2}M^2\right)^{-1}\)</td><td><code>M</code>, <code>T_T0</code></td><td><code>-</code></td><td>-</td><td>-</td></tr>
<tr><td><a href="./mach_angle.md">Mach Angle</a></td><td><code>compressible.mach_angle</code></td><td>\(\mu = \arcsin\left(\frac{1}{M}\right)\)</td><td><code>M</code>, <code>mu</code></td><td><code>-</code></td><td>-</td><td>-</td></tr>
<tr><td><a href="./normal_shock_density_ratio.md">Normal Shock Density Ratio</a></td><td><code>compressible.normal_shock_density_ratio</code></td><td>\(\frac{\rho_2}{\rho_1} = \frac{(\gamma+1)M_1^2}{(\gamma-1)M_1^2+2}\)</td><td><code>M1</code>, <code>rho2_rho1</code></td><td><code>-</code></td><td>-</td><td>-</td></tr>
<tr><td><a href="./normal_shock_m2.md">Normal Shock Downstream Mach Number</a></td><td><code>compressible.normal_shock_m2</code></td><td>\(M_2 = \sqrt{\frac{1 + \frac{\gamma-1}{2}M_1^2}{\gamma M_1^2 - \frac{\gamma-1}{2}}}\)</td><td><code>M1</code>, <code>M2</code></td><td><code>-</code></td><td>-</td><td>-</td></tr>
<tr><td><a href="./normal_shock_pressure_ratio.md">Normal Shock Static Pressure Ratio</a></td><td><code>compressible.normal_shock_pressure_ratio</code></td><td>\(\frac{p_2}{p_1} = 1 + \frac{2\gamma}{\gamma+1}(M_1^2-1)\)</td><td><code>M1</code>, <code>p2_p1</code></td><td><code>-</code></td><td>-</td><td>-</td></tr>
<tr><td><a href="./normal_shock_stagnation_pressure_ratio.md">Normal Shock Stagnation Pressure Ratio</a></td><td><code>compressible.normal_shock_stagnation_pressure_ratio</code></td><td>\(\frac{p_{02}}{p_{01}} = \left(\frac{(\gamma+1)M_1^2}{(\gamma-1)M_1^2+2}\right)^{\gamma/(\gamma-1)}\left(\frac{\gamma+1}{2\gamma M_1^2-(\gamma-1)}\right)^{1/(\gamma-1)}\)</td><td><code>M1</code>, <code>p02_p01</code></td><td><code>-</code></td><td>-</td><td>-</td></tr>
<tr><td><a href="./normal_shock_temperature_ratio.md">Normal Shock Temperature Ratio</a></td><td><code>compressible.normal_shock_temperature_ratio</code></td><td>\(\frac{T_2}{T_1} = \frac{p_2/p_1}{\rho_2/\rho_1} = \left(1+\frac{2\gamma}{\gamma+1}(M_1^2-1)\right)\frac{(\gamma-1)M_1^2+2}{(\gamma+1)M_1^2}\)</td><td><code>M1</code>, <code>T2_T1</code></td><td><code>-</code></td><td>-</td><td>-</td></tr>
<tr><td><a href="./oblique_shock_m2.md">Oblique Shock Downstream Mach</a></td><td><code>compressible.oblique_shock_m2</code></td><td>\(M_2 = \frac{M_{n2}}{\sin(\beta-\theta)}\)</td><td><code>beta</code>, <code>m2</code>, <code>mn2</code>, <code>theta</code></td><td><code>-</code></td><td>-</td><td>-</td></tr>
<tr><td><a href="./oblique_shock_mn1.md">Oblique Shock Normal Upstream Mach</a></td><td><code>compressible.oblique_shock_mn1</code></td><td>\(M_{n1} = M_1 \sin\beta\)</td><td><code>beta</code>, <code>m1</code>, <code>mn1</code></td><td><code>-</code></td><td>-</td><td>-</td></tr>
<tr><td><a href="./oblique_shock_theta_beta_m.md">Oblique Shock Theta-Beta-M Relation</a></td><td><code>compressible.oblique_shock_theta_beta_m</code></td><td>\(\tan\theta = \frac{2\cot\beta\left(M_1^2\sin^2\beta - 1\right)}{M_1^2\left(\gamma + \cos 2\beta\right)+2}\)</td><td><code>beta</code>, <code>theta</code></td><td><code>-</code></td><td><code>weak</code>, <code>strong</code></td><td>-</td></tr>
<tr><td><a href="./prandtl_meyer.md">Prandtl-Meyer Expansion Angle</a></td><td><code>compressible.prandtl_meyer</code></td><td>\(\nu = \sqrt{\frac{\gamma+1}{\gamma-1}} \tan^{-1}\!\left(\sqrt{\frac{\gamma-1}{\gamma+1}(M^2-1)}\right) - \tan^{-1}\!\left(\sqrt{M^2-1}\right)\)</td><td><code>M</code>, <code>nu</code></td><td><code>-</code></td><td>-</td><td>-</td></tr>
</tbody></table>

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
