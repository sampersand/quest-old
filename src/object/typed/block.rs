use crate::{Shared, Result};
use crate::parse::Parser;
use crate::object::{TypedObject, Object};
use std::fmt::{self, Debug, Display, Formatter};
use lazy_static::lazy_static;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Parens { Curly, Square, Round }

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct Block { parens: Parens, body: String }

impl Block {
	pub fn new<T: Into<String>>(parens: Parens, body: T) -> Block {
		Block { parens, body: body.into() }
	}
}

impl Parens {
	pub fn try_from_start(chr: char) -> Option<Parens> {
		match chr {
			'{' => Some(Parens::Curly),
			'[' => Some(Parens::Square),
			'(' => Some(Parens::Round),
			_ => None
		}
	}
	pub fn try_from_end(chr: char) -> Option<Parens> {
		match chr {
			'}' => Some(Parens::Curly),
			']' => Some(Parens::Square),
			')' => Some(Parens::Round),
			_ => None
		}
	}

}


impl Display for Block {
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		match self.parens {
			Parens::Curly => write!(f, "{{{}}}", self.body),
			Parens::Square => write!(f, "[{}]", self.body),
			Parens::Round => write!(f, "({})", self.body),
		}
	}
}

impl Debug for Block {
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		write!(f, "Block({:?}, {:?})", self.parens, self.body)
	}
}


impl TypedObject {
	pub fn new_block(parens: Parens, body: String) -> Self {
		TypedObject::new(Block::new(parens, body))
	}
}

impl Object {
	pub fn new_block(parens: Parens, body: String) -> Self {
		Object::new(TypedObject::new_block(parens, body))
	}
}

impl_typed_object!(Block, _, downcast_block, is_block);
impl_quest_conversion!("@block" (as_block_obj is_block) (into_block downcast_block) -> Block);

impl_type! { for Block, downcast_fn=downcast_block;
	fn "@block" (this) {
		this.into_object()
	}

	fn "()" (@this) args {
		use crate::{Environment, parse::Parser};

		let (body, is_square) = this.downcast_block().map(|block| (block.body.clone(), block.parens == Parens::Square)).expect("<todo: error here>");
		let parser = Shared::new(Parser::from_str(body));
		let parent = Some(this.env().clone());

		let stack = Some(Shared::new(crate::collections::List::new(args.iter().skip(1).map(|x| (*x).clone()).collect::<Vec<_>>())) as _);
		let env = Environment::execute(Environment::new(parser, parent, None, stack))?;
		let x = env.read().stack.write().pop().ok_or_else(|| crate::err::Error::NothingToReturn)?;
		x
		// if is_square {
		// 	use crate::{Environment, parse::Parser};
		// 	let env = Environment::execute(
		// 		Environment::_new_default_with_stream_and_parent(Shared::new(parser), Some(this.env().clone()))
		// 	)?;
		// 	let x = env.read().stack.write().pop().ok_or_else(|| crate::err::Error::NothingToReturn)?;
		// 	x
		// } else {
		// 	crate::parse::parse_str(body, Some(this.env().clone()))?
		// }
	}

	fn "__evaluate__" (@this, parser) {
		let do_exec = this.downcast_block().map(|block| block.parens == Parens::Round).unwrap_or(false);
		if do_exec {
			this.call_attr("()", &[])?
		} else {
			this.clone()
		}
	}
}

