use std::any::Any;
use lazy_static::lazy_static;
use std::collections::HashMap;
use std::fmt::{self, Debug, Formatter};
use crate::object::{Type, Object, AnyObject};

use crate::shared::Shared;
use crate::map::{Map, ParentMap};
use crate::err::Result;
use crate::object::types::pristine::PRISTINE_MAP;

use super::quest_funcs;

mod funcs {
	use super::*;
	use crate::object::types::{Boolean, Text};

	pub fn at_bool(_: &AnyObject) -> Object<Boolean> {
		Object::new_boolean(true)
	}

	pub fn at_text(obj: &AnyObject) -> Object<Text> {
		unimplemented!()
	}

	pub fn clone(obj: &AnyObject) -> AnyObject {
		unimplemented!()
		// L_CLONE => |obj, _| Ok(obj.duplicate())
	}

	pub fn strict_eql(obj: &AnyObject, rhs: &AnyObject) -> Object<Boolean> {
		Object::new_boolean(obj.id_eq(rhs))
	}

	pub fn strict_neq(obj: &AnyObject, rhs: &AnyObject) -> Result<Object<Boolean>> {
		obj.call_attr(quest_funcs::STRICT_EQL, &[rhs])?
		   .call_attr(quest_funcs::NOT, &[])?
		   .downcast_or_err::<Boolean>()
	}

	pub fn eql(obj: &AnyObject, rhs: &AnyObject) -> Result<Object<Boolean>> {
		obj.call_attr(quest_funcs::STRICT_EQL, &[rhs])?
		   .downcast_or_err::<Boolean>()
	}

	pub fn neq(obj: &AnyObject, rhs: &AnyObject) -> Result<Object<Boolean>> {
		obj.call_attr(quest_funcs::EQL, &[rhs])?
		   .call_attr(quest_funcs::NOT, &[])?
		   .downcast_or_err::<Boolean>()
	}

	pub fn not(obj: &AnyObject,) -> Result<Object<Boolean>> {
		obj.to_boolean()?.as_any()
		   .call_attr(quest_funcs::NOT, &[])?
		   .downcast_or_err::<Boolean>()
	}

	pub fn and(obj: &AnyObject, rhs: &AnyObject) -> Result<AnyObject> {
		if obj.to_boolean()?.is_true() {
			Ok(rhs.clone())
		} else {
			Ok(obj.clone())
		}
	}

	pub fn or(obj: &AnyObject, rhs: &AnyObject) -> Result<AnyObject> {
		if obj.to_boolean()?.is_true() {
			Ok(obj.clone())
		} else {
			Ok(rhs.clone())
		}
	}

	pub fn arrow_right(obj: &AnyObject, rhs: &AnyObject) -> Result<AnyObject> {
		rhs.call_attr(quest_funcs::ARROW_LEFT, &[&obj])
	}
}

lazy_static! {
	pub static ref BASIC_MAP: Shared<dyn Map> = object_map!{UNTYPED "Basic", ParentMap::new(PRISTINE_MAP.clone(), HashMap::new());
		quest_funcs::AT_BOOL => |o, _| Ok(funcs::at_bool(o)),
		quest_funcs::AT_TEXT => |o, _| Ok(funcs::at_text(o)),
		quest_funcs::L_CLONE => |o, _| Ok(funcs::clone(o)),

		quest_funcs::STRICT_EQL => |o, a| Ok(funcs::strict_eql(o, getarg!(a[0])?)),
		quest_funcs::STRICT_NEQ => |o, a| Ok(funcs::strict_neq(o, getarg!(a[0])?)?),
		quest_funcs::EQL => |o, a| Ok(funcs::eql(o, getarg!(a[0])?)?),
		quest_funcs::NEQ => |o, a| Ok(funcs::neq(o, getarg!(a[0])?)?),

		quest_funcs::NOT => |o, _| Ok(funcs::not(o)?),
		quest_funcs::AND => |o, a| funcs::and(o, getarg!(a[0])?),
		quest_funcs::OR => |o, a| funcs::or(o, getarg!(a[0])?),

		quest_funcs::ARROW_RIGHT => |o, a| funcs::arrow_right(o, getarg!(a[0])?),
	};
}


#[cfg(test)]
define_blank!(struct BlankObject;);

#[cfg(test)]
define_blank!(struct BlankButFalse, BLANK_BUT_FALSE_MAP;
	quest_funcs::AT_BOOL => |_, _| Ok(Object::new_boolean(false))
);

#[cfg(test)]
define_blank!(struct BooleanError, BOOLEAN_ERROR;
	quest_funcs::AT_BOOL => |_, _| Err(crate::err::Error::__Testing)
);

#[cfg(test)]
define_blank!(struct InvertStrictEql, INVERT_STRICT_EQL; 
	quest_funcs::STRICT_EQL => |o, a| Ok(Object::new_boolean(!funcs::strict_eql(&o.as_any(), getarg!(a[0]).unwrap()).is_true()))
);

#[cfg(test)]
mod fn_tests {
	use super::*;
	use crate::err::{Error, Result};
	use crate::object::types::{Boolean, Variable};


	#[test]
	fn at_bool() {
		assert_eq!(funcs::at_bool(&BlankObject::new_any()), true);
		assert_eq!(funcs::at_bool(&BlankButFalse::new_any()), true); // make sure it doesn't actually check internals if called directly
	}

	#[test]
	#[ignore]
	fn at_text() { unimplemented!("{}", quest_funcs::AT_TEXT) }

	#[test]
	#[ignore]
	fn clone() { unimplemented!("{}", quest_funcs::L_CLONE); }

	// Object are strictly equal if they have the same pointer
	#[test]
	fn strict_eql() {
		let ref obj = BlankObject::new_any();

		assert_eq!(funcs::strict_eql(obj, obj), true);
		assert_eq!(funcs::strict_eql(obj, &obj.clone()), true);
		assert_eq!(funcs::strict_eql(obj, &BlankObject::new_any()), false);
	}

	#[test]
	fn strict_neq() -> Result<()> {
		let ref obj = BlankObject::new_any();

		assert_eq!(funcs::strict_neq(obj, obj)?, false);
		assert_eq!(funcs::strict_neq(obj, &obj.clone())?, false);
		assert_eq!(funcs::strict_neq(obj, &BlankObject::new_any())?, true);

		// check to make sure overriding `===` works properly
		let ref obj = InvertStrictEql::new_any();
		assert_eq!(funcs::strict_neq(obj, obj)?, true);
		assert_eq!(funcs::strict_neq(obj, &obj.clone())?, true);
		assert_eq!(funcs::strict_neq(obj, &BlankObject::new_any())?, false);

		Ok(())
	}

	#[test]
	fn eql() -> Result<()> {
		let ref obj = BlankObject::new_any();

		assert_eq!(funcs::eql(obj, obj)?, true);
		assert_eq!(funcs::eql(obj, &obj.clone())?, true);
		assert_eq!(funcs::eql(obj, &BlankObject::new_any())?, false);

		// check to make sure overriding `===` works properly
		let ref obj = InvertStrictEql::new_any();
		assert_eq!(funcs::eql(obj, obj)?, false);
		assert_eq!(funcs::eql(obj, &obj.clone())?, false);
		assert_eq!(funcs::eql(obj, &BlankObject::new_any())?, true);

		Ok(())
	}

	#[test]
	fn neq() -> Result<()> {
		let ref obj = BlankObject::new_any();

		assert_eq!(funcs::neq(obj, obj)?, false);
		assert_eq!(funcs::neq(obj, &obj.clone())?, false);
		assert_eq!(funcs::neq(obj, &BlankObject::new_any())?, true);

		// check to make sure overriding `==` works properly
		define_blank!(struct InvertEql; 
			quest_funcs::EQL => |o, a| Ok(Object::new_boolean(!funcs::eql(&o.as_any(), getarg!(a[0])?)?.is_true()))
		);

		let ref obj = InvertEql::new_any();
		assert_eq!(funcs::neq(obj, obj)?, true);
		assert_eq!(funcs::neq(obj, &obj.clone())?, true);
		assert_eq!(funcs::neq(obj, &BlankObject::new_any())?, false);


		// check to make sure overriding `===` works properly
		let ref obj = InvertStrictEql::new_any();
		assert_eq!(funcs::neq(obj, obj)?, true);
		assert_eq!(funcs::neq(obj, &obj.clone())?, true);
		assert_eq!(funcs::neq(obj, &BlankObject::new_any())?, false);
		Ok(())
	}

	#[test]
	fn not() -> Result<()> {
		assert_eq!(funcs::not(&BlankObject::new_any())?, false);
		assert_eq!(funcs::not(&BlankButFalse::new_any())?, true);
		Ok(())
	}

	#[test]
	fn and() -> Result<()> {
		let ref t = BlankObject::new_any();
		let ref f = BlankButFalse::new_any();
		let ref e = BooleanError::new_any();
		let ref f2 = BlankButFalse::new_any();
		let ref t2 = BlankObject::new_any();

		assert!(funcs::and(t, t)?.id_eq(t));
		assert!(funcs::and(t, t2)?.id_eq(t2));
		assert!(funcs::and(t, f)?.id_eq(f));
		assert!(funcs::and(t, e)?.id_eq(e));

		assert!(funcs::and(f, t)?.id_eq(f));
		assert!(funcs::and(f, f2)?.id_eq(f));
		assert!(funcs::and(f, f)?.id_eq(f));
		assert!(funcs::and(f, e)?.id_eq(f));

		assert!(matches!(funcs::and(e, t).unwrap_err(), Error::__Testing));
		assert!(matches!(funcs::and(e, f).unwrap_err(), Error::__Testing));
		assert!(matches!(funcs::and(e, e).unwrap_err(), Error::__Testing));

		Ok(())
	}


	#[test]
	fn or() -> Result<()> {
		let ref t = BlankObject::new_any();
		let ref f = BlankButFalse::new_any();
		let ref e = BooleanError::new_any();
		let ref f2 = BlankButFalse::new_any();
		let ref t2 = BlankObject::new_any();

		assert!(funcs::or(t, t)?.id_eq(t));
		assert!(funcs::or(t, t2)?.id_eq(t));
		assert!(funcs::or(t, f)?.id_eq(t));
		assert!(funcs::or(t, e)?.id_eq(t));

		assert!(funcs::or(f, t)?.id_eq(t));
		assert!(funcs::or(f, f2)?.id_eq(f2));
		assert!(funcs::or(f, f)?.id_eq(f));
		assert!(funcs::or(f, e)?.id_eq(e));

		assert!(matches!(funcs::or(e, t).unwrap_err(), Error::__Testing));
		assert!(matches!(funcs::or(e, f).unwrap_err(), Error::__Testing));
		assert!(matches!(funcs::or(e, e).unwrap_err(), Error::__Testing));

		Ok(())
	}

	#[test]
	fn arrow_right() -> Result<()> {
		let ref obj1 = BlankObject::new_any();
		let ref obj2 = BlankObject::new_any();
		// first make sure the arrow returns an error if it doesnt exist
		match funcs::arrow_right(obj1, obj2) {
			Err(Error::AttrMissing { attr, obj }) => {
				assert!(obj.id_eq(&obj2));
				assert_eq!(attr.downcast_or_err::<Variable>()?, quest_funcs::ARROW_LEFT);
			},
			/* this might cause a bad instruction if it prints out an object and not an error */
			other => panic!("got bad response from funcs::arrow_right: {:?}", other)
		}

		lazy_static! {
			static ref RECEIVED: std::sync::Mutex<Option<(AnyObject, AnyObject)>> = std::sync::Mutex::new(None);
		}

		// now make sure it correctly reroutes
		define_blank!(struct CanTakeArrow, CAN_TAKE_ARROW;
			quest_funcs::ARROW_LEFT => |obj, args| {
				*RECEIVED.try_lock().unwrap() = Some((obj.clone(), getarg!(args[0])?.clone()));
				Ok(Object::new_null())
			}
		);

		assert!(RECEIVED.try_lock().unwrap().is_none());
		let ref cantake = CanTakeArrow::new_any();
		assert!(funcs::arrow_right(obj1, cantake)?.is_null()); // also to ensure extra args are ignored
		let (obj, arg) = RECEIVED.try_lock().unwrap().take().unwrap();
		assert!(cantake.id_eq(&obj), "{:?} != {:?}", cantake.id(), obj.id());
		assert!(arg.id_eq(obj1));

		Ok(())
	}
}

#[cfg(test)]
mod integration {
	use super::*;
	use quest_funcs::*;
	use crate::object::types::Boolean;

	#[test]
	fn at_bool() -> Result<()> {
		let ref obj = BlankObject::new_any();

		assert_eq!(obj.call_attr(AT_BOOL, &[])?.downcast_or_err::<Boolean>()?, true);
		assert_eq!(obj.call_attr(AT_BOOL, &[&BlankButFalse::new_any()])?.downcast_or_err::<Boolean>()?, true);
		assert_eq!(BlankButFalse::new_any().call_attr(AT_BOOL, &[])?.downcast_or_err::<Boolean>()?, false);

		Ok(())
	}

	#[test]
	#[ignore]
	fn at_text() -> Result<()> {
		unimplemented!()
	}

	#[test]
	#[ignore]
	fn clone() -> Result<()> {
		unimplemented!("clone")
	}

	#[test]
	fn strict_eql() -> Result<()> {
		let ref obj = BlankObject::new_any();

		assert_eq!(obj.call_attr(STRICT_EQL, &[obj])?.downcast_or_err::<Boolean>()?, true);
		assert_eq!(obj.call_attr(STRICT_EQL, &[obj, &BlankObject::new_any()])?.downcast_or_err::<Boolean>()?, true);
		assert_param_missing!(obj.call_attr(STRICT_EQL, &[]));

		Ok(())
	}

	#[test]
	fn strict_neq() -> Result<()> {
		let ref obj = BlankObject::new_any();

		assert_eq!(obj.call_attr(STRICT_NEQ, &[obj])?.downcast_or_err::<Boolean>()?, false);
		assert_eq!(obj.call_attr(STRICT_NEQ, &[obj, &BlankObject::new_any()])?.downcast_or_err::<Boolean>()?, false);
		assert_param_missing!(obj.call_attr(STRICT_NEQ, &[]));

		Ok(())
	}

	#[test]
	fn eql() -> Result<()> {
		let ref obj = BlankObject::new_any();

		assert_eq!(obj.call_attr(EQL, &[obj])?.downcast_or_err::<Boolean>()?, true);
		assert_eq!(obj.call_attr(EQL, &[obj, &BlankObject::new_any()])?.downcast_or_err::<Boolean>()?, true);
		assert_param_missing!(obj.call_attr(EQL, &[]));

		Ok(())
	}

	#[test]
	fn neq() -> Result<()> {
		let ref obj = BlankObject::new_any();

		assert_eq!(obj.call_attr(NEQ, &[obj])?.downcast_or_err::<Boolean>()?, false);
		assert_eq!(obj.call_attr(NEQ, &[obj, &BlankObject::new_any()])?.downcast_or_err::<Boolean>()?, false);
		assert_param_missing!(obj.call_attr(NEQ, &[]));

		Ok(())
	}

	#[test]
	fn not() -> Result<()> {
		let ref obj = BlankObject::new_any();

		assert_eq!(obj.call_attr(NOT, &[])?.downcast_or_err::<Boolean>()?, false);
		assert_eq!(obj.call_attr(NOT, &[&BlankButFalse::new_any()])?.downcast_or_err::<Boolean>()?, false);
		assert_eq!(BlankButFalse::new_any().call_attr(NOT, &[])?.downcast_or_err::<Boolean>()?, true);
		assert_param_missing!(obj.call_attr(NEQ, &[]));

		Ok(())
	}

	#[test]
	fn and() -> Result<()> {
		let ref obj = BlankObject::new_any();
		let ref fobj = BlankButFalse::new_any();

		assert!(fobj.call_attr(AND, &[obj])?.id_eq(obj));
		assert!(obj.call_attr(AND, &[fobj])?.id_eq(fobj));
		assert!(obj.call_attr(AND, &[fobj, obj])?.id_eq(fobj));
		assert_param_missing!(obj.call_attr(AND, &[]));

		Ok(())
	}

	#[test]
	fn or() -> Result<()> {
		let ref obj = BlankObject::new_any();
		let ref fobj = BlankButFalse::new_any();

		assert!(fobj.call_attr(OR, &[obj])?.id_eq(obj));
		assert!(obj.call_attr(OR, &[fobj])?.id_eq(obj));
		assert!(BlankButFalse::new_any().call_attr(OR, &[fobj, obj])?.id_eq(fobj));
		assert_param_missing!(obj.call_attr(OR, &[]));

		Ok(())
	}

	#[test]
	fn arrow_right() -> Result<()> {
		let ref obj = BlankObject::new_any();
		let ref obj2 = BlankObject::new_any();	

		match obj.call_attr(ARROW_LEFT, &[obj2]) {
			Err(crate::err::Error::AttrMissing { attr, obj }) => {
				assert!(obj.id_eq(&obj2));
				assert_eq!(attr.downcast_or_err::<crate::object::types::Variable>()?, ARROW_LEFT);
			},
			/* this might cause a bad instruction if it prints out an object and not an error */
			other => panic!("got bad response from funcs::arrow_right: {:?}", other)
		}
		assert_param_missing!(obj.call_attr(ARROW_RIGHT, &[]));

		Ok(())
	}

}

/*
mod integration {
	use super::*;
	use crate::object::types::Text;

	define_blank!(struct StrictlyEqAlways, STRICTLY_EQ_ALWAYS;
		STRICT_EQL => |_, _| Ok(Object::new_boolean(true))
	);

	define_blank!(struct StrictlyNeqAlways, STRICTLY_EQ_NEVER; 
		STRICT_NEQ => |_, _| Ok(Object::new_boolean(true))
	);

	define_blank!(struct EqNeverStrictCrash, EQ_NEVER_STRICT_CRASH;
		EQL => |_, _| Ok(Object::new_boolean(false)),
		STRICT_EQL => |_, _| unreachable!(),
		STRICT_NEQ => |_, _| unreachable!()
	);
	define_blank!(struct NeqNeverStrictCrash, NEQ_NEVER_STRICT_CRASH;
		NEQ => |_, _| Ok(Object::new_boolean(false)),
		EQL => |_, _| unreachable!(),
		STRICT_EQL => |_, _| unreachable!(),
		STRICT_NEQ => |_, _| unreachable!()
	);


	// Object are strictly equal if they have the same pointer
	#[test]
	fn strict_eql() -> Result<()> {
		// first check to see that the default `===` does what we expect
		let ref obj = BlankObject::new_any();
		let ref obj_clone = obj.clone();
		let ref obj_duplicate = BlankObject::new_any();

		assert_eq!(obj.call_attr(STRICT_EQL, &[obj])?.downcast_or_err::<Boolean>()?.is_true(), true);
		assert_eq!(obj.call_attr(STRICT_EQL, &[obj_clone])?.downcast_or_err::<Boolean>()?.is_true(), true);
		assert_eq!(obj.call_attr(STRICT_EQL, &[obj_duplicate])?.downcast_or_err::<Boolean>()?.is_true(), false);
		assert_eq!(obj.call_attr(STRICT_EQL, &[obj_duplicate, obj])?.downcast_or_err::<Boolean>()?.is_true(), false); // ensure duplicates are ignored

		assert_param_missing!(obj.call_attr(STRICT_EQL, &[]));

		// and now see if we can override it
		let ref obj = StrictlyEqAlways::new_any();
		let ref obj_clone = obj.clone();
		let ref obj_duplicate = StrictlyEqAlways::new_any();

		assert_eq!(obj.call_attr(STRICT_EQL, &[obj])?.downcast_or_err::<Boolean>()?.is_true(), true);
		assert_eq!(obj.call_attr(STRICT_EQL, &[obj_clone])?.downcast_or_err::<Boolean>()?.is_true(), true);
		assert_eq!(obj.call_attr(STRICT_EQL, &[obj_duplicate])?.downcast_or_err::<Boolean>()?.is_true(), true);

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

		// and now for `!==` being overriden directly
		let ref obj = StrictlyNeqAlways::new_any();
		let ref obj_clone = obj.clone();
		let ref obj_duplicate = StrictlyNeqAlways::new_any();

		assert_eq!(obj.call_attr(STRICT_NEQ, &[obj])?.downcast_or_err::<Boolean>()?.is_true(), true);
		assert_eq!(obj.call_attr(STRICT_NEQ, &[obj_clone])?.downcast_or_err::<Boolean>()?.is_true(), true);
		assert_eq!(obj.call_attr(STRICT_NEQ, &[obj_duplicate])?.downcast_or_err::<Boolean>()?.is_true(), true);

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
		assert_eq!(obj.call_attr(AT_TEXT, &[])?.downcast_or_err::<Text>()?.unwrap_data().as_ref(), TEXT);
		assert_eq!(obj.call_attr(AT_TEXT, &[&BlankObject::new_any()])?.downcast_or_err::<Text>()?.unwrap_data().as_ref(), TEXT);

		Ok(())
	}

	#[test]
	#[ignore]
	fn clone() { unimplemented!("{}", L_CLONE); }

	#[test]
	fn eql() -> Result<()> {
		// first check to make sure it works as we'd expect `===` to work
		let ref obj = BlankObject::new_any();
		let ref obj_clone = obj.clone();
		let ref obj_duplicate = BlankObject::new_any();

		assert_eq!(obj.call_attr(EQL, &[obj])?.downcast_or_err::<Boolean>()?.is_true(), true);
		assert_eq!(obj.call_attr(EQL, &[obj_clone])?.downcast_or_err::<Boolean>()?.is_true(), true);
		assert_eq!(obj.call_attr(EQL, &[obj_duplicate])?.downcast_or_err::<Boolean>()?.is_true(), false);
		assert_eq!(obj.call_attr(EQL, &[obj_duplicate, obj])?.downcast_or_err::<Boolean>()?.is_true(), false); // ensure duplicates are ignored

		assert_param_missing!(obj.call_attr(EQL, &[]));

		// now make sure modifying `===` will correctly redirect it
		let ref obj = StrictlyEqAlways::new_any();
		let ref obj_clone = obj.clone();
		let ref obj_duplicate = StrictlyEqAlways::new_any();

		assert_eq!(obj.call_attr(EQL, &[obj])?.downcast_or_err::<Boolean>()?.is_true(), true);
		assert_eq!(obj.call_attr(EQL, &[obj_clone])?.downcast_or_err::<Boolean>()?.is_true(), true);
		assert_eq!(obj.call_attr(EQL, &[obj_duplicate])?.downcast_or_err::<Boolean>()?.is_true(), true);

		// now make sure that you can overwrite `==` correctly
		let ref obj = EqNeverStrictCrash::new_any();
		let ref obj_clone = obj.clone();
		let ref obj_duplicate = EqNeverStrictCrash::new_any();

		assert_eq!(obj.call_attr(EQL, &[obj])?.downcast_or_err::<Boolean>()?.is_true(), false);
		assert_eq!(obj.call_attr(EQL, &[obj_clone])?.downcast_or_err::<Boolean>()?.is_true(), false);
		assert_eq!(obj.call_attr(EQL, &[obj_duplicate])?.downcast_or_err::<Boolean>()?.is_true(), false);

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

		// make sure that modifying `!==` won't affect it
		let ref obj = StrictlyNeqAlways::new_any();
		let ref obj_clone = obj.clone();
		let ref obj_duplicate = StrictlyNeqAlways::new_any();

		assert_eq!(obj.call_attr(NEQ, &[obj])?.downcast_or_err::<Boolean>()?.is_true(), false);
		assert_eq!(obj.call_attr(NEQ, &[obj_clone])?.downcast_or_err::<Boolean>()?.is_true(), false);
		assert_eq!(obj.call_attr(NEQ, &[obj_duplicate])?.downcast_or_err::<Boolean>()?.is_true(), true);

		// now make sure that you can overwrite `==` correctly

		let ref obj = EqNeverStrictCrash::new_any();
		let ref obj_clone = obj.clone();
		let ref obj_duplicate = EqNeverStrictCrash::new_any();

		assert_eq!(obj.call_attr(NEQ, &[obj])?.downcast_or_err::<Boolean>()?.is_true(), true);
		assert_eq!(obj.call_attr(NEQ, &[obj_clone])?.downcast_or_err::<Boolean>()?.is_true(), true);
		assert_eq!(obj.call_attr(NEQ, &[obj_duplicate])?.downcast_or_err::<Boolean>()?.is_true(), true);

		Ok(())
	}
	
	#[test]
	#[ignore]
	fn arrow_right() -> Result<()> {
		unimplemented!("this should be integration");

		let ref obj1 = BlankObject::new_any();
		let ref obj2 = BlankObject::new_any();
		// first make sure the arrow returns an error if it doesnt exist
		match funcs::arrow_right(obj1, &[obj2]).unwrap_err() {
			Error::AttrMissing { attr, obj } => {
				assert!(obj.id_eq(&obj2));
				assert_eq!(*attr.downcast_or_err::<Variable>()?.unwrap_data(), ARROW_LEFT);
			},
			_ => panic!("got bad err")
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
		assert!(funcs::arrow_right(obj1, &[cantake, obj2])?.is_null()); // also to ensure extra args are ignored
		let (obj, arg) = RECEIVED.try_lock().unwrap().take().unwrap();
		assert!(cantake.id_eq(&obj), "{:?} != {:?}", cantake.id(), obj.id());
		assert!(arg.id_eq(obj1));
		Ok(())
	}

	#[test]
	#[ignore]
	fn not() -> Result<()> {
		unimplemented!("this should be integration");
		assert_eq!(funcs::not(&BlankObject::new_any(), &[])?.downcast_or_err::<Boolean>()?.is_true(), false);
		assert_eq!(funcs::not(&BlankObject::new_any(), &[&BlankButFalse::new_any()])?.downcast_or_err::<Boolean>()?.is_true(), false);
		assert_eq!(funcs::not(&BlankButFalse::new_any(), &[])?.downcast_or_err::<Boolean>()?.is_true(), true);
		Ok(())
	}

	#[test]
	#[ignore]
	fn and() -> Result<()> {
		unimplemented!("this should be integration");

		let ref t = BlankObject::new_any();
		let ref f = BlankButFalse::new_any();
		let ref e = BooleanError::new_any();
		let ref f2 = BlankButFalse::new_any();
		let ref t2 = BlankObject::new_any();

		assert!(funcs::and(t, &[t])?.id_eq(t));
		assert!(funcs::and(t, &[t2])?.id_eq(t2));
		assert!(funcs::and(t, &[f])?.id_eq(f));
		assert!(funcs::and(t, &[f, e])?.id_eq(f));
		assert!(funcs::and(t, &[e])?.id_eq(e));

		assert!(funcs::and(f, &[t])?.id_eq(f));
		assert!(funcs::and(f, &[f2])?.id_eq(f));
		assert!(funcs::and(f, &[f])?.id_eq(f));
		assert!(funcs::and(f, &[t, e])?.id_eq(f));
		assert!(funcs::and(f, &[e])?.id_eq(f));

		assert!(matches!(funcs::and(e, &[t]).unwrap_err(), Error::__Testing));
		assert!(matches!(funcs::and(e, &[f2]).unwrap_err(), Error::__Testing));
		assert!(matches!(funcs::and(e, &[f]).unwrap_err(), Error::__Testing));
		assert!(matches!(funcs::and(e, &[f, e]).unwrap_err(), Error::__Testing));
		assert!(matches!(funcs::and(e, &[e]).unwrap_err(), Error::__Testing));

		Ok(())
	}


	#[test]
	#[ignore]
	fn or() -> Result<()> {
		unimplemented!("this should be integration");
		let ref t = BlankObject::new_any();
		let ref f = BlankButFalse::new_any();
		let ref e = BooleanError::new_any();
		let ref f2 = BlankButFalse::new_any();

		assert!(funcs::or(t, &[t])?.id_eq(t));
		assert!(funcs::or(t, &[&BlankObject::new_any()])?.id_eq(t));
		assert!(funcs::or(t, &[f])?.id_eq(t));
		assert!(funcs::or(t, &[f, e])?.id_eq(t));
		assert!(funcs::or(t, &[e])?.id_eq(t));

		assert!(funcs::or(f, &[t])?.id_eq(t));
		assert!(funcs::or(f, &[f2])?.id_eq(f2));
		assert!(funcs::or(f, &[f])?.id_eq(f));
		assert!(funcs::or(f, &[t, e])?.id_eq(t));
		assert!(funcs::or(f, &[e])?.id_eq(e));

		assert!(matches!(funcs::or(e, &[t]).unwrap_err(), Error::__Testing));
		assert!(matches!(funcs::or(e, &[f2]).unwrap_err(), Error::__Testing));
		assert!(matches!(funcs::or(e, &[f]).unwrap_err(), Error::__Testing));
		assert!(matches!(funcs::or(e, &[f, e]).unwrap_err(), Error::__Testing));
		assert!(matches!(funcs::or(e, &[e]).unwrap_err(), Error::__Testing));

		Ok(())
	}
}
*/


// #[cfg(test)]
// mod tests {
// 	use super::*;
// 	use crate::err::{Result, Error};
// 	use crate::object::types::{Boolean, Variable};

// 	define_blank!(struct BlankObject;);
// 	define_blank!(struct BlankButFalse, BLANK_BUT_FALSE_MAP;
// 		AT_BOOL => |_, _| Ok(Object::new_boolean(false))
// 	);

// 	define_blank!(struct BooleanError, BOOLEAN_ERROR;
// 		AT_BOOL => |_, _| Err(Error::__Testing)
// 	);


// 	// Object are strictly equal if they have the same pointer
// 	#[test]
// 	fn strict_eql() -> Result<()> {
// 		let ref obj = BlankObject::new_any();
// 		let ref obj_clone = obj.clone();
// 		let ref obj_duplicate = BlankObject::new_any();

// 		assert_eq!(funcs::strict_eql(obj, &[obj])?.downcast_or_err::<Boolean>()?.is_true(), true);
// 		assert_eq!(funcs::strict_eql(obj, &[obj_clone])?.downcast_or_err::<Boolean>()?.is_true(), true);
// 		assert_eq!(funcs::strict_eql(obj, &[obj_duplicate])?.downcast_or_err::<Boolean>()?.is_true(), false);
// 		assert_eq!(funcs::strict_eql(obj, &[obj_duplicate, obj])?.downcast_or_err::<Boolean>()?.is_true(), false); // ensure duplicates are ignored

// 		assert_param_missing!(funcs::strict_eql(obj, &[]));
// 		Ok(())
// 	}

// 	#[test]
// 	fn strict_neq() -> Result<()> {
// 		let ref obj = BlankObject::new_any();
// 		let ref obj_clone = obj.clone();
// 		let ref obj_duplicate = BlankObject::new_any();

// 		// make sure `!==` can override correctly
// 		assert_eq!(funcs::strict_neq(obj, &[obj])?.downcast_or_err::<Boolean>()?.is_true(), false);
// 		assert_eq!(funcs::strict_neq(obj, &[obj_clone])?.downcast_or_err::<Boolean>()?.is_true(), false);
// 		assert_eq!(funcs::strict_neq(obj, &[obj_duplicate])?.downcast_or_err::<Boolean>()?.is_true(), true);
// 		assert_eq!(funcs::strict_neq(obj, &[obj_duplicate, obj])?.downcast_or_err::<Boolean>()?.is_true(), true); // ensure duplciates are ignored

// 		assert_param_missing!(funcs::strict_neq(obj, &[]));
// 		Ok(())
// 	}

// 	#[test]
// 	fn at_bool() -> Result<()> {
// 		let ref obj = BlankObject::new_any();
// 		assert_eq!(funcs::at_bool(obj, &[])?.downcast_or_err::<Boolean>()?.is_true(), true);
// 		assert_eq!(funcs::at_bool(obj, &[&BlankButFalse::new_any()])?.downcast_or_err::<Boolean>()?.is_true(), true);

// 		Ok(())
// 	}

// 	#[test]
// 	#[ignore]
// 	fn at_text() { unimplemented!("{}", AT_TEXT) }

// 	#[test]
// 	#[ignore]
// 	fn clone() { unimplemented!("{}", L_CLONE); }

// 	#[test]
// 	fn eql() -> Result<()> {
// 		let ref obj = BlankObject::new_any();
// 		let ref obj_clone = obj.clone();
// 		let ref obj_duplicate = BlankObject::new_any();

// 		assert_eq!(funcs::eql(obj, &[obj])?.downcast_or_err::<Boolean>()?.is_true(), true);
// 		assert_eq!(funcs::eql(obj, &[obj_clone])?.downcast_or_err::<Boolean>()?.is_true(), true);
// 		assert_eq!(funcs::eql(obj, &[obj_duplicate])?.downcast_or_err::<Boolean>()?.is_true(), false);
// 		assert_eq!(funcs::eql(obj, &[obj_duplicate, obj])?.downcast_or_err::<Boolean>()?.is_true(), false); // ensure duplicates are ignored

// 		assert_param_missing!(funcs::eql(obj, &[]));

// 		/* we don't need to test to see that `===` is called; we only do if we define `==` to default to that. */
// 		// use std::sync::atomic::{AtomicBool, Ordering::Relaxed};
// 		// lazy_static! {
// 		// 	static ref STRICT_EQUALITY_CALLED: AtomicBool = AtomicBool::new(false);
// 		// }

// 		// define_blank!(struct StrictEqChanged, STRICT_EQ_BLANKMAP;
// 		// 	STRICT_EQL => |_, _| {
// 		// 		assert_eq!(STRICT_EQUALITY_CALLED.swap(true, Relaxed), false);
// 		// 		Ok(Object::new_boolean(true)) // result's always true, to test `!=`
// 		// 	}	
// 		// );

// 		// assert_eq!(STRICT_EQUALITY_CALLED.load(Relaxed), false);
// 		// // note: extra argument isn't needed here, as the funciton ignores it
// 		// assert_eq!(funcs::eql(&StrictEqChanged::new_any(), &[])?.downcast_or_err::<Boolean>()?.is_true(), true);
// 		// assert_eq!(STRICT_EQUALITY_CALLED.swap(false, Relaxed), true);
// 		// assert_eq!(funcs::eql(&StrictEqChanged::new_any(), &[])?.downcast_or_err::<Boolean>()?.is_true(), false);
// 		// assert_eq!(STRICT_EQUALITY_CALLED.swap(false, Relaxed), true);
// 		Ok(())
// 	}

// 	#[test]
// 	fn neq() -> Result<()> {
// 		let ref obj = BlankObject::new_any();
// 		let ref obj_clone = obj.clone();
// 		let ref obj_duplicate = BlankObject::new_any();

// 		assert_eq!(funcs::neq(obj, &[obj])?.downcast_or_err::<Boolean>()?.is_true(), false);
// 		assert_eq!(funcs::neq(obj, &[obj_clone])?.downcast_or_err::<Boolean>()?.is_true(), false);
// 		assert_eq!(funcs::neq(obj, &[obj_duplicate])?.downcast_or_err::<Boolean>()?.is_true(), true);
// 		assert_eq!(funcs::neq(obj, &[obj_duplicate, obj])?.downcast_or_err::<Boolean>()?.is_true(), true); // ensure duplciates are ignored

// 		assert_param_missing!(funcs::neq(obj, &[]));
// 		Ok(())
// 	}

// 	#[test]
// 	fn arrow_right() -> Result<()> {
// 		let ref obj1 = BlankObject::new_any();
// 		let ref obj2 = BlankObject::new_any();
// 		// first make sure the arrow returns an error if it doesnt exist
// 		match funcs::arrow_right(obj1, &[obj2]).unwrap_err() {
// 			Error::AttrMissing { attr, obj } => {
// 				assert!(obj.id_eq(&obj2));
// 				assert_eq!(*attr.downcast_or_err::<Variable>()?.unwrap_data(), ARROW_LEFT);
// 			},
// 			_ => panic!("got bad err")
// 		}

// 		lazy_static! {
// 			static ref RECEIVED: std::sync::Mutex<Option<(AnyObject, AnyObject)>> = std::sync::Mutex::new(None);
// 		}

// 		// now make sure it correctly reroutes
// 		define_blank!(struct CanTakeArrow, CAN_TAKE_ARROW;
// 			ARROW_LEFT => |obj, args| {
// 				*RECEIVED.try_lock().unwrap() = Some((obj.clone(), getarg!(args[0])?.clone()));
// 				Ok(Object::new_null())
// 			}
// 		);

// 		assert!(RECEIVED.try_lock().unwrap().is_none());
// 		let ref cantake = CanTakeArrow::new_any();
// 		assert!(funcs::arrow_right(obj1, &[cantake, obj2])?.is_null()); // also to ensure extra args are ignored
// 		let (obj, arg) = RECEIVED.try_lock().unwrap().take().unwrap();
// 		assert!(cantake.id_eq(&obj), "{:?} != {:?}", cantake.id(), obj.id());
// 		assert!(arg.id_eq(obj1));
// 		Ok(())
// 	}

// 	#[test]
// 	fn not() -> Result<()> {
// 		assert_eq!(funcs::not(&BlankObject::new_any(), &[])?.downcast_or_err::<Boolean>()?.is_true(), false);
// 		assert_eq!(funcs::not(&BlankObject::new_any(), &[&BlankButFalse::new_any()])?.downcast_or_err::<Boolean>()?.is_true(), false);
// 		assert_eq!(funcs::not(&BlankButFalse::new_any(), &[])?.downcast_or_err::<Boolean>()?.is_true(), true);
// 		Ok(())
// 	}

// 	#[test]
// 	fn and() -> Result<()> {
// 		let ref t = BlankObject::new_any();
// 		let ref f = BlankButFalse::new_any();
// 		let ref e = BooleanError::new_any();
// 		let ref f2 = BlankButFalse::new_any();
// 		let ref t2 = BlankObject::new_any();

// 		assert!(funcs::and(t, &[t])?.id_eq(t));
// 		assert!(funcs::and(t, &[t2])?.id_eq(t2));
// 		assert!(funcs::and(t, &[f])?.id_eq(f));
// 		assert!(funcs::and(t, &[f, e])?.id_eq(f));
// 		assert!(funcs::and(t, &[e])?.id_eq(e));

// 		assert!(funcs::and(f, &[t])?.id_eq(f));
// 		assert!(funcs::and(f, &[f2])?.id_eq(f));
// 		assert!(funcs::and(f, &[f])?.id_eq(f));
// 		assert!(funcs::and(f, &[t, e])?.id_eq(f));
// 		assert!(funcs::and(f, &[e])?.id_eq(f));

// 		assert!(matches!(funcs::and(e, &[t]).unwrap_err(), Error::__Testing));
// 		assert!(matches!(funcs::and(e, &[f2]).unwrap_err(), Error::__Testing));
// 		assert!(matches!(funcs::and(e, &[f]).unwrap_err(), Error::__Testing));
// 		assert!(matches!(funcs::and(e, &[f, e]).unwrap_err(), Error::__Testing));
// 		assert!(matches!(funcs::and(e, &[e]).unwrap_err(), Error::__Testing));

// 		Ok(())
// 	}


// 	#[test]
// 	fn or() -> Result<()> {
// 		let ref t = BlankObject::new_any();
// 		let ref f = BlankButFalse::new_any();
// 		let ref e = BooleanError::new_any();
// 		let ref f2 = BlankButFalse::new_any();

// 		assert!(funcs::or(t, &[t])?.id_eq(t));
// 		assert!(funcs::or(t, &[&BlankObject::new_any()])?.id_eq(t));
// 		assert!(funcs::or(t, &[f])?.id_eq(t));
// 		assert!(funcs::or(t, &[f, e])?.id_eq(t));
// 		assert!(funcs::or(t, &[e])?.id_eq(t));

// 		assert!(funcs::or(f, &[t])?.id_eq(t));
// 		assert!(funcs::or(f, &[f2])?.id_eq(f2));
// 		assert!(funcs::or(f, &[f])?.id_eq(f));
// 		assert!(funcs::or(f, &[t, e])?.id_eq(t));
// 		assert!(funcs::or(f, &[e])?.id_eq(e));

// 		assert!(matches!(funcs::or(e, &[t]).unwrap_err(), Error::__Testing));
// 		assert!(matches!(funcs::or(e, &[f2]).unwrap_err(), Error::__Testing));
// 		assert!(matches!(funcs::or(e, &[f]).unwrap_err(), Error::__Testing));
// 		assert!(matches!(funcs::or(e, &[f, e]).unwrap_err(), Error::__Testing));
// 		assert!(matches!(funcs::or(e, &[e]).unwrap_err(), Error::__Testing));

// 		Ok(())
// 	}

// 	mod integration {
// 		use super::*;
// 		use crate::object::types::Text;

// 		define_blank!(struct StrictlyEqAlways, STRICTLY_EQ_ALWAYS;
// 			STRICT_EQL => |_, _| Ok(Object::new_boolean(true))
// 		);

// 		define_blank!(struct StrictlyNeqAlways, STRICTLY_EQ_NEVER; 
// 			STRICT_NEQ => |_, _| Ok(Object::new_boolean(true))
// 		);

// 		define_blank!(struct EqNeverStrictCrash, EQ_NEVER_STRICT_CRASH;
// 			EQL => |_, _| Ok(Object::new_boolean(false)),
// 			STRICT_EQL => |_, _| unreachable!(),
// 			STRICT_NEQ => |_, _| unreachable!()
// 		);
// 		define_blank!(struct NeqNeverStrictCrash, NEQ_NEVER_STRICT_CRASH;
// 			NEQ => |_, _| Ok(Object::new_boolean(false)),
// 			EQL => |_, _| unreachable!(),
// 			STRICT_EQL => |_, _| unreachable!(),
// 			STRICT_NEQ => |_, _| unreachable!()
// 		);


// 		// Object are strictly equal if they have the same pointer
// 		#[test]
// 		fn strict_eql() -> Result<()> {
// 			// first check to see that the default `===` does what we expect
// 			let ref obj = BlankObject::new_any();
// 			let ref obj_clone = obj.clone();
// 			let ref obj_duplicate = BlankObject::new_any();

// 			assert_eq!(obj.call_attr(STRICT_EQL, &[obj])?.downcast_or_err::<Boolean>()?.is_true(), true);
// 			assert_eq!(obj.call_attr(STRICT_EQL, &[obj_clone])?.downcast_or_err::<Boolean>()?.is_true(), true);
// 			assert_eq!(obj.call_attr(STRICT_EQL, &[obj_duplicate])?.downcast_or_err::<Boolean>()?.is_true(), false);
// 			assert_eq!(obj.call_attr(STRICT_EQL, &[obj_duplicate, obj])?.downcast_or_err::<Boolean>()?.is_true(), false); // ensure duplicates are ignored

// 			assert_param_missing!(obj.call_attr(STRICT_EQL, &[]));

// 			// and now see if we can override it
// 			let ref obj = StrictlyEqAlways::new_any();
// 			let ref obj_clone = obj.clone();
// 			let ref obj_duplicate = StrictlyEqAlways::new_any();

// 			assert_eq!(obj.call_attr(STRICT_EQL, &[obj])?.downcast_or_err::<Boolean>()?.is_true(), true);
// 			assert_eq!(obj.call_attr(STRICT_EQL, &[obj_clone])?.downcast_or_err::<Boolean>()?.is_true(), true);
// 			assert_eq!(obj.call_attr(STRICT_EQL, &[obj_duplicate])?.downcast_or_err::<Boolean>()?.is_true(), true);

// 			Ok(())
// 		}

// 		#[test]
// 		fn strict_neq() -> Result<()> {
// 			// first check to see that the default `!==` does what we expect
// 			let ref obj = BlankObject::new_any();
// 			let ref obj_clone = obj.clone();
// 			let ref obj_duplicate = BlankObject::new_any();

// 			assert_eq!(obj.call_attr(STRICT_NEQ, &[obj])?.downcast_or_err::<Boolean>()?.is_true(), false);
// 			assert_eq!(obj.call_attr(STRICT_NEQ, &[obj_clone])?.downcast_or_err::<Boolean>()?.is_true(), false);
// 			assert_eq!(obj.call_attr(STRICT_NEQ, &[obj_duplicate])?.downcast_or_err::<Boolean>()?.is_true(), true);
// 			assert_eq!(obj.call_attr(STRICT_NEQ, &[obj_duplicate, obj])?.downcast_or_err::<Boolean>()?.is_true(), true); // ensure duplicates are ignored

// 			assert_param_missing!(obj.call_attr(STRICT_NEQ, &[]));

// 			// make sure `===` can override correctly and `!==` correctly deals with it
// 			let ref obj = StrictlyEqAlways::new_any();
// 			let ref obj_clone = obj.clone();
// 			let ref obj_duplicate = StrictlyEqAlways::new_any();

// 			assert_eq!(obj.call_attr(STRICT_NEQ, &[obj])?.downcast_or_err::<Boolean>()?.is_true(), false);
// 			assert_eq!(obj.call_attr(STRICT_NEQ, &[obj_clone])?.downcast_or_err::<Boolean>()?.is_true(), false);
// 			assert_eq!(obj.call_attr(STRICT_NEQ, &[obj_duplicate])?.downcast_or_err::<Boolean>()?.is_true(), false);

// 			// and now for `!==` being overriden directly
// 			let ref obj = StrictlyNeqAlways::new_any();
// 			let ref obj_clone = obj.clone();
// 			let ref obj_duplicate = StrictlyNeqAlways::new_any();

// 			assert_eq!(obj.call_attr(STRICT_NEQ, &[obj])?.downcast_or_err::<Boolean>()?.is_true(), true);
// 			assert_eq!(obj.call_attr(STRICT_NEQ, &[obj_clone])?.downcast_or_err::<Boolean>()?.is_true(), true);
// 			assert_eq!(obj.call_attr(STRICT_NEQ, &[obj_duplicate])?.downcast_or_err::<Boolean>()?.is_true(), true);

// 			Ok(())
// 		}

// 		#[test]
// 		fn at_bool() -> Result<()> {
// 			// first check to see that the default `@bool` does what we expect
// 			let ref obj = BlankObject::new_any();
// 			assert_eq!(obj.call_attr(AT_BOOL, &[])?.downcast_or_err::<Boolean>()?.is_true(), true);
// 			assert_eq!(obj.call_attr(AT_BOOL, &[&BlankButFalse::new_any()])?.downcast_or_err::<Boolean>()?.is_true(), true);

// 			// and now see if we can override it correctly
// 			let ref obj = BlankButFalse::new_any();
// 			assert_eq!(obj.call_attr(AT_BOOL, &[])?.downcast_or_err::<Boolean>()?.is_true(), false);
// 			assert_eq!(obj.call_attr(AT_BOOL, &[&BlankObject::new_any()])?.downcast_or_err::<Boolean>()?.is_true(), false);

// 			Ok(())
// 		}

// 		#[test]
// 		fn at_text() -> Result<()> {
// 			// we just want to make sure t he output is overriden, as the default
// 			// output isn't specified
// 			const TEXT: &'static str = "im textually savvy";
// 			define_blank!(struct BlankButText, BLANK_BUT_TEXT; 
// 				AT_TEXT => |_,_| Ok(Object::new_text_str(TEXT))
// 			);

// 			let ref obj = BlankButText::new_any();
// 			assert_eq!(obj.call_attr(AT_TEXT, &[])?.downcast_or_err::<Text>()?.unwrap_data().as_ref(), TEXT);
// 			assert_eq!(obj.call_attr(AT_TEXT, &[&BlankObject::new_any()])?.downcast_or_err::<Text>()?.unwrap_data().as_ref(), TEXT);

// 			Ok(())
// 		}

// 		#[test]
// 		#[ignore]
// 		fn clone() { unimplemented!("{}", L_CLONE); }

// 		#[test]
// 		fn eql() -> Result<()> {
// 			// first check to make sure it works as we'd expect `===` to work
// 			let ref obj = BlankObject::new_any();
// 			let ref obj_clone = obj.clone();
// 			let ref obj_duplicate = BlankObject::new_any();

// 			assert_eq!(obj.call_attr(EQL, &[obj])?.downcast_or_err::<Boolean>()?.is_true(), true);
// 			assert_eq!(obj.call_attr(EQL, &[obj_clone])?.downcast_or_err::<Boolean>()?.is_true(), true);
// 			assert_eq!(obj.call_attr(EQL, &[obj_duplicate])?.downcast_or_err::<Boolean>()?.is_true(), false);
// 			assert_eq!(obj.call_attr(EQL, &[obj_duplicate, obj])?.downcast_or_err::<Boolean>()?.is_true(), false); // ensure duplicates are ignored

// 			assert_param_missing!(obj.call_attr(EQL, &[]));

// 			// now make sure modifying `===` will correctly redirect it
// 			let ref obj = StrictlyEqAlways::new_any();
// 			let ref obj_clone = obj.clone();
// 			let ref obj_duplicate = StrictlyEqAlways::new_any();

// 			assert_eq!(obj.call_attr(EQL, &[obj])?.downcast_or_err::<Boolean>()?.is_true(), true);
// 			assert_eq!(obj.call_attr(EQL, &[obj_clone])?.downcast_or_err::<Boolean>()?.is_true(), true);
// 			assert_eq!(obj.call_attr(EQL, &[obj_duplicate])?.downcast_or_err::<Boolean>()?.is_true(), true);

// 			// now make sure that you can overwrite `==` correctly
// 			let ref obj = EqNeverStrictCrash::new_any();
// 			let ref obj_clone = obj.clone();
// 			let ref obj_duplicate = EqNeverStrictCrash::new_any();

// 			assert_eq!(obj.call_attr(EQL, &[obj])?.downcast_or_err::<Boolean>()?.is_true(), false);
// 			assert_eq!(obj.call_attr(EQL, &[obj_clone])?.downcast_or_err::<Boolean>()?.is_true(), false);
// 			assert_eq!(obj.call_attr(EQL, &[obj_duplicate])?.downcast_or_err::<Boolean>()?.is_true(), false);

// 			Ok(())
// 		}

// 		#[test]
// 		fn neq() -> Result<()> {
// 			// first check to make sure it works as we'd expect `!==` to work
// 			let ref obj = BlankObject::new_any();
// 			let ref obj_clone = obj.clone();
// 			let ref obj_duplicate = BlankObject::new_any();

// 			assert_eq!(obj.call_attr(NEQ, &[obj])?.downcast_or_err::<Boolean>()?.is_true(), false);
// 			assert_eq!(obj.call_attr(NEQ, &[obj_clone])?.downcast_or_err::<Boolean>()?.is_true(), false);
// 			assert_eq!(obj.call_attr(NEQ, &[obj_duplicate])?.downcast_or_err::<Boolean>()?.is_true(), true);
// 			assert_eq!(obj.call_attr(NEQ, &[obj_duplicate, obj])?.downcast_or_err::<Boolean>()?.is_true(), true); // ensure duplicates are ignored

// 			assert_param_missing!(obj.call_attr(NEQ, &[]));

// 			// now make sure modifying `===` will correctly redirect it
// 			let ref obj = StrictlyEqAlways::new_any();
// 			let ref obj_clone = obj.clone();
// 			let ref obj_duplicate = StrictlyEqAlways::new_any();

// 			assert_eq!(obj.call_attr(NEQ, &[obj])?.downcast_or_err::<Boolean>()?.is_true(), false);
// 			assert_eq!(obj.call_attr(NEQ, &[obj_clone])?.downcast_or_err::<Boolean>()?.is_true(), false);
// 			assert_eq!(obj.call_attr(NEQ, &[obj_duplicate])?.downcast_or_err::<Boolean>()?.is_true(), false);

// 			// make sure that modifying `!==` won't affect it
// 			let ref obj = StrictlyNeqAlways::new_any();
// 			let ref obj_clone = obj.clone();
// 			let ref obj_duplicate = StrictlyNeqAlways::new_any();

// 			assert_eq!(obj.call_attr(NEQ, &[obj])?.downcast_or_err::<Boolean>()?.is_true(), false);
// 			assert_eq!(obj.call_attr(NEQ, &[obj_clone])?.downcast_or_err::<Boolean>()?.is_true(), false);
// 			assert_eq!(obj.call_attr(NEQ, &[obj_duplicate])?.downcast_or_err::<Boolean>()?.is_true(), true);

// 			// now make sure that you can overwrite `==` correctly

// 			let ref obj = EqNeverStrictCrash::new_any();
// 			let ref obj_clone = obj.clone();
// 			let ref obj_duplicate = EqNeverStrictCrash::new_any();

// 			assert_eq!(obj.call_attr(NEQ, &[obj])?.downcast_or_err::<Boolean>()?.is_true(), true);
// 			assert_eq!(obj.call_attr(NEQ, &[obj_clone])?.downcast_or_err::<Boolean>()?.is_true(), true);
// 			assert_eq!(obj.call_attr(NEQ, &[obj_duplicate])?.downcast_or_err::<Boolean>()?.is_true(), true);

// 			Ok(())
// 		}
	
// 		#[test]
// 		#[ignore]
// 		fn arrow_right() -> Result<()> {
// 			unimplemented!("this should be integration");

// 			let ref obj1 = BlankObject::new_any();
// 			let ref obj2 = BlankObject::new_any();
// 			// first make sure the arrow returns an error if it doesnt exist
// 			match funcs::arrow_right(obj1, &[obj2]).unwrap_err() {
// 				Error::AttrMissing { attr, obj } => {
// 					assert!(obj.id_eq(&obj2));
// 					assert_eq!(*attr.downcast_or_err::<Variable>()?.unwrap_data(), ARROW_LEFT);
// 				},
// 				_ => panic!("got bad err")
// 			}

// 			lazy_static! {
// 				static ref RECEIVED: std::sync::Mutex<Option<(AnyObject, AnyObject)>> = std::sync::Mutex::new(None);
// 			}

// 			// now make sure it correctly reroutes
// 			define_blank!(struct CanTakeArrow, CAN_TAKE_ARROW;
// 				ARROW_LEFT => |obj, args| {
// 					*RECEIVED.try_lock().unwrap() = Some((obj.clone(), getarg!(args[0])?.clone()));
// 					Ok(Object::new_null())
// 				}
// 			);

// 			assert!(RECEIVED.try_lock().unwrap().is_none());
// 			let ref cantake = CanTakeArrow::new_any();
// 			assert!(funcs::arrow_right(obj1, &[cantake, obj2])?.is_null()); // also to ensure extra args are ignored
// 			let (obj, arg) = RECEIVED.try_lock().unwrap().take().unwrap();
// 			assert!(cantake.id_eq(&obj), "{:?} != {:?}", cantake.id(), obj.id());
// 			assert!(arg.id_eq(obj1));
// 			Ok(())
// 		}

// 		#[test]
// 		#[ignore]
// 		fn not() -> Result<()> {
// 			unimplemented!("this should be integration");
// 			assert_eq!(funcs::not(&BlankObject::new_any(), &[])?.downcast_or_err::<Boolean>()?.is_true(), false);
// 			assert_eq!(funcs::not(&BlankObject::new_any(), &[&BlankButFalse::new_any()])?.downcast_or_err::<Boolean>()?.is_true(), false);
// 			assert_eq!(funcs::not(&BlankButFalse::new_any(), &[])?.downcast_or_err::<Boolean>()?.is_true(), true);
// 			Ok(())
// 		}

// 		#[test]
// 		#[ignore]
// 		fn and() -> Result<()> {
// 			unimplemented!("this should be integration");

// 			let ref t = BlankObject::new_any();
// 			let ref f = BlankButFalse::new_any();
// 			let ref e = BooleanError::new_any();
// 			let ref f2 = BlankButFalse::new_any();
// 			let ref t2 = BlankObject::new_any();

// 			assert!(funcs::and(t, &[t])?.id_eq(t));
// 			assert!(funcs::and(t, &[t2])?.id_eq(t2));
// 			assert!(funcs::and(t, &[f])?.id_eq(f));
// 			assert!(funcs::and(t, &[f, e])?.id_eq(f));
// 			assert!(funcs::and(t, &[e])?.id_eq(e));

// 			assert!(funcs::and(f, &[t])?.id_eq(f));
// 			assert!(funcs::and(f, &[f2])?.id_eq(f));
// 			assert!(funcs::and(f, &[f])?.id_eq(f));
// 			assert!(funcs::and(f, &[t, e])?.id_eq(f));
// 			assert!(funcs::and(f, &[e])?.id_eq(f));

// 			assert!(matches!(funcs::and(e, &[t]).unwrap_err(), Error::__Testing));
// 			assert!(matches!(funcs::and(e, &[f2]).unwrap_err(), Error::__Testing));
// 			assert!(matches!(funcs::and(e, &[f]).unwrap_err(), Error::__Testing));
// 			assert!(matches!(funcs::and(e, &[f, e]).unwrap_err(), Error::__Testing));
// 			assert!(matches!(funcs::and(e, &[e]).unwrap_err(), Error::__Testing));

// 			Ok(())
// 		}


// 		#[test]
// 		#[ignore]
// 		fn or() -> Result<()> {
// 			unimplemented!("this should be integration");
// 			let ref t = BlankObject::new_any();
// 			let ref f = BlankButFalse::new_any();
// 			let ref e = BooleanError::new_any();
// 			let ref f2 = BlankButFalse::new_any();

// 			assert!(funcs::or(t, &[t])?.id_eq(t));
// 			assert!(funcs::or(t, &[&BlankObject::new_any()])?.id_eq(t));
// 			assert!(funcs::or(t, &[f])?.id_eq(t));
// 			assert!(funcs::or(t, &[f, e])?.id_eq(t));
// 			assert!(funcs::or(t, &[e])?.id_eq(t));

// 			assert!(funcs::or(f, &[t])?.id_eq(t));
// 			assert!(funcs::or(f, &[f2])?.id_eq(f2));
// 			assert!(funcs::or(f, &[f])?.id_eq(f));
// 			assert!(funcs::or(f, &[t, e])?.id_eq(t));
// 			assert!(funcs::or(f, &[e])?.id_eq(e));

// 			assert!(matches!(funcs::or(e, &[t]).unwrap_err(), Error::__Testing));
// 			assert!(matches!(funcs::or(e, &[f2]).unwrap_err(), Error::__Testing));
// 			assert!(matches!(funcs::or(e, &[f]).unwrap_err(), Error::__Testing));
// 			assert!(matches!(funcs::or(e, &[f, e]).unwrap_err(), Error::__Testing));
// 			assert!(matches!(funcs::or(e, &[e]).unwrap_err(), Error::__Testing));

// 			Ok(())
// 		}
// 	}
// }






