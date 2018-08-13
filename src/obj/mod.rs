#[macro_use]
mod macros;

mod id;
mod object;
mod interrupt;
pub mod classes;


pub use self::interrupt::{Result, Interrupt};
pub use self::id::Id;
pub use self::object::{QObject, SharedObject};