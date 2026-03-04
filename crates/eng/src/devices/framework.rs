use equations::SolveMethod;

#[derive(Debug, Clone)]
pub struct CalcStep {
    pub equation_path_id: String,
    pub solved_for: String,
    pub method: String,
    pub branch: Option<String>,
    pub inputs_used: Vec<(String, f64)>,
}

#[derive(Debug, Clone)]
pub struct PivotCalcResponse {
    pub value_si: f64,
    pub pivot_value: f64,
    pub path: Vec<CalcStep>,
    pub warnings: Vec<String>,
}

pub trait PivotCalcSpec {
    type Request;
    type Error;

    fn validate_request(&self, _req: &Self::Request) -> Result<(), Self::Error> {
        Ok(())
    }

    fn resolve_pivot(
        &self,
        req: &Self::Request,
        path: &mut Vec<CalcStep>,
    ) -> Result<f64, Self::Error>;

    fn validate_pivot(&self, pivot_value: f64) -> Result<(), Self::Error>;

    fn solve_target(
        &self,
        req: &Self::Request,
        pivot_value: f64,
        path: &mut Vec<CalcStep>,
    ) -> Result<f64, Self::Error>;
}

pub fn run_pivot_calculation<S: PivotCalcSpec>(
    spec: &S,
    req: S::Request,
) -> Result<PivotCalcResponse, S::Error> {
    spec.validate_request(&req)?;
    let mut path = Vec::<CalcStep>::new();
    let pivot_value = spec.resolve_pivot(&req, &mut path)?;
    spec.validate_pivot(pivot_value)?;
    let value_si = spec.solve_target(&req, pivot_value, &mut path)?;
    Ok(PivotCalcResponse {
        value_si,
        pivot_value,
        path,
        warnings: Vec::new(),
    })
}

pub fn method_label(method: SolveMethod) -> String {
    match method {
        SolveMethod::Auto => "auto",
        SolveMethod::Explicit => "explicit",
        SolveMethod::Numerical => "numerical",
    }
    .to_string()
}

pub fn path_text(path: &[CalcStep]) -> String {
    path.iter()
        .map(|s| match &s.branch {
            Some(b) => format!(
                "{}:{} via {} [{}]",
                s.equation_path_id, s.solved_for, s.method, b
            ),
            None => format!("{}:{} via {}", s.equation_path_id, s.solved_for, s.method),
        })
        .collect::<Vec<_>>()
        .join(" -> ")
}

#[derive(Debug, Clone, Copy)]
pub struct CalculatorKindSpec {
    pub key: &'static str,
    pub label: &'static str,
}

#[derive(Debug, Clone, Copy)]
pub struct CalculatorDeviceSpec {
    pub key: &'static str,
    pub name: &'static str,
    pub summary: &'static str,
    pub route: &'static str,
    pub pivot_label: &'static str,
    pub input_kinds: &'static [CalculatorKindSpec],
    pub output_kinds: &'static [CalculatorKindSpec],
    pub branches: &'static [&'static str],
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Clone, Copy)]
    struct DummyReq {
        input: f64,
        scale: f64,
    }

    #[derive(Debug)]
    struct DummyErr;

    struct DummySpec;

    impl PivotCalcSpec for DummySpec {
        type Request = DummyReq;
        type Error = DummyErr;

        fn resolve_pivot(
            &self,
            req: &Self::Request,
            path: &mut Vec<CalcStep>,
        ) -> Result<f64, Self::Error> {
            path.push(CalcStep {
                equation_path_id: "dummy.to_pivot".to_string(),
                solved_for: "pivot".to_string(),
                method: "explicit".to_string(),
                branch: None,
                inputs_used: vec![("input".to_string(), req.input)],
            });
            Ok(req.input + 1.0)
        }

        fn validate_pivot(&self, pivot_value: f64) -> Result<(), Self::Error> {
            if pivot_value > 0.0 {
                Ok(())
            } else {
                Err(DummyErr)
            }
        }

        fn solve_target(
            &self,
            req: &Self::Request,
            pivot_value: f64,
            path: &mut Vec<CalcStep>,
        ) -> Result<f64, Self::Error> {
            path.push(CalcStep {
                equation_path_id: "dummy.from_pivot".to_string(),
                solved_for: "target".to_string(),
                method: "explicit".to_string(),
                branch: None,
                inputs_used: vec![("pivot".to_string(), pivot_value)],
            });
            Ok(pivot_value * req.scale)
        }
    }

    #[test]
    fn run_pivot_calculation_collects_path_and_value() {
        let out = run_pivot_calculation(
            &DummySpec,
            DummyReq {
                input: 2.0,
                scale: 3.0,
            },
        )
        .expect("dummy calc should succeed");
        assert_eq!(out.pivot_value, 3.0);
        assert_eq!(out.value_si, 9.0);
        assert_eq!(out.path.len(), 2);
        assert_eq!(
            path_text(&out.path),
            "dummy.to_pivot:pivot via explicit -> dummy.from_pivot:target via explicit"
        );
    }
}
