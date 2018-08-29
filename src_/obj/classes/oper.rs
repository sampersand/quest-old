use env::{Environment, parse::{Parsable, Token, Precedence}};
use obj::{AnyObject, SharedObject};

use std::fmt::{self, Display, Formatter};	

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Oper {
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

use self::Oper::*;

pub type QOper = SharedObject<Oper>;

impl Display for Oper {
	#[inline]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		write!(f, "{}", self.as_str())
	}
}

impl Oper {
	pub fn as_str(&self) -> &'static str {
		match self {
			Period => ".", Comma => ",", Semicolon => ";",

			Add  => "+",  Sub  => "-",  Mul  => "*",  Div  => "/",  Pow  => "^",  Mod  => "%",
			AddI => "+=", SubI => "-=", MulI => "*=", DivI => "/=", PowI => "^=", ModI => "%=",

			BitAnd  => "&",  BitOr  => "|",  BitXor  => "^^",  BitShl  => "<<",  BitShr  => ">>",
 			BitAndI => "&=", BitOrI => "|=", BitXorI => "^^=", BitShlI => "<<=", BitShrI => ">>=",

 			Lt => "<", Gt => ">", Le => "<=", Ge => ">=", Eq => "==", Ne => "!=", Cmp => "<=>",
 			And => "&&", Or => "||",

 			Assign => "=", AssignL => "<-", AssignR => "->",
 			Not => "!", Neg => "-", Exists => "?", Incr => "++", Decr => "--"
		}
	}


	fn precedence(&self) -> Precedence {
		use env::parse::Precedence::*;
		match self {
			Period => Accessor,
			Oper::Comma => Precedence::Comma,
			Semicolon => EndOfLine,

			Add => AddSub,
			Sub => AddSub,
			Mul => MulDivMod,
			Div => MulDivMod,
			Oper::Pow => Precedence::Pow,
			Mod => MulDivMod,
			BitAnd => BwAnd,
			BitOr => BwOrXor,
			BitXor => BwOrXor,
			BitShl => BwShift,
			BitShr => BwShift,

			AddI => AssignmentMath,
			SubI => AssignmentMath,
			MulI => AssignmentMath,
			DivI => AssignmentMath,
			PowI => AssignmentMath,
			ModI => AssignmentMath,
			BitAndI => AssignmentBw,
			BitOrI => AssignmentBw,
			BitXorI => AssignmentBw,
			BitShlI => AssignmentBwShift,
			BitShrI => AssignmentBwShift,

			Lt => Ordering,
			Gt => Ordering,
			Le => Ordering,
			Ge => Ordering,
			Eq => Equality,
			Ne => Equality,
			Cmp => Equality,
			And => LogicalAnd,
			Or => LogicalOr,

			Assign => Assignment,
			AssignL => Assignment,
			AssignR => Assignment,
			Not => Unary,
			Neg => UnaryNeg,
			Exists => Unary,
			Incr => Unary,
			Decr => Unary,

		}
	}
}

impl Parsable for Oper {
	fn try_parse(env: &mut Environment) -> Option<Token> {
		macro_rules! parse_oper {
			($($oper:ident $regex:tt)*) => {
				$(
					if env.stream.try_get(regex!(concat!("\\A(?:", $regex, ")"))).is_some() {
						let oper = Oper::$oper;
						return Some(Token::Object(SharedObject::from(oper), oper.precedence(), Oper::execute));
					}
				)*
			}
		}

		// note that this percludes the use of `Neg` oper; we have to check it when the oper is parsed

		parse_oper! {
			// we want to parse the longer ones first, so we don't get `Lt` eating up the `<` in `<=`
			// noticibly, this  means that we need to parse for `X=` before `X`
			Cmp "<=>" And r"&&|and\b" Or r"\|\||or\b"

 			BitAndI "&=" BitOrI r"\|=" BitXorI r"\^\^=" BitShlI "<<=" BitShrI ">>="
 			BitAnd  "&"  BitOr  r"\|"  BitXor  r"\^\^"  BitShl  "<<"  BitShr  ">>" 

			AssignL "<-" AssignR "->" // notice this is before both the `<` and `-` operators
			Incr r"\+\+" Decr "--" // note before `+` and `-`

			AddI r"\+=" SubI "-=" MulI r"\*=" DivI "/=" PowI r"(\^|pow|\*\*)="   ModI  "(%|mod)="
			Add  r"\+"  Sub  "-"  Mul  r"\*"  Div  "/"  Pow  r"(\^|pow\b|\*\*)"  Mod  r"(%|mod\b)"

			Eq r"==\b" Ne "!=" Le "<=" Ge ">=" Lt "<" Ge ">"
			Assign "=" // notice this is after `==`

			Period r"\." Comma "," Semicolon ";"
			Not "!" Exists r"\?" // `Neg` is missing because its impossible to differentiate it from `Sub`; That's done in the Tree phase
		};
		None
	}
}

define_attrs! { for QOper;
	use QObject<Oper>;

	fn "@num" () {
		Ok(QBool::from(false))
	}
}







