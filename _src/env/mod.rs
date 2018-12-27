mod binding;
pub mod current;

pub use self::binding::Binding;

use shared::Shared;
use obj::AnyShared;
use std::collections::HashMap;

pub trait Parent {
	fn binding() -> Shared<Binding>;
}

pub type Mapping = HashMap<AnyShared, AnyShared>;


pub fn init(binding: Shared<Binding>) {
	assert_eq!(current::set_current(binding), None, "Initialized when a current existed!");
}