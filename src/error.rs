use core::result;
use thiserror::Error;
pub type Result<T> = result::Result<T, Error>;

#[derive(Error, Debug, PartialEq)]
pub enum Error {
    #[error("ItemNotFound {0}")]
    ItemNotFoundError(String),
    #[error("DuplicateIndex")]
    DuplicateIndexError,
}
