use std::ops::{Deref, DerefMut};
use std::borrow::Borrow;
use sync::{SpinRwLock, SpinReadGuard, SpinWriteGuard};
use std::collections::HashMap;
use std::hash::Hash;
use std::fmt::{self, Debug, Formatter};

#[derive(Debug)]
pub struct SyncMapReadGuard<'a: 'b, 'b, K: Hash + Eq + 'a, V: 'a, Q: Borrow<K> + 'b>(&'b Q, SpinReadGuard<'a, HashMap<K, V>>);

#[derive(Debug)]
pub struct SyncMapWriteGuard<'a: 'b, 'b, K: Hash + Eq + 'a, V: 'a, Q: Borrow<K> + 'b>(&'b Q, SpinWriteGuard<'a, HashMap<K, V>>);

pub struct SyncMap<K: Hash + Eq, V>(SpinRwLock<HashMap<K, V>>);

impl<K: Eq + Hash, V> SyncMap<K, V> {
	pub fn new() -> SyncMap<K, V> {
		SyncMap::default()
	}

	pub fn lock<'a>(&'a self) -> impl DerefMut<Target=HashMap<K, V>> + 'a {
		self.0.write()
	}

	pub fn read<'a>(&'a self) -> impl Deref<Target=HashMap<K, V>> + 'a {
		self.0.read()
	}
}

impl<K: Eq + Hash, V> SyncMap<K, V> {
	pub fn set(&self, key: K, value: V) -> Option<V> {
		self.0.write().insert(key, value)
	}

	pub fn get<'a, 'b, Q: Borrow<K> + 'b>(&'a self, key: &'b Q) -> Option<SyncMapReadGuard<'a, 'b, K, V, Q>> {
		if !self.has_key(key) {
			return None;
		}

		Some(SyncMapReadGuard(key, self.0.read()))
	}

	pub fn get_mut<'a, 'b, Q: Borrow<K> + 'b>(&'a self, key: &'b Q) -> Option<SyncMapWriteGuard<'a, 'b, K, V, Q>> {
		if !self.has_key(key) {
			return None;
		}

		Some(SyncMapWriteGuard(key, self.0.write()))
	}

	pub fn has_key<Q: Borrow<K>>(&self, key: &Q) -> bool {
		self.0.read().contains_key(key.borrow())
	}

	pub fn del<Q: Borrow<K>>(&self, key: &Q) -> Option<V> {
		self.0.write().remove(key.borrow())
	}
}

impl<K: Eq + Hash + Debug, V: Debug> Debug for SyncMap<K, V> {
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		if let Some(map) = self.0.try_read() {
			f.debug_map().entries(map.iter()).finish()
		} else {
			write!(f, "{{ <locked syncmap> }}")
		}
	}
}

impl<K: Eq + Hash + Clone, V: Clone> SyncMap<K, V> {
	pub fn try_clone(&self) -> Option<Self> {
		self.0.try_read().map(|map| SyncMap::from(map.clone()))
	}
}

impl<K: Eq + Hash + Clone, V: Clone> Clone for SyncMap<K, V> {
	fn clone(&self) -> Self {
		self.try_clone().expect("deadlock whilst cloning SyncMap")
	}
}

impl<K: Eq + Hash, V> From<HashMap<K, V>> for SyncMap<K, V> {
	fn from(map: HashMap<K, V>) -> SyncMap<K, V> {
		SyncMap(SpinRwLock::from(map))
	}
}

impl<K: Eq + Hash, V> Default for SyncMap<K, V> {
	fn default() -> SyncMap<K, V> {
		SyncMap(SpinRwLock::default())
	}
}

impl<'a, 'b, K: Eq + Hash + 'a, V: 'a, Q: Borrow<K> + 'b> Deref for SyncMapReadGuard<'a, 'b, K, V, Q> {
	type Target = V;
	#[inline]
	fn deref(&self) -> &V {
		self.1.get(self.0.borrow()).expect("Key was removed after its existance was checked for")
	}
}

impl<'a, 'b, K: Eq + Hash + 'a, V: 'a, Q: Borrow<K> + 'b> Deref for SyncMapWriteGuard<'a, 'b, K, V, Q> {
	type Target = V;
	#[inline]
	fn deref(&self) -> &V {
		self.1.get(self.0.borrow()).expect("Key was removed after its existance was checked for")
	}
}


impl<'a, 'b, K: Eq + Hash + 'a, V: 'a, Q: Borrow<K> + 'b> DerefMut for SyncMapWriteGuard<'a, 'b, K, V, Q> {
	#[inline]
	fn deref_mut(&mut self) -> &mut V {
		self.1.get_mut(self.0.borrow()).expect("Key was removed after its existance was checked for")
	}
}
