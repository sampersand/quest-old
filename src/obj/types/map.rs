use std::ops::{Deref, DerefMut};
use std::hash::{Hash, Hasher};
use std::collections::{HashMap, hash_map::{DefaultHasher, Entry}};
use std::fmt::{self, Display, Formatter};
use obj::{AnyShared, SharedObject, types::{IntoObject, Number}};

type InnerMap = HashMap<AnyShared, AnyShared>;

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct Map(InnerMap);

impl Map {
	pub fn new(data: InnerMap) -> Self {
		Map(data)
	}
}

impl From<InnerMap> for Map {
	#[inline]
	fn from(inp: InnerMap) -> Map {
		Map::new(inp)
	}
}

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
	type Target = InnerMap;

	#[inline]
	fn deref(&self) -> &InnerMap {
		&self.0
	}
}

impl DerefMut for Map {
	#[inline]
	fn deref_mut(&mut self) -> &mut InnerMap {
		&mut self.0
	}
}

impl IntoObject for InnerMap {
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
		Ok((!this.read().data.is_empty()).into_object())
	}

	fn "len" (this) {
		Ok(this.read().data.len().into_object())
	}

	fn "count_vals" (this, val) {
		let ref val = *val.read();
		let ref data = this.read().data;
		Ok(data.iter().filter(|(_k, v)| &*v.read() == val).count().into_object())
	}

	fn "has?" (this, key) {
		let ref key = *key.read();
		let ref data = this.read().data;
		Ok(data.keys().any(|obj| &*obj.read() == key).into_object())
	}

	fn "get" (this, key) {
		Ok(this.read().data.get(&key).cloned().unwrap_or_else(Object::null))
	}

	fn "set" (this, key, val) {
		Ok(this.write().data.insert(key, val).unwrap_or_else(Object::null))
	}

	fn "del" (this, pos) {
		Ok(this.write().data.remove(&pos).unwrap_or_else(Object::null))
	}

	fn "+" (this, other) env, {
		let other = other.read_into_map(env)?;
		this.write().data.0.extend(other.0);
		Ok(this)
	}

	fn _ () {
		any::get_default_attr(self, attr)
	}
}





