use serde::Deserialize;

mod add;
mod divide;
mod lookup;
mod multiply;
mod subtract;

pub use add::Add;
pub use divide::Divide;
pub use lookup::Lookup;
pub use multiply::Multiply;
pub use subtract::Subtract;

#[derive(Deserialize)]
pub struct OperationArgs {
    x: f64,
    y: f64,
}

#[derive(Debug, thiserror::Error)]
#[error("Math error")]
pub struct MathError;

#[derive(Debug, thiserror::Error)]
#[error("Math error")]
pub struct InitError;
