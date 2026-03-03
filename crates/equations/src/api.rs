use std::collections::HashMap;

use eng_core::units::typed::{ExprInput, QuantityKind, UnitInput};
use eng_core::units::{
    convert_equation_value_from_si, ensure_signature_matches_dimension,
    parse_equation_quantity_to_si,
};

use crate::{
    EquationError, Result, constants, facade::EqFacade, model::EquationDef,
    solve_engine::SolveMethod,
};

pub trait IntoEquationId {
    fn equation_id(&self) -> &str;
}

impl IntoEquationId for &str {
    fn equation_id(&self) -> &str {
        self
    }
}

impl IntoEquationId for String {
    fn equation_id(&self) -> &str {
        self.as_str()
    }
}

pub trait SolveStart {
    type Builder;
    fn into_builder(self, facade: EqFacade) -> Self::Builder;
}

impl SolveStart for &str {
    type Builder = SolveBuilder;
    fn into_builder(self, facade: EqFacade) -> Self::Builder {
        SolveBuilder::new(facade, self)
    }
}

impl SolveStart for String {
    type Builder = SolveBuilder;
    fn into_builder(self, facade: EqFacade) -> Self::Builder {
        SolveBuilder::new(facade, &self)
    }
}

#[derive(Debug, Clone)]
pub enum SolveInputValue {
    Si(f64),
    WithUnit(String),
    Typed(UnitInput),
    Expr(ExprInput),
}

pub trait IntoSolveInput {
    fn into_solve_input(self) -> SolveInputValue;
}

impl IntoSolveInput for f64 {
    fn into_solve_input(self) -> SolveInputValue {
        SolveInputValue::Si(self)
    }
}

impl IntoSolveInput for &str {
    fn into_solve_input(self) -> SolveInputValue {
        SolveInputValue::WithUnit(self.to_string())
    }
}

impl IntoSolveInput for String {
    fn into_solve_input(self) -> SolveInputValue {
        SolveInputValue::WithUnit(self)
    }
}

impl IntoSolveInput for UnitInput {
    fn into_solve_input(self) -> SolveInputValue {
        SolveInputValue::Typed(self)
    }
}

impl IntoSolveInput for ExprInput {
    fn into_solve_input(self) -> SolveInputValue {
        SolveInputValue::Expr(self)
    }
}

#[derive(Debug, Clone)]
pub struct SolveResult {
    pub equation_id: String,
    pub target: String,
    pub method: SolveMethod,
    pub branch: Option<String>,
    pub value_si: f64,
    pub residual_abs: f64,
    pub residual_rel: f64,
    pub warnings: Vec<String>,
    pub notes: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct SolveBuilder {
    facade: EqFacade,
    equation_id: String,
    target: Option<String>,
    givens: HashMap<String, SolveInputValue>,
    constant_overrides: HashMap<String, SolveInputValue>,
    method: SolveMethod,
    branch: Option<String>,
}

impl SolveBuilder {
    pub(crate) fn new(facade: EqFacade, equation_id: &str) -> Self {
        Self {
            facade,
            equation_id: equation_id.to_string(),
            target: None,
            givens: HashMap::new(),
            constant_overrides: HashMap::new(),
            method: SolveMethod::Auto,
            branch: None,
        }
    }

    pub fn for_target(mut self, target: &str) -> Self {
        self.target = Some(target.to_string());
        self
    }

    pub fn given<K, V>(mut self, name: K, value: V) -> Self
    where
        K: Into<String>,
        V: IntoSolveInput,
    {
        self.givens.insert(name.into(), value.into_solve_input());
        self
    }

    pub fn givens<I, K, V>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = (K, V)>,
        K: Into<String>,
        V: IntoSolveInput,
    {
        for (k, v) in values {
            self.givens.insert(k.into(), v.into_solve_input());
        }
        self
    }

    /// Override a registry constant by key/symbol/alias for this solve call.
    ///
    /// Use this only for advanced scenarios. Normal usage auto-resolves constants from the
    /// constants registry.
    pub fn override_constant<V>(mut self, name: &str, value: V) -> Self
    where
        V: IntoSolveInput,
    {
        self.constant_overrides
            .insert(name.to_string(), value.into_solve_input());
        self
    }

    /// Override multiple registry constants for this solve call.
    pub fn override_constants<I, K, V>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = (K, V)>,
        K: Into<String>,
        V: IntoSolveInput,
    {
        for (k, v) in values {
            self.constant_overrides
                .insert(k.into(), v.into_solve_input());
        }
        self
    }

    pub fn method(mut self, method: SolveMethod) -> Self {
        self.method = method;
        self
    }

    pub fn branch(mut self, branch: &str) -> Self {
        self.branch = Some(branch.to_string());
        self
    }

    pub fn value(self) -> Result<f64> {
        Ok(self.result()?.value_si)
    }

    pub fn value_in(self, unit: &str) -> Result<f64> {
        let (equation, result) = self.execute()?;
        let target = equation.variables.get(&result.target).ok_or_else(|| {
            EquationError::Validation(format!("target '{}' missing", result.target))
        })?;
        convert_equation_value_from_si(&target.dimension, unit, result.value_si).map_err(|e| {
            EquationError::Unit {
                variable: result.target,
                message: e.to_string(),
            }
        })
    }

    pub fn result(self) -> Result<SolveResult> {
        let (_equation, result) = self.execute()?;
        Ok(result)
    }

    pub fn equation_id(&self) -> &str {
        &self.equation_id
    }

    fn execute(self) -> Result<(&'static EquationDef, SolveResult)> {
        let target = self.target.ok_or_else(|| {
            EquationError::Validation("missing solve target: call for_target(...)".to_string())
        })?;
        let equation = self.facade.equation(&self.equation_id)?;
        if !equation.variables.contains_key(&target) {
            return Err(EquationError::InvalidSolveTarget {
                equation_key: equation.key.clone(),
                target,
                valid_targets: equation
                    .variables
                    .keys()
                    .cloned()
                    .collect::<Vec<_>>()
                    .join(", "),
            });
        }
        if let Some(branch) = self.branch.as_deref()
            && !equation.branches.iter().any(|b| b.name == branch)
        {
            let valid: Vec<String> = equation.branches.iter().map(|b| b.name.clone()).collect();
            return Err(EquationError::InvalidBranch {
                equation_key: equation.key.clone(),
                branch: branch.to_string(),
                valid_branches: valid.join(", "),
                suggestion: suggestion_suffix(branch, &valid),
            });
        }
        let constant_overrides_si = convert_constant_overrides_to_si(&self.constant_overrides)?;
        let mut auto_constants_si = resolve_auto_constants_for_equation(
            equation,
            &target,
            &self.givens,
            &constant_overrides_si,
        );
        apply_constant_overrides_to_symbol_map(&mut auto_constants_si, &constant_overrides_si);

        let missing: Vec<String> = equation
            .variables
            .keys()
            .filter(|k| {
                *k != &target
                    && !self.givens.contains_key(*k)
                    && !auto_constants_si.contains_key(*k)
            })
            .cloned()
            .collect();
        if !missing.is_empty() {
            return Err(EquationError::Validation(format!(
                "equation '{}' target '{}' missing givens: {}",
                equation.key,
                target,
                missing.join(", ")
            )));
        }
        let knowns_si = convert_givens_to_si(equation, &self.givens)?;
        let mut knowns_si = knowns_si;
        knowns_si.extend(auto_constants_si);

        let solved =
            equation.solve_with_method(&target, knowns_si, self.method, self.branch.as_deref())?;
        let residual_abs = solved.residual.abs();
        let residual_rel = residual_abs / solved.value_si.abs().max(1.0);
        let result = SolveResult {
            equation_id: self.equation_id,
            target: solved.target,
            method: solved.method_used,
            branch: self.branch,
            value_si: solved.value_si,
            residual_abs,
            residual_rel,
            warnings: Vec::new(),
            notes: Vec::new(),
        };
        Ok((equation, result))
    }
}

fn convert_givens_to_si(
    equation: &EquationDef,
    givens: &HashMap<String, SolveInputValue>,
) -> Result<HashMap<String, f64>> {
    let mut out = HashMap::with_capacity(givens.len());
    let valid_vars: Vec<String> = equation.variables.keys().cloned().collect();
    for (name, value) in givens {
        let var = equation.variables.get(name).ok_or_else(|| {
            let hint = suggestion_suffix(name, &valid_vars);
            let constant_hint = constants::get_by_identifier(name)
                .map(|_| ". This identifier matches a registry constant; use override_constant(...) instead of given(...).")
                .unwrap_or_default();
            EquationError::Validation(format!(
                "equation '{}' unknown given variable '{}'{}. valid variables: {}{}",
                equation.key,
                name,
                hint,
                valid_vars.join(", "),
                constant_hint
            ))
        })?;
        let v_si = match value {
            SolveInputValue::Si(v) => *v,
            SolveInputValue::WithUnit(text) => parse_equation_quantity_to_si(&var.dimension, text)
                .map_err(|e| EquationError::Unit {
                    variable: name.clone(),
                    message: format!(
                        "{} (input='{}'; expected formats like '2.5 MPa' or SI numeric)",
                        e, text
                    ),
                })?,
            SolveInputValue::Typed(q) => {
                ensure_kind_matches_dimension(q.kind, &var.dimension).map_err(|msg| {
                    EquationError::Unit {
                        variable: name.clone(),
                        message: msg,
                    }
                })?;
                q.value_si
            }
            SolveInputValue::Expr(q) => {
                ensure_signature_matches_dimension(q.signature, &var.dimension).map_err(|e| {
                    EquationError::Unit {
                        variable: name.clone(),
                        message: format!(
                            "{} (expression input must reduce to {})",
                            e, var.dimension
                        ),
                    }
                })?;
                q.value_si
            }
        };
        out.insert(name.clone(), v_si);
    }
    Ok(out)
}

fn convert_constant_overrides_to_si(
    overrides: &HashMap<String, SolveInputValue>,
) -> Result<HashMap<String, f64>> {
    let mut out = HashMap::new();
    for (name, value) in overrides {
        let constant = constants::get_by_identifier(name).ok_or_else(|| {
            EquationError::Validation(format!(
                "unknown constant override '{}'. use a constant key/symbol/alias from equations::constants",
                name
            ))
        })?;
        let v_si = match value {
            SolveInputValue::Si(v) => *v,
            SolveInputValue::WithUnit(text) => {
                parse_equation_quantity_to_si(&constant.dimension, text).map_err(|e| {
                    EquationError::Unit {
                        variable: name.clone(),
                        message: format!(
                            "{} (input='{}'; expected formats like '9.80665 m/s2' or SI numeric)",
                            e, text
                        ),
                    }
                })?
            }
            SolveInputValue::Typed(q) => {
                ensure_kind_matches_dimension(q.kind, &constant.dimension).map_err(|msg| {
                    EquationError::Unit {
                        variable: name.clone(),
                        message: msg,
                    }
                })?;
                q.value_si
            }
            SolveInputValue::Expr(q) => {
                ensure_signature_matches_dimension(q.signature, &constant.dimension).map_err(
                    |e| EquationError::Unit {
                        variable: name.clone(),
                        message: format!(
                            "{} (expression input must reduce to {})",
                            e, constant.dimension
                        ),
                    },
                )?;
                q.value_si
            }
        };
        out.insert(constant.key.to_string(), v_si);
    }
    Ok(out)
}

fn resolve_auto_constants_for_equation(
    equation: &EquationDef,
    target: &str,
    givens: &HashMap<String, SolveInputValue>,
    overrides_by_key: &HashMap<String, f64>,
) -> HashMap<String, f64> {
    let mut out = HashMap::new();
    for name in equation.variables.keys() {
        if name == target || givens.contains_key(name) {
            continue;
        }
        if let Some(constant) = constants::get_by_identifier(name) {
            let value = overrides_by_key
                .get(constant.key)
                .copied()
                .unwrap_or(constant.value);
            out.insert(name.clone(), value);
            out.insert(constant.key.to_string(), value);
            out.insert(constant.symbol_ascii.to_string(), value);
            for alias in constant.aliases {
                out.insert(alias.to_string(), value);
            }
        }
    }
    out
}

fn apply_constant_overrides_to_symbol_map(
    values: &mut HashMap<String, f64>,
    overrides_by_key: &HashMap<String, f64>,
) {
    for (key, value) in overrides_by_key {
        if let Some(constant) = constants::get(key) {
            values.insert(constant.key.to_string(), *value);
            values.insert(constant.symbol_ascii.to_string(), *value);
            for alias in constant.aliases {
                values.insert(alias.to_string(), *value);
            }
        }
    }
}

fn suggestion_suffix(input: &str, candidates: &[String]) -> String {
    closest_candidate(input, candidates)
        .map(|s| format!(" (did you mean '{}'?)", s))
        .unwrap_or_default()
}

fn closest_candidate<'a>(input: &str, candidates: &'a [String]) -> Option<&'a str> {
    let mut best: Option<(&str, usize)> = None;
    for c in candidates {
        let d = levenshtein(input, c);
        if d <= 3 {
            match best {
                Some((_, bd)) if d >= bd => {}
                _ => best = Some((c.as_str(), d)),
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

fn ensure_kind_matches_dimension(
    kind: QuantityKind,
    dimension: &str,
) -> std::result::Result<(), String> {
    let dim = dimension.trim().to_ascii_lowercase().replace(' ', "_");
    let ok = match dim.as_str() {
        "pressure" | "stress" => kind == QuantityKind::Pressure,
        "length" | "diameter" | "distance" | "roughness" => kind == QuantityKind::Length,
        "area" => kind == QuantityKind::Area,
        "density" => kind == QuantityKind::Density,
        "viscosity" | "dynamic_viscosity" => kind == QuantityKind::DynamicViscosity,
        "force" => kind == QuantityKind::Force,
        "moment" => kind == QuantityKind::Moment,
        "temperature" => kind == QuantityKind::Temperature,
        "thermal_conductivity" => kind == QuantityKind::ThermalConductivity,
        "heat_transfer_coefficient" => kind == QuantityKind::HeatTransferCoefficient,
        "mass_flow_rate" => kind == QuantityKind::MassFlowRate,
        "volumetric_flow_rate" => kind == QuantityKind::VolumetricFlowRate,
        "dimensionless" | "ratio" | "friction_factor" | "mach" => {
            kind == QuantityKind::Dimensionless
        }
        _ => true,
    };
    if ok {
        Ok(())
    } else {
        Err(format!(
            "typed quantity kind '{:?}' is not valid for variable dimension '{}'; use a matching typed constructor or SI numeric",
            kind, dimension
        ))
    }
}
