use std::{
    array::TryFromSliceError,
    fmt::{Display, Formatter},
};

use crate::RrError::Other;

#[derive(Debug)]
pub enum RrError {
    Message(String),
    Other(anyhow::Error),
}

impl RrError {
    pub(crate) fn message(message: String) -> RrError {
        RrError::Message(message)
    }
    pub(crate) fn not_find(name: &str) -> RrError {
        RrError::Message(name.to_owned())
    }
    pub(crate) fn none_error(name: &str) -> RrError {
        RrError::Message(name.to_owned())
    }
    pub(crate) fn data_error(name: &str) -> RrError {
        RrError::Message(name.to_owned())
    }
}

impl Display for RrError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        // write!(f,"{}", self.0)
        match &self {
            RrError::Message(s) => {
                write!(f, "RrError: {}", s)
            }
            RrError::Other(e) => {
                write!(f, "{}", e)
            }
        }
    }
}

impl std::error::Error for RrError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        if let RrError::Other(e) = self {
            e.source()
        } else {
            None
        }
    }
}

impl From<anyhow::Error> for RrError {
    fn from(r: anyhow::Error) -> Self {
        Other(r)
    }
}

// impl<T> From<T> for RrError where
//     T: std::error::Error
// {
//     fn from(r: T) -> Self {
//         Other(r)
//     }
// }

impl From<rocksdb::Error> for RrError {
    fn from(e: rocksdb::Error) -> Self {
        RrError::Other(anyhow::Error::from(e))
    }
}

impl From<TryFromSliceError> for RrError {
    fn from(e: TryFromSliceError) -> Self {
        RrError::Other(anyhow::Error::from(e))
    }
}
