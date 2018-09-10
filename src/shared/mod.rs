mod shared;
mod spinlock;

use std::process::Output;
pub use self::shared::Shared;
pub use self::spinlock::SpinLock;