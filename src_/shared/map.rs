use shared::{Shared, ReadGuard};

use std::ops::Deref;
use std::borrow::Borrow;
use std::hash::{Hash, Hasher};
use std::collections::HashMap;

#[derive(Debug)]
pub struct MapGuard<'a, K: Eq + Hash + 'a, V: 'a> {
	value: &'a V,
	guard: ReadGuard<'a, HashMap<K, V>>
}

pub type SharedMap<K, V> = Shared<HashMap<K, V>>;

impl<K: Eq + Hash, V> SharedMap<K, V> {
	#[inline]
	pub fn empty() -> SharedMap<K, V> {
		SharedMap::new(HashMap::default())
	}

	// pub fn get<Q: Borrow<K>>(&self, key: &Q) -> Option<MapGuard<K, V>> {
	pub fn get<'a, 'b, Q: Borrow<K> + 'b>(&'a self, key: &'b Q) -> Option<MapGuard<'a, K, V>> {
		let guard = self.read();
		let value = unsafe {
			use std::mem;
			// since we are guaranteed that `value`'s reference wont change (because we have the `ReadGuard`),
			// we're allowed to extend the lifetime to the lifetime of the guard itself
			mem::transmute::<&V, &'a V>(guard.get(key.borrow())?)
		};
		Some(MapGuard { value, guard })
	}

	pub fn insert(&self, key: K, val: V) -> Option<K> {
		unimplemented!()
	}

	pub fn contains_key<Q: Borrow<K>>(&self, key: &Q) -> bool {
		unimplemented!()
	}

	pub fn remove<Q: Borrow<K>>(&self, key: &Q) -> Option<K> {
		unimplemented!()
	}
}

impl<'a, K: Eq + Hash + 'a, V: 'a> Deref for MapGuard<'a, K, V> {
	type Target = V;

	#[inline]
	fn deref(&self) -> &V {
		&self.value
	}
}

impl<'a, K: Eq + Hash + 'a, V: 'a> AsRef<V> for MapGuard<'a, K, V> {
	#[inline]
	fn as_ref(&self) -> &V {
		self.value
	}
}
