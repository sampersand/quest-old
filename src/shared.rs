use std::sync::{Arc, RwLock};

#[derive(Debug)]
pub struct Shared<T> {
	data: Arc<RwLock<T>>
}

impl<T> Clone for Shared<T> {
	fn clone(&self) -> Shared<T> {
		Shared { data: self.data.clone() }
	}
}