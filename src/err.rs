use crate::{Shared, Object};

#[derive(Debug, Clone, PartialEq, Eq)]
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
	}
}

pub type Result = ::std::result::Result<Object, Error>;