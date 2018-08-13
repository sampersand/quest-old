use obj::{SharedObject, object::QObject};
use obj::classes::{QuestClass, DefaultAttrs};
use std::fmt::{self, Display, Formatter};
use std::error::Error as ErrorTrait;

#[derive(Debug)]
pub enum Exception {
	Custom(SharedObject),
	Rust(Box<dyn ErrorTrait>)
}

pub type QException = QObject<Exception>;

impl Display for Exception {
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		match self {
			Exception::Custom(val) => write!(f, "{:?}", val),
			Exception::Rust(ref err) => Display::fmt(err, f)
		}
	}
}

impl Display for QException {
	#[inline]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		Display::fmt(self.as_ref(), f)
	}
}

impl QuestClass for Exception {
	fn default_attrs() -> &'static DefaultAttrs<Self> { &DEFAULT_ATTRS }
}

impl<E: ErrorTrait + 'static> From<E> for QException {
	#[inline]
	fn from(inp: E) -> QException {
		QException::new(Exception::from(inp))
	}
}

impl<E: ErrorTrait + 'static> From<E> for Exception {
	#[inline]
	fn from(err: E) -> Exception {
		Exception::Rust(Box::new(err))
	}
}

impl From<SharedObject> for Exception {
	#[inline]
	fn from(err: SharedObject) -> Exception {
		Exception::Custom(err)
	}
}

define_attrs! {
	static ref DEFAULT_ATTRS for Exception;
	use QObject<Exception>;

	fn "@text" (this) with env {
		match this.as_ref() {
			Exception::Custom(ref obj) => Ok(obj.call_attr("@bool", &[], env)?),
			Exception::Rust(ref err) => Ok(Shared::from(QText::from(err.to_string())) as SharedObject),
		}
	}
}











