macro_rules! call_super {
	($ty:ident($attr:tt) for $obj:expr, $vars:expr, $env:expr) => {{
		$ty::default_attrs()[&$attr.into()].into_bound($obj).call(&$vars, $env)
		// x.call(&[obj], $env)
	}}
}

mod utils {
	use std::num::FpCategory::*;
	use env::Environment;
	use obj::QObject;

	#[derive(Debug, Clone, Copy, PartialEq)]
	pub enum IndexPos {
		InBounds(usize), // within the array
		OutOfBounds(usize), // out of bounds on the array
		Underflow(isize), // out of bounds, but too negative; the amount negative we are
		NotAnInt(f64),
	}

	use self::IndexPos::*;

	impl IndexPos {
		pub fn from_qobject(len: usize, pos: &QObject, env: &Environment) -> IndexPos {
			IndexPos::new(len, pos.as_num(env).expect("`@num` is required to index").to_f64())
		}

		pub fn new(len: usize, pos: f64) -> IndexPos {
			if pos.round() != pos || pos.is_infinite() {
				return NotAnInt(pos);
			}

			if len == 0 {
				return OutOfBounds(0);
			}

			let last = len - 1;
			let is_pos = pos.is_sign_positive();

			match pos.classify() {
				Zero   if is_pos => InBounds(0),
				Zero             => InBounds(last),
				Normal if pos.is_sign_positive() => if pos as usize <= last {
						InBounds(pos as usize)
					} else {
						OutOfBounds(pos as usize)
					},
				Normal => if -pos as usize <= last {
						InBounds((last as isize + pos as isize) as usize)
					} else {
						Underflow(pos as isize)
					},
				Subnormal => unimplemented!("todo: subnormal ({})", pos),
				Nan | Infinite => unreachable!()
			}
		}

		pub fn is_inbounds(&self) -> bool {
			if let InBounds(_) = self {
				true
			} else {
				false
			}
		}
	}
}


pub mod var;
pub mod boundfn;
pub mod null;

pub mod bool;
pub mod text;
pub mod num;
pub mod opers;

pub mod block;
pub mod list;
pub mod map;


pub use self::var::QVar;
pub use self::boundfn::QBoundFn;
pub use self::null::QNull;

pub use self::bool::QBool;
pub use self::text::QText;
pub use self::num::QNum;
pub use self::opers::QOper;

pub use self::block::QBlock;
pub use self::list::QList;
pub use self::map::QMap;