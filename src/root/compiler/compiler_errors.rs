use thiserror::Error;

/// An error in the compiler step, excluding errors covered in `EvalErrs`
#[derive(Error, Debug)]
pub enum CompErrs {
    #[error("Int literal ({0}) exceeds maximum value ({1}) for type")]
    IntLiteralExceedsMax(i128, i128),
    #[error("Int literal ({0}) below minimum value ({1}) for type")]
    IntLiteralBelowMin(i128, i128),
    #[error("Expected return ({0})")]
    ExpectedReturn(String),
    #[error("Expected return type ({0}) but found ({1})")]
    ExpectedReturnType(String, String),
    #[error("Expected return type ({0}), not none")]
    ExpectedSomeReturn(String),
    #[error("Expected return with no value")]
    ExpectedNoReturn,
    #[error("Cannot break - not in a loop")]
    CannotBreak,
}
