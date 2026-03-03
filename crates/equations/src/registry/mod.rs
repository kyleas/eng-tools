pub mod ids;
pub mod lint;
pub mod loader;
pub mod validate;

use std::{
    collections::HashMap,
    fs,
    path::{Path, PathBuf},
};

use crate::{
    equation_families,
    error::{EquationError, Result},
    model::EquationDef,
    registry::{
        ids::derive_path_id, lint::lint_registry, loader::load_equations_from_dir,
        validate::validate_registry_definitions,
    },
    solve_engine::{SolveMethod, SolveRequest, SolveResponse, solve_equation},
    testing::run_registry_tests,
};

#[derive(Debug, Clone)]
pub struct Registry {
    root_dir: PathBuf,
    equations: Vec<EquationDef>,
    by_key: HashMap<String, usize>,
    by_path_id: HashMap<String, usize>,
    by_alias: HashMap<String, usize>,
}

impl Registry {
    /// Load equations from the default in-repo registry directory.
    ///
    /// Resolution order:
    /// 1. `ENG_EQUATIONS_REGISTRY_DIR` environment variable (if set)
    /// 2. `<equations crate root>/registry`
    pub fn load_default() -> Result<Self> {
        if let Ok(path) = std::env::var("ENG_EQUATIONS_REGISTRY_DIR")
            && !path.trim().is_empty()
        {
            return Self::load_from_dir(path);
        }
        Self::load_from_dir(PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("registry"))
    }

    /// Load all equation YAML definitions from a registry directory.
    ///
    /// ```no_run
    /// use equations::Registry;
    /// let registry = Registry::load_from_dir("crates/equations/registry").expect("load");
    /// assert!(!registry.equations().is_empty());
    /// ```
    pub fn load_from_dir(dir: impl AsRef<Path>) -> Result<Self> {
        let root_dir = dir.as_ref().to_path_buf();
        let equations = load_equations_from_dir(&root_dir)?;
        let mut registry = Self {
            root_dir,
            equations,
            by_key: HashMap::new(),
            by_path_id: HashMap::new(),
            by_alias: HashMap::new(),
        };
        registry.reindex()?;
        Ok(registry)
    }

    pub fn root_dir(&self) -> &Path {
        &self.root_dir
    }

    pub fn equations(&self) -> &[EquationDef] {
        &self.equations
    }

    /// Validate registry structure and equation metadata.
    pub fn validate(&self) -> Result<()> {
        validate_registry_definitions(&self.equations)?;
        if uses_default_registry_root(&self.root_dir) {
            equation_families::load_default_validated(&self.equations)?;
        }
        Ok(())
    }

    /// Validate registry and run all registry-defined equation test cases.
    pub fn validate_with_tests(&self) -> Result<()> {
        self.validate()?;
        let summary = run_registry_tests(self)?;
        if summary.failed > 0 {
            return Err(EquationError::Validation(format!(
                "registry tests failed: {} failed, {} passed",
                summary.failed, summary.passed
            )));
        }
        Ok(())
    }

    /// Run non-fatal authoring lints and return warnings.
    pub fn lint(&self) -> Result<Vec<lint::LintWarning>> {
        lint_registry(&self.equations)
    }

    /// Lookup by authored short key (for example `hoop_stress`).
    pub fn get_by_key(&self, key: &str) -> Option<&EquationDef> {
        self.by_key.get(key).map(|idx| &self.equations[*idx])
    }

    /// Lookup by taxonomy-derived path id (for example `structures.hoop_stress`).
    pub fn get_by_path_id(&self, path_id: &str) -> Option<&EquationDef> {
        self.by_path_id
            .get(path_id)
            .map(|idx| &self.equations[*idx])
    }

    /// Lookup by configured alias.
    pub fn get_by_alias(&self, alias: &str) -> Option<&EquationDef> {
        self.by_alias.get(alias).map(|idx| &self.equations[*idx])
    }

    /// Lookup by key, path id, or alias in that order.
    pub fn get_by_any_id(&self, id: &str) -> Option<&EquationDef> {
        self.get_by_key(id)
            .or_else(|| self.get_by_path_id(id))
            .or_else(|| self.get_by_alias(id))
    }

    /// Resolve an equation by key/path_id/alias or return an error.
    pub fn equation(&self, id: &str) -> Result<&EquationDef> {
        self.get_by_any_id(id)
            .ok_or_else(|| EquationError::UnknownEquationId {
                id: id.to_string(),
                suggestion: self.equation_suggestion(id),
            })
    }

    /// Solve an equation in one call using automatic method selection.
    ///
    /// `equation_id` supports key, path id, or alias.
    pub fn solve<I, K>(
        &self,
        equation_id: &str,
        target: &str,
        knowns_si: I,
    ) -> Result<SolveResponse>
    where
        I: IntoIterator<Item = (K, f64)>,
        K: Into<String>,
    {
        self.solve_with_method(equation_id, target, knowns_si, SolveMethod::Auto, None)
    }

    /// Solve and return only the SI value.
    pub fn solve_value<I, K>(&self, equation_id: &str, target: &str, knowns_si: I) -> Result<f64>
    where
        I: IntoIterator<Item = (K, f64)>,
        K: Into<String>,
    {
        Ok(self.solve(equation_id, target, knowns_si)?.value_si)
    }

    /// Solve an equation in one call with explicit method/branch controls.
    pub fn solve_with_method<I, K>(
        &self,
        equation_id: &str,
        target: &str,
        knowns_si: I,
        method: SolveMethod,
        branch: Option<&str>,
    ) -> Result<SolveResponse>
    where
        I: IntoIterator<Item = (K, f64)>,
        K: Into<String>,
    {
        let equation = self.equation(equation_id)?;
        let knowns_si: HashMap<String, f64> =
            knowns_si.into_iter().map(|(k, v)| (k.into(), v)).collect();
        solve_equation(
            equation,
            SolveRequest {
                target: target.to_string(),
                knowns_si,
                method,
                branch: branch.map(str::to_string),
            },
        )
    }

    fn reindex(&mut self) -> Result<()> {
        self.by_key.clear();
        self.by_path_id.clear();
        self.by_alias.clear();
        for (idx, eq) in self.equations.iter().enumerate() {
            if self.by_key.insert(eq.key.clone(), idx).is_some() {
                return Err(EquationError::Validation(format!(
                    "duplicate key '{}' while indexing",
                    eq.key
                )));
            }
            let path_id = derive_path_id(eq);
            if self.by_path_id.insert(path_id.clone(), idx).is_some() {
                return Err(EquationError::Validation(format!(
                    "duplicate path_id '{}' while indexing",
                    path_id
                )));
            }
            for alias in &eq.aliases {
                if self.by_alias.insert(alias.clone(), idx).is_some() {
                    return Err(EquationError::Validation(format!(
                        "duplicate alias '{}' while indexing",
                        alias
                    )));
                }
            }
        }
        Ok(())
    }

    fn equation_suggestion(&self, input: &str) -> String {
        let mut candidates: Vec<&str> = self.by_key.keys().map(String::as_str).collect();
        candidates.extend(self.by_path_id.keys().map(String::as_str));
        candidates.extend(self.by_alias.keys().map(String::as_str));
        closest_candidate(input, &candidates)
            .map(|s| format!(" (did you mean '{}'?)", s))
            .unwrap_or_default()
    }
}

fn uses_default_registry_root(root_dir: &Path) -> bool {
    let default_root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("registry");
    match (fs::canonicalize(root_dir), fs::canonicalize(default_root)) {
        (Ok(a), Ok(b)) => a == b,
        _ => false,
    }
}

fn closest_candidate<'a>(input: &str, candidates: &'a [&str]) -> Option<&'a str> {
    let mut best: Option<(&str, usize)> = None;
    for &c in candidates {
        let d = levenshtein(input, c);
        if d <= 4 {
            match best {
                Some((_, bd)) if d >= bd => {}
                _ => best = Some((c, d)),
            }
        }
    }
    best.map(|(s, _)| s)
}

fn levenshtein(a: &str, b: &str) -> usize {
    if a == b {
        return 0;
    }
    let b_len = b.chars().count();
    let mut prev: Vec<usize> = (0..=b_len).collect();
    let mut curr = vec![0; b_len + 1];
    for (i, ca) in a.chars().enumerate() {
        curr[0] = i + 1;
        for (j, cb) in b.chars().enumerate() {
            let cost = usize::from(ca != cb);
            curr[j + 1] = (curr[j] + 1).min(prev[j + 1] + 1).min(prev[j] + cost);
        }
        prev.copy_from_slice(&curr);
    }
    prev[b_len]
}
