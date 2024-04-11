use crate::RrError;

#[derive(Debug)]
pub enum Error {
	KeyNotFound,
	KeyAlreadyExists,
	UnexpectedError,
	KeyOverflowError,
	ValueOverflowError,
	TryFromSliceError(&'static str),
	UTF8Error,
	RrError(RrError),
}

impl std::convert::From<std::io::Error> for Error {
	fn from(_e: std::io::Error) -> Error {
		Error::UnexpectedError
	}
}

impl From<RrError> for Error {
	fn from(e: RrError) -> Error {
		Error::RrError(e)
	}
}
