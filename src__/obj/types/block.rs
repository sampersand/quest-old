use parse::{Parsable, Stream, Token, Precedence};
use env::{Environment, Executable};

use std::ops::{Deref, DerefMut};
use std::hash::{Hash, Hasher};
use std::fmt::{self, Display, Formatter};
use obj::{AnyShared, SharedObject, types::IntoObject};
use super::shared::{self, Offset::*};

type Body = Vec<Token>;

#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
pub struct Block {
	raw: String,
	body: Body
}

pub(super) fn parse_block(start: char, stop: char, stream: &mut Stream) -> Option<(String, Body)> {
	if stream.as_str().starts_with(stop) {
		stream.eof = true;
	}

	if !stream.as_str().starts_with(start) {
		return None;
	}


	let mut raw = stream.as_str().to_string();
	stream.offset_by_char(start);
	let body = stream.collect();

	if !stream.as_str_raw().starts_with(stop) {
		panic!("Unmatched `{}` encountered!", start);
	}

	stream.eof = false;
	stream.offset_by_char(stop);

	let len = raw.len();
	raw.truncate(len - stream.as_str().len());
	raw.shrink_to_fit();

	Some((raw, body))
}

impl Parsable for Block {
	fn parse(stream: &mut Stream) -> Option<Token> {
		let (raw, body) = parse_block('{', '}', stream)?;

		Some(Token::new_literal(raw.clone(), Precedence::Block, move || Block::new(raw, body).into_object()))
	}
}

pub struct BlockExec;

impl Parsable for BlockExec {
	fn parse(stream: &mut Stream) -> Option<Token> {
		let (raw, body) = parse_block('(', ')', stream)?;

		Some(Token::new_env(raw.clone(), Precedence::Block, move |env| {
			let new_env = env.new_stack();
			new_env.execute(body.into_iter())?;

			if let Some(to_call) = env.pop() {
				let new_stack = new_env.stack().read().data.clone();
				env.push(to_call.read_call(&("()".into_object() as _), &new_stack, env)?);
			} else if let Some(obj) = new_env.pop() {
				env.push(obj);
			}
			Ok(())
		}))
	}
}


impl Block {
	#[inline]
	pub fn new(raw: String, body: Body) -> Self {
		Block { raw, body }
	}
}

// impl From<Body> for Block {
// 	#[inline]
// 	fn from(vec: Body) -> Block {
// 		Block::new(vec)
// 	}
// }

// impl IntoObject for Body {
// 	type Type = Block;
// 	#[inline]
// 	fn into_object(self) -> SharedObject<Block> {
// 		Block::from(self).into_object()
// 	}
// }

// impl<'a> IntoObject for &'a [Token] {
// 	type Type = Block;
// 	#[inline]
// 	fn into_object(self) -> SharedObject<Block> {
// 		Block::from(self.to_vec()).into_object()
// 	}
// }

impl Display for Block {
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		write!(f, "Block({:?})", self.raw)
	}
}

impl Deref for Block {
	type Target = Body;

	#[inline]
	fn deref(&self) -> &Body {
		&self.body
	}
}

impl DerefMut for Block {
	#[inline]
	fn deref_mut(&mut self) -> &mut Body {
		&mut self.body
	}
}

__impl_type! {
	for Block, with self attr;

	fn "()" (this) env, args, {
		let mut new_env = env.new_binding(args);
		let body = this.read().data.body.clone().into_iter();
		
		new_env.execute(body)?;
		Ok(new_env.pop().unwrap_or_else(Object::null))
		// this.read().data.call_bound(args, env)
	}

	fn _ () {
		any::__get_default(self, attr)
	}
}




