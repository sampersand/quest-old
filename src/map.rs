use obj::AnyShared;
use std::iter::{Extend, FromIterator};

// this is a hack so i dont have to implement hashing everywhere
#[derive(Debug, Clone, Default)]
pub struct ObjMap(Vec<(AnyShared, AnyShared)>);

impl ObjMap {
	pub fn new() -> ObjMap {
		ObjMap::default()
	}

	pub fn get(&self, key: &AnyShared) -> Option<&AnyShared> {
		for (k, v) in self.iter() {
			if k == key {
				return Some(v);
			}
		}
		None
	}


	pub fn insert(&mut self, key: AnyShared, val: AnyShared) -> Option<AnyShared> {
		for i in 0..self.0.len() {
			let (eq, v) = {
				let (k, v) = &self.0[i];
				(k == &key, v.clone())
			};

			if eq {
				self.0[i] = (key, val);
				return Some(v);
			}
		}
		self.0.push((key, val));
		None
	}

	pub fn remove(&mut self, key: &AnyShared) -> Option<AnyShared> {
		for i in 0..self.0.len() {
			if &self.0[i].0 == key {
				let (k, _) = self.0.remove(i);
				return Some(k);
			}
		}
		None
	}

	pub fn len(&self) -> usize {
		self.0.len()
	}

	pub fn iter(&self) -> impl Iterator<Item=&(AnyShared, AnyShared)> {
		self.0.iter()
	}

	pub fn is_empty(&self) -> bool {
		self.len() == 0
	}

	pub fn contains_key(&self, key: &AnyShared) -> bool {
		self.iter().any(|(k, _)| key == k)
	}

	pub fn keys(&self) -> impl Iterator<Item=&AnyShared> {
		self.iter().map(|(k, _)| k)
	}

	pub fn extend(&mut self, map: ObjMap) {
		for (k, v) in map.iter() {
			self.insert(k.clone(), v.clone());
		}
	}

}

impl FromIterator<(AnyShared, AnyShared)> for ObjMap {
	fn from_iter<T: IntoIterator<Item = (AnyShared, AnyShared)>>(iter: T) -> ObjMap {
		ObjMap(Vec::from_iter(iter))
	}
}

impl Eq for ObjMap {}
impl PartialEq for ObjMap {
	fn eq(&self, other: &ObjMap) -> bool {
		if self.len() != other.len() {
			return false;
		}
		self.iter().all(|(key, value)| other.get(key).map_or(false, |v| value.clone() == v.clone()))
	}
}


// impl Extend<(AnyShared, AnyShared)> for ObjMap {
// 	fn extend<T: IntoIterator<Item = (AnyShared, AnyShared)>>(&mut self, iter: T) {
// 		self.0.extend(iter.into())
// 	}
// }





