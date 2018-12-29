use crate::{Shared, Object};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Error {
	MissingKey { 
		key: Shared<Object>,
		obj: Shared<Object>
	},
	MissingArgument {
		func: &'static str,
		pos: usize
	},
	BadArgument {
		func: &'static str,
		msg: &'static str,
		position: usize,
		arg: Shared<Object>
	}
}

pub type Result = ::std::result::Result<Shared<Object>, Error>;