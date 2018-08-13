use std::error::Error;
use obj_::{QObject_, QObject__};
use std::fmt::{self, Display, Formatter};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Exception__ {
	Missing(QObject__),
	Return(usize, Option<QObject__>),
	Error(QObject__)
}

#[derive(Debug)]
pub enum Exception_ {
	Old(Exception__),
	New(::obj::Interrupt)
}

impl From<Exception__> for Exception_ {
	#[inline]
	fn from(inp: Exception__) -> Exception_ {
		Exception_::Old(inp)
	}
}

impl From<::obj::Interrupt> for Exception_ {
	#[inline]
	fn from(inp: ::obj::Interrupt) -> Exception_ {
		Exception_::New(inp)
	}
}

pub type Result_ = ::std::result::Result<QObject_, Exception_>;
pub type Result__ = ::std::result::Result<QObject__, Exception__>;


impl Display for Exception__ {
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		match self {
			Exception__::Missing(ref id) => write!(f, "Attribute `{}` is missing", id),
			Exception__::Return(ref amnt, ref obj) => write!(f, "Returning {:?} {} levels up", obj, amnt),
			Exception__::Error(ref err) => write!(f, "Error encountered: {}", err),
		}
	}
}

impl Error for Exception__ {
	fn description(&self) -> &str {
		match self {
			Exception__::Missing(_) => "attribute missing",
			Exception__::Return(_, _) => "returning from a call",
			Exception__::Error(_) => "error encountered"
		}
	}
}