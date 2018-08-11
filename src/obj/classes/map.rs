use obj::QObject;
use obj::classes::{QNum, QNull};


use std::collections::HashMap;
use std::ops::Deref;
use std::hash::{Hash, Hasher};
use std::fmt::{self, Display, Formatter};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct QMap(HashMap<QObject, QObject>);

impl QMap {
	#[inline]
	pub fn new(h: HashMap<QObject, QObject>) -> QMap {
		QMap(h)
	}
}

impl From<HashMap<QObject, QObject>> for QMap {
	#[inline]
	fn from(map: HashMap<QObject, QObject>) -> QMap {
		QMap::new(map)
	}
}

impl From<HashMap<QObject, QObject>> for QObject {
	#[inline]
	fn from(map: HashMap<QObject, QObject>) -> QObject {
		QMap::from(map).into()
	}
}

impl Deref for QMap {
	type Target = HashMap<QObject, QObject>;

	#[inline]
	fn deref(&self) -> &HashMap<QObject, QObject> {
		&self.0
	}
}

impl Display for QMap {
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		write!(f, "{{")?;
		let ref inner = self.0;
		if !inner.is_empty() {
			let mut iter = inner.iter();
			let first = iter.next().unwrap();
			write!(f, "{:#}: {:#}", first.0, first.1)?;
			for (k, v) in iter {
				write!(f, ", {:#}: {:#}", k, v)?;
			}
		}
		write!(f, "}}")
	}
}

impl Hash for QMap {
	fn hash<H: Hasher>(&self, h: &mut H) {
		// (*h!(self)).hash(h)
		unimplemented!("TODO: hash for QMap")
	}
}


default_attrs! { for QMap, with variant Map;
	use QObj;

	fn "@map" (this) {
		this.clone().into()
	}

	fn "@bool" (this) {
		this.0.is_empty().into()
	}

	fn "@list" (_this) with env vars obj {
		obj.call_attr("keys", &[], env)
	}

	fn "len" (this) {
		this.0.len().into()
	}

	fn "get" (this, key) {
		if let Some(val) = this.0.get(key) {
			val.clone()
		} else {
			warn!("Missing attribute `{}` for `{}`; returning qnull", key, this);
			().into()
		}
	}

	fn "set" (mut this, key, val) {
		this.0.insert(key.clone(), val.clone()).unwrap_or_else(|| QNull.into())
	}

	fn "has" (this, key) {
		this.0.contains_key(key).into()
	}

	fn "del" (mut this, key) {
		this.0.remove(key).unwrap_or_else(|| QNull.into())
	}
}