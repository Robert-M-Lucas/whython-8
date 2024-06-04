use thiserror::Error;

#[derive(Error, Debug)]
pub enum NRErrors {
    #[error("No top-level main function found")]
    NoMain,
    #[error("Cannot create 'impl' for an indirect type")]
    IndirectImpl
}
