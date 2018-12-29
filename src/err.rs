use crate::{Shared, Object};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Error {
	MissingKey(Shared<Object>),
	BadArgument(&'static str, Option<Shared<Object>>)
}

pub type Result = ::std::result::Result<Shared<Object>, Error>;