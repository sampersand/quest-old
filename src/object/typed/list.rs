use crate::{Object, Shared, collections::{self, Listing}};
use std::fmt::{self, Debug, Display, Formatter};
use lazy_static::lazy_static;

#[derive(Clone)]
pub struct List(Shared<dyn Listing>);

impl Eq for List {}
impl PartialEq for List {
	fn eq(&self, other: &List) -> bool {
		self.0.read()._to_vec() == other.0.read()._to_vec()		
	}
}

impl List {
	pub fn new(data: Shared<dyn Listing>) -> List {
		// List(Shared::new(collections::List::new(data)))
		List(data)
	}

	pub fn into_inner(self) -> Vec<Object> {
		self.0.read()._to_vec()
	}
}

impl Display for List {
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		write!(f, "[")?;
		if !self.0.read().is_empty() {
			let mut iter = self.clone().into_inner().into_iter();
			write!(f, "{}", iter.next().unwrap())?;
			for obj in iter {
				write!(f, ", {}", obj)?;
			}
		}
		write!(f, "]")
	}
}

impl Debug for List {
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		write!(f, "List({:?})", self.0)
	}
}

impl crate::object::IntoObject for Vec<Object> {
	fn into_object(self) -> crate::Object {
		crate::object::TypedObject::from(self).objectify()
	}
}

impl From<Vec<Object>> for crate::object::typed::Types {
	fn from(val: Vec<Object>) -> Self {
		crate::object::typed::Types::List(List::new(Shared::new(collections::List::new(val)) as _))
	}
}

impl From<Vec<Object>> for crate::object::TypedObject {
	fn from(obj: Vec<Object>) -> Self {
		List::new(Shared::new(collections::List::new(obj)) as _).into()
	}
}



impl_typed_conversion!(List, Shared<dyn Listing>);
impl_typed_object!(List, new_list, downcast_list, is_list);
impl_quest_conversion!("@list" (as_list_obj is_list) (into_list downcast_list) -> List);

impl_type! { for List, downcast_fn=downcast_list;
	fn "@list" (this) {
		this.into_object()
	}

	fn "@bool" (this) {
		(!this.0.is_empty()).into_object()
	}

	fn "==" (this, rhs) {
		(this == rhs.into_list()?).into_object()
	}

	fn "+" (this, rhs) {
		let mut vec = this.0.read()._to_vec();
		vec.extend_from_slice(&rhs.into_list()?.into_inner());
		vec.into_object()
	}

	fn "-" (this, rhs) {
		let rhs = rhs.into_list()?.into_inner();

		this.into_inner()
			.into_iter()
		    .filter(|ele| !rhs.contains(ele))
		    .collect::<Vec<_>>()
		    .into_object()
	}

	fn "*" (this, rhs) {
		let lim = *rhs.into_num()?.into_integer().as_ref() as isize;
		if lim < 0 {
			return Ok("".to_string().into_object());
		}

		let mut new = Vec::with_capacity(this.0.len() * (lim as usize));
		for _ in 0..lim {
			new.extend_from_slice(&this.0.read()._to_vec());
		}

		new.into_object()
	}

	fn "len" (this) {
		this.0.len().into_object()
	}

	fn "[]" (_this, _index) { todo!() }
	fn "[]=" (_this, _index, _val) { todo!() }
	fn "[]~" (_this, _index) { todo!() }
	fn "[]?" (_this, _index) { todo!() }

	// since `this` is actually a clone, this works
	fn "union" (this, rhs) {
		let mut this = this.into_inner();
		for rhs_ele in rhs.into_list()?.into_inner().into_iter() {
			if !this.contains(&rhs_ele) {
				this.push(rhs_ele);
			}
		}
		this.into_object()
	}

	fn "intersect" (this, rhs) {
		let rhs = rhs.into_list()?.into_inner();
		this.into_inner().into_iter()
		    .filter(|x| rhs.contains(x))
		    .collect::<Vec<_>>()
		    .into_object()
	}

	fn "symmetric_difference" (this, rhs) {
		let rhs = rhs.into_list()?.into_inner();
		let this = this.into_inner();
		this.clone()
		    .into_iter()
		    .filter(|ele| !rhs.contains(ele))
		    .chain(rhs.clone()
		              .into_iter()
		              .filter(|ele| !this.contains(ele)))
		    .collect::<Vec<_>>()
		    .into_object()
	}
}