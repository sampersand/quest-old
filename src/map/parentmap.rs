use std::collections::HashMap;
use crate::object::AnyObject;
use crate::shared::Shared;
use crate::map::Map;

#[derive(Debug, Clone)]
pub struct ParentMap<M: Map=HashMap<AnyObject, AnyObject>> {
	parent: Shared<dyn Map>,
	map: M
}

impl<M: Map> ParentMap<M> {
	pub fn new(parent: Shared<dyn Map>, map: M) -> ParentMap<M> {
		ParentMap { parent, map }
	}
	pub fn parent(&self) -> &Shared<dyn Map> {
		&self.parent
	}
}

impl<M: Map + Default> ParentMap<M> {
	pub fn new_default(parent: Shared<dyn Map>) -> ParentMap<M> {
		ParentMap::new(parent, M::default())
	}
}

impl<M: Map> Map for ParentMap<M> {
	fn get(&self, key: &AnyObject) -> Option<AnyObject> {
		self.map.get(key).or_else(|| self.parent.read().expect("read err in ParentMap::get").get(key))
	}

	fn set(&mut self, key: AnyObject, val: AnyObject) -> Option<AnyObject> {
		self.map.set(key, val)
	}

	fn del(&mut self, key: &AnyObject) -> Option<AnyObject> {
		self.map.del(key)
	}

	fn len(&self) -> usize {
		self.map.len()
	}

	fn has(&self, key: &AnyObject) -> bool {
		self.map.has(key) || self.parent.read().expect("read err in ParentMap::has").has(key)
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::object::Object;

	macro_rules! map {
		(shared; $($key:expr => $val:expr),*) => {Shared::new(map!($($key => $val),*)) as Shared<dyn Map>};
		($($key:expr => $val:expr),*) => {{
			let mut map = HashMap::<AnyObject, AnyObject>::new();
			$(assert!(map.insert($key, $val).is_none());)*
			map
		}}
	}

	macro_rules! var {
		($id:expr) => (Object::new_variable($id).as_any())
	}
	macro_rules! num {
		($num:expr) => (Object::new_number($num as f64).as_any())
	}

	#[test]
	fn new() {
		let parent = map! { shared;
			var!["A"] => num![1] // to make sure pmap.is_empty works
		};
		let pmap: ParentMap<HashMap<_, _>> = ParentMap::new_default(parent.clone());
		assert!(pmap.parent().ptr_eq(&parent));
		assert!(pmap.is_empty());

	}

	#[test]
	fn get() {
		let pmap = ParentMap::new(map!{ shared;
			var!["foo"] => num![1],
			var!["bar"] => num![2]
		}, map! {
			var!["foo"] => num![3],
			var!["baz"] => num![4]
		});

		assert!(pmap.get(&var!["foo"]).unwrap() == num![3]); // make sure override works
		assert!(pmap.get(&var!["bar"]).unwrap() == num![2]); // make sure parent works
		assert!(pmap.get(&var!["baz"]).unwrap() == num![4]); // make sure child works
		assert!(pmap.get(&var!["quux"]).is_none()); // make sure invalid works
	}

	#[test]
	fn set() {
		let parent = map!{ shared;
			var!["foo"] => num![1],
			var!["bar"] => num![2]
		};

		let mut pmap = ParentMap::new(parent.clone(), map! {
			var!["foo"] => num![3],
			var!["baz"] => num![4]
		});

		assert!(pmap.set(var!["foo"], num![5]).unwrap() == num![3]);
		assert!(pmap.set(var!["bar"], num![6]).is_none());
		assert!(pmap.set(var!["baz"], num![7]).unwrap() == num![4]);
		assert!(pmap.set(var!["quux"], num![8]).is_none());

		assert!(pmap.get(&var!["foo"]).unwrap() == num![5]);
		assert!(pmap.get(&var!["bar"]).unwrap() == num![6]);
		assert!(pmap.get(&var!["baz"]).unwrap() == num![7]);
		assert!(pmap.get(&var!["quux"]).unwrap() == num![8]);

		assert!(parent.read().unwrap().get(&var!["foo"]).unwrap() == num![1]);
		assert!(parent.read().unwrap().get(&var!["bar"]).unwrap() == num![2]);
		assert!(parent.read().unwrap().get(&var!["baz"]).is_none());
		assert!(parent.read().unwrap().get(&var!["quux"]).is_none());

		assert_eq!(pmap.len(), 4);
		assert_eq!(parent.read().unwrap().len(), 2);
	}

	#[test]
	fn del() {
		let parent = map!{ shared;
			var!["foo"] => num![1],
			var!["bar"] => num![2]
		};

		let mut pmap = ParentMap::new(parent.clone(), map! {
			var!["foo"] => num![3],
			var!["baz"] => num![4]
		});

		assert!(pmap.del(&var!["foo"]).unwrap() == num![3]);
		assert!(pmap.del(&var!["bar"]).is_none());
		assert!(pmap.del(&var!["baz"]).unwrap() == num![4]);
		assert!(pmap.del(&var!["quux"]).is_none());
		assert!(pmap.is_empty());

		assert!(parent.read().unwrap().get(&var!["foo"]).unwrap() == num![1]);
		assert!(parent.read().unwrap().get(&var!["bar"]).unwrap() == num![2]);
		assert!(parent.read().unwrap().get(&var!["baz"]).is_none());
		assert!(parent.read().unwrap().get(&var!["quux"]).is_none());
		assert_eq!(parent.read().unwrap().len(), 2);
	}

	#[test]
	fn has() {
		let parent = map!{ shared;
			var!["foo"] => num![1],
			var!["bar"] => num![2]
		};

		let pmap = ParentMap::new(parent, map! {
			var!["foo"] => num![3],
			var!["baz"] => num![4]
		});

		assert!(pmap.has(&var!["foo"]));
		assert!(pmap.has(&var!["bar"]));
		assert!(pmap.has(&var!["baz"]));
		assert!(!pmap.has(&var!["quux"]));
	}

	#[test]
	fn len() {
		let parent = map!{ shared;
			var!["foo"] => num![1],
			var!["bar"] => num![2]
		};

		let pmap = ParentMap::new(parent.clone(), map! {
			var!["foo"] => num![3],
			var!["baz"] => num![4],
			var!["quux"] => num![0] // it's 3 b/c `2` is indistinguishable from the parent's len
		});

		assert_eq!(pmap.len(), 3);
		assert_eq!(ParentMap::new(parent, map!{}).len(), 0);
	}

	#[test]
	fn is_empty() {
		let parent = map!{shared; var!["foo"] => num![1]};
		assert!(ParentMap::new(map!{shared;}, map!{}).is_empty());
		assert!(ParentMap::new(parent.clone(), map!{}).is_empty());
		assert!(!ParentMap::new(parent.clone(), map!{var!["baz"] => num![2]}).is_empty());
	}
}








