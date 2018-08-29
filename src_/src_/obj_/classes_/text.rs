use obj_::QObject__;
use obj_::classes_::{QVar, QNum, utils::IndexPos};
use env_::Environment__;

use regex::Regex;

use std::ops::Deref;
use std::fmt::{self, Debug, Display, Formatter};
use std::hash::{Hash, Hasher};
use std::str::FromStr;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct QText(String);

impl QText {
	pub fn new<S: ToString>(s: S) -> QText {
		QText(s.to_string())
	}
}

impl From<char> for QText {
	#[inline]
	fn from(inp: char) -> QText {
		QText::new(inp)
	}
}

impl From<String> for QText {
	#[inline]
	fn from(inp: String) -> QText {
		QText::new(inp)
	}
}

impl From<String> for QObject__ {
	#[inline]
	fn from(inp: String) -> QObject__ {
		QText::from(inp).into()
	}
}

impl From<char> for QObject__ {
	#[inline]
	fn from(inp: char) -> QObject__ {
		QText::from(inp).into()
	}
}

lazy_static! {
	pub static ref RE_SINGLE: Regex = regex!(r"\A'((?:\.|[^'])*)'");
}

impl AsRef<str> for QText {
	fn as_ref(&self) -> &str {
		self.0.as_ref()
	}
}

impl Deref for QText {
	type Target = String;

	#[inline]
	fn deref(&self) -> &String {
		&self.0
	}
}

impl Display for QText {
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		if f.alternate() {
			Debug::fmt(&self.0, f)
		} else {
			Display::fmt(&self.0, f)
		}
	}
}


default_attrs!{ for QText, with variant Text;
	use QObj;

	fn "@text" (this) {
		Ok(::obj_::QObject_::Old(this.clone().into()))
	}

	fn "@bool" (this) {
		Ok(::obj_::QObject_::Old((!this.0.is_empty()).into()))
	}

	fn "@var" (this) {
		Ok(::obj_::QObject_::Old(QVar::from_nonstatic_str(this.as_ref()).into()))
	}

	fn "@num" (this) {
		match QNum::from_str(&this) {
			Ok(num) => Ok(::obj_::QObject_::Old(num.into())),
			Err(err) => {
				warn!("Unable to convert {:?} to QNum ({:?}); returning QNull", this, err);
				Ok(::obj_::QObject_::Old(().into()))
			}
		}
	}

	fn "@cmd" (this) {
		unimplemented!("TODO: CMD")
	}

	// text attributes


	fn "empty!" (mut this) with _env _var obj{
		this.0.clear();
		Ok(::obj_::QObject_::Old(obj.clone()))
	}

	fn "empty?" (this) {
		Ok(::obj_::QObject_::Old(this.0.is_empty().into()))
	}

	fn "len" (this) {
		Ok(::obj_::QObject_::Old(this.0.len().into()))
	}

	fn "has" (this, var) with env {
		Ok(::obj_::QObject_::Old(if var.is_num() {
			IndexPos::from_qobject(this.0.len(), var, env).is_inbounds().into()
		} else if let Ok(text) = var.as_text(env) {
			this.0.contains(&text.as_str()).into()
		} else {
			panic!("Only Num or `@text` can be used to index")
		}))
	}

	fn "get" (this, pos) with env {
		Ok(::obj_::QObject_::Old(match IndexPos::from_qobject(this.0.len(), pos, env) {
			IndexPos::InBounds(pos) => this.0.chars().nth(pos).unwrap().into(),
			IndexPos::OutOfBounds(_) | IndexPos::Underflow(_) => ().into(),
			IndexPos::NotAnInt(pos) => panic!("Can't index with non-integer num `{}`", pos)
		}))
	}

	fn "set" (mut this, pos, val) with env args obj {
		let ref mut s = this.0;
		let len = s.len();

		let val_str = val.as_text(env).expect("`@text` is needed to set").as_str().to_owned();

		let pos = match IndexPos::from_qobject(s.len(), pos, env) {
			IndexPos::InBounds(pos) | IndexPos::OutOfBounds(pos) => pos,
			IndexPos::Underflow(pos) => panic!("Underflow! {} is out of bounds (len={})", pos, s.len()),
			IndexPos::NotAnInt(pos) => panic!("Can't index with non-integer num `{}`", pos)
		};

		let overflow_pos = pos + val_str.len();
		if overflow_pos >= s.len() {
			s.reserve(overflow_pos - len);
			let filler = args.get(2)
				.and_then(|x| x.as_text(env)
									.expect("`@text` is needed for filler char")
									.as_str()
									.chars()
									.next()
				).unwrap_or(' ');
			for _ in len..overflow_pos {
				s.push(filler);
			}
		}
		s.replace_range(pos..pos + val_str.len(), &val_str);
		Ok(::obj_::QObject_::Old(obj.clone()))
	}

	fn "del" (mut this, pos) with env {
		Ok(::obj_::QObject_::Old(match IndexPos::from_qobject(this.0.len(), pos, env) {
			IndexPos::InBounds(pos) => this.0.remove(pos).into(),
			IndexPos::OutOfBounds(_) | IndexPos::Underflow(_) => ().into(),
			IndexPos::NotAnInt(pos) => panic!("Can't index with non-integer num `{}`", pos)
		}))
	}

	fn "+" (this, rhs) with env {
		let mut s = this.0.clone();
		s.push_str(&rhs.as_text(env).expect("`@text` is needed for QList.+").as_str());
		Ok(::obj_::QObject_::Old(s.into()))
	}
}


