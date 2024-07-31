use crate::root::parser::parse::{ErrorTree, Span};
use nom_supreme::error::BaseErrorKind;
use thiserror::Error;

#[derive(Error, Debug)]
/// Errors occurring during parsing
pub enum ParseError {
    // #[error("Parser Error (rich parser errors have not been implemented yet)")]
    // ParserErrorsNotImplemented,
    #[error("Incomplete Parser Error (incomplete parser errors have not been implemented yet)")]
    ParserIncompleteErrorsNotImplemented,
    #[error("Expected {0}")]
    Expected(String),
    #[error("Failed parsing {0}")]
    NomErrorKind(String),
}

pub fn create_custom_error(e: String, l: Span) -> nom::Err<ErrorTree> {
    nom::Err::Error(ErrorTree::Base {
        location: l,
        kind: BaseErrorKind::External(e),
    })
}
