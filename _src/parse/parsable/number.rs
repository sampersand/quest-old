use crate::{Shared, IntoObject, Object};
use crate::parse::{self, Parser};
use crate::parse::parsable::{ParseFromStr, ParseOk, Named};
use std::error::Error;
use std::fmt::{self, Display, Formatter};

pub use crate::object::typed::Number;

#[derive(Debug)]
pub enum NumberParseError {
	InvalidSuffix { pos: usize, suffix: char },
	BadRadix { pos: usize, radix: char },
}

named!(Number);

impl ParseFromStr for Number {
	type Err = NumberParseError;
	fn from_str(text: &str) -> Result<ParseOk<Number>, NumberParseError> {
		// For now, this can only parse whole numbers. Also, no hexadecimal or stuff
		let mut chars = text.chars();
		let mut number = if let Some(number) = chars.next().and_then(|x| x.to_digit(10)) { // radix=10
			number
		} else {
			return Ok(ParseOk::NotFound)
		};

		let mut count = 1;

		for chr in chars {
			if let Some(digit) = chr.to_digit(10) {
				number = number * 10 + digit;
			} else if chr == '_' {
				/* do nothing */ 
			} else if chr.is_alphabetic() {
				return Err(NumberParseError::InvalidSuffix { pos: count, suffix: chr })
			} else {
				break;
			}
			count += 1;
		}

		Ok(ParseOk::Found(Number::new(number as f64), count))
	}
}

impl Display for NumberParseError {
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		match self {
			NumberParseError::InvalidSuffix { suffix, pos: _ } => write!(f, "Invalid suffix parsed: '{}'", suffix),
			NumberParseError::BadRadix { radix, pos: _ } => write!(f, "Bad radix encountered: '{}'", radix),
		}
	}
}

impl Error for NumberParseError {
	fn description(&self) -> &str {
		match self {
			NumberParseError::InvalidSuffix { .. } => "invalid suffix parsed",
			NumberParseError::BadRadix { .. } => "bad radix encountered"
		}
	}
}
