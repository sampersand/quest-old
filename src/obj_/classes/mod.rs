use std::fmt::Debug;
use obj::{Id, AnyObject};

pub type GetDefault = fn(&AnyObject, &Id) -> Option<AnyObject>;
pub type HasDefault = fn(&Id) -> bool; // same as `GET_DEFAULT` without actually executing it

pub trait QuestClass : Debug + Send + Sync + 'static {
	const GET_DEFAULTS: GetDefault;
	const HAS_DEFAULTS: HasDefault;
}

mod bool;
mod text;
mod var;
mod exception;
mod boundfn;

// these classes have special inner values that need to be accessed from `parse`
pub(crate) mod block;
pub(crate) mod null;
pub(crate) mod num;
pub(crate) mod oper;

pub use self::bool::QBool;
pub use self::num::QNum;
pub use self::null::QNull;
pub use self::text::QText;
pub use self::var::QVar;
pub use self::oper::QOper;
pub use self::block::QBlock;
pub use self::boundfn::QBoundFn;
pub use self::exception::QException;