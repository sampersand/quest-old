use env::{Environment, Stream, Binding};
use obj::{AnyObject, SharedObject, classes::{self, Class}};
use std::fmt::{self, Debug, Formatter};
use std::any::Any;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Precedence {
	Literal = 0, Accessor, BlockStart, // itself is for just tokens
	Unary, Pow, UnaryNeg,
	MulDivMod, AddSub,
	BwShift, BwAnd, BwOrXor,
	Ordering, Equality,
	LogicalAnd, LogicalOr,

	TernaryElse,
	TernaryIf,
	Assignment, AssignmentMath, AssignmentBwShift, AssignmentBw,
	Comma,
	EndOfLine
}

#[derive(Debug, Clone)]
pub enum Token {
	Object(AnyObject, Precedence, Executor),
	NoObject,
	Eof
}

impl<C: Debug + Send + Sync> From<C> for Token where SharedObject<C>: Class {
	#[inline]
	fn from(obj: C) -> Token {
		Token::Object(SharedObject::from(obj) as _, Precedence::Literal, Executor(|obj, _| obj))
	}
}

pub trait Parsable {
	fn try_parse(env: &mut Environment) -> Option<Token>;
	fn precedence(&self) -> Precedence { Precedence::Literal }
	fn execute(obj: &dyn Any, env: &mut Environment) -> AnyObject { unimplemented!("execute") }
}


fn_struct!{
	pub struct Parser(pub fn(&mut Environment) -> Option<Token>);
}


fn_struct!{
	pub struct Executor(pub fn(AnyObject, &mut Binding) -> AnyObject);
}

impl Parser {
	pub(super) fn default_parsers() -> Vec<Parser> {
		use obj::{Id, classes::{null::Null, num::Number, oper::Oper}};

		vec![
			Parser(Whitespace::try_parse), //, Whitespace::precedence, Whitespace::execute),
			Parser(Comment::try_parse), //, Comment::precedence, Comment::execute),
			Parser(LParen::try_parse), //, LParen::precedence, LParen::execute),
			Parser(Null::try_parse), //, QNull::precedence, QNull::execute),
			Parser(bool::try_parse), //, QBool::precedence, QBool::execute),
			Parser(String::try_parse), //, QText::precedence, QText::execute),
			Parser(Number::try_parse), //, QNum::precedence, QNum::execute),
			Parser(Oper::try_parse), //, QOper::precedence, QOper::execute),
			Parser(Id::try_parse), //, QVar::precedence, QVar::execute),

		]
	}
}

impl<'a> Environment<'a> {
	pub fn next_token(&mut self) -> Option<Token> {
		let parsers = self.stream.parsers.clone(); // we have to clone in case the parser modifies the list of parsers whilst the stream is being run
		for parser in parsers {
			if let Some(token) = (parser.0)(self) {
				return Some(token);
			}
		}
		None
	}

	pub fn parse(mut self) -> Binding { // we consume because `Stream` should be consumed by the end
		debug_assert!(self.binding.stack_is_empty(), "didn't start with an empty stack?");
		'outer: while !self.stream.as_str().is_empty() {
			match self.next_token() {
				Some(Token::Object(obj, precedence, exec)) => {
					self.binding.handle(obj, precedence, exec);
					continue 'outer; // we've found an object, retry thru parsers
				}
				Some(Token::NoObject) => continue 'outer, // we found whitespace, so just restart iter search
				Some(Token::Eof) => break 'outer, // we've hit EOF, so prematurely end the while loop
				None => panic!("No tokens found for {:#?}", self.stream)
			}
			// if we reach here, we've exhausted all parsers, and none of them worked
			panic!("No Tokens could be found starting at {}", &self.stream.as_str()[0..10]);
		}

		self.binding.finish()
	}
}



#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Whitespace;

impl Parsable for Whitespace {
	fn try_parse(env: &mut Environment) -> Option<Token> {
		env.stream.try_get(regex!(r"\A\s+")).and(Some(Token::NoObject))
	}
	// fn execute(self, _: &mut Environment) -> AnyObject {
	// 	unreachable!("Can't execute whitespace");
	// }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Comment;

impl Parsable for Comment {
	fn try_parse(env: &mut Environment) -> Option<Token> {
		env.stream.try_get(regex!(r"(?m)\A(//|#).*$")).and(Some(Token::NoObject))
	}
	// fn execute(self, _: &mut Environment) -> AnyObject {
	// 	unreachable!("Can't execute comments");
	// }
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LParen { Round, Square, Curl }

impl Parsable for LParen {
	fn try_parse(env: &mut Environment) -> Option<Token> {
		env.stream.try_get(regex!(r"\A[})\]]")).and(Some(Token::Eof))
	}
	// fn execute(self, _: &mut Environment) -> AnyObject {
	// 	unreachable!("Can't execute lparens");
	// }
}