use std::sync::RwLock;
use crate::object::{Object, AnyObject, Type};
use std::collections::{HashSet, HashMap};
use crate::{map::Map, shared::Shared};
use std::ops::{Deref, DerefMut};
use crate::err::{Result, Error};
use lazy_static::lazy_static;
use crate::util::{self, IndexError};

use super::quest_funcs::{
	AT_TEXT, AT_VAR, AT_BOOL, AT_NUM,
	CALL, L_EVAL, L_LEN,
	ADD, MUL,
	EQL, ADD_ASSIGN, 
	INDEX, INDEX_ASSIGN
};

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
		self.call_attr(AT_TEXT, &[])?
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

impl PartialEq<&'_ str> for Object<Text> {
	fn eq(&self, rhs: &&str) -> bool {
		self.data().read().expect("read err in Object<Text>::eq").as_ref() == *rhs
	}
}

impl AsRef<str> for Text {
	fn as_ref(&self) -> &str {
		&self.0
	}
}



impl_type! { for Text;
	AT_TEXT => |obj, _| Ok(obj.duplicate()),
	AT_VAR => |obj, _| Ok(Object::new_variable_from_string(obj.data().read().expect(data_err![read in Text, AT_VAR]).as_ref().to_string())),
	AT_BOOL => |obj, _| Ok(Object::new_boolean(!obj.data().read().expect(data_err![read in Text, AT_BOOL]).is_empty())),
	AT_NUM => |obj, _| Ok(Object::new(super::Number::parse_str(&obj.data().read().expect(data_err!(read in Text, AT_TEXT)))?).as_any()),

	CALL => |_, _| unimplemented!("{}", CALL),
	L_EVAL => |_, _| unimplemented!("This will be 'evaluate this text', possibly with new environment"),

	EQL => |obj, args| {
		let lhs = obj.data().read().expect(data_err![read in Text, EQL]);
		let rhs = getarg!(args[0] @ to_text)?;
		let rhs = rhs.data().read().expect(data_err![read in Text, EQL]);
		Ok(Object::new_boolean(*lhs == *rhs))
	},

	ADD => |obj, args| {
		let text_obj = getarg!(args[0] @ to_text)?;
		let mut new_text = obj.data().read().expect(data_err![read in Text, ADD]).as_ref().to_string();
		new_text.push_str(&text_obj.data().read().expect(data_err![read in Text, ADD]).as_ref());
		Ok(Object::new_text(new_text))
	},

	ADD_ASSIGN => |obj, args| {
		let text_obj = getarg!(args[0] @ to_text)?;
		// this might fail if we try adding the same number
		obj.data().write().expect(data_err![write in Text, ADD_ASSIGN]).push_str(&text_obj.data().read().expect(data_err![read in Text, ADD_ASSIGN]));
		Ok(obj.clone())
	},

	MUL => |obj, args| {
		let amnt_obj = getarg!(args[0] @ to_number)?;
		let amnt = amnt_obj.data().read().expect(data_err![read in Text, MUL]).to_integer();
		if amnt.is_negative() {
			return Err(Error::BadArgument{ pos: 0, arg: amnt_obj, msg: "received a negative value" });;
		}

		Ok(Object::new_text(obj.data().read().expect(data_err![read in Text, MUL]).repeat(amnt as usize)))
	},

	L_LEN => |obj, _| Ok(Object::new_number(obj.data().read().expect(data_err![read in Text, L_LEN]).chars().count() as f64)),
	INDEX => |obj, args| { // note you index starting at 1
		let this = obj.data().read().expect(data_err![read in Text, INDEX]);
		let start = getarg!(args[0] @ to_number)?.data().read().expect(data_err![read in Text, INDEX]).to_integer();
		let end = args.get(1).map(|x| x.to_number()).transpose()?.map(|x| x.data().read().expect(data_err![read in Text, INDEX]).to_integer());

		match util::get_index(start, end, this.len()) {
			Ok(range) => Ok(Object::new_text_str(this.get(range).expect(const_concat!["indexing failed in Text::", INDEX]))),
			Err(IndexError::ZeroPassed) => Err(Error::BadArgument { pos: 0, arg: Object::new_number(0.0).clone(), msg: "0 is not allowed for indexing" }), // making the number is bad
			Err(IndexError::StartOutOfBounds) | Err(IndexError::StartBiggerThanEnd) => Ok(Object::new_null())
		}
	},
	INDEX_ASSIGN => |obj, args| {
		let start = getarg!(args[0] @ to_number)?.data().read().expect(data_err![read in Text, INDEX_ASSIGN]).to_integer();
		let end = if args.len() >= 3 {
			Some(args[1].to_number()?.data().read().expect(data_err![read in Text, INDEX_ASSIGN]).to_integer())
		} else {
			None
		};

		let insertion = if args.len() == 2 {
			args[1].to_text()?
		} else {
			getarg!(args[2] @ to_text)?
		};

		let mut this = obj.data().write().expect(data_err![write in Text, INDEX_ASSIGN]);
		match util::get_index(start, end, this.len()) {
			Ok(range) => {
				this.replace_range(range, &insertion.data().read().expect(data_err![read in Text, INDEX_ASSIGN]));
				drop(this);
				Ok(obj.as_any())
			},
			Err(IndexError::ZeroPassed) => Err(Error::BadArgument { pos: 0, arg: Object::new_number(0.0).clone(), msg: "0 is not allowed for indexing" }), // making the number is bad
			Err(IndexError::StartOutOfBounds) | Err(IndexError::StartBiggerThanEnd) => Ok(Object::new_null())
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
				assert_eq!(*t!($obj).call_attr($attr, &$args)?.downcast_or_err::<$type>()?.unwrap_data(), $expected);
			)*
		}
	}

	#[test]
	fn at_text() -> Result<()> {
		assert_text_call_eq!(AT_TEXT Text; 
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
		let dup = obj.as_any().call_attr(AT_TEXT, &[])?.downcast_or_err::<Text>()?;
		assert_obj_duplicated!(obj, dup);
		Ok(())
	}

	#[test]
	fn at_var() -> Result<()> {
		assert_text_call_eq!(AT_VAR Variable;
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
		assert_text_call_eq!(AT_BOOL Boolean;
			("", []) => false,
			("\0", []) => true,
			("foo", []) => true,
			("bar baz quux", []) => true,
			("I ❤️ 🚀, they r cool", []) => true,
			("Ɣ㠨𥊗", [&t!("")]) => true // ensure extra args are ignored
		);
		Ok(())
	}

	#[test]
	fn at_num() -> Result<()> {
		assert_text_call_eq!(AT_NUM Number;
			("", []) => 0.0,
			("1", []) => 1.0,
			("1.0", []) => 1.0,
			("-1.00000", []) => -1.0,
			("3.14159265", []) => 3.14159265,
			("-12.34", []) => -12.34,
			("12e9", []) => 12e9,
			("12E+9", []) => 12E+9,
			("-12.41e+9", []) => -12.41e+9,
			("14.5", [&t!("1")]) => 14.5 // ensure extra args are ignored
		);

		match t!('a').call_attr(AT_NUM, &[]).unwrap_err() {
			Error::BadArgument { pos: 0, .. } => {},
			err => panic!("Bad error type returned: {:?}", err)
		}

		// assert!(t!('a').call_attr(AT_NUM, &[])?.is_null());

		Ok(())
	}

	#[test]
	#[ignore]
	fn call() -> Result<()> {
		unimplemented!("{}", CALL);
	}

	#[test]
	#[ignore]
	fn eval() -> Result<()> {
		unimplemented!("{}", L_EVAL);
	}

	#[test]
	fn eql() -> Result<()> {
		assert_text_call_eq!(EQL Boolean; 
			("", [&t!("")]) => true,
			("my name is not fred", [&t!("my name is fred")]) => false,
			("`jk, it's \0 not ºåΩ∂ª≈Z≥≥ afsoeifhawef", [&t!("`jk, it's \0 not ºåΩ∂ª≈Z≥≥ afsoeifhawef")]) => true,
			("I ❤️ Quest", [&t!("I ❤️ Quest")]) => true,
			("🚀s are cool! yah! 🚀", [&t!("🚀s are cool! yah!")]) => false,
			(" ", [&t!("")]) => false,
			("\0", [&t!("\0")]) => true,
			("test", [&t!("test"), &t!("ing")]) => true // ensure extra args are ignored
		);

		assert_param_missing!(t!("lol").call_attr(EQL, &[]));

		Ok(())
	}

	#[test]
	fn add() -> Result<()> {
		assert_text_call_eq!(ADD Text; 
			("", [&t!("")]) => "",
			("123", [&t!("456")]) => "123456",
			("`\0❤️🚀ºåΩ", [&t!("wotm8")]) => "`\0❤️🚀ºåΩwotm8",
			("I \u{2764}", [&t!("\u{fe0f} Lali")]) => "I ❤️ Lali",
			("\t\n \0", [&t!("\0 \n\t")]) => "\t\n \0\0 \n\t",
			("test", [&t!("ing"), &t!("123")]) => "testing" // ensure extra args are ignored
		);

		// make sure an object can be added to itself
		let t = t!("hi");
		assert_eq!(*t.call_attr(ADD, &[&t])?.downcast_or_err::<Text>()?.unwrap_data(), "hihi");

		// make sure it doesn't do an in-place edit
		let obj = Object::new_text_str("Hello, ");
		let dup = obj.as_any().call_attr(ADD, &[&t!("world")])?.downcast_or_err::<Text>()?;
		assert_eq!(*obj.unwrap_data(), "Hello, "); // make sure it's not edited in-place
		assert_eq!(*dup.unwrap_data(), "Hello, world");
		assert!(!obj._map_only_for_testing().ptr_eq(dup._map_only_for_testing()));

		assert_param_missing!(t!("lol").call_attr(ADD, &[]));

		Ok(())
	}

	#[test]
	#[ignore]
	fn add_assign() -> Result<()> {
		unimplemented!("{}", ADD_ASSIGN);
	}

	#[test]
	fn mul() -> Result<()> {
		assert_text_call_eq!(MUL Text; 
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
		let dup = obj.as_any().call_attr(MUL, &[&n!(3)])?.downcast_or_err::<Text>()?;
		assert_eq!(*obj.unwrap_data(), "foo"); // make sure it's not edited in-place
		assert_eq!(*dup.unwrap_data(), "foofoofoo");
		assert!(!obj._map_only_for_testing().ptr_eq(dup._map_only_for_testing()));

		// make sure texts (that are numbers) can be multiplied by themselves
		let t = t!("4");
		assert_eq!(*t.call_attr(MUL, &[&t])?.downcast_or_err::<Text>()?.unwrap_data(), "4444");

		// make sure negative numbers return an argument error
		match t!("_").call_attr(MUL, &[&n!(-2.0)]).unwrap_err() {
			Error::BadArgument{ pos: 0, msg: "received a negative value", .. } => {},
			err => panic!("Bad error type returned: {:?}", err)
		}

		assert_param_missing!(t!("lol").call_attr(MUL, &[]));

		Ok(())
	}

	#[test]
	fn mul_nonnum() -> Result<()> {
		assert_text_call_eq!(MUL Text; 
			("lol", [&Object::new_boolean(true).as_any()]) => "lol",
			("a", [&t!("4")]) => "aaaa",
			("test", [&Object::new_boolean(false).as_any(), &n!(2)]) => "" // ensure extra args are ignored
		);


		// make sure negative numbers return an argument error
		match t!("_").call_attr(MUL, &[&t!("-2")]).unwrap_err() {
			Error::BadArgument{ pos: 0, msg: "received a negative value", .. } => {},
			err => panic!("Bad Error type returned: {:?}", err)
		}
		Ok(())
	}

	#[test]
	fn len() -> Result<()> {
		assert_text_call_eq!(L_LEN Number; 
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
		unimplemented!("{}", INDEX)
	}

	#[test]
	#[ignore]
	fn index_assign() -> Result<()> {
		unimplemented!("{}", INDEX_ASSIGN)
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
		assert_eq!(Object::new_text_str("abc").as_any().to_text()?.unwrap_data().as_ref(), "abc");
		assert_eq!(Object::new_text_str("").as_any().to_text()?.unwrap_data().as_ref(), "");
		assert_eq!(Object::new_text_str("I ❤️ 🚀, they r cool").as_any().to_text()?.unwrap_data().as_ref(), "I ❤️ 🚀, they r cool");
		
		Ok(())
	}
}