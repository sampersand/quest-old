// use std::sync::PoisonError;
use crate::object::AnyObject;
use crate::parse;
use std::error::Error as ErrorTrait;

#[derive(Debug)]
pub enum Error {
	CastError { obj: AnyObject, into: &'static str },
	AttrMissing { attr: AnyObject, obj: AnyObject },
	MissingArgument { pos: usize, args: Vec<AnyObject> },
	BadArgument { pos: usize, arg: AnyObject, msg: &'static str },
	Boxed(Box<dyn ErrorTrait>),
	ParseError(parse::ParseError),
	#[cfg(test)]
	__Testing // this error is used to throw custom errors when testing
}

#[must_use]
pub type Result<T> = std::result::Result<T, Error>;