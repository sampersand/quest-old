use crate::{Shared, IntoObject, Object};
use crate::parse::{self, Parser};
use crate::parse::parsable::{ParseFromStr, Named};
use std::error::Error;
use std::fmt::{self, Display, Formatter};

pub use crate::object::typed::Variable;

#[derive(Debug)]
pub enum VariableParseError {
	MissingSigilSuffix { sigil: char },
	InvalidSigilSuffix { sigil: char, suffix: char }
}

impl Named for Variable {
	const NAME: &'static str = "Variable";
}

impl ParseFromStr for Variable {
	type Err = VariableParseError;
	fn from_str(text: &str) -> Option<Result<(Variable, usize), VariableParseError>> {
		use self::VariableParseError::*;

		fn is_valid_variable_start(c: char) -> bool {
			c.is_alphabetic() || c == '_'
		}

		fn is_valid_variable_character(c: char) -> bool {
			c.is_alphanumeric() || c == '_'
		}

		fn parse_normal_str(mut chars: std::str::Chars, variable: &mut String) -> Result<usize, VariableParseError> {
			unimplemented!()
		}

		let mut chars = text.chars();
		let mut count = 1;

		match chars.next()? {
			'`' => unimplemented!(),
			sigil @ '$' | sigil @ '@' => match chars.next() {
				None => return Some(Err(MissingSigilSuffix { sigil })),
				Some(suffix) if suffix.is_whitespace() => return Some(Err(InvalidSigilSuffix { sigil, suffix })),
				Some('`') => unimplemented!(),
				Some(chr) if is_valid_variable_character(chr) => {
					let mut variable = String::with_capacity(2);
					variable.push(sigil);
					variable.push(chr);
					count += 1;
					unimplemented!()
				},
				Some(chr) => { // eg `$|`
					let mut variable = String::with_capacity(2);
					variable.push(sigil);
					variable.push(chr);
					return Some(Ok((Variable::from_string(variable), 2)))
				}
			},
			chr if is_valid_variable_start(chr) => {
				return 
			}
				variable.push(c);
				for c in chars {
					// if !c.is_alphabetic()
				}
			},
			_ => return None
		}
		unimplemented!()

		// for c in chars {
		// 	if c.is_alphabetic
		// }


		// // For now, this can only parse whole numbers. Also, no hexadecimal or stuff
		// let mut chars = text.chars();
		// let mut number = chars.next()?.to_digit(10)?; // radix=10
		// let mut count = 1;

		// for chr in chars {
		// 	if let Some(digit) = chr.to_digit(10) {
		// 		number = number * 10 + digit;
		// 	} else if chr == '_' {
		// 		/* do nothing */ 
		// 	} else if chr.is_alphabetic() {
		// 		return Some(Err(VariableParseError::InvalidSuffix { pos: count, suffix: chr }))
		// 	} else {
		// 		break;
		// 	}
		// 	count += 1;
		// }

		// Some(Ok((Variable::new(number as f64), count)))
	}
}

impl Display for VariableParseError {
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		match self {
			VariableParseError::InvalidSuffix { suffix, pos: _ } => write!(f, "Invalid suffix parsed: '{}'", suffix),
			VariableParseError::BadRadix { radix, pos: _ } => write!(f, "Bad radix encountered: '{}'", radix),
		}
	}
}

impl Error for VariableParseError {
	fn description(&self) -> &str {
		match self {
			VariableParseError::InvalidSuffix { .. } => "invalid suffix parsed",
			VariableParseError::BadRadix { .. } => "bad radix encountered"
		}
	}
}



// use crate::{Shared, Object, IntoObject};
// use crate::parse::{self, Parsable, Parser};
// pub use crate::object::typed::Variable;

// named!(Variable);

// impl Parsable for Variable {
// 	fn try_parse(parser: &Shared<Parser>) -> parse::Result<Object> {
// 		// let (variable, index) = ParseResult::try_from(Variable::from_str(parser.read().as_ref()))?;
// 		unimplemented!()
// 		// match Variable::parse(parser.read().as_ref()) {
// 		// 	ParseResult::Err(err) => {
// 		// 	//	...
// 		// 		unimplemented!()
// 		// 	},
// 		// 	o @ ParseResult::None | o @ ParseResult::Ok => o
// 		// 	o @ ParseResult::Eof | o @ ParseResult::Restart => {
// 		// 		warn!(target: "parser", "Variable parser returned unexpected result: {:?}", o);
// 		// 		return o;
// 		// 	}
// 		// }
// 		// let variable = Variable::parse(parser.read().as_ref());

// 		// if let Some((variable, index)) = variable {
// 		// 	let mut parser = parser.write();
// 		// 	let res = parser.advance(index-1);
// 		// 	debug_assert_eq!(variable, Variable::parse(&res).unwrap().0);
// 		// 	debug!(target: "parser", "Variable parsed. chars={:?}", res);
// 		// 	ParseResult::Ok(variable.into_object())
// 		// } else {
// 		// 	trace!(target: "parser", "No variable found. stream={:?}", parser.read().beginning());
// 		// 	ParseResult::None
// 		// }
// 	}
// }