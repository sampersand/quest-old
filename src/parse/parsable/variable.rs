use crate::{Shared, IntoObject, Object};
use crate::parse::{self, Parser};
use crate::parse::parsable::{ParseFromStr, ParseOk, Named};
use std::error::Error;
use std::fmt::{self, Display, Formatter};

pub use crate::object::typed::Variable;

#[derive(Debug)]
pub enum VariableParseError {
	UnterminatedQuoted,
	NotAllowedAsSuffix { sigil: char, suffix: char }
}

impl Named for Variable {
	const NAME: &'static str = "Variable";
}

impl ParseFromStr for Variable {
	type Err = VariableParseError;
	fn from_str(text: &str) -> Result<ParseOk<Variable>, VariableParseError> {
		use self::VariableParseError::*;
		use std::str::Chars;
		const QUOTED_BOUND: char = '`';
		const DOLLAR_SIGIL: char = '$';
		const ATSIGN_SIGIL: char = '@';

		fn is_valid_variable_start(c: char) -> bool {
			c.is_alphabetic() || c == '_'
		}

		// note that this doesn't actually add '`' to the end of the string.
		fn parse_quoted_variable(mut chars: Chars, mut variable: String, mut count: usize) -> Result<ParseOk<Variable>, VariableParseError> {
			const QUOTED_ESCAPE: char = '\\';

			loop {
				count += 1;
				match chars.next().ok_or_else(|| UnterminatedQuoted)? {
					QUOTED_BOUND => return Ok(ParseOk::Found(Variable::from_string(variable), count)),
					QUOTED_ESCAPE => {
						variable.push(chars.next().ok_or_else(|| UnterminatedQuoted)?);
						count += 1;
					},
					other => variable.push(other)
				}
			}
		}

		fn parse_normal_variable(mut chars: Chars, mut variable: String, mut count: usize) -> Result<ParseOk<Variable>, VariableParseError> {
			for chr in chars {
				if chr.is_alphanumeric() || chr == '_' {
					variable.push(chr);
					count += 1;
				} else {
					break
				}
			}
 			Ok(ParseOk::Found(Variable::from_string(variable), count))
		}

		let mut chars = text.chars();
		match (if let Some(chr) = chars.next() { chr } else { return Ok(ParseOk::NotFound) }) {
			QUOTED_BOUND => return parse_quoted_variable(chars, String::new(), 1),

			sigil @ DOLLAR_SIGIL | sigil @ ATSIGN_SIGIL => {
				let mut variable = String::with_capacity(1);
				variable.push(sigil);

				match chars.next().ok_or_else(|| UnterminatedQuoted)? {
					suffix if suffix.is_whitespace() => return Err(NotAllowedAsSuffix { sigil, suffix }),
					QUOTED_BOUND => return parse_quoted_variable(chars, variable, 2),
					chr if is_valid_variable_start(chr) => {
						variable.push(chr);
						return parse_normal_variable(chars, variable, 2);
					},
					other => {
						variable.push(other);
						return Ok(ParseOk::Found(Variable::from_string(variable), 2));
					}
				}
			}

			chr if is_valid_variable_start(chr) => {
				let mut variable = String::with_capacity(1);
				variable.push(chr);
				return parse_normal_variable(chars, variable, 1);
			},

			_ => return Ok(ParseOk::NotFound)
		}
	}
}

impl Display for VariableParseError {
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		match self {
			VariableParseError::UnterminatedQuoted => write!(f, "Unterminated quoted string encountered"),
			VariableParseError::NotAllowedAsSuffix { sigil, suffix } => write!(f, "Sigil '{}' cannot be followed by '{}'", sigil, suffix)
		}
	}
}

impl Error for VariableParseError {
	fn description(&self) -> &str {
		match self {
			VariableParseError::UnterminatedQuoted => "unterminated quoted string encountered",
			VariableParseError::NotAllowedAsSuffix { .. } => "sigil followed by an invalid suffix"
		}
	}
}