use crate::testing::solve_check::values_close;

pub fn methods_consistent(explicit: f64, numerical: f64, abs: f64, rel: f64) -> bool {
    values_close(explicit, numerical, abs, rel)
}
