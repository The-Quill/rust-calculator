#[path = "constants.rs"]
pub mod constants;
use constants::*;

pub fn execute(left: i32, op: Operator, right: i32) -> i32 {
    let func = match op {
        Operator::Addition => add,
        Operator::Subtraction => subtract,
        Operator::Multiplication => multiply,
        Operator::Division => divide,
        Operator::Exponentiation => exponent,
    };
    func(left, right)
}

fn add(left: i32, right: i32) -> i32 {
    left + right
}
fn subtract(left: i32, right: i32) -> i32 {
    left - right
}
fn multiply(left: i32, right: i32) -> i32 {
    left * right
}
fn divide(left: i32, right: i32) -> i32 {
    left / right
}
fn exponent(left: i32, right: i32) -> i32 {
    i32::pow(left, right.unsigned_abs())
}
