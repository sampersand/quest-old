mod number;
mod variable;

pub use self::{
	number::Number,
	variable::Variable
};

use crate::{shared::Shared, map::Map};

pub trait Type : 'static + Send + Sync {
	fn get_type_map() -> Shared<dyn Map>;
}

#[cfg(test)]
mod tests {
	use super::*;
	
	fn _is_type_send_sync<T: Type>() {
		fn _send_sync<T: Send + Sync>() {}
		_send_sync::<T>()
	}
}