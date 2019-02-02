use crate::object::AnyObject;

#[derive(Debug)]
pub enum Error {
	CastError { obj: AnyObject, into: &'static str },
	AttrMissing { attr: AnyObject, obj: AnyObject },
	MissingArgument { pos: usize, args: Vec<AnyObject> },
	__Testing
}

pub type Result<T> = std::result::Result<T, Error>;