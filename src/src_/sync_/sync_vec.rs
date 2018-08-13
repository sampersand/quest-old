use std::ops::DerefMut;
use sync_::{SpinRwLock, SpinReadGuard};
use std::fmt::{self, Debug, Formatter};


pub struct SyncVec<T>(SpinRwLock<Vec<T>>);

impl<T> SyncVec<T> {
	pub fn new() -> SyncVec<T> {
		SyncVec::default()
	}

	pub fn read(&self) -> SpinReadGuard<Vec<T>> {
		self.0.read()
	}
}

impl<T: Debug> Debug for SyncVec<T> {
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		if let Some(map) = self.0.try_read() {
			f.debug_list().entries(map.iter()).finish()
		} else {
			write!(f, "{{ <locked syncmap> }}")
		}
	}
}

impl<T: Clone> SyncVec<T> {
	pub fn try_clone(&self) -> Option<Self> {
		self.0.try_read().map(|map| SyncVec::from(map.clone()))
	}
}

impl<T: Clone> Clone for SyncVec<T> {
	fn clone(&self) -> Self {
		self.try_clone().expect("deadlock whilst cloning syncvec")
	}
}

impl<T> From<Vec<T>> for SyncVec<T> {
	fn from(map: Vec<T>) -> SyncVec<T> {
		SyncVec(SpinRwLock::from(map))
	}
}

impl<T> Default for SyncVec<T> {
	fn default() -> SyncVec<T> {
		SyncVec(SpinRwLock::default())
	}
}
