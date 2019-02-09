use std::fmt::{self, Display, Formatter};
use crate::object::{Object, AnyObject};
use crate::err::{Result, Error};
use std::ops::Deref;

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
		self.call_attr("@list", &[])?
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
	"@list" => |obj, _| unimplemented!(),
	"@bool" => |obj, _| unimplemented!(),
	"==" => |obj, args| unimplemented!(),
	"+" => |obj, args| unimplemented!(),
	"-" => |obj, args| unimplemented!(),
	"*" => |obj, args| unimplemented!(),
	"len" => |obj, _| unimplemented!(),

	"[]" => |obj, args| unimplemented!(),
	"[]=" => |obj, args| unimplemented!(),

	"|" => |obj, args| unimplemented!(), // union
	"&" => |obj, args| unimplemented!(), // intersect
	"^" => |obj, args| unimplemented!(), // symmetric_difference
} 

