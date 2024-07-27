use thiserror::Error;

#[derive(Error, Debug)]
/// Errors occurring during parsing
pub enum ParseError {
    #[error("Parser Error (rich parser errors have not been implemented yet)")]
    ParserErrorsNotImplemented,
}
