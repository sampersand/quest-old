use std::error::Error;
use std::ops::Try;
use obj::QObject;
use std::fmt::{self, Display, Formatter};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Exception {
	Missing(QObject),
	Return(usize, Option<QObject>),
	Error(QObject)
}

pub type Result = ::std::result::Result<QObject, Exception>;


impl Display for Exception {
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		match self {
			Exception::Missing(ref id) => write!(f, "Attribute `{}` is missing", id),
			Exception::Return(ref amnt, ref obj) => write!(f, "Returning {:?} {} levels up", obj, amnt),
			Exception::Error(ref err) => write!(f, "Error encountered: {}", err),
		}
	}
}

impl Error for Exception {
	fn description(&self) -> &str {
		match self {
			Exception::Missing(_) => "attribute missing",
			Exception::Return(_, _) => "returning from a call",
			Exception::Error(_) => "error encountered"
		}
	}
}