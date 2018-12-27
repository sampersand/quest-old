use env::{Parent, Mapping};
use shared::Shared;
use obj::AnyShared;
use std::hash::{Hash, Hasher};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum BindingParent {
	Root,
	Normal(Shared<Binding>),
	Forked(Shared<Binding>)
}

impl Default for BindingParent {
	fn default() -> Self {
		BindingParent::Root
	}
}


#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Binding {
	parent: BindingParent,
	map: Mapping,
}
//also need a call stack, for reasons of multithreading. check out: https://play.rust-lang.org/?gist=ff1e27eab7f0af6ba031734f932487d7&version=stable&mode=debug&edition=2015

impl Binding {
	pub fn empty() -> Shared<Self> {
		Shared::new(Binding { parent: BindingParent::Root, map: Mapping::default() })
	}

	pub fn new_child<P: Parent>() -> Shared<Self> {
		Binding::new(P::binding(), Mapping::default())
	}

	pub fn new(parent: Shared<Binding>, map: Mapping) -> Shared<Self> {
		Shared::new(Binding { parent: BindingParent::Normal(parent), map })
	}

	pub fn fork<P: Parent>() -> Shared<Self> {
		Shared::new(Binding{ parent: BindingParent::Forked(P::binding()), map: Mapping::default() })
	}
}

// map properties
impl Binding {
	pub fn get(&self, key: &AnyShared) -> Option<AnyShared> {
		if let Some(val) = self.map.get(key) {
			return Some(val.clone());
		}

		match self.parent {
			BindingParent::Root => None,
			BindingParent::Normal(ref parent) | BindingParent::Forked(ref parent) => parent.read().get(key)
		}
	}

	pub fn set(&mut self, key: AnyShared, val: AnyShared) -> Option<AnyShared> {
		self.map.insert(key, val)
	}

	pub fn del(&mut self, key: &AnyShared) -> Option<AnyShared> {
		self.map.remove(key)
	}

	pub fn has(&self, key: &AnyShared) -> bool {
		if self.map.contains_key(key) {
			return true;
		}

		match self.parent {
			BindingParent::Root => false,
			BindingParent::Normal(ref parent) | BindingParent::Forked(ref parent) => parent.read().has(key)
		}
	}
}

impl Hash for Binding {
	fn hash<H: Hasher>(&self, h: &mut H) {
		self.parent.hash(h);
		debug!("TODO: hash for `Binding`");
		(&self.map as *const _ as usize).hash(h);
	}
}