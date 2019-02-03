// use std::sync::PoisonError;
use crate::object::AnyObject;
use std::error::Error as ErrorTrait;

#[derive(Debug)]
pub enum Error {
	CastError { obj: AnyObject, into: &'static str },
	AttrMissing { attr: AnyObject, obj: AnyObject },
	MissingArgument { pos: usize, args: Vec<AnyObject> },
	// #[cfg(test)]
	// PoisonError,
	__Testing
}

#[must_use]
pub type Result<T> = std::result::Result<T, Error>;

// #[cfg(test)]
// impl<T> From<PoisonError<T>> for Error {
// 	fn from(_: PoisonError<T>) -> Error {
// 		Error::PoisonError
// 	}
// }