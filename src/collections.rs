mod parental;
mod map;
mod list;

pub use self::{
	parental::ParentalMap,
	map::Map,
	list::List
};

use crate::Object;
use std::fmt::Debug;

pub trait Collection : Debug + Send + Sync {
	fn len(&self) -> usize;
	fn is_empty(&self) -> bool {
		self.len() == 0
	}
}

pub trait Mapping : Collection {
	fn get(&self, key: &Object) -> Option<Object>;
	fn set(&mut self, key: Object, val: Object) -> Option<Object>;
	fn del(&mut self, key: &Object) -> Option<Object>;
	fn has(&self, key: &Object) -> bool;
}

pub trait Listing : Collection {
	fn push(&mut self, obj: Object);
	fn pop(&mut self) -> Option<Object>;
}