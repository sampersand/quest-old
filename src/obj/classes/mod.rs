use shared::Shared;
use std::fmt::Debug;
use std::hash::Hash;
use env::Environment;
use std::collections::HashMap;
use obj::{Id, Result, object::QObject, SharedObject};

pub type DefaultAttrs<T> = HashMap<Id, fn(&Shared<QObject<T>>, &[&SharedObject], &Environment) -> Result<SharedObject>>;

pub trait QuestClass : Debug + 'static {
	fn default_attrs() -> &'static DefaultAttrs<Self> where Self: Sized;
}

mod bool;
mod text;
mod var;
mod exception;

// these classes have special inner values that need to be accessed from `parse`
pub(crate) mod null;
pub(crate) mod num;
pub(crate) mod oper;

pub use self::bool::QBool;
pub use self::num::QNum;
pub use self::null::QNull;
pub use self::text::QText;
pub use self::var::QVar;
pub use self::oper::QOper;
pub use self::exception::QException;