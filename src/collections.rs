mod parental;
mod map;
mod list;

pub use self::{
	parental::ParentalMap,
	map::Map,
	list::List
};

use crate::{Shared, Object};
use std::fmt::Debug;

pub trait Collection : Debug + Send + Sync {
	fn len(&self) -> usize;
	fn is_empty(&self) -> bool {
		self.len() == 0
	}
}

pub trait Mapping : Collection {
	fn get(&self, key: &Shared<Object>) -> Option<Shared<Object>>;
	fn set(&mut self, key: Shared<Object>, val: Shared<Object>) -> Option<Shared<Object>>;
	fn del(&mut self, key: &Shared<Object>) -> Option<Shared<Object>>;
	fn has(&self, key: &Shared<Object>) -> bool;
}

pub trait Listing : Collection {
	fn push(&mut self, obj: Shared<Object>);
	fn pop(&mut self) -> Option<Shared<Object>>;
}