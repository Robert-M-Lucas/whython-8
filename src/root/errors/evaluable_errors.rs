use thiserror::Error;

#[derive(Error, Debug)]
pub enum EvalErrs {
    #[error("Expected an indirection of {0} but found {1}")]
    BadIndirection(usize, usize),
    #[error("Cannot use a function ({0}) without a call")]
    FunctionMustBeCalled(String),
    #[error("Cannot evaluate a standalone type ({0})")]
    CannotEvalStandaloneType(String),
    // #[error("Operator ({0}) can only be used as a prefix operator, not infix")]
    // FoundPrefixNotInfixOp(String),
    #[error("Infix operator ({0}) can only be used for type ({1}) if method ({2}) accepting 2 arguments is implemented for ({1}). ({2}) implementation only accepts ({3}) arguments")]
    OpWrongArgumentCount(String, String, String, usize),
    #[error("Expected operation to return type ({0}) but found ({1})")]
    OpWrongReturnType(String, String),
    #[error("Expected operation to return type ({0}) but found no return")]
    OpNoReturn(String)
}
