use thiserror::Error;

#[derive(Error, Debug)]
pub enum ParseError {
    // #[error("Names cannot contain character '{0}' (UTF-8 Code: {1:?})")]
    // BadName(char, Vec<u8>),
}
