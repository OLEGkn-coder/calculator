use thiserror::Error;

#[derive(Debug, Error)]
pub enum CalcError {
    #[error("Division by zero")]
    DivByZero,
    #[error("Empty expression")]
    EmptyExp,
    #[error("Expression has no brackets")]
    NoBracket,
}

pub type Res<T> = Result<T, CalcError>;
