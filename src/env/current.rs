use shared::{Shared, SpinLock};
use env::Binding;
use std::mem;

lazy_static! {
	static ref LOCK: SpinLock = SpinLock::unlocked();
}

static mut CURRENT: Option<Shared<Binding>> = None;

pub fn current() -> Shared<Binding> {
	let lock = LOCK.lock();
	unsafe {
		if CURRENT.is_none() {
			CURRENT = Some(Shared::default());
		}
		CURRENT.clone().unwrap()
	}
}

pub fn set_current(binding: Shared<Binding>) -> Option<Shared<Binding>> {
	let lock = LOCK.lock();
	unsafe {
		mem::replace(&mut CURRENT, Some(binding))
	}
}

pub fn del_current() -> Option<Shared<Binding>> {
	let lock = LOCK.lock();
	unsafe {
		CURRENT.take()
	}
}

impl Binding {
	pub fn current() -> Shared<Binding> {
		current()
	}
}