use std::sync::RwLock;
use crate::object::{Object, AnyObject, Type};
use std::collections::{HashSet, HashMap};
use crate::{map::Map, shared::Shared};
use std::ops::{Deref, DerefMut};
use crate::err::{Result, Error};
use lazy_static::lazy_static;
use crate::util::{self, IndexError};


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
	type Target = String;
	fn deref(&self) -> &String {
		&self.0
	}
}

impl DerefMut for Text {
	fn deref_mut(&mut self) -> &mut String {
		&mut self.0
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
	"@text" => |obj, _| Ok(Object::new_text(obj.data().read().expect("read err in Text::@text").as_ref().to_string())),
	"@var" => |obj, _| Ok(Object::new_variable_from_string(obj.data().read().expect("read err in Text::@var").as_ref().to_string())),
	"@bool" => |obj, _| Ok(Object::new_boolean(!obj.data().read().expect("read err in Text::@bool").is_empty())),
	"@num" => |obj, _| Ok(Object::new(super::Number::parse_str(&obj.data().read().expect("read err in Text::@bool"))?).as_any()),

	"()" => |_, _| unimplemented!("()"),
	"eval" => |_, _| unimplemented!("This will be 'evaluate this text', possibly with new environment"),

	"==" => |obj, args| {
		Ok(Object::new_boolean(*obj.data().read().expect("read err in Text::==") == *getarg!(args[0] @ to_text)?.data().read().expect("read err in Text::==")))
	},
	"+" => |obj, args| {
		let text_obj = getarg!(args[0] @ to_text)?;
		let mut new_text = obj.data().read().expect("read err in Text::+").as_ref().to_string();
		new_text.push_str(&text_obj.data().read().expect("read err in Text::+").as_ref());
		Ok(Object::new_text(new_text))
	},

	"+=" => |obj, args| {
		let text_obj = getarg!(args[0] @ to_text)?;
		// this might fail if we try adding the same number
		obj.data().write().expect("write err in Text::+=").push_str(&text_obj.data().read().expect("read err in Text::+="));
		Ok(obj.clone())
	},

	"*" => |obj, args| {
		let amnt_obj = getarg!(args[0] @ to_number)?;
		let amnt = amnt_obj.data().read().expect("read err in Text::*").to_integer();
		if amnt.is_negative() {
			return Err(Error::BadArgument{ pos: 0, arg: amnt_obj, msg: "received a negative value" });;
		}

		Ok(Object::new_text(obj.data().read().expect("read err in Text::*").repeat(amnt as usize)))
	},

	"len" => |obj, _| Ok(Object::new_number(obj.data().read().expect("read err in Text::len").chars().count() as f64)),
	"[]" => |obj, args| { // note you index starting at 1
		let this = obj.data().read().expect("read err in Text::[]");
		let start = getarg!(args[0] @ to_number)?.data().read().expect("read err in Text::[]").to_integer();
		let end = args.get(1).map(|x| x.to_number()).transpose()?.map(|x| x.data().read().expect("read err in Text::[]").to_integer());

		match util::get_index(start, end, this.len()) {
			Ok(range) => Ok(Object::new_text_str(this.get(range).expect("indexing failed in Text::[]"))),
			Err(IndexError::ZeroPassed) => Err(Error::BadArgument { pos: 0, arg: Object::new_number(0.0).clone(), msg: "0 is not allowed for indexing" }), // making the number is bad
			Err(IndexError::StartTooBig) | Err(IndexError::StartBiggerThanEnd) => Ok(Object::new_null())
		}
	},
	"[]=" => |obj, args| {
		let start = getarg!(args[0] @ to_number)?.data().read().expect("read err in Text::[]=").to_integer();
		let end = if args.len() >= 3 {
			Some(args[1].to_number()?.data().read().expect("read err in Text::[]=").to_integer())
		} else {
			None
		};

		let insertion = if args.len() == 2 {
			args[1].to_text()?
		} else {
			getarg!(args[2] @ to_text)?
		};

		let mut this = obj.data().write().expect("write err in Text::[]=");

		match util::get_index(start, end, this.len()) {
			Ok(range) => {
				this.replace_range(range, &insertion.data().read().expect("read err in Text::[]="));
				drop(this);
				Ok(obj.as_any())
			},
			Err(IndexError::ZeroPassed) => Err(Error::BadArgument { pos: 0, arg: Object::new_number(0.0).clone(), msg: "0 is not allowed for indexing" }), // making the number is bad
			Err(IndexError::StartTooBig) | Err(IndexError::StartBiggerThanEnd) => Ok(Object::new_null())
		}
	},
}


#[cfg(test)]
mod fn_tests {
	use super::*;
	use crate::object::types::{Boolean, Number, Variable};
	use crate::err::Error;

	macro_rules! t {
		($text:expr) => (Object::new_text_str($text).as_any())
	}

	macro_rules! n {
		($num:expr) => (Object::new_number($num as f64).as_any())
	}

	macro_rules! assert_text_call_eq {
		($attr:tt $type:ty; $(($obj:expr, $args:tt) => $expected:expr),*) => {
			$(
				assert_eq!(**t!($obj).call_attr($attr, &$args)?.downcast_or_err::<$type>()?.data().read().unwrap(), $expected);
			)*
		}
	}

	#[test]
	fn at_text() -> Result<()> {
		assert_text_call_eq!("@text" Text; 
			("", []) => "",
			(r#"`\"\'`"#, []) => r#"`\"\'`"#,
			("my name is not fred", []) => "my name is not fred",
			("jk, \0its not", []) => "jk, \0its not",
			("I ❤️ Quest", []) => "I ❤️ Quest",
			("🚀s are cool! yah! 🚀", []) => "🚀s are cool! yah! 🚀",
			("\0", []) => "\0",
			("test", [&t!("ing")]) => "test" // ensure extra args are ignored
		);

		// make sure that it acutally duplicates the map
		let obj = Object::new_text_str("hi there");
		let dup = obj.call_attr("@text", &[])?.downcast_or_err::<Text>()?;
		assert_eq!(*obj.data().read().unwrap(), *dup.data().read().unwrap());
		assert!(!obj._map_only_for_testing().ptr_eq(dup._map_only_for_testing()));
		Ok(())
	}

	#[test]
	fn at_var() -> Result<()> {
		assert_text_call_eq!("@var" Variable;
			("", []) => "",
			("``", []) => "``",
			("my name is not fred", []) => "my name is not fred",
			("`jk, \0its not`", []) => "`jk, \0its not`",
			("I ❤️ Quest", []) => "I ❤️ Quest",
			("🚀s are cool! yah! 🚀", []) => "🚀s are cool! yah! 🚀",
			("\0", []) => "\0",
			("test", [&t!("ing")]) => "test" // ensure extra args are ignored
		);
		Ok(())
	}


	#[test]
	fn at_bool() -> Result<()> {
		assert_text_call_eq!("@bool" Boolean;
			("", []) => false,
			("\0", []) => true,
			("foo", []) => true,
			("bar baz quux", []) => true,
			("I ❤️ 🚀, they r cool", []) => true,
			("Ɣ㠨𥊗", [&t!("")]) => true // ensure extra arge are ignored
		);
		Ok(())
	}

	#[test]
	#[ignore]
	fn at_num() -> Result<()> {
		unimplemented!("TODO: @num")
	}

	#[test]
	#[ignore]
	fn exec() -> Result<()> {
		unimplemented!("TODO: ()");
	}

	#[test]
	#[ignore]
	fn eval() -> Result<()> {
		unimplemented!("TODO: eval");
	}

	#[test]
	fn equality() -> Result<()> {
		assert_text_call_eq!("==" Boolean; 
			("", [&t!("")]) => true,
			("my name is not fred", [&t!("my name is fred")]) => false,
			("`jk, it's \0 not ºåΩ∂ª≈Z≥≥ afsoeifhawef", [&t!("`jk, it's \0 not ºåΩ∂ª≈Z≥≥ afsoeifhawef")]) => true,
			("I ❤️ Quest", [&t!("I ❤️ Quest")]) => true,
			("🚀s are cool! yah! 🚀", [&t!("🚀s are cool! yah!")]) => false,
			(" ", [&t!("")]) => false,
			("\0", [&t!("\0")]) => true,
			("test", [&t!("test"), &t!("ing")]) => true // ensure extra args are ignored
		);


		// check to see if too few args are passed it handles it right
		match t!("lol").call_attr("==", &[]).unwrap_err() {
			Error::MissingArgument { pos: 0, .. } => {},
			err => panic!("Bad error type returned: {:?}", err)
		}

		Ok(())
	}

	#[test]
	fn add() -> Result<()> {
		assert_text_call_eq!("+" Text; 
			("", [&t!("")]) => "",
			("123", [&t!("456")]) => "123456",
			("`\0❤️🚀ºåΩ", [&t!("wotm8")]) => "`\0❤️🚀ºåΩwotm8",
			("I \u{2764}", [&t!("\u{fe0f} Lali")]) => "I ❤️ Lali",
			("\t\n \0", [&t!("\0 \n\t")]) => "\t\n \0\0 \n\t",
			("test", [&t!("ing"), &t!("123")]) => "testing" // ensure extra args are ignored
		);

		// make sure an object can be added to itself
		let t = t!("hi");
		assert_eq!(**t.call_attr("+", &[&t])?.downcast_or_err::<Text>()?.data().read().unwrap(), "hihi");

		// make sure it doesn't do an in-place edit
		let obj = Object::new_text_str("Hello, ");
		let dup = obj.call_attr("+", &[&t!("world")])?.downcast_or_err::<Text>()?;
		assert_eq!(**obj.data().read().unwrap(), "Hello, "); // make sure it's not edited in-place
		assert_eq!(**dup.data().read().unwrap(), "Hello, world");
		assert!(!obj._map_only_for_testing().ptr_eq(dup._map_only_for_testing()));

		// check to see if too few args are passed it handles it right
		match t!("lol").call_attr("+", &[]).unwrap_err() {
			Error::MissingArgument { pos: 0, .. } => {},
			err => panic!("Bad error type returned: {:?}", err)
		}

		Ok(())
	}

	#[test]
	#[ignore]
	fn add_ignore() -> Result<()> {
		unimplemented!("+=");
	}

	#[test]
	fn mul() -> Result<()> {
		assert_text_call_eq!("*" Text; 
			("1234", [&n!(0)]) => "",
			("1234", [&n!(1)]) => "1234",
			("", [&n!(3)]) => "",
			("a", [&n!(9)]) => "aaaaaaaaa",
			("\0", [&n!(3)]) => "\0\0\0",
			("\u{2764}\u{fe0f}", [&n!(4)]) => "❤️❤️❤️❤️",
			("what", [&n!(3.4)]) => "whatwhatwhat", // test non-integer values
			("what", [&n!(1.9)]) => "what", // test non-integer values
			("test", [&n!(2), &t!("ing")]) => "testtest" // ensure extra args are ignored
		);

		// make sure it doesn't do an in-place edit
		let obj = Object::new_text_str("foo");
		let dup = obj.call_attr("*", &[&n!(3)])?.downcast_or_err::<Text>()?;
		assert_eq!(**obj.data().read().unwrap(), "foo"); // make sure it's not edited in-place
		assert_eq!(**dup.data().read().unwrap(), "foofoofoo");
		assert!(!obj._map_only_for_testing().ptr_eq(dup._map_only_for_testing()));

		// make sure texts (that are numbers) can be multiplied by themselves
		let t = t!("4");
		assert_eq!(**t.call_attr("*", &[&t])?.downcast_or_err::<Text>()?.data().read().unwrap(), "4444");

		// make sure negative numbers return an argument error
		match t!("_").call_attr("*", &[&n!(-2.0)]).unwrap_err() {
			Error::BadArgument{ pos: 0, msg: "received a negative value", .. } => {},
			err => panic!("Bad error type returned: {:?}", err)
		}

		// check to see if too few args are passed it handles it right
		match t!("lol").call_attr("*", &[]).unwrap_err() {
			Error::MissingArgument { pos: 0, .. } => {},
			err => panic!("Bad error type returned: {:?}", err)
		}

		Ok(())
	}

	#[test]
	fn mul_nonnum() -> Result<()> {
		assert_text_call_eq!("*" Text; 
			("lol", [&Object::new_boolean(true).as_any()]) => "lol",
			("a", [&t!("4")]) => "aaaa",
			("test", [&Object::new_boolean(false).as_any(), &n!(2)]) => "" // ensure extra args are ignored
		);


		// make sure negative numbers return an argument error
		match t!("_").call_attr("*", &[&t!("-2")]).unwrap_err() {
			Error::BadArgument{ pos: 0, msg: "received a negative value", .. } => {},
			err => panic!("Bad Error type returned: {:?}", err)
		}
		Ok(())
	}

	#[test]
	fn len() -> Result<()> {
		assert_text_call_eq!("len" Number; 
			("", []) => 0.0,
			("123", []) => 3.0,
			("\x7f\x00\n\0", []) => 4.0,
			("🚀", []) => 1.0,
			("I Like 🚀", []) => 8.0,
			("test", [&t!("ing")]) => 4.0 // ensure extra args are ignored
		);

		Ok(())
	}

	#[test]
	#[ignore]
	fn index() -> Result<()> {
		unimplemented!("[]")
	}

	#[test]
	#[ignore]
	fn index_assign() -> Result<()> {
		unimplemented!("[]=")
	}
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