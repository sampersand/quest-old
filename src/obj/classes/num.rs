use env::{Environment, parse::{Parsable, Token}};
use obj::{AnyObject, SharedObject};

use std::str::FromStr;
use std::error::Error;
use std::fmt::{self, Debug, Display, Formatter};

pub type QNum = SharedObject<Number>;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct Number(i32, u32);


impl QNum {
	pub fn from_number<N: Into<Number>>(num: N) -> QNum {
		num.into().into()
	}
}

macro_rules! num_convert {
	($($normal:ty)*; $($float:ty)*) => {
		$(
			impl From<$normal> for Number {
				#[inline]
				fn from(num: $normal) -> Number {
					Number(num as i32, 0)
				}
			}

			impl From<Number> for $normal {
				#[inline]
				fn from(num: Number) -> $normal {
					assert_eq!(num.1, 0, "Can't convert non-integer number {:?} into an integer", num);
					num.0 as $normal
				}
			}
		)*
		$(
			impl From<$float> for Number {
				#[inline]
				fn from(num: $float) -> Number {
					unimplemented!("TODO: From floating point numbers")
				}
			}
			impl From<Number> for $float {
				#[inline]
				fn from(num: Number) -> $float {
					unimplemented!("TODO: Into floating point numbers");
				}
			}
		)*
	}
}

num_convert!(i8 i16 i32 i64 isize u8 u16 u32 u64 usize ; f32 f64);

impl Debug for Number {
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		write!(f, "Number({})", self)
	}
}

impl Display for Number {
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		write!(f, "{}.{}", self.0, self.1)
	}
}

impl Parsable for Number {
	fn try_parse(env: &mut Environment) -> Option<Token> {
		if let Some(data) = env.stream.try_get(regex!(r"(?i)\A0(x[\da-f]+|o[0-7]+|b[01]+|d\d+)\b")) {
			assert_eq!(data.chars().next(), Some('0'));
			match data.chars().nth(1).expect("no regex supplied, but it must have matched") {
				'x' | 'X' => unimplemented!(),
				'o' | 'O' => unimplemented!(),
				'b' | 'B' => unimplemented!(),
				'd' | 'D' => unimplemented!(),
				other => unreachable!("found invalid radix that was matched: `{}`", other)
			}
		}

		if let Some(data) = env.stream.try_get(regex!(r"\A[-+]?\d+(\.\d+)?([eE][-+]?\d+)?\b")) {
			let mut base_exp = data.splitn(2, 'e');
			let base = base_exp.next().expect("there should always be a first match");
			let exp: i32 = base_exp.next().map(|exp| exp.parse().expect("invalid exp got past the regex?")).unwrap_or(0);
			let mut deci = base.splitn(2, '.');

			let above_one: i32 = deci.next().expect("there should always be a first match").parse().expect("invalid base got past the regex?");
			let mantissa: u32 = deci.next().map(|m| m.parse().expect("invalid mantissa got past regex?")).unwrap_or(0);

			if exp != 0 {
				panic!("TODO: incorperate exponent (ie sci notation)");
			}

			return Some(Number(above_one, mantissa).into())
		}

		None
	}
}

define_attrs! { for QNum;
	use QObject<Number>;
	fn "@num" (this) {
		Ok(QNum::from(this.clone()))
	}
}











