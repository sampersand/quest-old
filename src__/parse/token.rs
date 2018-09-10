use std::fmt::{self, Debug, Display, Formatter};
use obj::{AnyShared, AnyResult, Result};
use std::hash::{Hash, Hasher};
use env::{Environment, Executable, Peeker};


// #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
// pub enum Precedence {
// 	Literal, Accessor, Block,
// 	Pow,
// 	MulDivMod,
// 	AddSub,

// 	AssignAug,
// 	Assign,
// 	Endline
// }

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Precedence {
	Literal = 0, Accessor, Block,
	Unary, Pow, UnaryNeg,
	MulDivMod, AddSub,
	BwShift, BwAnd, BwOrXor,
	Ordering, Equality,
	And, Or,

	TernaryElse,
	TernaryIf,
	Assign, AssignAug,
	Comma,
	Endline
}


impl Default for Precedence {
	fn default() -> Precedence {
		Precedence::Literal
	}
}

pub trait TokenFn : Executable + Send + Sync {
	fn duplicate(&self) -> Box<dyn TokenFn>;
}

impl<T: Executable + Clone + Send + Sync + 'static> TokenFn for T {
	fn duplicate(&self) -> Box<dyn TokenFn> {
		Box::new(self.clone())
	}
}

pub struct Token {
	data: String,
	prec: Precedence,
	func: Box<dyn TokenFn>,
}

impl Token {
	pub fn new<S: Into<String>, F: TokenFn + 'static>(data: S, prec: Precedence, func: F) -> Self {
		Token {
			data: data.into(),
			prec,
			func: Box::new(func),
		}
	}
	pub fn new_literal<S: Into<String>, F: FnOnce() -> AnyShared + Clone + Send + Sync + 'static>(data: S, prec: Precedence, func: F) -> Self {
		Token::new(
			data,
			prec,
			move |env: &Environment, _: &mut Peeker| Ok(env.push(func()))
		)
	}

	pub fn new_env<S: Into<String>, F: FnOnce(&Environment) -> Result<()> + Clone + Send + Sync + 'static>(data: S, prec: Precedence, func: F) -> Self {
		Token::new(
			data,
			prec,
			move |env: &Environment, _: &mut Peeker| func(env)
		)
	}

	pub fn execute(self, env: &Environment, iter: &mut Peeker) -> Result<()> {
		self.func.execute(env, iter)
	}

	pub fn prec(&self) -> Precedence {
		self.prec
	}
}

impl Clone for Token {
	fn clone(&self) -> Self {
		Token {
			data: self.data.clone(),
			func: self.func.duplicate(),
			prec: self.prec
		}
	}
}

#[inline(always)]
fn as_ptr<T: ?Sized>(x: &Box<T>) -> *const () {
	&**x as *const _ as *const ()
}

impl Eq for Token {}
impl PartialEq for Token {
	fn eq(&self, other: &Token) -> bool {
		if as_ptr(&self.func) == as_ptr(&other.func) {
			assert_eq!(self.data, other.data, "function pointers are equal, but data isn't?");
		}
		self.data == other.data
	}
}

impl Hash for Token {
	fn hash<H: Hasher>(&self, h: &mut H) {
		self.data.hash(h);
	}
}

impl Debug for Token {
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		struct BoxPtr(*const ());

		impl Debug for BoxPtr {
			fn fmt(&self, f: &mut Formatter) -> fmt::Result {
				write!(f, "{:p}", self.0)
			}
		}

		f.debug_struct("Token")
		 .field("data", &self.data)
		 .field("func", &BoxPtr(as_ptr(&self.func)))
		 .finish()
	}
}
