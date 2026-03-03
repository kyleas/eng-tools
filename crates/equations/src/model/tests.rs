use indexmap::IndexMap;
use schemars::JsonSchema;
use serde::de::Deserializer;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, JsonSchema)]
pub struct TestsConfig {
    #[schemars(description = "Unit normalization policy for test input values.")]
    #[serde(default)]
    pub units_policy: UnitsPolicy,
    #[schemars(description = "Optional relation residual tolerance override.")]
    pub relation_tolerance: Option<RelationTolerance>,
    #[schemars(description = "Optional solve tolerance override.")]
    pub solve_tolerance: Option<CaseTolerance>,
    /// Authoring convenience alias for a single test case; normalized into `cases` at load time.
    #[schemars(description = "Single-case shorthand; equivalent to one entry in `cases`.")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub case: Option<TestCase>,
    #[schemars(description = "List of test cases for this equation.")]
    pub cases: Vec<TestCase>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum UnitsPolicy {
    #[default]
    AutoConvertToSi,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct RelationTolerance {
    pub abs: f64,
    pub rel: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct TestCase {
    #[serde(default = "default_case_id")]
    pub id: String,
    #[serde(default)]
    pub description: String,
    #[serde(default)]
    pub branch: Option<String>,
    #[schemars(
        description = "Full known-valid variable state; should include all equation variables."
    )]
    pub full_state: IndexMap<String, QuantityInput>,
    pub tolerances: Option<CaseTolerance>,
    #[serde(default)]
    pub verify: TestCaseVerify,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct TestCaseVerify {
    #[serde(default = "default_true")]
    pub residual_zero: bool,
    #[serde(default)]
    pub solve_targets: Vec<TestSolveTargetSpec>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(untagged)]
pub enum TestSolveTargetSpec {
    TargetOnly(String),
    Detailed(TestSolveTarget),
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct TestSolveTarget {
    pub target: String,
    #[serde(default)]
    pub methods: Option<Vec<MethodKind>>,
    pub tolerances: Option<CaseTolerance>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct CaseTolerance {
    pub abs: Option<f64>,
    pub rel: Option<f64>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum MethodKind {
    Explicit,
    Numerical,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(untagged)]
pub enum QuantityInput {
    Scalar(f64),
    StringValue(String),
    ValueUnit { value: f64, unit: String },
}

impl TestSolveTargetSpec {
    pub fn target(&self) -> &str {
        match self {
            Self::TargetOnly(target) => target,
            Self::Detailed(d) => &d.target,
        }
    }

    pub fn methods_or<'a>(&'a self, fallback: &'a [MethodKind]) -> &'a [MethodKind] {
        match self {
            Self::TargetOnly(_) => fallback,
            Self::Detailed(d) => d.methods.as_deref().unwrap_or(fallback),
        }
    }

    pub fn tolerances(&self) -> Option<&CaseTolerance> {
        match self {
            Self::TargetOnly(_) => None,
            Self::Detailed(d) => d.tolerances.as_ref(),
        }
    }
}

const fn default_true() -> bool {
    true
}

impl Default for TestCaseVerify {
    fn default() -> Self {
        Self {
            residual_zero: true,
            solve_targets: Vec::new(),
        }
    }
}

fn default_case_id() -> String {
    "case".to_string()
}

#[derive(Debug, Clone, Deserialize)]
#[serde(untagged)]
enum CaseListOrSingle {
    List(Vec<TestCaseInput>),
    Single(TestCaseInput),
}

#[derive(Debug, Clone, Deserialize)]
#[serde(untagged)]
enum TestCaseInput {
    Full(TestCase),
    FullStateKeyOnly {
        full_state: IndexMap<String, QuantityInput>,
    },
    FullStateOnly(IndexMap<String, QuantityInput>),
}

#[derive(Debug, Clone, Deserialize)]
struct TestsConfigWire {
    #[serde(default)]
    units_policy: UnitsPolicy,
    relation_tolerance: Option<RelationTolerance>,
    solve_tolerance: Option<CaseTolerance>,
    #[serde(default)]
    cases: Option<CaseListOrSingle>,
    #[serde(default)]
    case: Option<TestCaseInput>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(untagged)]
enum TestsConfigWireOrCases {
    Config(TestsConfigWire),
    Cases(Vec<TestCaseInput>),
}

impl<'de> Deserialize<'de> for TestsConfig {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let input = TestsConfigWireOrCases::deserialize(deserializer)?;
        let (units_policy, relation_tolerance, solve_tolerance, raw_cases) = match input {
            TestsConfigWireOrCases::Config(wire) => {
                let mut raw_cases = match wire.cases {
                    Some(CaseListOrSingle::List(list)) => list,
                    Some(CaseListOrSingle::Single(single)) => vec![single],
                    None => Vec::new(),
                };
                if let Some(case) = wire.case {
                    raw_cases.push(case);
                }
                (
                    wire.units_policy,
                    wire.relation_tolerance,
                    wire.solve_tolerance,
                    raw_cases,
                )
            }
            TestsConfigWireOrCases::Cases(cases) => (UnitsPolicy::default(), None, None, cases),
        };

        let cases = raw_cases
            .into_iter()
            .enumerate()
            .map(|(idx, c)| match c {
                TestCaseInput::FullStateOnly(full_state) => TestCase {
                    id: format!("case_{}", idx + 1),
                    description: String::new(),
                    branch: None,
                    full_state,
                    tolerances: None,
                    verify: TestCaseVerify::default(),
                },
                TestCaseInput::FullStateKeyOnly { full_state } => TestCase {
                    id: format!("case_{}", idx + 1),
                    description: String::new(),
                    branch: None,
                    full_state,
                    tolerances: None,
                    verify: TestCaseVerify::default(),
                },
                TestCaseInput::Full(mut tc) => {
                    if tc.id.trim().is_empty() {
                        tc.id = format!("case_{}", idx + 1);
                    }
                    tc
                }
            })
            .collect();

        Ok(Self {
            units_policy,
            relation_tolerance,
            solve_tolerance,
            case: None,
            cases,
        })
    }
}
