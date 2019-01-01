use crate::Object;
use std::fmt::{self, Debug, Display, Formatter};
use lazy_static::lazy_static;

#[derive(Clone, PartialEq, Eq, Default)]
pub struct List(Vec<Object>);

impl List {
	pub fn new(data: Vec<Object>) -> List {
		List(data)
	}

	pub fn into_inner(self) -> Vec<Object> {
		self.0
	}
}

impl Display for List {
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		write!(f, "[")?;
		if !self.0.is_empty() {
			let mut iter = self.0.iter();
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



impl_typed_conversion!(List, Vec<Object>);
impl_typed_object!(List, new_list, downcast_list, is_list);
impl_quest_conversion!("@list" (as_list_obj is_list) (into_list downcast_list) -> List);

impl_type! { for List, downcast_fn=downcast_list;
	fn "@bool" (this) {
		(!this.0.is_empty()).into_object()
	}

	fn "+" (this, rhs) {
		let mut this = this;
		this.0.extend_from_slice(&rhs.into_list()?.0);
		this.into_object()
	}

	fn "-" (this, rhs) {
		let rhs = rhs.into_list()?.0;
		this.0.into_iter()
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
			new.extend_from_slice(&this.0);
		}

		new.into_object()
	}

	fn "len" (this) {
		this.0.len().into_object()
	}

	fn "get" (_this, _index) { todo!() }
	fn "set" (_this, _index, _val) { todo!() }
	fn "has" (_this, _index) { todo!() }
	fn "del" (_this, _index) { todo!() }

	// since `this` is actually a clone, this works
	fn "union" (this, rhs) {
		let mut this = this.0;
		for rhs_ele in rhs.into_list()?.0.into_iter() {
			if !this.contains(&rhs_ele) {
				this.push(rhs_ele);
			}
		}
		this.into_object()
	}

	fn "intersect" (this, rhs) {
		let rhs = rhs.into_list()?.0;
		this.0.into_iter()
		    .filter(|x| rhs.contains(x))
		    .collect::<Vec<_>>()
		    .into_object()
	}

	fn "symmetric_difference" (this, rhs) {
		let rhs = rhs.into_list()?.0;
		this.0.clone()
		    .into_iter()
		    .filter(|ele| !rhs.contains(ele))
		    .chain(rhs.clone()
		              .into_iter()
		              .filter(|ele| !this.0.contains(ele)))
		    .collect::<Vec<_>>()
		    .into_object()
	}
}




