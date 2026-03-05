#[derive(Debug)]
pub enum OdeSolveError<E> {
    Rhs(E),
    NonFiniteState { x: f64, y1: f64, y2: f64 },
}

pub fn rk4_step_2<E, F>(
    x: f64,
    y1: f64,
    y2: f64,
    h: f64,
    mut rhs: F,
) -> Result<(f64, f64), OdeSolveError<E>>
where
    F: FnMut(f64, f64, f64) -> Result<(f64, f64), E>,
{
    let (k1y1, k1y2) = rhs(x, y1, y2).map_err(OdeSolveError::Rhs)?;
    let (k2y1, k2y2) =
        rhs(x + 0.5 * h, y1 + 0.5 * h * k1y1, y2 + 0.5 * h * k1y2).map_err(OdeSolveError::Rhs)?;
    let (k3y1, k3y2) =
        rhs(x + 0.5 * h, y1 + 0.5 * h * k2y1, y2 + 0.5 * h * k2y2).map_err(OdeSolveError::Rhs)?;
    let (k4y1, k4y2) = rhs(x + h, y1 + h * k3y1, y2 + h * k3y2).map_err(OdeSolveError::Rhs)?;

    let next_y1 = y1 + (h / 6.0) * (k1y1 + 2.0 * k2y1 + 2.0 * k3y1 + k4y1);
    let next_y2 = y2 + (h / 6.0) * (k1y2 + 2.0 * k2y2 + 2.0 * k3y2 + k4y2);
    if !next_y1.is_finite() || !next_y2.is_finite() {
        return Err(OdeSolveError::NonFiniteState {
            x: x + h,
            y1: next_y1,
            y2: next_y2,
        });
    }
    Ok((next_y1, next_y2))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rk4_two_state_tracks_simple_harmonic_oscillator() {
        // y1' = y2, y2' = -y1 with y1(0)=1, y2(0)=0 => y1=cos(x), y2=-sin(x)
        let mut x = 0.0_f64;
        let mut y1 = 1.0_f64;
        let mut y2 = 0.0_f64;
        let h = 0.01_f64;
        for _ in 0..100 {
            let (n1, n2) =
                rk4_step_2(x, y1, y2, h, |_, a, b| Ok::<_, ()>((b, -a))).expect("rk4 step");
            x += h;
            y1 = n1;
            y2 = n2;
        }
        assert!((y1 - x.cos()).abs() < 5e-7);
        assert!((y2 + x.sin()).abs() < 5e-7);
    }
}
