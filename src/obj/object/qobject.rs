use env::Environment;
use obj::{Id, attrs::AttrId, classes};
use std::ops::Deref;
use super::{QObj, Classes};
use std::fmt::{self, Debug, Display, Formatter};


// #[cfg(feature = "single-threaded")]
// type RefC<T> = ::std::rc::Rc<T>;

// #[cfg(not(feature = "single-threaded"))]
type RefC<T> = ::std::sync::Arc<T>;

#[derive(Clone, Hash, PartialEq, Eq)]
pub struct QObject(RefC<QObj>);

impl Debug for QObject {
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		Debug::fmt(&self.0.class().deref(), f)
	}
}

impl From<RefC<QObj>> for QObject {
	#[inline]
	fn from(obj: RefC<QObj>) -> QObject {
		QObject(obj)
	}
}

impl From<QObj> for QObject {
	#[inline]
	fn from(obj: QObj) -> QObject {
		QObject::from(RefC::new(obj))
	}
}

impl Deref for QObject {
	type Target = QObj;
	fn deref(&self) -> &QObj {
		self.0.deref()
	}
}

impl Display for QObject {
	#[inline]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		Display::fmt(&self.0, f)
	}
}

impl QObject {
	pub fn get_attr<I: Into<AttrId> + Clone>(&self, id: I) -> Option<QObject> {
		let id = id.into();
		if let Some(attr) = self.attrs.get(id.clone(), self) {
			Some(attr)
		} else {
			warn!("Attribute `{}` doesn't exist in `{}`", id, self);
			None
		}
	}

	pub fn set_attr<I: Into<AttrId>>(&self, id: I, obj: QObject) -> Option<QObject> {
		self.attrs.set(id, obj)
	}

	pub fn call(&self, args: &[&QObject], env: &Environment) -> QObject {
		self.call_attr("()", args, env)
	}

	pub fn call_local(&self, args: &[&QObject], env: &Environment) -> QObject {
		self.call_attr("{}", args, env)
	}

	pub fn call_attr<I: Into<AttrId>>(&self, id: I, args: &[&QObject], env: &Environment) -> QObject {
		let id = id.into();

		if let Some(qboundfn) = self.attrs.get(id.clone(), self) {
			if let Classes::BoundFn(boundfn) = qboundfn.class().deref() {
				boundfn.call(args, env)
			} else {
				panic!("BoundFn is needed to call attr")
			}
		} else {
			warn!("Missing attribute {} for {:?}", id, self);
			().into()
		}
	}

	pub fn del_attr<I: Into<AttrId>>(&self, id: I) -> Option<QObject> {
		self.attrs.del(id)
	}

	pub fn has_attr<I: Into<AttrId>>(&self, id: I) -> bool {
		self.attrs.has(id)
	}
}


macro_rules! define_check {
	($($fn:ident $ty:ident)*) => {
		impl QObject {
			$(
				pub fn $fn(&self) -> bool {
					if let Classes::$ty(_) = self.class().deref() {
						true
					} else {
						false
					}
				}
			)*
		}
	}
}

define_check! {
	is_var   Var   is_boundfn BoundFn is_null Null
	is_bool  Bool  is_text   Text   is_num  Num
	is_block Block is_list   List   is_map  Map
}


macro_rules! define_conversion {
	// ($($try_fn:ident $fn:ident $class:ident $attr:tt $obj:ident)*) => {
	($($try_cast:ident $as:ident $class:ident $attr:tt $obj:ident)*) => {
		impl QObject {
			$(
				pub fn $try_cast(&self) -> Option<classes::$obj> { // should return a reference
					if let Classes::$class(class) = self.class().deref() {
						Some(class.clone())
					} else {
						None
					}
				}

				pub fn $as(&self, env: &Environment) -> Option<classes::$obj> {
					if !self.has_attr($attr) {
						return None;
					}

					let obj_arc = self.call_attr($attr, &[], &env).0;
					let obj = RefC::try_unwrap(obj_arc).expect(concat!("Unable to get a unique `", stringify!($obj), "` (from calling \"", $attr, "\")"));
					match obj.into_class() {
						Classes::$class(class) => Some(class),
						invalid_class => panic!(concat!("`", $attr, "` for {:?} should return a ", stringify!($class), ", not `{:?}`"), self, invalid_class)
					}
				}
			)*
		}
	}
}

define_conversion! {
	try_cast_bool as_bool Bool "@bool" QBool
	try_cast_list as_list List "@list" QList
	try_cast_map  as_map  Map  "@map"  QMap
	try_cast_num  as_num  Num  "@num"  QNum
	try_cast_text as_text Text "@text" QText
	try_cast_var  as_var  Var  "@var"  QVar
}
