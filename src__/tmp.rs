use std::collections::HashMap;
use quest::{*, obj::{*, types::*}};

pub fn var(data: &'static str) -> AnyShared {
	Id::from(data).into_object() as AnyShared
}

pub fn text<S: AsRef<str>>(data: S) -> AnyShared {
	Text::from(data.as_ref()).into_object() as AnyShared
}

pub fn num(data: Integer) -> AnyShared {
	Number::from(data).into_object() as AnyShared
}

pub fn list(data: Vec<AnyShared>) -> AnyShared {
	List::from(data).into_object() as AnyShared
}

pub fn map(data: HashMap<AnyShared, AnyShared>) -> AnyShared {
	Map::from(data).into_object() as AnyShared
}

macro_rules! var {
	($x:expr) => (var($x))
}
macro_rules! text {
	($x:expr) => (text($x))
}
macro_rules! num {
	($x:expr) => (num($x))
}
macro_rules! list {
	($($x:expr),*) => (list(vec![$($x),*]))
}
macro_rules! map {
	($($x:expr => $y:expr),*) => (map({
		let mut m = ::std::collections::HashMap::<AnyShared, AnyShared>::new();
		$(m.insert($x, $y);)*
		m
	}))
}
