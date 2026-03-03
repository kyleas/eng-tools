use std::sync::OnceLock;

use crate::{
    Registry,
    api::{IntoEquationId, IntoSolveInput, SolveResult, SolveStart},
    api_context::ContextSolveBuilder,
    error::Result,
    model::EquationDef,
};

/// Global convenience facade for the default equations registry.
///
/// This supports the simplest usage pattern:
/// `use equations::{eq};` then `eq.solve(...)`.
#[derive(Debug, Clone, Copy)]
pub struct EqFacade;

static REGISTRY_RESULT: OnceLock<std::result::Result<Registry, String>> = OnceLock::new();

impl EqFacade {
    /// Access the lazily loaded default registry.
    pub fn registry(&self) -> Result<&'static Registry> {
        match REGISTRY_RESULT.get_or_init(|| Registry::load_default().map_err(|e| e.to_string())) {
            Ok(registry) => Ok(registry),
            Err(message) => Err(crate::error::EquationError::Validation(format!(
                "failed to load default registry: {message}"
            ))),
        }
    }

    /// Resolve equation by key/path_id/alias from the default registry.
    pub fn equation(&self, id: &str) -> Result<&'static EquationDef> {
        self.registry()?.equation(id)
    }

    /// Validate the default registry.
    pub fn validate(&self) -> Result<()> {
        self.registry()?.validate()
    }

    /// Validate the default registry and run registry-defined tests.
    pub fn validate_with_tests(&self) -> Result<()> {
        self.registry()?.validate_with_tests()
    }

    /// Start an ergonomic solve builder.
    ///
    /// Accepts either a generic equation id (`&str`) or a generated typed equation handle.
    pub fn solve<S>(&self, solve_start: S) -> S::Builder
    where
        S: SolveStart,
    {
        solve_start.into_builder(*self)
    }

    /// Start a context-aware solve builder that can resolve variables from named fluid/material contexts.
    pub fn solve_with_context<E>(&self, equation_id: E) -> ContextSolveBuilder
    where
        E: IntoEquationId,
    {
        ContextSolveBuilder::new(*self, equation_id.equation_id())
    }

    /// One-liner helper: solve and return SI value.
    pub fn solve_value<E, I, K, V>(&self, equation_id: E, target: &str, knowns: I) -> Result<f64>
    where
        E: IntoEquationId,
        I: IntoIterator<Item = (K, V)>,
        K: Into<String>,
        V: IntoSolveInput,
    {
        self.solve(equation_id.equation_id())
            .for_target(target)
            .givens(knowns)
            .value()
    }

    /// One-liner helper: solve and return requested unit value.
    pub fn solve_value_in<E, I, K, V>(
        &self,
        equation_id: E,
        target: &str,
        knowns: I,
        unit: &str,
    ) -> Result<f64>
    where
        E: IntoEquationId,
        I: IntoIterator<Item = (K, V)>,
        K: Into<String>,
        V: IntoSolveInput,
    {
        self.solve(equation_id.equation_id())
            .for_target(target)
            .givens(knowns)
            .value_in(unit)
    }

    /// One-liner helper: solve and return full diagnostics result.
    pub fn solve_result<E, I, K, V>(
        &self,
        equation_id: E,
        target: &str,
        knowns: I,
    ) -> Result<SolveResult>
    where
        E: IntoEquationId,
        I: IntoIterator<Item = (K, V)>,
        K: Into<String>,
        V: IntoSolveInput,
    {
        self.solve(equation_id.equation_id())
            .for_target(target)
            .givens(knowns)
            .result()
    }
}
