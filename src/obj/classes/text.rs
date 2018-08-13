use parse::{Parsable, Stream};

use obj::object::QObject;
use obj::classes::{QuestClass, DefaultAttrs, QException, num};
use std::fmt::{self, Display, Formatter};	
use std::str::FromStr;

pub type QText = QObject<String>;


impl AsRef<str> for QText {
	#[inline]
	fn as_ref(&self) -> &str {
		String::as_ref(self.as_ref())
	}
}

impl QuestClass for String {
	fn default_attrs() -> &'static DefaultAttrs<Self> { &DEFAULT_ATTRS }
}

impl Parsable for String {
	type Value = String;

	fn try_parse(stream: &mut Stream) -> Option<String> {
		// double quotes
		if let Some(text) = stream.try_get(regex!(r#"\A("(\\.|[^"])*")"#)) {
			assert_eq!(text.chars().next(), Some('\"'), "Invalid first char encountered");
			assert_eq!(text.chars().last(), Some('\"'), "Invalid first char encountered");
			return Some(text[1..text.len() - 1].to_string());
		}

		// single string
		if let Some(text) = stream.try_get(regex!(r"\A('(\\.|[^'])*')")) {
			assert_eq!(text.chars().next(), Some('\''), "Invalid first char encountered");
			assert_eq!(text.chars().last(), Some('\''), "Invalid first char encountered");
			return Some(text[1..text.len() - 1].to_string());
		}

		// grave quote
		if stream.as_str().chars().next() == Some('`') {
			unimplemented!("TODO: Command strings (`string`)");
		}

		None
	}
}


define_attrs! {
	static ref DEFAULT_ATTRS for String;
	use QObject<String>;

	fn "@text" (this) {
		Ok(this.clone())
	}

	fn "@bool" (this) {
		Ok(QBool::from(this.is_empty()))
	}
	fn "@var" (this) {
		Ok(QVar::from_nonstatic_str(this.as_ref()))
	}

	fn "@num" (this) {
		let mut stream: Stream = Stream::from_str(this.as_ref());
		Ok(QNum::from(num::Number::try_parse(&mut stream).expect("Invalid number given (todo: make this an exception)")))
	}

	fn "@cmd" (this) {
		unimplemented!("TODO: CMD");
		Ok(QNum::from_number(1i32))
	}


	// text attributes

	fn "empty!" (mut this) with _env _var obj {
		this.clear();
		drop(this);
		Ok(obj.clone())
	}

	fn "empty?" (this) {
		Ok(QBool::from(this.is_empty()))
	}

	fn "len" (this) {
		Ok(QNum::from_number(this.len()))
	}
}
	// fn "has" (this, index) with env {
	// 	Ok(if var.is_num() {
	// 		IndexPos::from_qobject(this.0.len(), var, env).is_inbounds().into()
	// 	} else if let Ok(text) = var.as_text(env) {
	// 		this.0.contains(&text.as_str()).into()
	// 	} else {
	// 		panic!("Only Num or `@text` can be used to index")
	// 	})
	// }

// 	fn "get" (this, pos) with env {
// 		Ok(match IndexPos::from_qobject(this.0.len(), pos, env) {
// 			IndexPos::InBounds(pos) => this.0.chars().nth(pos).unwrap().into(),
// 			IndexPos::OutOfBounds(_) | IndexPos::Underflow(_) => ().into(),
// 			IndexPos::NotAnInt(pos) => panic!("Can't index with non-integer num `{}`", pos)
// 		})
// 	}

// 	fn "set" (mut this, pos, val) with env args obj {
// 		let ref mut s = this.0;
// 		let len = s.len();

// 		let val_str = val.as_text(env).expect("`@text` is needed to set").as_str().to_owned();

// 		let pos = match IndexPos::from_qobject(s.len(), pos, env) {
// 			IndexPos::InBounds(pos) | IndexPos::OutOfBounds(pos) => pos,
// 			IndexPos::Underflow(pos) => panic!("Underflow! {} is out of bounds (len={})", pos, s.len()),
// 			IndexPos::NotAnInt(pos) => panic!("Can't index with non-integer num `{}`", pos)
// 		};

// 		let overflow_pos = pos + val_str.len();
// 		if overflow_pos >= s.len() {
// 			s.reserve(overflow_pos - len);
// 			let filler = args.get(2)
// 				.and_then(|x| x.as_text(env)
// 									.expect("`@text` is needed for filler char")
// 									.as_str()
// 									.chars()
// 									.next()
// 				).unwrap_or(' ');
// 			for _ in len..overflow_pos {
// 				s.push(filler);
// 			}
// 		}
// 		s.replace_range(pos..pos + val_str.len(), &val_str);
// 		Ok(obj.clone())
// 	}

// 	fn "del" (mut this, pos) with env {
// 		Ok(match IndexPos::from_qobject(this.0.len(), pos, env) {
// 			IndexPos::InBounds(pos) => this.0.remove(pos).into(),
// 			IndexPos::OutOfBounds(_) | IndexPos::Underflow(_) => ().into(),
// 			IndexPos::NotAnInt(pos) => panic!("Can't index with non-integer num `{}`", pos)
// 		})
// 	}

// 	fn "+" (this, rhs) with env {
// 		let mut s = this.0.clone();
// 		s.push_str(&rhs.as_text(env).expect("`@text` is needed for QList.+").as_str());
// 		Ok(s.into())
// 	}
// }


