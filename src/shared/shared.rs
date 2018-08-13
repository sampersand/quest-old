use std::ops::CoerceUnsized;
use std::marker::Unsize;

use std::ops::{Deref, DerefMut};
use std::fmt::{self, Debug, Formatter};
use std::cell::UnsafeCell;
use std::sync::{Arc, RwLock, RwLockReadGuard, RwLockWriteGuard, TryLockError};
use std::thread;

#[must_use = "if unused the Shared will immediately unlock"]
pub struct ReadGuard<'a, T: ?Sized + 'a>(&'a Shared<T>, RwLockReadGuard<'a, ()>);

#[must_use = "if unused the Shared will immediately unlock"]
pub struct WriteGuard<'a, T: ?Sized + 'a>(&'a Shared<T>, RwLockWriteGuard<'a, ()>);

pub struct Shared<T: ?Sized>(Arc<SharedInner<T>>);

struct SharedInner<T: ?Sized>{
	lock: RwLock<()>, // todo: make this an actual implementation of a RwLock (and not use the inbuilt one)
	data: UnsafeCell<T>
}

unsafe impl<T: ?Sized + Send> Send for SharedInner<T> {}
unsafe impl<T: ?Sized + Send + Sync> Sync for SharedInner<T> {}

impl<T: ?Sized + Unsize<U>, U: ?Sized> CoerceUnsized<Shared<U>> for Shared<T> {}

impl<T: ?Sized> Clone for Shared<T> {
	fn clone(&self) -> Self {
		Shared(self.0.clone())
	}
}

impl<T: Sized> Shared<T> {
	pub fn new(t: T) -> Shared<T> {
		Shared(Arc::new(
			SharedInner {
				lock: RwLock::new(()),
				data: UnsafeCell::from(t)
			}
		))
	}

	#[inline]
	pub fn try_unwrap(self) -> Result<T, Self> {
		Arc::try_unwrap(self.0).map(|inner| inner.data.into_inner()).map_err(Shared)
	}

}
impl<T: Clone> Shared<T> {
	#[inline]
	pub fn try_clone_inner(&self) -> Option<Shared<T>> {
		self.try_read().map(|lock| Shared::new(T::clone(&lock)))
	}
}

impl<T: ?Sized> Shared<T> {
	#[inline]
	unsafe fn data(&self) -> &mut T {
		&mut *self.0.data.get()
	}

	pub fn read(&self) -> ReadGuard<T> {
		if cfg!(feature = "single-threaded") {
			return self.try_read().expect("Blocking read encountered");
		}

		loop {
			trace!("Inside shared read lock");
			if let Some(guard) = self.try_read() {
				return guard;
			}
			thread::yield_now();
		}
	}

	pub fn write(&self) -> WriteGuard<T> {
		if cfg!(feature = "single-threaded") {
			return self.try_write().expect("Blocking read encountered");
		}

		loop {
			trace!("Inside shared write lock");
			if let Some(guard) = self.try_write() {
				return guard;
			}
			thread::yield_now();
		}
	}

	pub fn try_read(&self) -> Option<ReadGuard<T>> {
		match self.0.lock.try_read() {
			Ok(val) => Some(ReadGuard(self, val)),
			Err(TryLockError::WouldBlock) => None,
			Err(TryLockError::Poisoned(err)) => panic!("Poisoned shared queue encountered: {}", err)
		}
	}

	pub fn try_write(&self) -> Option<WriteGuard<T>> {
		match self.0.lock.try_write() {
			Ok(val) => Some(WriteGuard(self, val)),
			Err(TryLockError::WouldBlock) => None,
			Err(TryLockError::Poisoned(err)) => panic!("Poisoned shared queue encountered: {}", err)
		}
	}
}

impl<T: Default> Default for Shared<T> {
	#[inline]
	fn default() -> Shared<T> {
		Shared::new(T::default())
	}
}

impl<T> From<T> for Shared<T> {
	#[inline]
	fn from(inp: T) -> Shared<T> {
		Shared::new(inp)
	}
}

impl<T: Eq + ?Sized> Eq for Shared<T> {}
impl<T: PartialEq + ?Sized> PartialEq for Shared<T> {
	fn eq(&self, other: &Shared<T>) -> bool {
		if self as *const Shared<T> == other as *const Shared<T> {
			return true;
		}
		*self.read() == *other.read()
	}
}

impl<'a, T: 'a + ?Sized> Deref for ReadGuard<'a, T> {
	type Target = T;

	#[inline]
	fn deref(&self) -> &T {
		unsafe {
			self.0.data()
		}
	}
}

impl<'a, T: 'a + ?Sized> Deref for WriteGuard<'a, T> {
	type Target = T;

	#[inline]
	fn deref(&self) -> &T {
		unsafe {
			self.0.data()
		}
	}
}

impl<'a, T: 'a + ?Sized> DerefMut for WriteGuard<'a, T> {
	#[inline]
	fn deref_mut(&mut self) -> &mut T {
		unsafe {
			self.0.data()
		}
	}
}

impl<T: Debug + ?Sized> Debug for Shared<T> {
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		if f.alternate() {
			f.debug_struct("Shared")
			 .field("lock", &self.0.lock.try_read().and(Ok("<unlocked>")).unwrap_or("<locked>"))
			 .field("data", unsafe{ &self.data() }).finish()
		} else {
			if let Some(data) = self.try_read() {
				f.debug_tuple("Shared").field(&&data.deref()).finish()
			} else {
				f.debug_tuple("Shared").field(&"<locked>").finish()
			}
		}
	}
}

impl<'a, T: Debug> Debug for ReadGuard<'a, T> {
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		f.debug_tuple("SharedReadGuard").field(&self.0).finish()
	}
}

impl<'a, T: Debug> Debug for WriteGuard<'a, T> {
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		f.debug_tuple("SharedWriteGuard").field(&self.0).finish()
	}
}