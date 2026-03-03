# Units & Quantities

The engineering unit system is **dimension-based**, not hardcoded permutation-based. Unit expressions are parsed, reduced with dimensional algebra, and validated against expected variable dimensions.

## System Model

- Atomic unit registry defines aliases, canonical symbols, scale factors, and dimension signatures.
- Parser handles unit expressions and quantity arithmetic.
- Reducer computes canonical SI value + final dimension signature.
- Validator checks the final dimension against the variable's expected dimension.

## What Is Supported

- Atomic units + strict aliases (`m`, `ft`, `Pa`, `psi`, `L`, `gal`, `s`, `min`, ...).
- Compound units (`kg/(m*s)`, `Pa*s`, `N*s/m^2`, `W/(m*K)`, `J/(kg*K)`).
- Quantity expressions with `+`, `-`, `*`, `/`, and parentheses.
- Equivalent expressions normalized to the same canonical dimensions.

## What Gets Checked

- Parse syntax validity.
- Unit token/alias validity.
- Dimensional operator rules:
  - `+` / `-` require same dimensions.
  - `*` / `/` combine exponents algebraically.
- Final expression dimension matches expected variable dimension.

## Advanced Conversion Behavior

- Dynamic viscosity equivalence: `Pa*s` == `kg/(m*s)` == `N*s/m^2`.
- Volumetric flow conversions across time/volume units (`gal/min`, `L/s`, `m^3/hr`).
- Thermal/transport forms normalize by dimension even when syntactically different.

## What Is Intentionally Restricted

- Mixed-dimension addition/subtraction is rejected.
- Unknown or ambiguous unit tokens are rejected, not guessed.
- Affine temperature pitfalls are guarded; prefer canonical absolute temperature (`K`) unless a dedicated path says otherwise.

## Input Paths and Tradeoffs

- `f64`: fastest if already SI.
- Typed constructors: explicit units with low overhead.
- `qty!(...)`: preferred fixed-expression Rust path.
- Runtime strings: boundary-input convenience path.

## Example Validation Outcomes

- Valid: `5 MPa + 12 psi` (same pressure dimension).
- Valid: `3 ft + 2 in` (same length dimension).
- Invalid: `5 MPa + 3 m` (pressure + length).
- Invalid: unknown unit token (`blarg`) or malformed expression.
