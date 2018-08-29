use std::ops::{Deref, DerefMut};
use std::hash::{Hash, Hasher};
use std::collections::{HashMap, hash_map::{DefaultHasher, Entry}};
use std::fmt::{self, Display, Formatter};
use obj::{AnyShared, SharedObject, types::{IntoObject, Number}};

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct Map(HashMap<AnyShared, AnyShared>);

impl Hash for Map {
	fn hash<H: Hasher>(&self, h: &mut H) {
		let mut keys = self.0.keys().collect::<Vec<_>>();
		keys.sort_by(|x, y| {
			let mut hx = DefaultHasher::new();
			let mut hy = DefaultHasher::new();
			x.hash(&mut hx);
			y.hash(&mut hy);
			hx.finish().cmp(&hy.finish())
		});

		for key in keys {
			self.get(key).unwrap().hash(h);
		}
	}
}

impl Deref for Map {
	type Target = HashMap<AnyShared, AnyShared>;

	#[inline]
	fn deref(&self) -> &HashMap<AnyShared, AnyShared> {
		&self.0
	}
}

impl DerefMut for Map {
	#[inline]
	fn deref_mut(&mut self) -> &mut HashMap<AnyShared, AnyShared> {
		&mut self.0
	}
}

impl IntoObject for HashMap<AnyShared, AnyShared> {
	type Type = Map;
	fn into_object(self) -> SharedObject<Map> {
		Map(self).into_object()
	}
}

impl Display for Map {
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		write!(f, "{{{}}}", self.0.iter().map(|(k, v)| format!("{}: {}", k, v)).collect::<Vec<_>>().join(", "))

	}
}

impl_type! {
	for Map, with self attr;

	fn "@bool" (this) {
		Ok((!this.is_empty()).into_object())
	}

	fn "len" (this) {
		Ok(this.len().into_object())
	}

	fn "has" (this, key) {
		Ok(this.0.keys().any(|k| *k.read() == *key).into_object())
	}

	fn "get" (this, shared key) {
		this.get(key).map(Clone::clone).ok_or_else(|| panic!("TODO: error for doesnt exist"))
	}

	fn "set" (mut this, shared key, shared val) {
		Ok(this.insert(key.clone(), val.clone()).unwrap_or_else(|| Object::null()))
	}

	fn "del" (mut this, shared key) {
		if let Some(val) = this.remove(key) {
			Ok(val)
		} else {
			panic!("TODO: error for doesnt exist")
		}
	}

	fn "[]" (this, shared key) {
		Ok(this.get(key).map(Clone::clone).unwrap_or_else(|| Object::null()))
	}

	fn "[]=" (mut this, shared key, shared val) {
		this.insert(key.clone(), val.clone());
		Ok(val.clone())
	}

	fn "[]~" (mut this, shared pos) {
		Ok(this.remove(pos).unwrap_or_else(|| Object::null()))
	}

	fn _ (_) {
		any::get_default_attr(self, attr)
	}
}

