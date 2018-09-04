use parse::{Parsable, Stream, Token};
use env::Environment;

use obj::{AnyShared, SharedObject, types::IntoObject};
use std::ops::{Deref, DerefMut};
use std::fmt::{self, Debug, Display, Formatter};
use super::shared::{self, Offset};

#[derive(Debug, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Text(String);

impl Parsable for Text {
	fn parse(stream: &mut Stream) -> Option<Token> {
		let mut offset;
		let mut data = String::new();
		{
			let mut chars = stream.chars();
			let quote = chars.next()?;
			offset = quote.len_utf8();
			
			if quote != '\'' && quote != '\"' {
				return None;
			}

			loop {
				let chr = if let Some(chr) = chars.next() {
					chr
				} else {
					break
				};

				offset += chr.len_utf8();

				match chr {
					'\\' => {
						let c = chars.next().expect("Unterminated quote found");
						offset += c.len_utf8();

						data.push(match c {
							'n' => '\n',
							't' => '\t',
							'r' => '\r',
							'x' => unimplemented!("TODO: escape sequence \\x"),
							'u' => unimplemented!("TODO: escape sequence \\u"),
							'#' => '#',
							_ => panic!("Unrecognized escape character `\\{}`", c)
						});
					},

					'#' => match chars.next() {
						Some('{') => unimplemented!("lol"),
						Some(other) => { data.push('#'); data.push(other) },
						None => data.push('#')
					},

					_ if chr == quote => break,
					_ => data.push(chr)
				}
			}
		}

		let text = stream.offset_to(offset);
		Some(Token::new_literal(text, Default::default(), move || data.into_object()))
	}
}


impl From<String> for Text {
	#[inline]
	fn from(text: String) -> Text {
		Text(text)
	}
}

impl<'a> From<&'a str> for Text {
	#[inline]
	fn from(inp: &'a str) -> Text {
		Text::from(inp.to_string())
	}
}

impl IntoObject for String {
	type Type = Text;
	fn into_object(self) -> SharedObject<Text> {
		Text::from(self).into_object()
	}
}

impl Deref for Text {
	type Target = String;

	#[inline]
	fn deref(&self) -> &String {
		&self.0
	}
}

impl DerefMut for Text {
	#[inline]
	fn deref_mut(&mut self) -> &mut String {
		&mut self.0
	}
}

impl Display for Text {
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		Debug::fmt(&self.0, f)
	}
}

impl_type! {
	for Text, with self attr;

	fn "@text" (this) {
		Ok(this.read().duplicate())
	}

	fn "@num" (this) {
		Ok(Number::parse_str(this.read().data.chars())
			.map(|(num, _)| num.into_object() as AnyShared)
			.unwrap_or_else(Object::null))
	}

	fn "@list" (this) {
		Ok(this.read().data.chars().map(|c| c.to_string().into_object() as AnyShared).collect::<Vec<_>>().into_object() as AnyShared)
	}

	fn "@bool" (this) {
		Ok((!this.read().data.is_empty()).into_object())
	}

	fn "+" (this) env, args, {
		let dup = this.read().duplicate();
		dup.read_call(&("+=".into_object() as AnyShared), args, env)
	}

	fn "+=" (this, other) env, {
		let other = other.read_into_text(env)?;
		this.write().data.push_str(&other);
		Ok(this)
	}

	fn "len" (this) {
		Ok(this.read().data.len().into_object())
	}

	fn "count" (this, text) env, {
		let text = text.read_into_text(env)?.0;
		let ref data = this.read().data;
		Ok(data.matches(&text).count().into_object())
	}

	fn "has?" (this, key) env, {
		let key = key.read_into_text(env)?.0;
		Ok(this.read().data.contains(&key).into_object())
	}

	fn "del" (this, key) env, {
		unimplemented!()
	}

	fn "set" (this, key, val) env, {
		unimplemented!()
	}

	fn "get" (this, start; end) env, {
		let ref data = this.read().data;
		let start = start.read_into_num(env)?;
		let end = end.and_then(|x| x.read_into_num(env).ok()).unwrap_or(start + Number::one());

		let start = shared::offset(data.len(), start)?;
		let end = shared::offset(data.len(), end)?;

		use self::Offset::*;

		match (start, end) {
			(Valid(s), Valid(e)) if s < e => Ok(data[s..e].to_string().into_object()), // begin < end
			(Valid(_), Valid(_)) => Ok("".to_string().into_object()), // begin >= end

			(Valid(s), OutOfBounds(_)) => Ok(data[s..].to_string().into_object()), 
			_ => Ok(Object::null()) // everything else is nil
		}
	}

	fn _ () {
		any::get_default_attr(self, attr)
	}
}




