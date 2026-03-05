#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ConvergenceReport {
    pub iterations: usize,
    pub converged: bool,
    pub residual_abs: f64,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct RootSolveOutput {
    pub root: f64,
    pub report: ConvergenceReport,
}

#[derive(Debug)]
pub enum RootSolveError<E> {
    Residual(E),
    InvalidBracket {
        lo: f64,
        hi: f64,
        f_lo: f64,
        f_hi: f64,
    },
    NonFiniteResidual {
        x: f64,
        value: f64,
    },
    MaxIterations {
        lo: f64,
        hi: f64,
        last: f64,
    },
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct RootScanResult {
    pub roots: usize,
    pub scanned_samples: usize,
}

pub fn bisect_by_sign_change<E, F>(
    mut lo: f64,
    mut hi: f64,
    tol: f64,
    max_iter: usize,
    mut residual: F,
) -> Result<RootSolveOutput, RootSolveError<E>>
where
    F: FnMut(f64) -> Result<f64, E>,
{
    let mut f_lo = residual(lo).map_err(RootSolveError::Residual)?;
    let f_hi = residual(hi).map_err(RootSolveError::Residual)?;
    if !f_lo.is_finite() {
        return Err(RootSolveError::NonFiniteResidual { x: lo, value: f_lo });
    }
    if !f_hi.is_finite() {
        return Err(RootSolveError::NonFiniteResidual { x: hi, value: f_hi });
    }
    if f_lo.abs() <= tol {
        return Ok(RootSolveOutput {
            root: lo,
            report: ConvergenceReport {
                iterations: 0,
                converged: true,
                residual_abs: f_lo.abs(),
            },
        });
    }
    if f_hi.abs() <= tol {
        return Ok(RootSolveOutput {
            root: hi,
            report: ConvergenceReport {
                iterations: 0,
                converged: true,
                residual_abs: f_hi.abs(),
            },
        });
    }
    if f_lo * f_hi > 0.0 {
        return Err(RootSolveError::InvalidBracket { lo, hi, f_lo, f_hi });
    }

    let mut last = 0.5 * (lo + hi);
    for i in 1..=max_iter {
        let mid = 0.5 * (lo + hi);
        let f_mid = residual(mid).map_err(RootSolveError::Residual)?;
        if !f_mid.is_finite() {
            return Err(RootSolveError::NonFiniteResidual {
                x: mid,
                value: f_mid,
            });
        }
        last = mid;
        if f_mid.abs() <= tol || (hi - lo).abs() <= tol {
            return Ok(RootSolveOutput {
                root: mid,
                report: ConvergenceReport {
                    iterations: i,
                    converged: true,
                    residual_abs: f_mid.abs(),
                },
            });
        }
        if f_lo * f_mid <= 0.0 {
            hi = mid;
        } else {
            lo = mid;
            f_lo = f_mid;
        }
    }

    Err(RootSolveError::MaxIterations { lo, hi, last })
}

pub fn find_roots_by_scan_bisection<E, F>(
    x_min: f64,
    x_max: f64,
    samples: usize,
    residual_tol: f64,
    dedup_tol: f64,
    mut residual: F,
) -> Result<(Vec<f64>, RootScanResult), RootSolveError<E>>
where
    F: FnMut(f64) -> Result<f64, E>,
{
    if samples < 2 {
        return Ok((
            Vec::new(),
            RootScanResult {
                roots: 0,
                scanned_samples: samples,
            },
        ));
    }

    let mut roots = Vec::<f64>::new();
    let mut prev_x = x_min;
    let mut prev_f = residual(prev_x).map_err(RootSolveError::Residual)?;
    if !prev_f.is_finite() {
        return Err(RootSolveError::NonFiniteResidual {
            x: prev_x,
            value: prev_f,
        });
    }
    for i in 1..=samples {
        let frac = (i as f64) / (samples as f64);
        let x = x_min + (x_max - x_min) * frac;
        let f = residual(x).map_err(RootSolveError::Residual)?;
        if !f.is_finite() {
            return Err(RootSolveError::NonFiniteResidual { x, value: f });
        }
        if prev_f.abs() <= residual_tol {
            roots.push(prev_x);
        } else if f.abs() <= residual_tol {
            roots.push(x);
        } else if prev_f * f < 0.0 {
            let out = bisect_by_sign_change(prev_x, x, residual_tol, 120, &mut residual)?;
            roots.push(out.root);
        }
        prev_x = x;
        prev_f = f;
    }

    roots.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
    roots.dedup_by(|a, b| (*a - *b).abs() < dedup_tol);
    let result = RootScanResult {
        roots: roots.len(),
        scanned_samples: samples + 1,
    };
    Ok((roots, result))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bisection_solves_quadratic_root() {
        let out = bisect_by_sign_change(1.0, 2.0, 1e-12, 200, |x| Ok::<_, ()>(x * x - 2.0))
            .expect("bisection should converge");
        assert!((out.root - 2.0_f64.sqrt()).abs() < 1e-10);
        assert!(out.report.converged);
    }

    #[test]
    fn scan_finds_two_roots_for_shifted_quadratic() {
        let (roots, meta) = find_roots_by_scan_bisection(-5.0, 5.0, 250, 1e-10, 1e-7, |x| {
            Ok::<_, ()>((x - 2.0) * (x + 3.0))
        })
        .expect("scan roots");
        assert_eq!(meta.roots, 2);
        assert!(roots.iter().any(|r| (*r - 2.0).abs() < 1e-6));
        assert!(roots.iter().any(|r| (*r + 3.0).abs() < 1e-6));
    }
}
