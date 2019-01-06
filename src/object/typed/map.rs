use crate::Shared;
use crate::collections::Mapping;
use std::fmt::{self, Debug, Display, Formatter};
use lazy_static::lazy_static;

#[derive(Clone)]
pub struct Map(Shared<dyn Mapping>);

impl Map {
	pub fn new(data: Shared<dyn Mapping>) -> Map {
		Map(data)
	}

	pub fn into_inner(self) -> Shared<dyn Mapping> {
		self.0
	}
}

impl Display for Map {
	fn fmt(&self, _f: &mut Formatter) -> fmt::Result {
		unimplemented!("TODO: display for Map");
		/*
		write!(f, "{{")?;
		if !self.0.is_empty() {
			let mut iter = self.0.iter();
			iter.0;
			write!(f, "{}", iter.next().unwrap())?;
			for obj in iter {
				write!(f, ", {}", obj)?;
			}
		}
		write!(f, "}}")*/
	}
}

impl Eq for Map {}
impl PartialEq for Map {
	fn eq(&self, _other: &Map) -> bool {
		unimplemented!("TODO: equal for Map")
	}
}

impl Debug for Map {
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		write!(f, "Map({:?})", self.0)
	}
}



impl_typed_conversion!(Map, Shared<dyn Mapping>);
impl_typed_object!(Map, new_map, downcast_map, is_map);
impl_quest_conversion!("@map" (as_map_obj is_map) (into_map downcast_map) -> Map);

impl_type! { for Map, downcast_fn=downcast_map;
	fn "@map" (this) {
		this.into_object()
	}

	fn "@list" (_this) { todo!(); }
	fn "@bool" (this) {
		(!this.0.is_empty()).into_object()
	}

	fn "==" (this, rhs) {
		(this == rhs.into_map()?).into_object()
	}

	fn "len" (this) {
		this.0.len().into_object()
	}


	fn "+" (_this, _rhs) { todo!("iterable for mapping is needed") }
	fn "-" (_this, _rhs) { todo!("iterable for mapping is needed") }
	fn "union" (_this, _rhs) { todo!("iterable for mapping is needed") }
	fn "intersect" (_this, _rhs) { todo!("iterable for mapping is needed") }
	fn "symmetric_difference" (_this, _rhs) { todo!("iterable for mapping is needed") }

	fn "fetch" (@this, key) {
		this.into_map()?.0.get(key).ok_or_else(|| MissingKey {
			key: key.clone(),
			obj: this.clone()
		})?
	}

	fn "[]" (this, key) {
		this.0.get(key).unwrap_or_else(Object::new_null)
	}

	fn "[]=" (this, key, val) {
		let inner = this.0;
		let res = inner.write().set(key.clone(), val.clone()).unwrap_or_else(Object::new_null);
		drop(inner);
		res
	}

	fn "[]~" (this, key) {
		let inner = this.0;
		let res = inner.write().del(key).unwrap_or_else(Object::new_null);
		drop(inner);
		res
	}

	fn "[]?" (this, key) {
		this.0.has(key).into_object()
	}
}