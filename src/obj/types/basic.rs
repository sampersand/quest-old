use env::{Parent, Binding, Mapping};
use shared::Shared;
use std::sync::atomic::{AtomicBool, Ordering};

use obj::{Object, AnyShared, SharedObject};

pub struct Basic;


impl Parent for Basic {
	fn binding() -> Shared<Binding> {
		let binding = DEFAULT_ATTRS.clone();
		if USED.compare_and_swap(false, true, Ordering::Relaxed) == false {
			let ins = Object::new_var("inspect").any();
			let i = Object::new_bound(inspect).any();

			if &ins == &i {}
			use std::hash::{Hash, Hasher};
			use std::collections::hash_map::DefaultHasher;

			let mut h = DefaultHasher::new();
			ins.hash(&mut h);
			h.finish();
			let mut b = binding.write();
			b.set(ins, i);
		}
		binding
	}
}

fn inspect(me: &SharedObject<Basic>, args: &[AnyShared]) -> AnyShared {
	unimplemented!("inspect")
}

lazy_static! {
	static ref DEFAULT_ATTRS: Shared<Binding> = Binding::new(Binding::empty(), Mapping::default());
	static ref USED: AtomicBool = AtomicBool::new(false);
}