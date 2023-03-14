#[derive(Debug)]
pub enum Error {
    KeyNotFound,
    KeyAlreadyExists,
    UnexpectedError,
    KeyOverflowError,
    ValueOverflowError,
    TryFromSliceError(&'static str),
    UTF8Error,
}

impl std::convert::From<std::io::Error> for Error {
    fn from(_e: std::io::Error) -> Error {
        Error::UnexpectedError
    }
}
