use obj::{AnyObject, SharedObject};

use std::fmt::{self, Debug, Display, Formatter};
use std::error::Error as ErrorTrait;

pub trait SafeError: ErrorTrait + Send + Sync {}
impl<T: ErrorTrait + Send + Sync> SafeError for T {}


#[derive(Debug)]
pub enum Exception {
	Custom(AnyObject),
	Rust(Box<dyn SafeError>)
}

pub type QException = SharedObject<Exception>;

impl Display for Exception {
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		match self {
			Exception::Custom(val) => write!(f, "{:?}", val),
			Exception::Rust(ref err) => <SafeError as Debug>::fmt(unimplemented!(), f)
			// Exception::Rust(ref err) => <SafeError as Debug>::fmt(&&err, f)
		}
	}
}

impl<E: SafeError + 'static> From<E> for Exception {
	#[inline]
	fn from(err: E) -> Exception {
		Exception::Rust(Box::new(err))
	}
}

impl From<AnyObject> for Exception {
	#[inline]
	fn from(err: AnyObject) -> Exception {
		Exception::Custom(err)
	}
}

define_attrs! { for QException;
	use QObject<Exception>;

	fn "@text" (this) with env {
		match this.as_ref() {
			Exception::Custom(ref obj) => Ok(obj.call_attr("@bool", &[], env)?),
			Exception::Rust(ref err) => Ok(Shared::from(QText::from(err.to_string())) as AnyObject),
		}
	}
}











