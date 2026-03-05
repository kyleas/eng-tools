# Rockets

## Equation Summary

<table><thead><tr><th>Equation</th><th>Path ID</th><th>LaTeX</th><th>Targets</th><th>Default</th><th>Branches</th><th>Subcategory</th></tr></thead><tbody>
<tr><td><a href="./cstar_ideal.md">Ideal Characteristic Velocity</a></td><td><code>rockets.cstar_ideal</code></td><td>\(c^* = \sqrt{\frac{R T_c}{\gamma}} \left(\frac{\gamma+1}{2}\right)^{(\gamma+1)/(2(\gamma-1))}\)</td><td><code>c_star</code></td><td><code>-</code></td><td>-</td><td>-</td></tr>
<tr><td><a href="./specific_impulse_ideal.md">Ideal Specific Impulse</a></td><td><code>rockets.specific_impulse_ideal</code></td><td>\(I_{sp} = \frac{C_f c^*}{g_0}\)</td><td><code>C_f</code>, <code>I_sp</code>, <code>c_star</code></td><td><code>-</code></td><td>-</td><td>-</td></tr>
<tr><td><a href="./thrust_coefficient_ideal.md">Ideal Thrust Coefficient</a></td><td><code>rockets.thrust_coefficient_ideal</code></td><td>\(C_f = \sqrt{\frac{2\gamma^2}{\gamma-1}\left(\frac{2}{\gamma+1}\right)^{(\gamma+1)/(\gamma-1)}\left(1-\left(\frac{p_e}{p_c}\right)^{(\gamma-1)/\gamma}\right)} + \left(\frac{p_e}{p_c}-\frac{p_a}{p_c}\right)\frac{A_e}{A_t}\)</td><td><code>C_f</code></td><td><code>-</code></td><td>-</td><td>-</td></tr>
<tr><td><a href="./thrust_from_mass_flow.md">Thrust From Mass Flow and Effective Exhaust Velocity</a></td><td><code>rockets.thrust_from_mass_flow</code></td><td>\(F = \dot{m} c_{eff}\)</td><td><code>F</code>, <code>c_eff</code>, <code>m_dot</code></td><td><code>-</code></td><td>-</td><td>-</td></tr>
</tbody></table>

## Browse

- [Ideal Characteristic Velocity](./cstar_ideal.md)
- [Ideal Specific Impulse](./specific_impulse_ideal.md)
- [Ideal Thrust Coefficient](./thrust_coefficient_ideal.md)
- [Thrust From Mass Flow and Effective Exhaust Velocity](./thrust_from_mass_flow.md)
