use crate::{Object, Shared, parse::Parser};
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
	ParserError { msg: &'static str, parser: Shared<Parser> }
}

pub type Result = ::std::result::Result<Object, Error>;