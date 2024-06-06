use thiserror::Error;

#[derive(Error, Debug)]
pub enum ParseError {
    #[error("Parser Error (rich parser errors have not been implemented yet)")]
    ParserErrorsNotImplemented
}
