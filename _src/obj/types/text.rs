use shared::Shared;
use obj::{Object, AnyShared, SharedObject};
use obj::types::Basic;
use env::{Parent, Mapping, Binding};

#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
pub struct Text(String);

impl Text {
	#[inline(always)]
	pub fn new(text: String) -> Self {
		Text(text)
	}
}

impl Object<Text> {
	pub fn new_text<B: Into<Text>>(val: B) -> Self {
		Object::new(val.into())
	}
}

impl AnyShared {
	pub fn cast_text(self) -> Result<SharedObject<Text>, Self> {
		unimplemented!();
	}
}

impl From<String> for Text {
	fn from(text: String) -> Self {
		Text::new(text)
	}
}

impl<'a> From<&'a str> for Text {
	fn from(inp: &'a str) -> Self {
		Text::new(inp.to_string())
	}
}


impl Parent for Text {
	fn binding() -> Shared<Binding> {
		DEFAULT_TEXT_ATTRS.clone()
	}
}

lazy_static! {
	static ref DEFAULT_TEXT_ATTRS: Shared<Binding> = Binding::new(Basic::binding(), {
		let mut h = Mapping::new();
		h
	});
}