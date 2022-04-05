use std::array::TryFromSliceError;

use anyhow::{anyhow};

use crate::RrError::Other;

#[derive(Debug)]
pub enum RrError {
    Message(String),
    Other(anyhow::Error)
}

impl RrError {
    pub(crate) fn message(message: String) -> RrError {
        RrError::Message( message )
    }
    pub(crate) fn not_find(name: &str) -> RrError { RrError::Message (name.to_owned()) }
    pub(crate) fn none_error(name: &str) -> RrError { RrError::Message (name.to_owned()) }
}

// impl<T> From<T> for RrError where
//     T: std::error::Error
// {
//     fn from(r: T) -> Self {
//         Other(r)
//     }
// }

impl From<RrError> for anyhow::Error{
    fn from(e: RrError) -> Self {
        anyhow!(e)
    }
}

impl From<anyhow::Error> for RrError{
    fn from(r: anyhow::Error) -> Self {
        Other(r)
    }
}

impl From<ckb_rocksdb::Error> for RrError {
    fn from(e: ckb_rocksdb::Error) -> Self {
        RrError::Other(anyhow!(e))
    }
}

impl From<TryFromSliceError> for RrError {
    fn from(e: TryFromSliceError) -> Self {
        RrError::Other(anyhow!(e))
    }
}
