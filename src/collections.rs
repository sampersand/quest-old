mod parental;
mod map;
mod list;

pub use self::{
	parental::ParentalMap,
	map::Map,
	list::List
};

use crate::SharedObject;
use std::fmt::Debug;

pub trait Collection : Debug + Send + Sync {
	fn len(&self) -> usize;
	fn is_empty(&self) -> bool {
		self.len() == 0
	}
}

pub trait Mapping : Collection {
	fn get(&self, key: &SharedObject) -> Option<SharedObject>;
	fn set(&mut self, key: SharedObject, val: SharedObject) -> Option<SharedObject>;
	fn del(&mut self, key: &SharedObject) -> Option<SharedObject>;
	fn has(&self, key: &SharedObject) -> bool;
}

pub trait Listing : Collection {
	fn push(&mut self, obj: SharedObject);
	fn pop(&mut self) -> Option<SharedObject>;
}