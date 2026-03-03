use std::collections::HashMap;

#[derive(Debug, Clone, Copy, serde::Serialize)]
pub struct EngineeringConstant {
    pub key: &'static str,
    pub name: &'static str,
    pub symbol_latex: &'static str,
    pub symbol_unicode: &'static str,
    pub symbol_ascii: &'static str,
    pub dimension: &'static str,
    pub unit: &'static str,
    pub value: f64,
    pub exact: bool,
    pub source: &'static str,
    pub note: &'static str,
    pub description: &'static str,
    pub aliases: &'static [&'static str],
}

#[doc(hidden)]
mod generated {
    include!(concat!(env!("OUT_DIR"), "/typed_constants.rs"));
}

pub use generated::*;

pub fn all() -> &'static [EngineeringConstant] {
    ALL_CONSTANTS
}

pub fn get(key_or_alias: &str) -> Option<EngineeringConstant> {
    ALL_CONSTANTS.iter().copied().find(|c| {
        c.key.eq_ignore_ascii_case(key_or_alias)
            || c.aliases
                .iter()
                .any(|a| a.eq_ignore_ascii_case(key_or_alias))
    })
}

pub fn symbol_map() -> HashMap<String, f64> {
    let mut out = HashMap::new();
    for c in ALL_CONSTANTS {
        out.insert(c.key.to_string(), c.value);
        out.insert(c.symbol_ascii.to_string(), c.value);
        for alias in c.aliases {
            out.insert((*alias).to_string(), c.value);
        }
    }
    out
}

pub fn get_by_identifier(identifier: &str) -> Option<EngineeringConstant> {
    ALL_CONSTANTS.iter().copied().find(|c| {
        c.key.eq_ignore_ascii_case(identifier)
            || c.symbol_ascii.eq_ignore_ascii_case(identifier)
            || c.aliases.iter().any(|a| a.eq_ignore_ascii_case(identifier))
    })
}

pub fn expression_identifiers() -> Vec<String> {
    let mut ids = Vec::new();
    for c in ALL_CONSTANTS {
        ids.push(c.key.to_string());
        ids.push(c.symbol_ascii.to_string());
        for alias in c.aliases {
            ids.push((*alias).to_string());
        }
    }
    ids.sort();
    ids.dedup();
    ids
}
