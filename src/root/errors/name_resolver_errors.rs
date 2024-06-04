use thiserror::Error;

#[derive(Error, Debug)]
pub enum NRErrors {
    #[error("No top-level main function found")]
    NoMain,
    #[error("Cannot create 'impl' for an indirect type")]
    IndirectImpl,
    #[error("Cannot find the subname ({0}) of a function ({1})")]
    FunctionSubname(String, String),
    #[error("Cannot find method ({0}) of type ({1})")]
    CannotFindMethod(String, String),
    #[error("Two attributes found with the same name ({0})")]
    SameAttributeName(String)
}
