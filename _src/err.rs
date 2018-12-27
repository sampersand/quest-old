use std::error::Error as ErrorTrait;
use std::fmt::{self, Display, Formatter};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Error {
	#[doc(hidden)]
	__Nonexhaustive
}

pub type Result<T> = ::std::result::Result<T, Error>;


impl Display for Error {
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		match self {
			Error::__Nonexhaustive => unreachable!("Don't make instances of __Nonexhaustive")
		}
	}
}

impl ErrorTrait for Error {
	fn description(&self) -> &str {
		match self {
			Error::__Nonexhaustive => unreachable!("Don't make instances of __Nonexhaustive"),
		}
	}

	fn cause(&self) -> Option<&dyn ErrorTrait> {
		match self {
			Error::__Nonexhaustive => unreachable!("Dont make instances of __Nonexhaustive")
		}
	}
}
