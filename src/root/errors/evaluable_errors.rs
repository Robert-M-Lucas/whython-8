use thiserror::Error;

#[derive(Error, Debug)]
pub enum EvaluableErrors {
    #[error("Expected an indirection of {0} but found {1}")]
    BadIndirection(usize, usize),
}
