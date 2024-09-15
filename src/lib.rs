#[derive(Default)]
pub struct MathOperation {
    pub operator: String,
    pub left: u64,
    pub right: u64,
}

const PLUS: &str = "+";
const MINUS: &str = "-";
const MULTIPLY: &str = "*";
const DIVIDE: &str = "/";

pub fn operation(arg: MathOperation) -> u128 {
    let result = match arg.operator.as_str() {
        PLUS => arg.left as u128 + arg.right as u128,
        MINUS => arg.left as u128 - arg.right as u128,
        MULTIPLY => arg.left as u128 * arg.right as u128,
        DIVIDE => arg.left as u128 / arg.right as u128,
        _ => 0,
    };
    result
}
