use std::any::Any;
use lazy_static::lazy_static;
use std::collections::HashMap;
use std::fmt::{self, Debug, Formatter};
use crate::object::{Type, Object, AnyObject};

use crate::shared::Shared;
use crate::map::{Map, ParentMap};
use crate::err::Result;
use crate::object::types::pristine::PRISTINE_MAP;

use super::quest_funcs::{
	STRICT_EQ, STRICT_NEQ, EQ, NEQ,
	NOT, AND, OR,
	ARROW_LEFT, ARROW_RIGHT,
	AT_BOOL, AT_TEXT,
	CLONE
};

fn strict_eq(obj: &AnyObject, args: &[&AnyObject]) -> Result<AnyObject> {
	Ok(Object::new_boolean(obj.id() == getarg!(args[0])?.id()))
}

fn strict_neq(obj: &AnyObject, args: &[&AnyObject]) -> Result<AnyObject> {
	obj.call_attr(STRICT_EQ, args)?.call_attr(NOT, &[])
}

fn at_bool(_: &AnyObject, _: &[&AnyObject]) -> Result<AnyObject> {
	Ok(Object::new_boolean(true))
}

fn at_text(obj: &AnyObject, _: &[&AnyObject]) -> Result<AnyObject> {
	unimplemented!()
}

fn clone(_obj: &AnyObject, _: &[&AnyObject]) -> Result<AnyObject> {
	unimplemented!()
	// CLONE => |obj, _| Ok(obj.duplicate())
}

fn eq(obj: &AnyObject, args: &[&AnyObject]) -> Result<AnyObject> {
	obj.call_attr(STRICT_EQ, args)
}

fn neq(obj: &AnyObject, args: &[&AnyObject]) -> Result<AnyObject> {
	obj.call_attr(EQ, args)?.call_attr(NOT, &[])
}

fn assignment_arrow_right(obj: &AnyObject, args: &[&AnyObject]) -> Result<AnyObject> {
	getarg!(args[0])?.call_attr(ARROW_LEFT, &[obj])
}

fn not(obj: &AnyObject, _: &[&AnyObject]) -> Result<AnyObject> {
	obj.to_boolean()?.as_any().call_attr(NOT, &[])
}

fn and(obj: &AnyObject, args: &[&AnyObject]) -> Result<AnyObject> {
	if obj.to_boolean()?.is_true() {
		getarg!(args[0]).map(Clone::clone)
	} else {
		Ok(obj.clone())
	}
}

fn or(obj: &AnyObject, args: &[&AnyObject]) -> Result<AnyObject> {
	if obj.to_boolean()?.is_true() {
		Ok(obj.clone())
	} else {
		getarg!(args[0]).map(Clone::clone)
	}
}

lazy_static! {
	pub static ref BASIC_MAP: Shared<dyn Map> = object_map!{UNTYPED "Basic", ParentMap::new(PRISTINE_MAP.clone(), HashMap::new());
		STRICT_EQ => strict_eq,
		STRICT_NEQ => strict_neq,
		AT_BOOL => at_bool,
		AT_TEXT => at_text,
		CLONE => clone,

		EQ => eq,
		NEQ => neq,
		ARROW_RIGHT => assignment_arrow_right,

		NOT => not,
		AND => and,
		OR => or
	};
}

// TODO: integration tests

#[cfg(test)]
mod tests {
	use super::*;
	use super::super::basic;
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
		AT_BOOL => |_, _| Ok(Object::new_boolean(false))
	);

	define_blank!(struct BooleanError, BOOLEAN_ERROR;
		AT_BOOL => |_, _| Err(Error::__Testing)
	);


	// Object are strictly equal if they have the same pointer
	#[test]
	fn strict_eq() -> Result<()> {
		let ref obj = BlankObject::new_any();
		let ref obj_clone = obj.clone();
		let ref obj_duplicate = BlankObject::new_any();

		assert_eq!(basic::strict_eq(obj, &[obj])?.downcast_or_err::<Boolean>()?.is_true(), true);
		assert_eq!(basic::strict_eq(obj, &[obj_clone])?.downcast_or_err::<Boolean>()?.is_true(), true);
		assert_eq!(basic::strict_eq(obj, &[obj_duplicate])?.downcast_or_err::<Boolean>()?.is_true(), false);
		assert_eq!(basic::strict_eq(obj, &[obj_duplicate, obj])?.downcast_or_err::<Boolean>()?.is_true(), false); // ensure duplicates are ignored

		assert_param_missing!(basic::strict_eq(obj, &[]));
		Ok(())
	}

	#[test]
	fn strict_neq() -> Result<()> {
		let ref obj = BlankObject::new_any();
		let ref obj_clone = obj.clone();
		let ref obj_duplicate = BlankObject::new_any();

		// make sure `!==` can override correctly
		assert_eq!(basic::strict_neq(obj, &[obj])?.downcast_or_err::<Boolean>()?.is_true(), false);
		assert_eq!(basic::strict_neq(obj, &[obj_clone])?.downcast_or_err::<Boolean>()?.is_true(), false);
		assert_eq!(basic::strict_neq(obj, &[obj_duplicate])?.downcast_or_err::<Boolean>()?.is_true(), true);
		assert_eq!(basic::strict_neq(obj, &[obj_duplicate, obj])?.downcast_or_err::<Boolean>()?.is_true(), true); // ensure duplciates are ignored

		assert_param_missing!(basic::strict_neq(obj, &[]));
		Ok(())
	}

	#[test]
	fn at_bool() -> Result<()> {
		let ref obj = BlankObject::new_any();
		assert_eq!(basic::at_bool(obj, &[])?.downcast_or_err::<Boolean>()?.is_true(), true);
		assert_eq!(basic::at_bool(obj, &[&BlankButFalse::new_any()])?.downcast_or_err::<Boolean>()?.is_true(), true);

		Ok(())
	}

	#[test]
	#[ignore]
	fn at_text() {
		unimplemented!()
	}

	#[test]
	fn eq() -> Result<()> {
		let ref obj = BlankObject::new_any();
		let ref obj_clone = obj.clone();
		let ref obj_duplicate = BlankObject::new_any();

		assert_eq!(basic::eq(obj, &[obj])?.downcast_or_err::<Boolean>()?.is_true(), true);
		assert_eq!(basic::eq(obj, &[obj_clone])?.downcast_or_err::<Boolean>()?.is_true(), true);
		assert_eq!(basic::eq(obj, &[obj_duplicate])?.downcast_or_err::<Boolean>()?.is_true(), false);
		assert_eq!(basic::eq(obj, &[obj_duplicate, obj])?.downcast_or_err::<Boolean>()?.is_true(), false); // ensure duplicates are ignored

		assert_param_missing!(basic::eq(obj, &[]));

		/* we don't need to test to see that `===` is called; we only do if we define `==` to default to that. */
		// use std::sync::atomic::{AtomicBool, Ordering::Relaxed};
		// lazy_static! {
		// 	static ref STRICT_EQUALITY_CALLED: AtomicBool = AtomicBool::new(false);
		// }

		// define_blank!(struct StrictEqChanged, STRICT_EQ_BLANKMAP;
		// 	STRICT_EQ => |_, _| {
		// 		assert_eq!(STRICT_EQUALITY_CALLED.swap(true, Relaxed), false);
		// 		Ok(Object::new_boolean(true)) // result's always true, to test `!=`
		// 	}	
		// );

		// assert_eq!(STRICT_EQUALITY_CALLED.load(Relaxed), false);
		// // note: extra argument isn't needed here, as the funciton ignores it
		// assert_eq!(basic::eq(&StrictEqChanged::new_any(), &[])?.downcast_or_err::<Boolean>()?.is_true(), true);
		// assert_eq!(STRICT_EQUALITY_CALLED.swap(false, Relaxed), true);
		// assert_eq!(basic::eq(&StrictEqChanged::new_any(), &[])?.downcast_or_err::<Boolean>()?.is_true(), false);
		// assert_eq!(STRICT_EQUALITY_CALLED.swap(false, Relaxed), true);
		Ok(())
	}

	#[test]
	fn neq() -> Result<()> {
		let ref obj = BlankObject::new_any();
		let ref obj_clone = obj.clone();
		let ref obj_duplicate = BlankObject::new_any();

		assert_eq!(basic::neq(obj, &[obj])?.downcast_or_err::<Boolean>()?.is_true(), false);
		assert_eq!(basic::neq(obj, &[obj_clone])?.downcast_or_err::<Boolean>()?.is_true(), false);
		assert_eq!(basic::neq(obj, &[obj_duplicate])?.downcast_or_err::<Boolean>()?.is_true(), true);
		assert_eq!(basic::neq(obj, &[obj_duplicate, obj])?.downcast_or_err::<Boolean>()?.is_true(), true); // ensure duplciates are ignored

		assert_param_missing!(basic::neq(obj, &[]));
		Ok(())
	}

	#[test]
	fn assignment_arrow_right() -> Result<()> {
		let ref obj1 = BlankObject::new_any();
		let ref obj2 = BlankObject::new_any();
		// first make sure the arrow returns an error if it doesnt exist
		match basic::assignment_arrow_right(obj1, &[obj2]).unwrap_err() {
			Error::AttrMissing { attr, obj } => {
				assert!(obj.id_eq(&obj2));
				assert_eq!(**attr.downcast_or_err::<Variable>()?.data().read().unwrap(), ARROW_LEFT);
			},
			_ => unimplemented!("got bad err")
		}

		lazy_static! {
			static ref RECEIVED: std::sync::Mutex<Option<(AnyObject, AnyObject)>> = std::sync::Mutex::new(None);
		}

		// now make sure it correctly reroutes
		define_blank!(struct CanTakeArrow, CAN_TAKE_ARROW;
			ARROW_LEFT => |obj, args| {
				*RECEIVED.try_lock().unwrap() = Some((obj.clone(), getarg!(args[0])?.clone()));
				Ok(Object::new_null())
			}
		);

		assert!(RECEIVED.try_lock().unwrap().is_none());
		let ref cantake = CanTakeArrow::new_any();
		assert!(basic::assignment_arrow_right(obj1, &[cantake, obj2])?.is_null()); // also to ensure extra args are ignored
		let (obj, arg) = RECEIVED.try_lock().unwrap().take().unwrap();
		assert!(cantake.id_eq(&obj), "{:?} != {:?}", cantake.id(), obj.id());
		assert!(arg.id_eq(obj1));
		Ok(())
	}

	#[test]
	fn not() -> Result<()> {
		assert_eq!(basic::not(&BlankObject::new_any(), &[])?.downcast_or_err::<Boolean>()?.is_true(), false);
		assert_eq!(basic::not(&BlankObject::new_any(), &[&BlankButFalse::new_any()])?.downcast_or_err::<Boolean>()?.is_true(), false);
		assert_eq!(basic::not(&BlankButFalse::new_any(), &[])?.downcast_or_err::<Boolean>()?.is_true(), true);
		Ok(())
	}

	#[test]
	fn and() -> Result<()> {
		let ref t = BlankObject::new_any();
		let ref f = BlankButFalse::new_any();
		let ref e = BooleanError::new_any();
		let ref f2 = BlankButFalse::new_any();
		let ref t2 = BlankObject::new_any();

		assert!(basic::and(t, &[t])?.id_eq(t));
		assert!(basic::and(t, &[t2])?.id_eq(t2));
		assert!(basic::and(t, &[f])?.id_eq(f));
		assert!(basic::and(t, &[f, e])?.id_eq(f));
		assert!(basic::and(t, &[e])?.id_eq(e));

		assert!(basic::and(f, &[t])?.id_eq(f));
		assert!(basic::and(f, &[f2])?.id_eq(f));
		assert!(basic::and(f, &[f])?.id_eq(f));
		assert!(basic::and(f, &[t, e])?.id_eq(f));
		assert!(basic::and(f, &[e])?.id_eq(f));

		assert!(matches!(basic::and(e, &[t]).unwrap_err(), Error::__Testing));
		assert!(matches!(basic::and(e, &[f2]).unwrap_err(), Error::__Testing));
		assert!(matches!(basic::and(e, &[f]).unwrap_err(), Error::__Testing));
		assert!(matches!(basic::and(e, &[f, e]).unwrap_err(), Error::__Testing));
		assert!(matches!(basic::and(e, &[e]).unwrap_err(), Error::__Testing));

		Ok(())
	}


	#[test]
	fn or() -> Result<()> {
		let ref t = BlankObject::new_any();
		let ref f = BlankButFalse::new_any();
		let ref e = BooleanError::new_any();
		let ref f2 = BlankButFalse::new_any();

		assert!(basic::or(t, &[t])?.id_eq(t));
		assert!(basic::or(t, &[&BlankObject::new_any()])?.id_eq(t));
		assert!(basic::or(t, &[f])?.id_eq(t));
		assert!(basic::or(t, &[f, e])?.id_eq(t));
		assert!(basic::or(t, &[e])?.id_eq(t));

		assert!(basic::or(f, &[t])?.id_eq(t));
		assert!(basic::or(f, &[f2])?.id_eq(f2));
		assert!(basic::or(f, &[f])?.id_eq(f));
		assert!(basic::or(f, &[t, e])?.id_eq(t));
		assert!(basic::or(f, &[e])?.id_eq(e));

		assert!(matches!(basic::or(e, &[t]).unwrap_err(), Error::__Testing));
		assert!(matches!(basic::or(e, &[f2]).unwrap_err(), Error::__Testing));
		assert!(matches!(basic::or(e, &[f]).unwrap_err(), Error::__Testing));
		assert!(matches!(basic::or(e, &[f, e]).unwrap_err(), Error::__Testing));
		assert!(matches!(basic::or(e, &[e]).unwrap_err(), Error::__Testing));

		Ok(())
	}

	mod integration {
		use super::*;
		use crate::object::types::Text;

		define_blank!(struct StrictlyEqAlways, STRICTLY_EQ_ALWAYS;
			STRICT_EQ => |_, _| Ok(Object::new_boolean(true))
		);

		define_blank!(struct StrictlyNeqAlways, STRICTLY_EQ_NEVER; 
			STRICT_NEQ => |_, _| Ok(Object::new_boolean(true))
		);

		define_blank!(struct EqNeverStrictCrash, EQ_NEVER_STRICT_CRASH;
			EQ => |_, _| Ok(Object::new_boolean(false)),
			NEQ => |_, _| unreachable!(),
			STRICT_EQ => |_, _| unreachable!(),
			STRICT_NEQ => |_, _| unreachable!()
		);
		define_blank!(struct NeqNeverStrictCrash, NEQ_NEVER_STRICT_CRASH;
			NEQ => |_, _| Ok(Object::new_boolean(false)),
			EQ => |_, _| unreachable!(),
			STRICT_EQ => |_, _| unreachable!(),
			STRICT_NEQ => |_, _| unreachable!()
		);


		// Object are strictly equal if they have the same pointer
		#[test]
		fn strict_eq() -> Result<()> {
			// first check to see that the default `===` does what we expect
			let ref obj = BlankObject::new_any();
			let ref obj_clone = obj.clone();
			let ref obj_duplicate = BlankObject::new_any();

			assert_eq!(obj.call_attr(STRICT_EQ, &[obj])?.downcast_or_err::<Boolean>()?.is_true(), true);
			assert_eq!(obj.call_attr(STRICT_EQ, &[obj_clone])?.downcast_or_err::<Boolean>()?.is_true(), true);
			assert_eq!(obj.call_attr(STRICT_EQ, &[obj_duplicate])?.downcast_or_err::<Boolean>()?.is_true(), false);
			assert_eq!(obj.call_attr(STRICT_EQ, &[obj_duplicate, obj])?.downcast_or_err::<Boolean>()?.is_true(), false); // ensure duplicates are ignored

			assert_param_missing!(obj.call_attr(STRICT_EQ, &[]));

			// and now see if we can override it
			let ref obj = StrictlyEqAlways::new_any();
			let ref obj_clone = obj.clone();
			let ref obj_duplicate = StrictlyEqAlways::new_any();

			assert_eq!(obj.call_attr(STRICT_EQ, &[obj])?.downcast_or_err::<Boolean>()?.is_true(), true);
			assert_eq!(obj.call_attr(STRICT_EQ, &[obj_clone])?.downcast_or_err::<Boolean>()?.is_true(), true);
			assert_eq!(obj.call_attr(STRICT_EQ, &[obj_duplicate])?.downcast_or_err::<Boolean>()?.is_true(), true);

			assert_param_missing!(obj.call_attr(STRICT_EQ, &[]));
			Ok(())
		}

		#[test]
		fn strict_neq() -> Result<()> {
			// first check to see that the default `!==` does what we expect
			let ref obj = BlankObject::new_any();
			let ref obj_clone = obj.clone();
			let ref obj_duplicate = BlankObject::new_any();

			assert_eq!(obj.call_attr(STRICT_NEQ, &[obj])?.downcast_or_err::<Boolean>()?.is_true(), false);
			assert_eq!(obj.call_attr(STRICT_NEQ, &[obj_clone])?.downcast_or_err::<Boolean>()?.is_true(), false);
			assert_eq!(obj.call_attr(STRICT_NEQ, &[obj_duplicate])?.downcast_or_err::<Boolean>()?.is_true(), true);
			assert_eq!(obj.call_attr(STRICT_NEQ, &[obj_duplicate, obj])?.downcast_or_err::<Boolean>()?.is_true(), true); // ensure duplicates are ignored

			assert_param_missing!(obj.call_attr(STRICT_NEQ, &[]));

			// make sure `===` can override correctly and `!==` correctly deals with it
			let ref obj = StrictlyEqAlways::new_any();
			let ref obj_clone = obj.clone();
			let ref obj_duplicate = StrictlyEqAlways::new_any();

			assert_eq!(obj.call_attr(STRICT_NEQ, &[obj])?.downcast_or_err::<Boolean>()?.is_true(), false);
			assert_eq!(obj.call_attr(STRICT_NEQ, &[obj_clone])?.downcast_or_err::<Boolean>()?.is_true(), false);
			assert_eq!(obj.call_attr(STRICT_NEQ, &[obj_duplicate])?.downcast_or_err::<Boolean>()?.is_true(), false);

			assert_param_missing!(obj.call_attr(STRICT_NEQ, &[]));

			// and now for `!==` being overriden directly
			let ref obj = StrictlyNeqAlways::new_any();
			let ref obj_clone = obj.clone();
			let ref obj_duplicate = StrictlyNeqAlways::new_any();

			assert_eq!(obj.call_attr(STRICT_NEQ, &[obj])?.downcast_or_err::<Boolean>()?.is_true(), true);
			assert_eq!(obj.call_attr(STRICT_NEQ, &[obj_clone])?.downcast_or_err::<Boolean>()?.is_true(), true);
			assert_eq!(obj.call_attr(STRICT_NEQ, &[obj_duplicate])?.downcast_or_err::<Boolean>()?.is_true(), true);

			assert_param_missing!(obj.call_attr(STRICT_NEQ, &[]));

			Ok(())
		}

		#[test]
		fn at_bool() -> Result<()> {
			// first check to see that the default `@bool` does what we expect
			let ref obj = BlankObject::new_any();
			assert_eq!(obj.call_attr(AT_BOOL, &[])?.downcast_or_err::<Boolean>()?.is_true(), true);
			assert_eq!(obj.call_attr(AT_BOOL, &[&BlankButFalse::new_any()])?.downcast_or_err::<Boolean>()?.is_true(), true);

			// and now see if we can override it correctly
			let ref obj = BlankButFalse::new_any();
			assert_eq!(obj.call_attr(AT_BOOL, &[])?.downcast_or_err::<Boolean>()?.is_true(), false);
			assert_eq!(obj.call_attr(AT_BOOL, &[&BlankObject::new_any()])?.downcast_or_err::<Boolean>()?.is_true(), false);

			Ok(())
		}

		#[test]
		fn at_text() -> Result<()> {
			// we just want to make sure t he output is overriden, as the default
			// output isn't specified
			const TEXT: &'static str = "im textually savvy";
			define_blank!(struct BlankButText, BLANK_BUT_TEXT; 
				AT_TEXT => |_,_| Ok(Object::new_text_str(TEXT))
			);

			let ref obj = BlankButText::new_any();
			assert_eq!(obj.call_attr(AT_TEXT, &[])?.downcast_or_err::<Text>()?.data().read().unwrap().as_ref(), TEXT);
			assert_eq!(obj.call_attr(AT_TEXT, &[&BlankObject::new_any()])?.downcast_or_err::<Text>()?.data().read().unwrap().as_ref(), TEXT);

			Ok(())
		}

		#[test]
		fn eq() -> Result<()> {
			// first check to make sure it works as we'd expect `===` to work
			let ref obj = BlankObject::new_any();
			let ref obj_clone = obj.clone();
			let ref obj_duplicate = BlankObject::new_any();

			assert_eq!(obj.call_attr(EQ, &[obj])?.downcast_or_err::<Boolean>()?.is_true(), true);
			assert_eq!(obj.call_attr(EQ, &[obj_clone])?.downcast_or_err::<Boolean>()?.is_true(), true);
			assert_eq!(obj.call_attr(EQ, &[obj_duplicate])?.downcast_or_err::<Boolean>()?.is_true(), false);
			assert_eq!(obj.call_attr(EQ, &[obj_duplicate, obj])?.downcast_or_err::<Boolean>()?.is_true(), false); // ensure duplicates are ignored

			assert_param_missing!(obj.call_attr(EQ, &[]));

			// now make sure modifying `===` will correctly redirect it
			let ref obj = StrictlyEqAlways::new_any();
			let ref obj_clone = obj.clone();
			let ref obj_duplicate = StrictlyEqAlways::new_any();

			assert_eq!(obj.call_attr(EQ, &[obj])?.downcast_or_err::<Boolean>()?.is_true(), true);
			assert_eq!(obj.call_attr(EQ, &[obj_clone])?.downcast_or_err::<Boolean>()?.is_true(), true);
			assert_eq!(obj.call_attr(EQ, &[obj_duplicate])?.downcast_or_err::<Boolean>()?.is_true(), true);

			assert_param_missing!(obj.call_attr(EQ, &[]));

			// now make sure that you can overwrite `==` correctly
			let ref obj = EqNeverStrictCrash::new_any();
			let ref obj_clone = obj.clone();
			let ref obj_duplicate = EqNeverStrictCrash::new_any();

			assert_eq!(obj.call_attr(EQ, &[obj])?.downcast_or_err::<Boolean>()?.is_true(), false);
			assert_eq!(obj.call_attr(EQ, &[obj_clone])?.downcast_or_err::<Boolean>()?.is_true(), false);
			assert_eq!(obj.call_attr(EQ, &[obj_duplicate])?.downcast_or_err::<Boolean>()?.is_true(), false);

			assert_param_missing!(obj.call_attr(EQ, &[]));

			Ok(())
		}

		#[test]
		fn neq() -> Result<()> {
			// first check to make sure it works as we'd expect `!==` to work
			let ref obj = BlankObject::new_any();
			let ref obj_clone = obj.clone();
			let ref obj_duplicate = BlankObject::new_any();

			assert_eq!(obj.call_attr(NEQ, &[obj])?.downcast_or_err::<Boolean>()?.is_true(), false);
			assert_eq!(obj.call_attr(NEQ, &[obj_clone])?.downcast_or_err::<Boolean>()?.is_true(), false);
			assert_eq!(obj.call_attr(NEQ, &[obj_duplicate])?.downcast_or_err::<Boolean>()?.is_true(), true);
			assert_eq!(obj.call_attr(NEQ, &[obj_duplicate, obj])?.downcast_or_err::<Boolean>()?.is_true(), true); // ensure duplicates are ignored

			assert_param_missing!(obj.call_attr(NEQ, &[]));

			// now make sure modifying `===` will correctly redirect it
			let ref obj = StrictlyEqAlways::new_any();
			let ref obj_clone = obj.clone();
			let ref obj_duplicate = StrictlyEqAlways::new_any();

			assert_eq!(obj.call_attr(NEQ, &[obj])?.downcast_or_err::<Boolean>()?.is_true(), false);
			assert_eq!(obj.call_attr(NEQ, &[obj_clone])?.downcast_or_err::<Boolean>()?.is_true(), false);
			assert_eq!(obj.call_attr(NEQ, &[obj_duplicate])?.downcast_or_err::<Boolean>()?.is_true(), false);

			assert_param_missing!(obj.call_attr(NEQ, &[]));

			// make sure that modifying `!==` won't affect it
			let ref obj = StrictlyNeqAlways::new_any();
			let ref obj_clone = obj.clone();
			let ref obj_duplicate = StrictlyNeqAlways::new_any();

			assert_eq!(obj.call_attr(NEQ, &[obj])?.downcast_or_err::<Boolean>()?.is_true(), false);
			assert_eq!(obj.call_attr(NEQ, &[obj_clone])?.downcast_or_err::<Boolean>()?.is_true(), false);
			assert_eq!(obj.call_attr(NEQ, &[obj_duplicate])?.downcast_or_err::<Boolean>()?.is_true(), true);

			assert_param_missing!(obj.call_attr(NEQ, &[]));


			// now make sure that you can overwrite `==` correctly

			let ref obj = EqNeverStrictCrash::new_any();
			let ref obj_clone = obj.clone();
			let ref obj_duplicate = EqNeverStrictCrash::new_any();

			assert_eq!(obj.call_attr(NEQ, &[obj])?.downcast_or_err::<Boolean>()?.is_true(), false);
			assert_eq!(obj.call_attr(NEQ, &[obj_clone])?.downcast_or_err::<Boolean>()?.is_true(), false);
			assert_eq!(obj.call_attr(NEQ, &[obj_duplicate])?.downcast_or_err::<Boolean>()?.is_true(), false);

			assert_param_missing!(obj.call_attr(NEQ, &[]));

			Ok(())
		}
		
		#[test]
		fn assignment_arrow_right() -> Result<()> {
			let ref obj1 = BlankObject::new_any();
			let ref obj2 = BlankObject::new_any();
			// first make sure the arrow returns an error if it doesnt exist
			match basic::assignment_arrow_right(obj1, &[obj2]).unwrap_err() {
				Error::AttrMissing { attr, obj } => {
					assert!(obj.id_eq(&obj2));
					assert_eq!(**attr.downcast_or_err::<Variable>()?.data().read().unwrap(), ARROW_LEFT);
				},
				_ => unimplemented!("got bad err")
			}

			lazy_static! {
				static ref RECEIVED: std::sync::Mutex<Option<(AnyObject, AnyObject)>> = std::sync::Mutex::new(None);
			}

			// now make sure it correctly reroutes
			define_blank!(struct CanTakeArrow, CAN_TAKE_ARROW;
				ARROW_LEFT => |obj, args| {
					*RECEIVED.try_lock().unwrap() = Some((obj.clone(), getarg!(args[0])?.clone()));
					Ok(Object::new_null())
				}
			);

			assert!(RECEIVED.try_lock().unwrap().is_none());
			let ref cantake = CanTakeArrow::new_any();
			assert!(basic::assignment_arrow_right(obj1, &[cantake, obj2])?.is_null()); // also to ensure extra args are ignored
			let (obj, arg) = RECEIVED.try_lock().unwrap().take().unwrap();
			assert!(cantake.id_eq(&obj), "{:?} != {:?}", cantake.id(), obj.id());
			assert!(arg.id_eq(obj1));
			Ok(())
		}

		#[test]
		fn not() -> Result<()> {
			assert_eq!(basic::not(&BlankObject::new_any(), &[])?.downcast_or_err::<Boolean>()?.is_true(), false);
			assert_eq!(basic::not(&BlankObject::new_any(), &[&BlankButFalse::new_any()])?.downcast_or_err::<Boolean>()?.is_true(), false);
			assert_eq!(basic::not(&BlankButFalse::new_any(), &[])?.downcast_or_err::<Boolean>()?.is_true(), true);
			Ok(())
		}

		#[test]
		fn and() -> Result<()> {
			let ref t = BlankObject::new_any();
			let ref f = BlankButFalse::new_any();
			let ref e = BooleanError::new_any();
			let ref f2 = BlankButFalse::new_any();
			let ref t2 = BlankObject::new_any();

			assert!(basic::and(t, &[t])?.id_eq(t));
			assert!(basic::and(t, &[t2])?.id_eq(t2));
			assert!(basic::and(t, &[f])?.id_eq(f));
			assert!(basic::and(t, &[f, e])?.id_eq(f));
			assert!(basic::and(t, &[e])?.id_eq(e));

			assert!(basic::and(f, &[t])?.id_eq(f));
			assert!(basic::and(f, &[f2])?.id_eq(f));
			assert!(basic::and(f, &[f])?.id_eq(f));
			assert!(basic::and(f, &[t, e])?.id_eq(f));
			assert!(basic::and(f, &[e])?.id_eq(f));

			assert!(matches!(basic::and(e, &[t]).unwrap_err(), Error::__Testing));
			assert!(matches!(basic::and(e, &[f2]).unwrap_err(), Error::__Testing));
			assert!(matches!(basic::and(e, &[f]).unwrap_err(), Error::__Testing));
			assert!(matches!(basic::and(e, &[f, e]).unwrap_err(), Error::__Testing));
			assert!(matches!(basic::and(e, &[e]).unwrap_err(), Error::__Testing));

			Ok(())
		}
	}
}






