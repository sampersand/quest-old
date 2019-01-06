use super::RustFn;
use std::fmt::{self, Debug, Display, Formatter};
use std::cmp::Ordering;
use crate::{Object, Result, IntoObject};

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum Oper {
	UnaryPos, UnaryNeg,

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
	UnaryPos_BitNot_Not,
	Pow,
	UnaryNeg,
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
	fn precedence(&self) -> Precedence {
		match self {
			Period
			  | ColonColon    => Precedence::Period_ColonColon,
			UnaryPos
			  | BitNot
			  | Not           => Precedence::UnaryPos_BitNot_Not,
			Pow               => Precedence::Pow,
			UnaryNeg          => Precedence::UnaryNeg,
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

	fn symbol(&self) -> &'static str {
		match self {
			UnaryPos => "+@", UnaryNeg => "-@",
			Add => "+", Sub => "-", Mul => "*", Div => "/", Mod => "%", Pow => "**",
			AddEq => "+=", SubEq => "-=", MulEq => "*=", DivEq => "/=", ModEq => "%=", PowEq => "**=",
			BitShl => "<<", BitShr => ">>", BitAnd => "&", BitOr => "|", BitXor => "^", BitNot => "~",
			BitShlEq => "<<=", BitShrEq => ">>=", BitAndEq => "&=", BitOrEq => "|=", BitXorEq => "^=",
			Eql => "==", Lth => "<", Gth => ">", Cmp => "<=>", Neq => "!=", Leq => "<=", Geq => ">=",
			And => "and", Or => "or", Not => "not",
			Assign => "=", ArrowRight => "->", ArrowLeft => "<-",
			Period => ".", ColonColon => "::", Endline => ";", Comma => ",",
			Other(_, _) => unreachable!("Shouldn't be calling `symbol` on a rustfn")
		}
	}

	fn call(&self, args: &[&Object]) -> Result {
		macro_rules! arg {
			($pos:expr) => (args.get($pos).ok_or_else(|| $crate::Error::MissingArgument{ func: self.symbol(), pos: $pos })?);
		}

		match self {
			UnaryPos | UnaryNeg | BitNot | Not => arg!(0).call_attr(self.symbol(), &[]),
			Add | Sub | Mul | Div | Mod | Pow
				| AddEq | SubEq | MulEq | DivEq | ModEq | PowEq
				| BitShl | BitShr | BitAnd | BitOr
				| BitXor | BitShlEq | BitShrEq | BitAndEq | BitOrEq | BitXorEq 
				| Eql | Neq | Lth | Leq | Gth | Geq | Cmp
				| And | Or
				| Assign | ArrowRight | ArrowLeft
				| Period | Comma | ColonColon | Endline => arg!(0).call_attr(self.symbol(), &[arg!(1)]),
			Other(_, func) => func.call(args)
		}
	}
}


impl Debug for Oper {
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		if f.alternate() {
			write!(f, "Oper({:#})", self)
		} else {
			write!(f, "Oper({}", self)
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

impl Display for Oper {
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		if let Other(_, rustfn) = self {
			Display::fmt(rustfn, f)
		} else {
			write!(f, "{}", self.symbol())
		}
	}
}
