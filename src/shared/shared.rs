use std::ops::CoerceUnsized;
use std::marker::Unsize;

use std::{any::Any, cmp::Ordering};
use std::hash::{Hash, Hasher};
use std::ops::{Deref, DerefMut};
use std::fmt::{self, Debug, Display, Formatter};
use std::cell::UnsafeCell;
use std::sync::{Arc, Weak as StdWeak, RwLock, RwLockReadGuard, RwLockWriteGuard, TryLockError};
use std::thread;

#[must_use = "if unused the Shared will immediately unlock"]
pub struct ReadGuard<'a, T: ?Sized + 'a>(&'a Shared<T>, RwLockReadGuard<'a, ()>);

#[must_use = "if unused the Shared will immediately unlock"]
pub struct WriteGuard<'a, T: ?Sized + 'a>(&'a Shared<T>, RwLockWriteGuard<'a, ()>);

pub struct Shared<T: ?Sized>(Arc<SharedInner<T>>);
pub struct Weak<T: ?Sized>(StdWeak<SharedInner<T>>);


#[derive(Debug)]
struct SharedInner<T: ?Sized> {
	lock: RwLock<()>, // todo: make this an actual implementation of a RwLock (and not use the inbuilt one)
	data: UnsafeCell<T>
}

#[must_use = "memory leak occurs if rawshared isn't used correctly"]
// used for converting to and from raw
#[derive(Debug)]
pub struct RawShared<T: ?Sized>(*const SharedInner<T>);

unsafe impl<T: ?Sized + Send> Send for SharedInner<T> {}
unsafe impl<T: ?Sized + Send + Sync> Sync for SharedInner<T> {}

impl<T: ?Sized + Unsize<U>, U: ?Sized> CoerceUnsized<Shared<U>> for Shared<T> {}
impl<T: ?Sized + Unsize<U>, U: ?Sized> CoerceUnsized<Weak<U>> for Weak<T> {}
impl<T: ?Sized + Unsize<U>, U: ?Sized> CoerceUnsized<RawShared<U>> for RawShared<T> {}

impl<T: ?Sized> Clone for Shared<T> {
	fn clone(&self) -> Self {
		Shared(self.0.clone())
	}
}

impl<T: ?Sized> Clone for Weak<T> {
	fn clone(&self) -> Self {
		Weak(self.0.clone())
	}
}

impl<T: Sized> Shared<T> {
	pub fn new(t: T) -> Self {
		Shared(Arc::new(
			SharedInner {
				lock: RwLock::new(()),
				data: UnsafeCell::from(t)
			} ))
	}
}


impl<T: ?Sized> RawShared<T> {
	pub fn convert<I>(self) -> RawShared<I> {
		RawShared(self.0 as *const SharedInner<I>)
	}
}

impl<T: 'static + ?Sized> Shared<T> {
	pub fn into_raw(self) -> RawShared<T> {
		RawShared(Arc::<SharedInner<T>>::into_raw(self.0))
	}
}

impl<T: 'static + Sized> Shared<T> {
	pub unsafe fn from_raw(raw: RawShared<T>) -> Self {
		Shared(Arc::from_raw(raw.0))
	}
}

impl<T: ?Sized> Weak<T> {
	pub fn upgrade(&self) -> Option<Shared<T>> {
		self.0.upgrade().map(Shared)
	}
}

impl<T> Default for Weak<T> {
	fn default() -> Self {
		Weak(StdWeak::new())
	}
}

impl<T: ?Sized> Shared<T> {
	pub fn downgrade(&self) -> Weak<T> {
		Weak(Arc::downgrade(&self.0))
	}

	pub fn is_locked(&self) -> bool {
		self.try_write().is_none()
	}

	#[inline]
	pub unsafe fn data(&self) -> &mut T {
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
			return self.try_write().expect("Blocking write encountered");
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

impl<T: Ord + ?Sized> Ord for Shared<T> {
	fn cmp(&self, other: &Shared<T>) -> Ordering {
		self.read().cmp(&other.read())
	}
}
impl<T: PartialOrd + ?Sized> PartialOrd for Shared<T> {
	fn partial_cmp(&self, other: &Shared<T>) -> Option<Ordering> {
		self.read().partial_cmp(&other.read())
	}	
}

impl<T: Eq + ?Sized> Eq for Shared<T> {}
impl<T: PartialEq + ?Sized> PartialEq for Shared<T> {
	fn eq(&self, other: &Shared<T>) -> bool {
		if self as *const Shared<T> == other as *const Shared<T> {
			true
		} else {
			(*self.read()) == (*other.read())
		}
	}
}


impl<T: Hash + ?Sized> Hash for Shared<T> {
	fn hash<H: Hasher>(&self, h: &mut H) {
		(*self.read()).hash(h);
	}
}


impl<T: Display + ?Sized> Display for Shared<T> {
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		write!(f, "{}", &*self.read())
	}
}

impl<T: Debug + ?Sized> Debug for Shared<T> {
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		if f.alternate() {
			f.debug_struct("Shared")
			 .field("lock", &self.0.lock.try_write().and(Ok("<unlocked>")).unwrap_or("<locked>"))
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

impl<T: ?Sized> Debug for Weak<T> {
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		write!(f, "Weak")
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