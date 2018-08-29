#[macro_use]
mod macros;

mod id;
mod object;
mod interrupt;
pub mod classes;


pub use self::interrupt::{Result, Interrupt};
pub use self::id::Id;
pub use self::object::QObject;

use shared::Shared;
use std::any::Any;
pub type SharedObject<T> = Shared<QObject<T>>;
pub type AnyObject = SharedObject<dyn Any + Send + Sync>;

impl From<!> for AnyObject {
	fn from(_: !) -> AnyObject {
		unreachable!()
	}
}
