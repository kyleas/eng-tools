# Devices

- [Devices Guide](./guide.md)

| Device | Key | Summary | Modes |
| --- | --- | --- | --- |
| [Pipe Pressure Drop](./pipe_loss.md) | `pipe_loss` | Composes Reynolds + friction model + Darcy-Weisbach for pipe pressure loss. | Fixed friction factor, Colebrook |
| [Isentropic Calculator](./isentropic_calc.md) | `isentropic_calc` | Calculator-style compressible device: solve any supported isentropic input to any supported output through Mach pivot orchestration. | Input kinds: Mach, MachAngle, Prandtl-Meyer angle, p/p0, T/T0, rho/rho0, A/A*, Branch-aware inversion for A/A* |
| [Normal Shock Calculator](./normal_shock_calc.md) | `normal_shock_calc` | Calculator-style compressible device: solve normal-shock input kinds to target kinds through deterministic M1 pivot orchestration. | Input kinds: M1, M2, p2/p1, rho2/rho1, T2/T1, p02/p01, Target kinds: M1, M2, p2/p1, rho2/rho1, T2/T1, p02/p01 |
