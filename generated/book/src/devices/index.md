# Devices

- [Devices Guide](./guide.md)

| Device | Key | Summary | Modes |
| --- | --- | --- | --- |
| [Pipe Pressure Drop](./pipe_loss.md) | `pipe_loss` | Composes Reynolds + friction model + Darcy-Weisbach for pipe pressure loss. | Fixed friction factor, Colebrook |
| [Isentropic Calculator](./isentropic_calc.md) | `isentropic_calc` | Calculator-style compressible device: solve any supported isentropic input to any supported output through Mach pivot orchestration. | Input kinds: Mach, MachAngle, Prandtl-Meyer angle, p/p0, T/T0, rho/rho0, A/A*, Branch-aware inversion for A/A* |
| [Normal Shock Calculator](./normal_shock_calc.md) | `normal_shock_calc` | Calculator-style compressible device: solve normal-shock input kinds to target kinds through deterministic M1 pivot orchestration. | Input kinds: M1, M2, p2/p1, rho2/rho1, T2/T1, p02/p01, Target kinds: M1, M2, p2/p1, rho2/rho1, T2/T1, p02/p01 |
| [Nozzle Flow Calculator](./nozzle_flow_calc.md) | `nozzle_flow_calc` | Calculator-style quasi-1D nozzle device: solve isentropic nozzle input kinds to target kinds through Mach pivot orchestration. | Input kinds: Mach, A/A*, p/p0, T/T0, rho/rho0, Branch-aware inversion for A/A* -> Mach, Optional stagnation-reference scaling for static p/T/rho outputs |
| [Oblique Shock Calculator](./oblique_shock_calc.md) | `oblique_shock_calc` | Calculator-style compressible device: solve oblique-shock input pairs (M1+beta / M1+theta) to target outputs with explicit weak/strong branch handling. | Input pairs: (M1, beta), (M1, theta), Branch-aware inversion for (M1, theta) -> beta |
| [Fanno Flow Calculator](./fanno_flow_calc.md) | `fanno_flow_calc` | Calculator-style compressible device: solve Fanno star-reference input kinds to target kinds through Mach pivot orchestration. | Input kinds: Mach, T/T*, p/p*, rho/rho*, p0/p0*, 4fL*/D, Branch-aware inversion for ratio -> Mach |
| [Rayleigh Flow Calculator](./rayleigh_calc.md) | `rayleigh_calc` | Calculator-style compressible device: solve Rayleigh star-reference input kinds to target kinds through Mach pivot orchestration. | Input kinds: Mach, T/T*, p/p*, rho/rho*, T0/T0*, p0/p0*, V/V*, Branch-aware inversion for selected ratio -> Mach paths |
