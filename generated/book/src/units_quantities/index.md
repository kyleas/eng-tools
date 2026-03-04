# Units & Quantities

The unit engine is dimension-based. Equivalent expressions reduce to canonical SI using dimensional algebra.

Supported expression operators: `+`, `-`, `*`, `/`, parentheses.

- Addition/subtraction require same dimensions.
- Multiplication/division combine dimensions algebraically.
- Bare numbers are dimensionless.
- Input is validated against each variable's expected dimension.

Temperature note: affine temperature pitfalls are intentionally guarded; use canonical absolute temperature where required.
