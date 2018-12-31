use std::sync::{Arc, RwLock};
use std::hash::{Hash, Hasher};
use std::{marker::Unsize, ops::CoerceUnsized};
use std::thread;
use std::ops::{Deref, DerefMut};
use std::fmt::{self, Debug, Display, Formatter};

#[derive(Default)]
pub struct Shared<T: ?Sized> {
	data: Arc<RwLock<T>>
}

impl<T: Unsize<U> + ?Sized, U: ?Sized> CoerceUnsized<Shared<U>> for Shared<T> {}

impl<T> Shared<T> {
	pub fn new(data: T) -> Shared<T> {
		Shared {
			data: Arc::new(RwLock::new(data))
		}
	}
}

impl<T: Clone> Shared<T> {
	pub fn duplicate(&self) -> Self {
		Shared::new(self.read().clone())
	}
}

impl<T: ?Sized> Shared<T> {
	pub fn ptr_eq(&self, other: &Shared<T>) -> bool {
		Arc::ptr_eq(&self.data, &other.data)
	}

	pub fn read<'a>(&'a self) -> impl Deref<Target=T> + 'a {
		loop {
			if let Some(lock) = self.try_read() {
				return lock
			} else {
				trace!("Waiting for a read");
				thread::yield_now();
			}
		}
	}

	pub fn try_read<'a>(&'a self) -> Option<impl Deref<Target=T> + 'a> {
		use std::sync::TryLockError;
		match self.data.try_read() {
			Ok(lock) => Some(lock),
			Err(TryLockError::Poisoned(err)) => panic!("Poisoned lock encountered when reading: {:?}", err),
			Err(TryLockError::WouldBlock) => None
		}
	}


	pub fn write<'a>(&'a self) -> impl DerefMut<Target=T> + 'a {
		loop {
			if let Some(lock) = self.try_write() {
				return lock
			} else {
				trace!("Waiting for a write");
				thread::yield_now();
			}
		}
	}

	pub fn try_write<'a>(&'a self) -> Option<impl DerefMut<Target=T> + 'a> {
		use std::sync::TryLockError;
		match self.data.try_write() {
			Ok(lock) => Some(lock),
			Err(TryLockError::Poisoned(err)) => panic!("Poisoned lock encountered when writing: {:?}", err),
			Err(TryLockError::WouldBlock) => None
		}
	}
}

impl<T: Display + ?Sized> Display for Shared<T> {
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		Display::fmt(&*self.read(), f)
	}
}

impl<T: Debug + ?Sized> Debug for Shared<T> {
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		if f.alternate() {
			write!(f, "Shared({:#?})", &*self.read())
		} else {
			write!(f, "Shared({:?})", &*self.read())
		}
	}
}

impl<T: ?Sized> Clone for Shared<T> {
	fn clone(&self) -> Shared<T> {
		Shared { data: self.data.clone() }
	}
}

impl<T: Eq + ?Sized> Eq for Shared<T> {}
impl<T: PartialEq + ?Sized> PartialEq for Shared<T> {
	fn eq(&self, other: &Shared<T>) -> bool {
		if self.ptr_eq(other) {
			true
		} else {
			self.data.read().expect("Bad read").eq(&other.data.read().expect("Bad read"))
		}
	}
}

impl<T: Hash + ?Sized> Hash for Shared<T> {
	fn hash<H: Hasher>(&self, h: &mut H) {
		self.data.read().expect("Bad read").hash(h)
	}
}