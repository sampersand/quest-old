use std::sync::RwLock;
use crate::object::{Object, AnyObject, Type};
use std::collections::{HashSet, HashMap};
use crate::{map::Map, shared::Shared};
use std::ops::Deref;
use crate::err::Result;
use lazy_static::lazy_static;


#[derive(Debug, PartialEq, Eq, Hash, Clone, Default)]
pub struct Text(String);

impl Text {
	#[inline]
	pub fn new(text: String) -> Text {
		Text(text)
	}
}

impl Object<Text> {
	pub fn new_text(text: String) -> Object<Text> {
		Object::new(Text::new(text))
	}
	pub fn new_text_str<T: ToString>(text: T) -> Object<Text> {
		Object::new(Text::new(text.to_string()))
	}
}

impl AnyObject {
	pub fn to_text(&self) -> Result<Object<Text>> {
		self.call_attr("@text", &[])?
			.downcast_or_err::<Text>()
	}
}

impl Deref for Text {
	type Target = str;
	fn deref(&self) -> &str {
		&self.0
	}
}


impl From<Text> for String {
	fn from(text: Text) -> String {
		text.0
	}
}

impl From<String> for Text {
	fn from(text: String) -> Text {
		Text::new(text)
	}
}

impl AsRef<str> for Text {
	fn as_ref(&self) -> &str {
		&self.0
	}
}


impl_type! { for Text;
	"@var" => |obj, _| Ok(Object::new_variable_from_string(obj.data().read().expect("read err in Text::@var").as_ref().to_string())),
	"@text" => |obj, _| Ok(Object::new_text(obj.data().read().expect("read err in Text::@var").as_ref().to_string())),
	"==" => |obj, args| {
		Ok(Object::new_boolean(*obj.data().read().expect("read err in Text::==") == *getarg!(args[0] @ to_text)?.data().read().expect("read err in Text::==")))
	},

	// "@text" => |obj, _| Ok(Object::new_text(obj.data().read().expect("read err in Text::@var").as_ref().clone())),
}



#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn new() {
		assert_eq!(Text::new("".to_string()).as_ref(), "");
		assert_eq!(Text::new("foobar".to_string()).as_ref(), "foobar");
		assert_eq!(Text::new("a b c d ".to_string()).as_ref(), "a b c d ");
		assert_eq!(Text::new("I ❤️ Quest".to_string()).as_ref(), "I \u{2764}\u{fe0f} Quest");
		assert_eq!(Text::new("🚀s are cool!".to_string()).as_ref(), "\u{1f680}s are cool!");
		assert_eq!(Text::new("Ɣ㠨𥊗".to_string()).as_ref(), "\u{194}㠨\u{25297}");
	}

	#[test]
	fn from_string() {
		assert_eq!(Text::from("foobarbaz".to_string()).as_ref(), "foobarbaz");
		assert_eq!(Text::from("__!_@#__$*!~".to_string()).as_ref(), "__!_@#__$*!~");
		assert_eq!(Text::from("lol".to_string()).as_ref(), "lol");
		assert_eq!(Text::from("I ❤️ 🚀, they r cool".to_string()).as_ref(), "I \u{2764}\u{fe0f} \u{1f680}, they r cool");
		assert_eq!(Text::from("Ɣ㠨𥊗".to_string()).as_ref(), "\u{194}㠨\u{25297}");
	}

	#[test]
	fn new_text() {
		assert_eq!(Object::new(Text::new("quest is fun".to_string())), Object::new_text_str("quest is fun"));
		assert_eq!(Object::new(Text::new("".to_string())), Object::new_text("".to_string()));
	}

	#[test]
	fn to_text() -> Result<()> {
		assert_eq!(Object::new_text_str("abc").as_any().to_text()?.data().read().unwrap().as_ref(), "abc");
		assert_eq!(Object::new_text_str("").as_any().to_text()?.data().read().unwrap().as_ref(), "");
		assert_eq!(Object::new_text_str("I ❤️ 🚀, they r cool").as_any().to_text()?.data().read().unwrap().as_ref(), "I ❤️ 🚀, they r cool");
		
		Ok(())
	}
}