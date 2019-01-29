use crate::{Object, Shared, parse::Parser};
use std::error;
use std::fmt::{self, Display, Formatter};
use std::io;

#[derive(Debug/*, Clone, PartialEq, Eq*/)]
pub enum Error {
	MissingKey { 
		key: Object,
		obj: Object
	},
	MissingArgument {
		func: &'static str,
		pos: usize
	},
	ConversionFailure {
		func: &'static str,
		obj: Object
	},
	BadArgument {
		func: &'static str,
		msg: &'static str,
		position: usize,
		obj: Object
	},
	IoError(io::Error),
	NothingParsableFound(Shared<Parser>),
	ParserError { msg: &'static str, parser: Shared<Parser> },
	NothingToReturn,
	Boxed(Box<dyn error::Error>),
	Return { env: Shared<crate::Environment>, obj: Option<Object> }
}

pub type Result<T> = ::std::result::Result<T, Error>;
pub type ObjResult = Result<Object>;

impl Display for Error {
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		unimplemented!()
	}
}

impl error::Error for Error {
	fn description(&self) -> &str {
		unimplemented!()
	}

	fn cause(&self) -> Option<&dyn error::Error> {
		unimplemented!()
	}
}