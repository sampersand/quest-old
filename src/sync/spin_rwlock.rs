use std::thread;
use std::cell::UnsafeCell;
use std::fmt::{self, Debug, Formatter};
use std::sync::{self, RwLock, TryLockError};
use std::ops::{Deref, DerefMut};
use std::hash::Hash;

#[derive(Debug)]
pub struct SpinWriteGuard<'a, T: 'a>(&'a SpinRwLock<T>, sync::RwLockWriteGuard<'a, ()>);//&'a SpinRwLock<T>);
#[derive(Debug)]
pub struct SpinReadGuard<'a, T: 'a>(&'a SpinRwLock<T>, sync::RwLockReadGuard<'a, ()>);//&'a SpinRwLock<T>);

#[derive(Debug, Default)]
pub struct SpinRwLock<T>{
	lock: RwLock<()>,
	data: UnsafeCell<T>
}

unsafe impl<T: Send> Send for SpinRwLock<T> {}
unsafe impl<T: Send> Sync for SpinRwLock<T> {}

impl<T> SpinRwLock<T> {
	#[inline]
	pub fn new(data: T) -> SpinRwLock<T> {
		SpinRwLock { 
			lock: RwLock::new(()),
			data: UnsafeCell::new(data)
		}
	}

	pub fn into_inner(self) -> T {
		self.data.into_inner()
	}

	#[inline]
	pub unsafe fn data(&self) -> &mut T {
		&mut *self.data.get()
	}

	pub fn read(&self) -> SpinReadGuard<T> {
		loop {
			if let Some(lock) = self.try_read() {
				return lock;
			} else {
				thread::yield_now();
			}
		}
	}

	pub fn write(&self) -> SpinWriteGuard<T> {
		loop {
			if let Some(lock) = self.try_write() {
				return lock;
			} else {
				thread::yield_now();
			}
		}
	}

	pub fn read_unwrap(&self, place: &'static str) -> SpinReadGuard<T> {
		self.try_read().expect(place)
	}

	pub fn write_unwrap(&self, place: &'static str) -> SpinWriteGuard<T> {
		self.try_write().expect(place)
	}

	pub fn try_read(&self) -> Option<SpinReadGuard<T>> {
		match self.lock.try_read() {
			Ok(guard) => Some(SpinReadGuard(self, guard)),
			Err(err) => match err {
				TryLockError::Poisoned(err) => panic!("poisoned lock encountered: {:?}", err),
				TryLockError::WouldBlock if cfg!(feature = "single-threaded") => panic!("Single threaded would block"),
				TryLockError::WouldBlock => None
			}
		}
	}

	pub fn try_write(&self) -> Option<SpinWriteGuard<T>> {
		match self.lock.try_write() {
			Ok(guard) => Some(SpinWriteGuard(self, guard)),
			Err(err) => match err {
				TryLockError::Poisoned(err) => panic!("poisoned lock encountered: {:?}", err),
				TryLockError::WouldBlock if cfg!(feature = "single-threaded") => panic!("Single threaded would block"),
				TryLockError::WouldBlock => None
			}
		}
	}
}

impl<T: Eq> Eq for SpinRwLock<T>{}
impl<T: PartialEq> PartialEq for SpinRwLock<T> {
	fn eq(&self, other: &SpinRwLock<T>) -> bool {
		if let (Some(selfguard), Some(otherguard)) = (self.try_read(), other.try_read()) {
			*selfguard == *otherguard
		} else {
			false
		}
	}
}

impl<T: Clone> SpinRwLock<T> {
	pub fn try_clone(&self) -> Option<SpinRwLock<T>> {
		Some(SpinRwLock::new(self.try_read()?.clone()))
	}
}

impl<T> From<T> for SpinRwLock<T> {
	#[inline]
	fn from(inp: T) -> SpinRwLock<T> {
		SpinRwLock::new(inp)
	}
}

impl<'a, T: 'a> Drop for SpinWriteGuard<'a, T> {
	#[inline]
	fn drop(&mut self) {
		// ...
	}
}

impl<'a, T: 'a> Drop for SpinReadGuard<'a, T> {
	#[inline]
	fn drop(&mut self) {
		// ...
	}
}

impl<'a, T: 'a> Deref for SpinReadGuard<'a, T> {
	type Target = T;
	#[inline]
	fn deref(&self) -> &T {
		unsafe{ self.0.data() }
	}
}

impl<'a, T: 'a> Deref for SpinWriteGuard<'a, T> {
	type Target = T;
	#[inline]
	fn deref(&self) -> &T {
		unsafe{ self.0.data() }
	}
}

impl<'a, T: 'a> DerefMut for SpinWriteGuard<'a, T> {
	#[inline]
	fn deref_mut(&mut self) -> &mut T {
		unsafe{ self.0.data() }
	}
}
