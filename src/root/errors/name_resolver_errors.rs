use thiserror::Error;

#[allow(dead_code)]
#[derive(Error, Debug)]
/// Errors occurring during name resolution
pub enum NRErrs {
    #[error("No top-level main function found")]
    NoMain,
    #[error("Cannot create 'impl' for an indirect type")]
    IndirectImpl,
    #[error("Cannot find the subname ({0}) of a function ({1})")]
    NoFunctionSubname(String, String),
    #[error("Cannot find method ({0}) of type ({1})")]
    CannotFindMethod(String, String),
    #[error("Two attributes found with the same name ({0})")]
    SameAttributeName(String),
    #[error("Function reference cannot have indirection here")]
    FunctionIndirectionError,
    #[error("Identifier ({0}) not found")]
    IdentifierNotFound(String),
    #[error("Expected type ({0}), found function of same name")]
    FoundFunctionNotType(String),
    #[error("Type ({0}) not found")]
    TypeNotFound(String),
    #[error("Expected type, not method or attribute")]
    ExpectedTypeNotMethodOrAttribute,
    #[error("Cannot find name ({0})")]
    CannotFindName(String),
    #[error("Cannot find constant attribute ({0})")]
    CannotFindConstantAttribute(String),
    #[error("Method ({0}) not implemented for type ({1}) required for operator ({2})")]
    OpMethodNotImplemented(String, String, String),
    #[error("Operator ({0}) cannot be used as a prefix operator")]
    OpCantBePrefix(String),
    #[error("Operator ({0}) cannot be used as an infix operator")]
    OpCantBeInfix(String),
    #[error("Size of type ({0}) cannot be determined due to circular definition with no indirection ({1})")]
    CircularType(String, String),
    #[error("Circular import [{0}]")]
    CircularImport(String),
}
