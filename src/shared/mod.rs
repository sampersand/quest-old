mod shared;
pub mod map;

pub use self::shared::{Shared, ReadGuard, WriteGuard};
pub use self::map::SharedMap;

use std::{any::Any, fmt::Debug};
pub trait SafeAny : Debug + Any + Send + Sync {}
impl<T: Debug + Any + Send + Sync> SafeAny for T {}
