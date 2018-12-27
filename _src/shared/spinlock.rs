use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;

pub const LOCKED: bool = true;
pub const UNLOCKED: bool = false;

#[derive(Debug)]
pub struct SpinLock(AtomicBool);

#[must_use = "no point in locking then unlocking"]
struct SpinLockGuard<'a>(&'a SpinLock);

impl Default for SpinLock {
	#[inline]
	fn default() -> Self {
		SpinLock::new(UNLOCKED)
	}
}

impl SpinLock {
	#[inline]
	pub fn new(locked: bool) -> Self {
		SpinLock(AtomicBool::new(locked))
	}

	#[inline]
	pub fn unlocked() -> Self {
		SpinLock::new(UNLOCKED)
	}

	#[inline]
	pub fn locked() -> Self {
		SpinLock::new(LOCKED)
	}
	

	pub fn lock<'a>(&'a self) -> impl Drop + 'a {
		if cfg!(feature = "single-threaded") {
			return self.try_lock().expect("Blocking write encountered for spinlock");
		}

		loop {
			trace!("Inside spinlock lock loop");
			if let Some(guard) = self.try_lock() {
				trace!("spinlock has been acquired");
				return guard;
			}
			thread::yield_now();
		}
	}

	pub fn try_lock<'a>(&'a self) -> Option<impl Drop + 'a> {
		if let UNLOCKED = self.0.compare_and_swap(UNLOCKED, LOCKED, Ordering::Acquire) {
			Some(SpinLockGuard(self))
		} else {
			None
		}
	}
}

impl<'a> Drop for SpinLockGuard<'a> {
	fn drop(&mut self) {
		trace!("spinlock is being released");
		assert_eq!((self.0).0.swap(UNLOCKED, Ordering::Release), LOCKED, "lock was unlocked when released?");
	}
}