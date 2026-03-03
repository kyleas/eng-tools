use std::collections::BTreeSet;

use crate::expr::parser::Expr;

pub fn collect_symbols(expr: &Expr) -> BTreeSet<String> {
    let mut out = BTreeSet::new();
    collect(expr, &mut out);
    out
}

fn collect(expr: &Expr, out: &mut BTreeSet<String>) {
    match expr {
        Expr::Number(_) => {}
        Expr::Var(v) => {
            out.insert(v.clone());
        }
        Expr::Unary { rhs, .. } => collect(rhs, out),
        Expr::Binary { lhs, rhs, .. } => {
            collect(lhs, out);
            collect(rhs, out);
        }
        Expr::Call { args, .. } => {
            for arg in args {
                collect(arg, out);
            }
        }
    }
}
