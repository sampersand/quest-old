use obj::{Id, AnyObject};
use obj::object::QObject;
use obj::classes::{QuestClass, QException};

use std::fmt::{self, Debug, Display, Formatter};
use std::error::Error;

#[derive(Debug)]
pub enum Interrupt {
	Return(usize, Option<AnyObject>),
	Exception(QException)
}

pub type Result<T> = ::std::result::Result<T, Interrupt>;

impl From<QException> for Interrupt {
	#[inline]
	fn from(exception: QException) -> Interrupt {
		Interrupt::Exception(exception)
	}
}


impl Display for Interrupt {
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		match self {
			Interrupt::Return(amnt, ref obj) => write!(f, "Returning {:?} {} levels up", obj, amnt),
			Interrupt::Exception(ref exc) => write!(f, "Exception encountered: {:?}", exc),
		}
	}
}

impl Error for Interrupt {
	fn description(&self) -> &str {
		match self {
			Interrupt::Return(_, _) => "returning from a call",
			Interrupt::Exception(_) => "exception encountered"
		}
	}
}