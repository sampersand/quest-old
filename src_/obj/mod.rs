#[macro_use]
mod macros;

mod object;
mod id;
pub mod classes;

use self::object::QObject;
pub use self::id::Id;

pub type Result<T> = ::std::result::Result<T, u128>;

use self::classes::Class;
use shared::Shared;
use std::any::Any;

pub type SharedObject<T> = Shared<QObject<T>>;
pub type AnyObject = SharedObject<dyn Any + Send + Sync>;

impl From<!> for AnyObject {
	fn from(_: !) -> AnyObject {
		unreachable!()
	}
}

// impl PartialEq for AnyObject {
// 	fn eq(&self, other: &AnyObject) -> bool {
// 		self.call_attr("==", other)
// 	}
// }