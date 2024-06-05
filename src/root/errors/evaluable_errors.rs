use thiserror::Error;

#[derive(Error, Debug)]
pub enum EvalErrs {
    #[error("Expected an indirection of {0} but found {1}")]
    BadIndirection(usize, usize),
    #[error("Cannot use a function ({0}) without a call")]
    FunctionMustBeCalled(String),
    #[error("Cannot evaluate a standalone type ({0})")]
    CannotEvalStandaloneType(String),
}
