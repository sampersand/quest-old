use parse::{Parsable, Stream};
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

pub type QOper = SharedObject<Oper>;

impl Display for Oper {
	#[inline]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		write!(f, "{}", self.as_str())
	}
}

impl Oper {
	pub fn as_str(&self) -> &'static str {
		use self::Oper::*;
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
}

impl Parsable for QOper {
	type Value = QOper;
	fn try_parse(stream: &mut Stream) -> Option<QOper> {
		macro_rules! parse_oper {
			($($oper:ident $regex:tt)*) => {
				$(
					if stream.try_get(regex!(concat!("\\A(?:", $regex, ")"))).is_some() {
						return Some(Oper::$oper.into());
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







