use super::RustFn;
use std::fmt::{self, Debug, Display, Formatter};
use std::cmp::Ordering;
use crate::{Object, Shared, Error, Result, IntoObject, Environment};
use crate::parse::Parser;

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum Oper {
	Pos, Neg,

	  Add,   Sub,   Mul,   Div,   Mod,   Pow,
	AddEq, SubEq, MulEq, DivEq, ModEq, PowEq,

	  BitShl,   BitShr,   BitAnd,   BitOr,   BitXor, BitNot,
	BitShlEq, BitShrEq, BitAndEq, BitOrEq, BitXorEq,

	Eql, Neq, Lth, Leq, Gth, Geq, Cmp,
	And, Or, Not,

	Assign, ArrowRight, ArrowLeft,
	Period, ColonColon, Comma, Endline,

	Other(Precedence, RustFn)
}

use self::Oper::*;

#[repr(u8)]
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Precedence {
	Period_ColonColon,
	Pos_BitNot_Not,
	Pow,
	Neg,
	Mul_Div_Mod,
	Add_Sub,
	BitShl_BitShr,
	BitAnd,
	BitOr_BitXor,
	Lth_Gth_Leq_Geq,
	Eq_Ne_Cmp,
	And,
	Or,
	Assignment,
	CompoundAssignment,
	Comma,
	Endline
}

impl Oper {
	// ie `~x`
	fn is_unary_and_on_lhs(&self) -> bool {
		match self {
			Pos | Neg | BitNot | Not => true,
			_ => false
		}
	}

	// this disallows users to change the `precedence` function to get the precedence of opers.
	// that might change in the future
	pub fn handle(&self, parser: &Shared<Parser>) -> Result {
		if self.is_unary_and_on_lhs() {
			unimplemented!("TODO: unary opers");
		}

		let lhs = Environment::current().read().stack.write().pop().ok_or_else(|| Error::MissingArgument {
			func: self.sigil(),
			pos: 0,
		})?;

		trace!(target: "execute", "Oper={:?} found a lhs={:?}", self, lhs);


		while let Some(mut object) = Parser::next_object(&parser).transpose()? {
			trace!(target: "execute", "Oper={:?} received next object={:?}", self, object);

			if let Some(oper) = object.downcast_oper() {
				if self > &oper || (self >= &oper && /* is (r/l?) associative*/ false) {
					trace!(target: "execute", "Oper={:?} found an oper more tightly bound={:?}", self, oper);
					object = oper.handle(parser)?;
				} else {
					trace!(target: "execute", "Oper={:?} found a less-tightly-bound oper={:?}", self, oper);
					drop(oper);
					parser.read().rollback(object); // ie rollback the oper
					break;
				}
			}

			Environment::current().read().stack.write().push(object);
		}

		let rhs = Environment::current().read().stack.write().pop().ok_or_else(|| Error::MissingArgument {
			func: self.sigil(),
			pos: 1
		})?;

		trace!(target: "execute", "Oper={:?} found a rhs={:?}", self, rhs);
		trace!(target: "execute", "Oper={:?} is executing ({:?}, {:?})", self, lhs, rhs);
		return self.call(&[&lhs, &rhs]);

		// Err(Error::ParserError { msg: "No rhs found for oper", parser: parser.clone() })
	}
}

impl Oper {
	fn _all_opers_but_other() -> &'static [Oper] {
		&[Pos, Neg, Add, Sub, Mul, Div, Mod, Pow,
		  AddEq, SubEq, MulEq, DivEq, ModEq, PowEq,
		  BitShl, BitShr, BitAnd, BitOr, BitXor, BitNot,
		  BitShlEq, BitShrEq, BitAndEq, BitOrEq, BitXorEq,
		  Eql, Neq, Lth, Leq, Gth, Geq, Cmp, And, Or, Not,
		  Assign, ArrowRight, ArrowLeft, Period, ColonColon,
		  Comma, Endline]
	}

	// i think it might be interesting to have this take from the current environment
	// however, for the sake of makign sure this thing works, i wont for now
	pub fn from_str(text: &str) -> Option<(Oper, usize)> {
		let mut all_opers = Oper::_all_opers_but_other()
			.iter()
			.map(|oper| (*oper, oper.sigil()))
			.collect::<Vec<_>>();

		// you can make it `rsigil.cmp(lsigil)` for more efficiency, but this makes more sense to me.
		all_opers.sort_by(|(_, lsigil), (_, rsigil)| lsigil.len().cmp(&rsigil.len()));
		all_opers.reverse();

		all_opers.into_iter()
			.find(|(_, sigil)| text.starts_with(sigil))
			.map(|(oper, sigil)| (oper, sigil.len()))
	}

	fn precedence(&self) -> Precedence {
		match self {
			Period
			  | ColonColon    => Precedence::Period_ColonColon,
			Pos
			  | BitNot
			  | Not           => Precedence::Pos_BitNot_Not,
			Pow               => Precedence::Pow,
			Neg          => Precedence::Neg,
			Mul
			  | Div
			  | Mod           => Precedence::Mul_Div_Mod,
			Add
			  | Sub           => Precedence::Add_Sub,
			BitShl
			  | BitShr        => Precedence::BitShl_BitShr,
			BitAnd            => Precedence::BitAnd,
			BitOr
			  | BitXor        => Precedence::BitOr_BitXor,
			Lth
			  | Gth
			  | Leq
			  | Geq           => Precedence::Lth_Gth_Leq_Geq,
			Eql
			  | Neq
			  | Cmp           => Precedence::Eq_Ne_Cmp,
			And               => Precedence::And,
			Or                => Precedence::Or,
			Assign
			  | ArrowRight
			  | ArrowLeft     => Precedence::Assignment,
			AddEq
			  | SubEq
			  | MulEq
			  | DivEq
			  | ModEq
			  | PowEq
			  | BitShlEq
			  | BitShrEq
			  | BitAndEq
			  | BitOrEq
			  | BitXorEq      => Precedence::CompoundAssignment,
			Comma             => Precedence::Comma,
			Endline           => Precedence::Endline,
			Other(prec, _)    => *prec
		}
	}

	fn sigil(&self) -> &'static str {
		match self {
			Pos => "+@", Neg => "-@",
			Add => "+", Sub => "-", Mul => "*", Div => "/", Mod => "%", Pow => "**",
			AddEq => "+=", SubEq => "-=", MulEq => "*=", DivEq => "/=", ModEq => "%=", PowEq => "**=",
			BitShl => "<<", BitShr => ">>", BitAnd => "&", BitOr => "|", BitXor => "^", BitNot => "~",
			BitShlEq => "<<=", BitShrEq => ">>=", BitAndEq => "&=", BitOrEq => "|=", BitXorEq => "^=",
			Eql => "==", Lth => "<", Gth => ">", Cmp => "<=>", Neq => "!=", Leq => "<=", Geq => ">=",
			And => "and", Or => "or", Not => "not",
			Assign => "=", ArrowRight => "->", ArrowLeft => "<-",
			Period => ".", ColonColon => "::", Endline => ";", Comma => ",",
			Other(_, _) => unreachable!("Shouldn't be calling `sigil` on a rustfn")
		}
	}

	fn call(&self, args: &[&Object]) -> Result {
		macro_rules! arg {
			($pos:expr) => (args.get($pos).ok_or_else(|| $crate::Error::MissingArgument{ func: self.sigil(), pos: $pos })?);
		}

		match self {
			Pos | Neg | BitNot | Not => arg!(0).call_attr(self.sigil(), &[]),
			Add | Sub | Mul | Div | Mod | Pow
				| AddEq | SubEq | MulEq | DivEq | ModEq | PowEq
				| BitShl | BitShr | BitAnd | BitOr
				| BitXor | BitShlEq | BitShrEq | BitAndEq | BitOrEq | BitXorEq 
				| Eql | Neq | Lth | Leq | Gth | Geq | Cmp
				| And | Or
				| Assign | ArrowRight | ArrowLeft
				| Period | Comma | ColonColon | Endline => arg!(0).call_attr(self.sigil(), &[arg!(1)]),
			Other(_, func) => func.call(args)
		}
	}
}

impl PartialOrd for Oper {
	fn partial_cmp(&self, rhs: &Oper) -> Option<Ordering> {
		Some(self.cmp(rhs))
	}
}

impl Ord for Oper {
	fn cmp(&self, rhs: &Oper) -> Ordering {
		self.precedence().cmp(&rhs.precedence())
	}
}

impl Debug for Oper {
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		if f.alternate() {
			write!(f, "Oper({:#})", self)
		} else {
			write!(f, "Oper({})", self)
		}
	}
}

impl Display for Oper {
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		if let Other(_, rustfn) = self {
			Display::fmt(rustfn, f)
		} else {
			write!(f, "{}", self.sigil())
		}
	}
}


impl_typed_object!(Oper, new_oper, downcast_oper, is_oper);
impl_quest_conversion!("@oper" (as_oper_obj is_oper) (into_oper downcast_oper) -> Oper);

impl_type! { for Oper, downcast_fn=downcast_oper;
	fn "@text" (this) {
		this.sigil().to_string().into_object()
	}

	fn "()" (this) args {
		this.call(args)?
	}
}



