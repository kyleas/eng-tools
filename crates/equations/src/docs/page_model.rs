use serde::Serialize;

use crate::{
    constants,
    docs::examples::build_examples,
    expr::{collect_symbols, parse_expression},
    model::{EquationDef, MethodKind},
    normalize::{
        is_numerically_supported, resolved_default_unit, resolved_display, resolved_symbol,
    },
    registry::ids::derive_path_id,
};

#[derive(Debug, Clone, Serialize)]
pub struct EquationPageModel {
    pub key: String,
    pub path_id: String,
    pub slug: String,
    pub category: String,
    pub subcategories: Vec<String>,
    pub name: String,
    pub description: String,
    pub latex: String,
    pub unicode: String,
    pub ascii: String,
    pub aliases: Vec<String>,
    pub default_target: Option<String>,
    pub solve_targets: Vec<SolveTargetSummary>,
    pub branches: Vec<BranchSummary>,
    pub diagram_labels: Vec<DiagramLabelSummary>,
    pub assumptions: Vec<String>,
    pub references: Vec<PageReference>,
    pub variables: Vec<VariableSummary>,
    pub uses_constants: Vec<ConstantUsageSummary>,
    pub examples: Vec<ExampleSummary>,
}

#[derive(Debug, Clone, Serialize)]
pub struct VariableSummary {
    pub key: String,
    pub name: String,
    pub symbol: String,
    pub symbol_authored: bool,
    pub dimension: String,
    pub default_unit: String,
    pub description: String,
    pub resolver_source: Option<String>,
    pub resolver_kind: Option<String>,
    pub resolver_property: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct PageReference {
    pub source: String,
    pub note: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct SolveTargetSummary {
    pub target: String,
    pub methods: Vec<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct BranchSummary {
    pub name: String,
    pub condition: String,
    pub preferred: bool,
}

#[derive(Debug, Clone, Serialize)]
pub struct DiagramLabelSummary {
    pub variable: String,
    pub label: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct ExampleSummary {
    pub label: String,
    pub style: String,
    pub code: String,
    pub target: Option<String>,
    pub signature: Option<String>,
    pub argument_order: Vec<ExampleArgumentSummary>,
}

#[derive(Debug, Clone, Serialize)]
pub struct ExampleArgumentSummary {
    pub name: String,
    pub description: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct ConstantUsageSummary {
    pub key: String,
    pub name: String,
    pub symbol_latex: String,
    pub symbol_unicode: String,
    pub symbol_ascii: String,
}

pub fn build_page_models(equations: &[EquationDef]) -> Vec<EquationPageModel> {
    let mut pages: Vec<EquationPageModel> = equations
        .iter()
        .map(|eq| {
            let disp = resolved_display(eq);
            EquationPageModel {
                key: eq.key.clone(),
                path_id: derive_path_id(eq),
                slug: eq.effective_slug().to_string(),
                category: eq.taxonomy.category.clone(),
                subcategories: eq.taxonomy.subcategories.clone(),
                name: eq.name.clone(),
                description: if disp.description.is_empty() {
                    eq.name.clone()
                } else {
                    disp.description
                },
                latex: disp.latex,
                unicode: disp.unicode,
                ascii: disp.ascii,
                aliases: eq.aliases.clone(),
                default_target: eq.solve.default_target.clone(),
                solve_targets: build_solve_targets(eq),
                branches: eq
                    .branches
                    .iter()
                    .map(|b| BranchSummary {
                        name: b.name.clone(),
                        condition: b.condition.clone(),
                        preferred: b.preferred,
                    })
                    .collect(),
                diagram_labels: crate::normalize::resolved_diagram_labels(eq)
                    .into_iter()
                    .map(|(variable, label)| DiagramLabelSummary { variable, label })
                    .collect(),
                assumptions: eq.assumptions.clone(),
                references: eq
                    .references
                    .iter()
                    .map(|r| PageReference {
                        source: r.source.clone(),
                        note: r.note.clone(),
                    })
                    .collect(),
                variables: eq
                    .variables
                    .iter()
                    .map(|(key, v)| VariableSummary {
                        key: key.clone(),
                        name: v.name.clone(),
                        symbol: resolved_symbol(key, v.symbol.as_deref()),
                        symbol_authored: v
                            .symbol
                            .as_deref()
                            .map(str::trim)
                            .is_some_and(|s| !s.is_empty()),
                        dimension: v.dimension.clone(),
                        default_unit: resolved_default_unit(
                            &v.dimension,
                            v.default_unit.as_deref(),
                        )
                        .unwrap_or_else(|| "?".to_string()),
                        description: v.description.clone().unwrap_or_default(),
                        resolver_source: v.resolver.as_ref().map(|r| r.source.clone()),
                        resolver_kind: v.resolver.as_ref().map(|r| r.kind.to_string()),
                        resolver_property: v.resolver.as_ref().map(|r| r.property.clone()),
                    })
                    .collect(),
                uses_constants: collect_used_constants(eq),
                examples: build_examples(eq)
                    .into_iter()
                    .map(|e| ExampleSummary {
                        label: e.label,
                        style: e.style,
                        code: e.code,
                        target: e.target,
                        signature: e.signature,
                        argument_order: e
                            .argument_order
                            .into_iter()
                            .map(|a| ExampleArgumentSummary {
                                name: a.name,
                                description: a.description,
                            })
                            .collect(),
                    })
                    .collect(),
            }
        })
        .collect();
    pages.sort_by(|a, b| a.path_id.cmp(&b.path_id));
    pages
}

fn collect_used_constants(eq: &EquationDef) -> Vec<ConstantUsageSummary> {
    let mut found = Vec::new();
    let mut seen = std::collections::BTreeSet::new();
    let mut exprs = Vec::new();
    exprs.push(eq.relation.residual.as_str());
    exprs.extend(eq.solve.explicit_forms.values().map(String::as_str));
    exprs.extend(eq.branches.iter().map(|b| b.condition.as_str()));

    for expression in exprs {
        let Ok(parsed) = parse_expression(expression) else {
            continue;
        };
        for symbol in collect_symbols(&parsed) {
            if let Some(c) = constants::get_by_identifier(&symbol)
                && seen.insert(c.key.to_string())
            {
                found.push(ConstantUsageSummary {
                    key: c.key.to_string(),
                    name: c.name.to_string(),
                    symbol_latex: c.symbol_latex.to_string(),
                    symbol_unicode: c.symbol_unicode.to_string(),
                    symbol_ascii: c.symbol_ascii.to_string(),
                });
            }
        }
    }

    found.sort_by(|a, b| a.key.cmp(&b.key));
    found
}

fn build_solve_targets(eq: &EquationDef) -> Vec<SolveTargetSummary> {
    let mut out: Vec<SolveTargetSummary> = eq
        .variables
        .keys()
        .filter_map(|target| {
            let mut methods = Vec::new();
            if eq.solve.explicit_forms.contains_key(target) {
                methods.push(MethodKind::Explicit);
            }
            if is_numerically_supported(eq, target) {
                methods.push(MethodKind::Numerical);
            }
            if methods.is_empty() {
                return None;
            }
            Some(SolveTargetSummary {
                target: target.clone(),
                methods: methods
                    .into_iter()
                    .map(|m| match m {
                        MethodKind::Explicit => "explicit".to_string(),
                        MethodKind::Numerical => "numerical".to_string(),
                    })
                    .collect(),
            })
        })
        .collect();
    out.sort_by(|a, b| a.target.cmp(&b.target));
    out
}
