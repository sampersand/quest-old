use std::any::Any;
use lazy_static::lazy_static;
use std::collections::HashMap;
use std::fmt::{self, Debug, Formatter};
use crate::object::{Type, Object, AnyObject};
use crate::shared::Shared;
use crate::map::{Map, ParentMap};
use crate::object::types::pristine::PRISTINE_MAP;


lazy_static! {
	pub static ref BASIC_MAP: Shared<dyn Map> = object_map!{UNTYPED "Basic", ParentMap::new(PRISTINE_MAP.clone(), HashMap::new());
		"===" => |obj, args| Ok(Object::new_boolean(obj.id() == getarg!(args[0])?.id())),
		"!==" => |obj, args| obj.call_attr("===", args)?.call_attr("!", &[]),
		"@bool" => |_, _| Ok(Object::new_boolean(true)),
		"@text" => |_, _| unimplemented!(),
		// "clone" => |obj, _| Ok(obj.duplicate())

		"==" => |obj, args| obj.call_attr("===", args),
		"!=" => |obj, args| obj.call_attr("==", args)?.call_attr("!", &[]),
		"->" => |obj, args| getarg!(args[0])?.call_attr("<-", &[obj]),

		"!" => |obj, _| obj.to_boolean()?.call_attr("!", &[]),
		"and" => |obj, args| if obj.to_boolean()?.is_true() {
				getarg!(args[0]).map(Clone::clone)
			} else {
				Ok(obj.clone())
			},
		"or" => |obj, args| if obj.to_boolean()?.is_true() {
				Ok(obj.clone())
			} else {
				getarg!(args[0]).map(Clone::clone)
			},
	};
}


#[cfg(test)]
mod test {
	use super::*;
	use crate::err::{Result, Error};
	use std::fmt::{self, Debug, Formatter};
	use crate::object::types::{Boolean, Variable};
	use std::hash::{Hash, Hasher};

	macro_rules! define_blank {
		(struct $struct:ident;) => { define_blank!(struct $struct, BLANK_MAP;); };
		(struct $struct:ident, $map:ident; $($impl_type_block:tt)*) => {
			struct $struct;
			impl Hash for $struct {
				fn hash<H: Hasher>(&self, _: &mut H) {
					unreachable!()
				}
			}

			impl Eq for $struct {}
			impl PartialEq for $struct {
				fn eq(&self, _: &$struct) -> bool {
					unreachable!()
				}
			}

			impl Debug for $struct {
				fn fmt(&self, _: &mut Formatter) -> fmt::Result {
					unreachable!()
				}
			}

			impl $struct {
				fn new_any() -> AnyObject {
					Object::new($struct).as_any()
				}
			}

			impl_type!{ for $struct, map $map; $($impl_type_block)* }
		}
	}


	define_blank!(struct BlankObject;);
	define_blank!(struct BlankButFalse, BLANK_BUT_FALSE_MAP;
		"@bool" => |_, _| Ok(Object::new_boolean(false))
	);
	define_blank!(struct BooleanError, BOOLEAN_ERROR;
		"@bool" => |_, _| Err(Error::__Testing)
	);


	// Object are strictly equal if they have the same pointer
	#[test]
	fn strict_equality() -> Result<()> {
		let ref obj = BlankObject::new_any();
		let ref obj_clone = obj.clone();
		let ref obj_duplicate = BlankObject::new_any();

		assert_eq!(obj.call_attr("===", &[obj])?.downcast_or_err::<Boolean>()?.is_true(), true);
		assert_eq!(obj.call_attr("===", &[obj_clone])?.downcast_or_err::<Boolean>()?.is_true(), true);
		assert_eq!(obj.call_attr("===", &[obj_duplicate])?.downcast_or_err::<Boolean>()?.is_true(), false);
		assert_eq!(obj.call_attr("===", &[obj_duplicate, obj])?.downcast_or_err::<Boolean>()?.is_true(), false); // ensure duplicates are ignored

		assert_eq!(obj.call_attr("!==", &[obj])?.downcast_or_err::<Boolean>()?.is_true(), false);
		assert_eq!(obj.call_attr("!==", &[obj_clone])?.downcast_or_err::<Boolean>()?.is_true(), false);
		assert_eq!(obj.call_attr("!==", &[obj_duplicate])?.downcast_or_err::<Boolean>()?.is_true(), true);
		assert_eq!(obj.call_attr("!==", &[obj_duplicate, obj])?.downcast_or_err::<Boolean>()?.is_true(), true); // ensure duplciates are ignored


		// check to see if too few args are passed it handles it right
		match obj.call_attr("===", &[]).unwrap_err() {
			Error::MissingArgument { pos: 0, .. } => {},
			_ => panic!("got bad err")
		};

		match obj.call_attr("!==", &[]).unwrap_err() {
			Error::MissingArgument { pos: 0, .. } => {},
			_ => panic!("got bad err")
		};

		define_blank!(struct BlankButTrueEquality, BLANK_BUT_TRUE_EQUALITY;
			"===" => |_, _| Ok(Object::new_boolean(true))
		);

		let ref obj = BlankButTrueEquality::new_any();
		let ref obj_clone = obj.clone();
		let ref obj_duplicate = BlankButTrueEquality::new_any();

		// ensure overriding works for `===`
		assert_eq!(obj.call_attr("===", &[obj])?.downcast_or_err::<Boolean>()?.is_true(), true);
		assert_eq!(obj.call_attr("===", &[obj_clone])?.downcast_or_err::<Boolean>()?.is_true(), true);
		assert_eq!(obj.call_attr("===", &[obj_duplicate])?.downcast_or_err::<Boolean>()?.is_true(), true);
		assert_eq!(obj.call_attr("===", &[obj_duplicate, obj])?.downcast_or_err::<Boolean>()?.is_true(), true); // ensure duplicates are ignored

		// make sure `!==` reroutes correctly too
		assert_eq!(obj.call_attr("!==", &[obj])?.downcast_or_err::<Boolean>()?.is_true(), false);
		assert_eq!(obj.call_attr("!==", &[obj_clone])?.downcast_or_err::<Boolean>()?.is_true(), false);
		assert_eq!(obj.call_attr("!==", &[obj_duplicate])?.downcast_or_err::<Boolean>()?.is_true(), false);
		assert_eq!(obj.call_attr("!==", &[obj_duplicate, obj])?.downcast_or_err::<Boolean>()?.is_true(), false); // ensure duplciates are ignored


		Ok(())
	}

	#[test]
	fn at_bool() -> Result<()> {
		let ref obj = BlankObject::new_any();
		assert_eq!(obj.call_attr("@bool", &[])?.downcast_or_err::<Boolean>()?.is_true(), true);
		assert_eq!(obj.call_attr("@bool", &[&BlankButFalse::new_any()])?.downcast_or_err::<Boolean>()?.is_true(), true);

		let ref obj = BlankButFalse::new_any();
		assert_eq!(obj.call_attr("@bool", &[])?.downcast_or_err::<Boolean>()?.is_true(), false);
		assert_eq!(obj.call_attr("@bool", &[&BlankObject::new_any()])?.downcast_or_err::<Boolean>()?.is_true(), false);

		Ok(())
	}

	#[test]
	#[ignore]
	fn at_text() {
		unimplemented!()
	}

	#[test]
	fn equality() -> Result<()> {
		define_blank!(struct BlankObject; );
		let ref obj = BlankObject::new_any();
		let ref obj_clone = obj.clone();
		let ref obj_duplicate = BlankObject::new_any();

		assert_eq!(obj.call_attr("==", &[obj])?.downcast_or_err::<Boolean>()?.is_true(), true);
		assert_eq!(obj.call_attr("==", &[obj_clone])?.downcast_or_err::<Boolean>()?.is_true(), true);
		assert_eq!(obj.call_attr("==", &[obj_duplicate])?.downcast_or_err::<Boolean>()?.is_true(), false);
		assert_eq!(obj.call_attr("==", &[obj_duplicate, obj])?.downcast_or_err::<Boolean>()?.is_true(), false); // ensure duplicates are ignored

		assert_eq!(obj.call_attr("!=", &[obj])?.downcast_or_err::<Boolean>()?.is_true(), false);
		assert_eq!(obj.call_attr("!=", &[obj_clone])?.downcast_or_err::<Boolean>()?.is_true(), false);
		assert_eq!(obj.call_attr("!=", &[obj_duplicate])?.downcast_or_err::<Boolean>()?.is_true(), true);
		assert_eq!(obj.call_attr("!=", &[obj_duplicate, obj])?.downcast_or_err::<Boolean>()?.is_true(), true); // ensure duplciates are ignored

		// check to see if too few args are passed it handles it right
		match obj.call_attr("==", &[]).unwrap_err() {
			Error::MissingArgument { pos: 0, .. } => {},
			_ => panic!("got bad err")
		};

		match obj.call_attr("!=", &[]).unwrap_err() {
			Error::MissingArgument { pos: 0, .. } => {},
			_ => panic!("got bad err")
		};

		// equality reroutes to `===`, so check to make sure it actually does if we override `===`
		use std::sync::atomic::{AtomicBool, Ordering::Relaxed};
		lazy_static! {
			static ref STRICT_EQUALITY_CALLED: AtomicBool = AtomicBool::new(false);
		}

		define_blank!(struct StrictEqChanged, STRICT_EQ_BLANKMAP;
			"===" => |_, _| {
				assert_eq!(STRICT_EQUALITY_CALLED.swap(true, Relaxed), false);
				Ok(Object::new_boolean(true)) // result's always true, to test `!=`
			}	
		);

		assert_eq!(STRICT_EQUALITY_CALLED.load(Relaxed), false);
		// note: extra argument isn't needed here, as the funciton ignores it
		assert_eq!(StrictEqChanged::new_any().call_attr("==", &[])?.downcast_or_err::<Boolean>()?.is_true(), true);
		assert_eq!(STRICT_EQUALITY_CALLED.swap(false, Relaxed), true);
		assert_eq!(StrictEqChanged::new_any().call_attr("!=", &[])?.downcast_or_err::<Boolean>()?.is_true(), false);
		assert_eq!(STRICT_EQUALITY_CALLED.swap(false, Relaxed), true);
		Ok(())

	}

	#[test]
	fn assignment_arrow_right() -> Result<()> {

		let obj1 = BlankObject::new_any();
		let obj2 = BlankObject::new_any();
		// first make sure the arrow returns an error if it doesnt exist
		match obj1.call_attr("->", &[&obj2]).unwrap_err() {
			Error::AttrMissing { attr, obj } => {
				assert!(obj.id_eq(&obj2));
				assert_eq!(**attr.downcast_or_err::<Variable>()?.data().read().unwrap(), "<-");
			},
			_ => unimplemented!("got bad err")
		}

		lazy_static! {
			static ref RECEIVED: std::sync::Mutex<Option<(AnyObject, AnyObject)>> = std::sync::Mutex::new(None);
		}

		// now make sure it correctly reroutes
		define_blank!(struct CanTakeArrow, CAN_TAKE_ARROW;
			"<-" => |obj, args| {
				*RECEIVED.try_lock().unwrap() = Some((obj.clone(), getarg!(args[0])?.clone()));
				Ok(Object::new_null())
			}
		);

		assert!(RECEIVED.try_lock().unwrap().is_none());
		let cantake = CanTakeArrow::new_any();
		assert!(obj1.call_attr("->", &[&cantake, &obj2])?.is_null()); // also to ensure extra args are ignored
		let (obj, arg) = RECEIVED.try_lock().unwrap().take().unwrap();
		assert!(cantake.id_eq(&obj), "{:?} != {:?}", cantake.id(), obj.id());
		assert!(arg.id_eq(&obj1));
		Ok(())
	}

	#[test]
	fn negate() -> Result<()> {
		assert_eq!(BlankObject::new_any().call_attr("!", &[])?.downcast_or_err::<Boolean>()?.is_true(), false);
		assert_eq!(BlankObject::new_any().call_attr("!", &[&BlankButFalse::new_any()])?.downcast_or_err::<Boolean>()?.is_true(), false);
		assert_eq!(BlankButFalse::new_any().call_attr("!", &[])?.downcast_or_err::<Boolean>()?.is_true(), true);
		Ok(())
	}

	#[test]
	fn and() -> Result<()> {
		let ref t = BlankObject::new_any();
		let ref f = BlankButFalse::new_any();
		let ref e = BooleanError::new_any();
		let ref f2 = BlankButFalse::new_any();
		let ref t2 = BlankObject::new_any();

		assert!(t.call_attr("and", &[t])?.id_eq(t));
		assert!(t.call_attr("and", &[t2])?.id_eq(t2));
		assert!(t.call_attr("and", &[f])?.id_eq(f));
		assert!(t.call_attr("and", &[f, e])?.id_eq(f));
		assert!(t.call_attr("and", &[e])?.id_eq(e));

		assert!(f.call_attr("and", &[t])?.id_eq(f));
		assert!(f.call_attr("and", &[f2])?.id_eq(f));
		assert!(f.call_attr("and", &[f])?.id_eq(f));
		assert!(f.call_attr("and", &[t, e])?.id_eq(f));
		assert!(f.call_attr("and", &[e])?.id_eq(f));

		assert!(matches!(e.call_attr("and", &[t]).unwrap_err(), Error::__Testing));
		assert!(matches!(e.call_attr("and", &[f2]).unwrap_err(), Error::__Testing));
		assert!(matches!(e.call_attr("and", &[f]).unwrap_err(), Error::__Testing));
		assert!(matches!(e.call_attr("and", &[f, e]).unwrap_err(), Error::__Testing));
		assert!(matches!(e.call_attr("and", &[e]).unwrap_err(), Error::__Testing));

		Ok(())
	}


	#[test]
	fn or() -> Result<()> {
		let ref t = BlankObject::new_any();
		let ref f = BlankButFalse::new_any();
		let ref e = BooleanError::new_any();
		let ref f2 = BlankButFalse::new_any();

		assert!(t.call_attr("or", &[t])?.id_eq(t));
		assert!(t.call_attr("or", &[&BlankObject::new_any()])?.id_eq(t));
		assert!(t.call_attr("or", &[f])?.id_eq(t));
		assert!(t.call_attr("or", &[f, e])?.id_eq(t));
		assert!(t.call_attr("or", &[e])?.id_eq(t));

		assert!(f.call_attr("or", &[t])?.id_eq(t));
		assert!(f.call_attr("or", &[f2])?.id_eq(f2));
		assert!(f.call_attr("or", &[f])?.id_eq(f));
		assert!(f.call_attr("or", &[t, e])?.id_eq(t));
		assert!(f.call_attr("or", &[e])?.id_eq(e));

		assert!(matches!(e.call_attr("or", &[t]).unwrap_err(), Error::__Testing));
		assert!(matches!(e.call_attr("or", &[f2]).unwrap_err(), Error::__Testing));
		assert!(matches!(e.call_attr("or", &[f]).unwrap_err(), Error::__Testing));
		assert!(matches!(e.call_attr("or", &[f, e]).unwrap_err(), Error::__Testing));
		assert!(matches!(e.call_attr("or", &[e]).unwrap_err(), Error::__Testing));

		Ok(())
	}
}











