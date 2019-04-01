use std::fmt::{self, Display, Formatter};
use crate::object::{literals, Object, AnyObject};
use crate::error::{Result, Error};
use std::ops::{Deref, DerefMut};

type ObjList = Vec<AnyObject>;

#[derive(Debug, PartialEq, Clone, Default, Hash)]
pub struct List(ObjList);

impl List {
	#[inline]
	pub fn new(list: ObjList) -> List {
		List(list)
	}

	#[inline]
	pub fn empty() -> List {
		List::default()
	}
}

impl Object<List> {
	pub fn new_list<T: Into<List>>(list: T) -> Object<List> {
		Object::new(list.into())
	}
}

impl AnyObject {
	pub fn to_list(&self) -> Result<Object<List>> {
		self.call_attr(literals::AT_LIST, &[])?.downcast_or_err::<List>()
	}
}

impl PartialEq<List> for Object<List> {
	fn eq(&self, rhs: &List) -> bool {
		self.data().read().expect("read error in Object<List>::eq").as_ref() == rhs.as_ref()
	}
}

impl AsRef<[AnyObject]> for List {
	fn as_ref(&self) -> &[AnyObject] {
		self.0.as_ref()
	}
}

impl Deref for List {
	type Target = Vec<AnyObject>;
	fn deref(&self) -> &Vec<AnyObject> {
		&self.0
	}
}

impl DerefMut for List {
	fn deref_mut(&mut self) -> &mut Vec<AnyObject> {
		&mut self.0
	}
}

impl From<ObjList> for List {
	fn from(list: ObjList) -> List {
		List::new(list)
	}
}

impl From<List> for ObjList {
	fn from(list: List) -> ObjList {
		list.0
	}
}

mod funcs {
	use super::{List, ObjList};
	use crate::object::types::{Number, Text, Map, Boolean};
	use crate::object::{AnyObject, Object, literals};
	use crate::error::Result;

	pub fn at_list(list: &Object<List>) -> Object<List> {
		list.duplicate()
	}

	pub fn at_map(list: &Object<List>) -> Result<Object<Map>> {
		unimplemented!()
	}

	pub fn at_bool(list: &Object<List>) -> Object<Boolean> {
		Object::new_boolean(!list.data().read().expect("read err in List::funcs::at_bool").is_empty())
	}

	pub fn at_text(list: &Object<List>) -> Result<Object<Text>> {
		let list = list.data()
			.read().expect("read err in list::funcs::at_text")
			.iter().map(Object::to_text).collect::<Result<Vec<_>>>()?;

		let mut text = String::with_capacity(2 + 3 * list.len());
		text.push('[');

		if !list.is_empty() {
			text.push_str(list[0].data().read().expect("read err in List::funcs::at_text").as_ref());
			for ele in &list[1..] {
				text.push_str(", ");
				text.push_str(ele.data().read().expect("read err in List::funcs::at_text").as_ref());
			}
		}

		text.push(']');
		text.shrink_to_fit();
		Ok(Object::new_text(text))
	}

	pub fn eql(list: &Object<List>, rhs: &Object<List>) -> Result<Object<Boolean>> {
		let list = list.data().read().expect("read err in List::funcs::eql");
		let rhs = rhs.data().read().expect("read err in List::funcs::eql");
		if list.len() != rhs.len() {
			return Ok(Object::new_boolean(false));
		}
		for i in 0..list.len() {
			if list[i].call_attr(literals::EQL, &[&rhs[i]])?.to_boolean()? == false {
				return Ok(Object::new_boolean(false))
			}
		}
		Ok(Object::new_boolean(true))
	}

	pub fn add(list: &Object<List>, rhs: &Object<List>) -> Object<List> {
		let list = list.data().read().expect("read err in List::funcs::add");
		let rhs = rhs.data().read().expect("read err in List::funcs::add");
		let mut sum = ObjList::with_capacity(list.len() + rhs.len());
		sum.extend_from_slice(&list);
		sum.extend_from_slice(&rhs);
		Object::new_list(sum)
	}

	pub fn add_assign(list: &Object<List>, rhs: &Object<List>) -> Object<List> {
		if list.id_eq(rhs) {
			// ie adding list to itself
			// let ref mut list = &mut *list.data().write().expect("write err in List::funcs::add_assign");
			// list.0.extend_from_slice(&list);
			unimplemented!()
		} else {
			let ref mut list = &mut *list.data().write().expect("write err in List::funcs::add_assign");
			list.0.extend_from_slice(&rhs.data().read().expect("read err in List::funcs::add_assign").as_ref());
		}
		list.clone()
	}

	pub fn mul(list: &Object<List>, rhs: &Object<Number>) -> Object<List> {
		unimplemented!();
	}

	pub fn mul_assign(list: &Object<List>, rhs: &Object<Number>) -> Object<List> {
		unimplemented!();
	}

	pub fn len(list: &Object<List>) -> Object<Number> {
		Object::new_number(list.data().read().expect("read err in List::funcs::len").len() as f64)
	}

	pub fn push(list: &Object<List>, obj: AnyObject) -> Object<List> {
		list.data().write().expect("write err in List::funcs::push")
			.push(obj);
		list.clone()
	}

	pub fn pop(list: &Object<List>) -> AnyObject {
		list.data().write().expect("write err in List::funcs::pop")
			.pop()
			.unwrap_or_else(|| Object::new_null() as _)
	}
}

impl_type! { for List;
	literals::AT_LIST => |l, _| Ok(funcs::at_list(l)),
	literals::AT_MAP => |l, _| Ok(funcs::at_map(l)?),
	literals::AT_BOOL => |l, _| Ok(funcs::at_bool(l)),
	literals::AT_TEXT => |l, _| Ok(funcs::at_text(l)?),

	literals::EQL => |l, a| Ok(funcs::eql(l, &__getarg!(a[0]: List)?)?),
	literals::ADD => |l, a| Ok(funcs::add(l, &__getarg!(a[0] @@ to_list)?)),
	literals::ADD_ASSIGN => |l, a| Ok(funcs::add_assign(l, &__getarg!(a[0] @@ to_list)?)),
	literals::MUL => |l, a| Ok(funcs::mul(l, &__getarg!(a[0] @@ to_number)?)),
	literals::MUL_ASSIGN => |l, a| Ok(funcs::mul_assign(l, &__getarg!(a[0] @@ to_number)?)),
	literals::L_LEN => |l, _| Ok(funcs::len(l)),

	literals::L_PUSH => |l, a| Ok(funcs::push(l, getarg!(a[0])?.clone())),
	literals::L_POP => |l, _| Ok(funcs::pop(l))


	// literals::INDEX => |o, a| funcs::index(o, &__getarg!(a[0])),
	// literals::INDEX_ASSIGN => |o, a| funcs::index_assign(o, &__getarg!(a[0])?),
	// literals::INDEX_DELETE => |o, a| funcs::index_delete(o, &__getarg!(a[0])?),

	// literals::B_OR => |o, a| Ok(funcs::b_or(o, &__getarg!(a[0] @@ to_list)?)), // union
	// literals::B_AND => |o, a| Ok(funcs::b_and(o, &__getarg!(a[0] @@ to_list)?)), // intersect
	// literals::B_XOR => |o, a| Ok(funcs::b_xor(o, &__getarg!(a[0] @@ to_list)?)), // symmetric_difference
}




