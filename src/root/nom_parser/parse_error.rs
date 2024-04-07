use std::io;
use std::path::PathBuf;
use thiserror::Error;
use crate::root::parser::line_info::LineInfo;

#[derive(Error, Debug)]
pub enum ParseError {
    #[error("File read error on path '{0}'")]
    FileRead(PathBuf, io::Error),
    #[error("Error: {1}\n{0}")]
    Nested(LineInfo, Box<crate::root::parser::parse::ParseError>),
    #[error("Error: Operator '{1}' not recognised\n{0}")]
    OperatorNotRecognised(LineInfo, String),
    #[error("Error: 'mod' must be followed by a path\n{0}")]
    ModNotFollowedByPath(LineInfo),
    #[error("Error: Keyword ('{1}') cannot be followed by . or #\n{0}")]
    KeywordFollowed(LineInfo, String),
    #[error("Error: Closing '{1}' not found (started on line {2})\n{0}")]
    NotClosed(LineInfo, char, usize),
    #[error("Error: Unknown escape code '{1}'\n{0}")]
    UnknownEscapeCode(LineInfo, char),
    #[error("Error: String literal started on line {1} not closed\n{0}")]
    UnclosedString(LineInfo, usize),
    #[error("Error: Closing '{1}' found with no corresponding opening bracket\n{0}")]
    NoOpening(LineInfo, char),
    #[error("Error: Names cannot contain character '{1}' (UTF-8 Code: {2:?})\n{0}")]
    BadName(LineInfo, char, Vec<u8>),
    #[error("Initialiser must specify a type after '@'")]
    NoInitialiserType(LineInfo),
    #[error("Error: Initialiser type must be followed by braces containing attribute values\n{0}")]
    NoInitialiserContents(LineInfo),
    #[error("Error: Attribute cannot be empty (must be a value between commas)\n{0}")]
    NoInitialiserAttribute(LineInfo)
}