mod object;
mod id;

use self::object::QObject;
pub use self::id::Id;

use shared::Shared;
use std::any::Any;

pub type SharedObject<T> = Shared<QObject<T>>;
pub type AnyObject = SharedObject<dyn Any + Send + Sync>;

impl From<!> for AnyObject {
	fn from(_: !) -> AnyObject {
		unreachable!()
	}
}
