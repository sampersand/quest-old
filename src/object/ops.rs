use std::fmt::{self, Debug, Formatter};
use std::hash::{Hash, Hasher};
use crate::object::Data;

#[derive(Clone, Copy)]
pub struct Ops {
	pub eq: fn(&Data, &Data) -> bool,
	pub debug: fn(&Data, &mut Formatter) -> fmt::Result,
	pub hash: fn(&Data, &mut Hasher),
	pub clone: fn(&Data) -> Data
}

impl Ops {
	pub fn from<T: Eq + Debug + Hash + Clone + 'static>() -> Ops {
		Ops {
			eq: |this, other| 
				if let Some(other) = other.try_as_ref::<T>() {
					this.try_as_ref::<T>().expect("'this' passed to `eq` wasn't of right type'") == other
				} else {
					false
				},
			debug: |this, f| 
				Debug::fmt(this.try_as_ref::<T>().expect("'this' passed to `debug` wasn't of right type"), f),
			hash: |this, h| {
				assert!(this.is::<T>(), "'this' passed to `hash` wasn't of right type?");
				println!("todo: hash");
				// if let Some(this) = this.try_as_ref::<T>() {
				// 	Hash::hash(this, h as &mut Hasher)
				// } else {
				// 	Hash::hash(0, h)
				// }
			},
			clone: |this|
				Data::new(this.try_as_ref::<T>().expect("'this' passed to `clone` wasn't of right type") .clone())
		}
	}
}

impl Debug for Ops {
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		struct PointerDebug(*const ());
		impl Debug for PointerDebug {
			fn fmt(&self, f: &mut Formatter) -> fmt::Result {
				write!(f, "{:p}", self.0)
			}
		}

		f.debug_struct("Ops")
			.field("eq", &PointerDebug(self.eq as *const ()))
			.field("debug", &PointerDebug(self.debug as *const ()))
			.field("hash", &PointerDebug(self.hash as *const ()))
			.finish()
	}
}

impl Eq for Ops {}
impl PartialEq for Ops {
	fn eq(&self, other: &Ops) -> bool {
		(self.eq as usize) == (other.eq as usize) &&
		(self.debug as usize) == (other.debug as usize) &&
		(self.hash as usize) == (other.hash as usize)
	}
}

impl Hash for Ops {
	fn hash<H: Hasher>(&self, h: &mut H) {
		(self.eq as usize).hash(h);
		(self.debug as usize).hash(h);
		(self.hash as usize).hash(h);
	}
}