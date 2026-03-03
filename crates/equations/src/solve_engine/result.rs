use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum SolveMethod {
    Auto,
    Explicit,
    Numerical,
}

#[derive(Debug, Clone)]
pub struct SolveRequest {
    pub target: String,
    pub knowns_si: HashMap<String, f64>,
    pub method: SolveMethod,
    pub branch: Option<String>,
}

#[derive(Debug, Clone)]
pub struct SolveResponse {
    pub target: String,
    pub value_si: f64,
    pub method_used: SolveMethod,
    pub residual: f64,
}
