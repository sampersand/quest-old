#[macro_use]
mod macros;

mod id;
mod object;
mod interrupt;
pub mod classes;


pub use self::interrupt::{Result, Interrupt};
pub use self::id::Id;
pub use self::object::QObject;

use shared::{Shared, SafeAny};
pub type AnyObject = Shared<QObject<dyn SafeAny>>;
pub type SharedObject<T> = Shared<QObject<T>>;

impl From<!> for AnyObject {
	fn from(_: !) -> AnyObject {
		unreachable!()
	}
}
