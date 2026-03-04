# Devices

- [Devices Guide](./guide.md)

| Device | Key | Summary | Modes |
| --- | --- | --- | --- |
| [Pipe Pressure Drop](./pipe_loss.md) | `pipe_loss` | Composes Reynolds + friction model + Darcy-Weisbach for pipe pressure loss. | Fixed friction factor, Colebrook |
| [Isentropic Calculator](./isentropic_calc.md) | `isentropic_calc` | Calculator-style compressible device: solve any supported isentropic input to any supported output through Mach pivot orchestration. | Input kinds: Mach, MachAngle, p/p0, T/T0, rho/rho0, A/A*, Branch-aware inversion for A/A* |
