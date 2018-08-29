use std::sync::Arc;
use obj_::QObject__;
use obj_::classes_::{QNum, QNull, QList};
use sync_::SyncMap;

use std::ops::Deref;
use std::hash::{Hash, Hasher};
use std::fmt::{self, Display, Formatter};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct QMap(Arc<SyncMap<QObject__, QObject__>>);

impl QMap {
	#[inline]
	pub fn new<M: Into<Arc<SyncMap<QObject__, QObject__>>>>(map: M) -> QMap {
		QMap(map.into())
	}
}

impl From<SyncMap<QObject__, QObject__>> for QMap {
	#[inline]
	fn from(map: SyncMap<QObject__, QObject__>) -> QMap {
		QMap::new(map)
	}
}

impl From<SyncMap<QObject__, QObject__>> for QObject__ {
	#[inline]
	fn from(map: SyncMap<QObject__, QObject__>) -> QObject__ {
		QMap::from(map).into()
	}
}

impl Deref for QMap {
	type Target = SyncMap<QObject__, QObject__>;

	#[inline]
	fn deref(&self) -> &SyncMap<QObject__, QObject__> {
		&self.0
	}
}

impl Display for QMap {
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		write!(f, "{{")?;
		let ref inner = self.0.try_read().expect("deadlock on map display");
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
		Ok(::obj_::QObject_::Old(this.clone().into()))
	}

	fn "@bool" (this) {
		Ok(::obj_::QObject_::Old(this.0.read().is_empty().into()))
	}

	fn "keys" (this) {
		Ok(::obj_::QObject_::Old(QList::new(this.0.read().keys().map(QObject__::clone).collect()).into()))
	}

	fn "@list" (_this) with env vars obj {
		obj.call_attr("keys", &[], env)
	}

	fn "empty!" (mut this) with _env _var obj{
		this.0.write().clear();
		Ok(::obj_::QObject_::Old(obj.clone()))
	}

	fn "empty?" (this) {
		Ok(::obj_::QObject_::Old(this.0.read().is_empty().into()))
	}

	fn "len" (this) {
		Ok(::obj_::QObject_::Old(this.0.read().len().into()))
	}

	fn "get" (this, key) {
		if let Some(val) = this.0.get(key) {
			Ok(::obj_::QObject_::Old(val.clone()))
		} else {
			debug!("Missing attribute `{}` for `{}`; returning qnull", key, this);
			Ok(::obj_::QObject_::Old(().into()))
		}
	}

	fn "set" (mut this, key, val) {
		Ok(::obj_::QObject_::Old(this.0.set(key.clone(), val.clone()).unwrap_or_else(|| QNull.into())))
	}

	fn "has" (this, key) {
		Ok(::obj_::QObject_::Old(this.0.has_key(key).into()))
	}

	fn "del" (mut this, key) {
		Ok(::obj_::QObject_::Old(this.0.del(key).unwrap_or_else(|| QNull.into())))
	}
}


