use thiserror::Error;

#[allow(dead_code)]
#[derive(Error, Debug)]
/// Errors occurring during evaluation
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
    InfixOpWrongArgumentCount(String, String, String, usize),
    #[error("Prefix operator ({0}) can only be used for type ({1}) if method ({2}) accepting 1 arguments is implemented for ({1}). ({2}) implementation only accepts ({3}) arguments")]
    PrefixOpWrongArgumentCount(String, String, String, usize),
    #[error("Expected operation to return type ({0}) but found ({1})")]
    OpWrongReturnType(String, String),
    #[error("Expected operation to return type ({0}) but found no return")]
    OpNoReturn(String),
    #[error("Expected type ({0}) but found ({1})")]
    ExpectedDifferentType(String, String),
    #[error("Expected type ({0}) but found none")]
    ExpectedType(String),
    #[error("Expected a type but found none")]
    ExpectedNotNone,
    #[error("Expected a function name")]
    ExpectedFunctionName,
    #[error("Expected a reference type but found ({0})")]
    ExpectedReference(String),
    #[error("Expected type ({0}) but function returns ({1})")]
    BadFunctionReturn(String, String),
    #[error("Expected type ({0}) but function doesn't return a value")]
    ExpectedFunctionReturn(String),
    #[error("Function ({0}) expects ({1}) arguments but found ({2})")]
    BadFunctionArgCount(String, usize, usize),
    #[error("Type ({0}) does not have attributes")]
    TypeDoesntHaveAttributes(String),
    #[error("Type ({0}) cannot be initialised")]
    TypeCannotBeInitialised(String)
}

// return Err(WErr::n(OpWrongReturnType(global_table.get_type_name(into.type_ref()), global_table.get_type_name(&new_type)), location.clone()));
