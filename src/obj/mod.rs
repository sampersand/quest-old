mod err;
mod id;
mod object;
mod attrs;

pub mod types;

pub use self::id::Id;
pub use self::types::Type;
pub use self::object::Object;
pub use self::err::{Error, Result};

use std::any::Any;
use shared::{Shared, Weak};

pub type AnyObject = Object<dyn Any + Send + Sync>;
pub type WeakObject = Weak<AnyObject>;
pub type SharedObject<T> = Shared<Object<T>>;
pub type AnyShared = Shared<AnyObject>;

pub type SharedResult<T> = Result<SharedObject<T>>;
pub type AnyResult = Result<AnyShared>;