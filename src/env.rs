use crate::shared::Shared;
use crate::map::Map;

pub fn current() -> Shared<dyn Map> {
	Shared::new(::std::collections::HashMap::new())
	// unimplemented!()
}