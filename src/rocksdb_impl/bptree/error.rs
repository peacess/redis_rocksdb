use crate::RrError;

#[derive(Debug)]
pub enum Error {
    KeyNotFound,
    KeyAlreadyExists,
    Unexpected,
    KeyOverflow,
    ValueOverflow,
    TryFromSlice(&'static str),
    UTF8,
    RrError(RrError),
}

impl std::convert::From<std::io::Error> for Error {
    fn from(_e: std::io::Error) -> Error {
        Error::Unexpected
    }
}

impl From<RrError> for Error {
    fn from(e: RrError) -> Error {
        Error::RrError(e)
    }
}
