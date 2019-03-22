use std::fmt::{self, Display, Formatter};
use crate::object::{Object, AnyObject};
use crate::err::{Result, Error};
use std::ops::Deref;

use crate::object::literal::consts::{
	AT_LIST, AT_TEXT, AT_BOOL, AT_MAP,
	EQL,
	ADD, SUB, MUL,

	INDEX, INDEX_ASSIGN, INDEX_DELETE,
	B_OR, B_AND, B_XOR,
	L_LEN
};

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
		self.call_attr(AT_LIST, &[])?.downcast_or_err::<List>()
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
	type Target = [AnyObject];
	fn deref(&self) -> &[AnyObject] {
		self.0.deref()
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

impl_type! { for List; 
	AT_LIST => |obj, _| unimplemented!(),
	AT_MAP => |obj, _| unimplemented!(),
	AT_BOOL => |obj, _| unimplemented!(),
	AT_TEXT => |obj, _| unimplemented!(),

	EQL => |obj, args| unimplemented!(),
	ADD => |obj, args| unimplemented!(),
	SUB => |obj, args| unimplemented!(),
	MUL => |obj, args| unimplemented!(),
	L_LEN => |obj, _| unimplemented!(),

	INDEX => |obj, args| unimplemented!(),
	INDEX_ASSIGN => |obj, args| unimplemented!(),
	INDEX_DELETE => |obj, args| unimplemented!(),

	B_OR => |obj, args| unimplemented!(), // union
	B_AND => |obj, args| unimplemented!(), // intersect
	B_XOR => |obj, args| unimplemented!(), // symmetric_difference
}








