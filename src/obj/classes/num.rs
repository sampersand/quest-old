use obj::{Classes, QObject, Exception};
use obj::classes::QNull;
use env::Environment;

use regex::Regex;
use std::{num, str::FromStr};
use std::hash::{Hash, Hasher};
use std::fmt::{self, Display, Formatter};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct QNum(f64);


impl QNum {
	#[inline]
	pub fn new(val: f64) -> QNum {
		QNum(val)
	}

	pub fn to_f64(&self) -> f64 {
		self.0
	}

	pub fn try_to_usize(&self) -> Option<usize> {
		self.try_to_int().map(|x| x as usize)
	}

	pub fn try_to_i32(&self) -> Option<i32> {
		self.try_to_int().map(|x| x as i32)
	}

	pub fn try_to_int(&self) -> Option<i64> {
		if self.0.floor() == self.0 {
			Some(self.0 as i64)
		} else {
			None
		}
	}
}

impl AsRef<f64> for QNum {
	fn as_ref(&self) -> &f64 {
		&self.0
	}
}

impl Display for QNum {
	#[inline]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		Display::fmt(&self.0, f)
	}
}

macro_rules! impl_from_builtin {
	($($ty:ident)*) => {
		$(
			impl From<$ty> for QNum {
				#[inline]
				fn from(num: $ty) -> QNum {
					QNum(num as _)
				}
			}

			impl From<$ty> for QObject {
				#[inline]
				fn from(num: $ty) -> QObject {
					QNum::from(num).into()
				}
			}

			impl From<QNum> for $ty {
				#[inline]
				fn from(inp: QNum) -> $ty {
					inp.0 as $ty
				}
			}
		)*
	}
}
impl_from_builtin!(i8 u8 i16 u16 i32 u32 i64 u64 f32 f64 usize isize);

impl Eq for QNum {} // lol bad...

impl Hash for QNum {
	fn hash<H: Hasher>(&self, hasher: &mut H) {
		self.0.to_bits().hash(hasher)
	}
}

#[derive(Debug)]
pub enum ParseError {
	Float(num::ParseFloatError),
	Int(num::ParseIntError),
	NoMatches(String)
}

impl From<num::ParseFloatError> for ParseError {
	fn from(err: num::ParseFloatError) -> ParseError {
		ParseError::Float(err)
	}
}

impl From<num::ParseIntError> for ParseError {
	fn from(err: num::ParseIntError) -> ParseError {
		ParseError::Int(err)
	}
}

lazy_static! {
	pub static ref RE_DECI: Regex = regex!(r"\A(?:0[dD](\d+))\b");
	pub static ref RE_HEX: Regex = regex!(r"\A(?:0[xX]([\da-f]+))\b");
	pub static ref RE_BINARY: Regex = regex!(r"\A(?:0[bB]([01]+))\b");
	pub static ref RE_OCTAL: Regex = regex!(r"\A(?:0[oO]([0-7]+))\b");
	pub static ref RE_FLOAT: Regex = regex!(r"\A([-+]?\d+(?:\.\d+)?(?:[eE][+-]?\d+)?)\b");
}

impl FromStr for QNum {
	type Err = ParseError;
	fn from_str(inp: &str) -> Result<QNum, ParseError> {
		macro_rules! match_radix {
			($regex:ident, $radix:expr) => {
				if let Some(caps) = $regex.captures(inp) {
					debug_assert_eq!(caps.get(1).unwrap().start(), 2);
					return Ok(QNum::new(u64::from_str_radix(&caps[1], $radix)? as f64))
				}
			}
		}

		match_radix!(RE_DECI, 10);
		match_radix!(RE_HEX, 16);
		match_radix!(RE_BINARY, 2);
		match_radix!(RE_OCTAL, 8);

		if let Some(caps) = RE_FLOAT.captures(inp) {
			debug_assert_eq!(caps.get(0).unwrap().start(), 0);
			Ok(QNum::new(f64::from_str(&caps[0])?))
		} else {
			Err(ParseError::NoMatches(inp.to_string()))
		}
	}
}


fn into_qnum(pos: &QObject, env: &Environment) -> QNum {
	pos.as_num(env).expect("`@num` is required to interact with `QNum`")
}

fn into_f64(pos: &QObject, env: &Environment) -> f64 {
	into_qnum(pos, env).to_f64()
}

default_attrs! { for QNum, with variant Num;
	use QObj;

	fn "@bool" (this) {
		Ok((this.0 != 0.0).into())
	}

	fn "@num" (this) {
		Ok(this.clone().into())
	}

	fn "@text" (this) {
		if let Some(int) = this.try_to_int() {
			Ok(int.to_string().into())
		} else {
			Ok(this.0.to_string().into())
		}
	}

	fn "()" (_this, rhs) with env _vars obj {
		obj.call_attr("*", &[rhs], env)
	}

	fn "." (_this, rhs) with env vars obj {
		if rhs.is_num() {
			unimplemented!("TODO: `.` for QNum")
		} else {
			call_super!(QObj(".") for obj, vars, env)
		}
	}

	fn "round" (this; places = ().into()) with env {
		if !places.is_null() {
			let places = into_f64(&places, env);
			if places != 0.0 {
				unimplemented!("TODO: Round with non-zero amount")
			}
		}
		Ok(this.0.round().into())
	}

	fn "abs" (this) {
		Ok(this.0.abs().into())
	}

	fn "+" (this, rhs) with env { Ok((this.0 + into_f64(rhs, env)).into()) }
	fn "-" (this, rhs) with env { Ok((this.0 - into_f64(rhs, env)).into()) }
	fn "*" (this, rhs) with env { Ok((this.0 * into_f64(rhs, env)).into()) }
	fn "/" (this, rhs) with env { Ok((this.0 / into_f64(rhs, env)).into()) }
	fn "^" (this, rhs) with env { Ok((this.0.powf(into_f64(rhs, env))).into()) }
	fn "%" (this, rhs) with env { Ok((this.0 % into_f64(rhs, env)).into()) }

	fn "<"  (this, rhs) with env { Ok((this.0 <  into_f64(rhs, env)).into()) }
	fn "<=" (this, rhs) with env { Ok((this.0 <= into_f64(rhs, env)).into()) }
	fn ">"  (this, rhs) with env { Ok((this.0 >  into_f64(rhs, env)).into()) }
	fn ">=" (this, rhs) with env { Ok((this.0 >= into_f64(rhs, env)).into()) }
	fn "==" (this, rhs) with env { Ok((this.0 == into_f64(rhs, env)).into()) }
	fn "!=" (this, rhs) with env { Ok((this.0 !=  into_f64(rhs, env)).into()) }
	fn "<=>" (this, rhs) with env {
		let other = into_f64(rhs, env);
		if this.0 < other {
			Ok((-1.0).into())
		} else if this.0 == other {
			Ok(0.0.into())
		} else {
			Ok(1.0.into())
		}
	}

	fn "~" (this){
		if let Some(int) = this.try_to_int() {
			Ok((!int).into())
		} else {
			panic!("Can't apply `~` to non-int `{}`", this)
		}
	}
	fn "|" (this, rhs) with env {
		let rhs = into_qnum(rhs, env);
		if let (Some(l), Some(r)) = (this.try_to_int(), rhs.try_to_int()) {
			Ok((l | r).into())
		} else {
			panic!("Can't apply `|` to with nonints found: `{}`, `{}`", this, rhs)
		}
	}
	fn "&" (this, rhs) with env {
		let rhs = into_qnum(rhs, env);
		if let (Some(l), Some(r)) = (this.try_to_int(), rhs.try_to_int()) {
			Ok((l & r).into())
		} else {
			panic!("Can't apply `&` to with nonints found: `{}`, `{}`", this, rhs)
		}
	}
	fn "^^" (this, rhs) with env {
		let rhs = into_qnum(rhs, env);
		if let (Some(l), Some(r)) = (this.try_to_int(), rhs.try_to_int()) {
			Ok((l ^ r).into())
		} else {
			panic!("Can't apply `&` to with nonints found: `{}`, `{}`", this, rhs)
		}
	}
	fn "<<" (this, rhs) with env {
		let rhs = into_qnum(rhs, env);
		if let (Some(l), Some(r)) = (this.try_to_int(), rhs.try_to_int()) {
			Ok((l ^ r).into())
		} else {
			panic!("Can't apply `&` to with nonints found: `{}`, `{}`", this, rhs)
		}
	}
	fn ">>" (this, rhs) with env {
		let rhs = into_qnum(rhs, env);
		if let (Some(l), Some(r)) = (this.try_to_int(), rhs.try_to_int()) {
			Ok((l ^ r).into())
		} else {
			panic!("Can't apply `&` to with nonints found: `{}`, `{}`", this, rhs)
		}
	}
}



