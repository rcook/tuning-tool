use tuning_tool_lib::symbolic::Expression;
use tuning_tool_lib::symbolic::Value::{R, Z};

pub(crate) fn evaluate(expr: Expression) -> f64 {
    match expr.evaluate() {
        Some(R(value)) => value,
        Some(Z(value)) => value as f64,
        _ => todo!(),
    }
}
