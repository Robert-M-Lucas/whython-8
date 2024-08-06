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
    #[error("Failed to open file [{0}]")]
    FailedToOpenFile(String)
}

pub fn create_custom_error(e: String, l: Span) -> nom::Err<ErrorTree> {
    nom::Err::Error(create_custom_error_tree(e, l))
}

pub fn create_custom_error_tree(e: String, l: Span) -> ErrorTree {
    ErrorTree::Base {
        location: l,
        kind: BaseErrorKind::External(e),
    }
}

pub fn to_error_tree<'a>(e: nom::Err<ErrorTree<'a>>, s: Span<'a>) -> ErrorTree<'a> {
    match e {
        nom::Err::Incomplete(_n) => {
            create_custom_error_tree("Expected more characters".to_string(), s)
        }
        nom::Err::Error(e) => e,
        nom::Err::Failure(f) => f,
    }
}
