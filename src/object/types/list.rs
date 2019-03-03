use std::fmt::{self, Display, Formatter};
use crate::object::{Object, AnyObject};
use crate::err::{Result, Error};
use std::ops::Deref;

use super::quest_funcs::{
	AT_LIST, AT_TEXT, AT_BOOL,
	EQ,
	ADD, SUB, MUL,
	INDEX, INDEX_ASSIGN, INDEX_DELETE,
	BW_OR, BW_AND, BW_XOR,
	MISC_LEN
};

#[derive(Debug, PartialEq, Clone, Default, Hash)]
pub struct List(Vec<AnyObject>);

impl List {
	#[inline]
	pub fn new(list: Vec<AnyObject>) -> List {
		List(list)
	}
}

impl Object<List> {
	pub fn new_list(list: Vec<AnyObject>) -> Object<List> {
		Object::new(List::new(list))
	}
}

impl AnyObject {
	pub fn to_list(&self) -> Result<Object<List>> {
		self.call_attr(AT_LIST, &[])?
			.downcast_or_err::<List>()
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

impl From<Vec<AnyObject>> for List {
	fn from(list: Vec<AnyObject>) -> List {
		List::new(list)
	}
}

impl From<List> for Vec<AnyObject> {
	fn from(list: List) -> Vec<AnyObject> {
		list.0
	}
}

impl_type! { for List; 
	AT_LIST => |obj, _| unimplemented!(),
	AT_BOOL => |obj, _| unimplemented!(),
	EQ => |obj, args| unimplemented!(),
	ADD => |obj, args| unimplemented!(),
	SUB => |obj, args| unimplemented!(),
	MUL => |obj, args| unimplemented!(),
	MISC_LEN => |obj, _| unimplemented!(),

	INDEX => |obj, args| unimplemented!(),
	INDEX_ASSIGN => |obj, args| unimplemented!(),
	INDEX_DELETE => |obj, args| unimplemented!(),

	BW_OR => |obj, args| unimplemented!(), // union
	BW_AND => |obj, args| unimplemented!(), // intersect
	BW_XOR => |obj, args| unimplemented!(), // symmetric_difference
} 

