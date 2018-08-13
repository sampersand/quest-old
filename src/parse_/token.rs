use std::fmt::Debug;
use parse::Stream;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Paren { Curly, Square, Round }

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Literal { Variable, Number, Text }

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Operator {
	Period, Comma, Semicolon,

	Add, Sub, Mul, Div, Pow, Mod,
	BitAnd, BitOr, BitXor, BitShl, BitShr,

	AddI, SubI, MulI, DivI, PowI, ModI,
	BitAndI, BitOrI, BitXorI, BitShlI, BitShrI,

	Lt, Gt, Le, Ge, Eq, Ne, Cmp,
	And, Or,

	Assign, AssignL, AssignR,
	Not, Neg, Exists, Incr, Decr,
}


#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Token<'a> {
	Eof,
	Whitespace(&'a str),
	Comment(&'a str),

	Literal(Literal, &'a str),

	OpenParen(Paren),
	CloseParen(Paren),

	Oper(Operator)
}

impl<'a> Token<'a> {
	pub fn len(&self) -> usize {
		if let Token::Eof = self {
			return ::std::usize::MAX;
		}
		self.as_str().len()
	}

	pub fn as_str(&self) -> &'a str {
		use self::Token::*;
		use self::Paren::*;
		use self::Operator::*;

		match self {
			Eof => "<eof>",
			Whitespace(data) | Comment(data) | Literal(_, data) => data,

			OpenParen(paren) => match paren {
				Curly => "{",
				Square => "[",
				Round => "(",
			},
			CloseParen(paren) => match paren {
				Curly => "}",
				Square => "]",
				Round => ")"
			},

			Oper(Sep(sep)) => match sep {
				Period => ".",
				Comma => ",",
				Semicolon => ";"
			},
			Oper(Aug)
		}
	}
}



#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Seperator { Period, Comma, Semicolon }

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Augmentable {
	Add, Sub, Mul, Div, Pow, Mod,
	BitAnd, BitOr, BitXor, BitShl, BitShr
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Comparison {
	Lt, Gt, Le, Ge, Eq, Ne, Cmp,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Assignment { Assign, AssignL, AssignR }

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Unary { Not, Neg, Exists, Incr, Decr }

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Operator {
	Sep(Seperator),
	Aug(Augmentable),
	Cmp(Comparison),
	Assign(Assignment),
	Unary(Unary),
	And, Or
}