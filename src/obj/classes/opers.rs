use regex::Regex;
use parse::{Tree, Source};
use std::fmt::{self, Display, Formatter};
use env::Environment;
use obj::{QObject, Result, classes::{QNull, QList}};


macro_rules! define_opers {
	(pub enum $qopers:ident {
		$($variant:ident(
			mod $mod:ident { struct $oper:ident($attr:expr, $regex:expr); $($impls:tt)* }
		)),*
	}) => {
		#[derive(Debug, Clone, PartialEq, Eq, Hash)]
		pub enum $qopers {
			$($variant($oper)),*
		}

		impl Display for $qopers {
			fn fmt(&self, f: &mut Formatter) -> fmt::Result {
				match self {
					$($qopers::$variant(val) => Display::fmt(val, f)),*
				}
			}
		}
		$(
			pub use self::$mod::$oper;
			pub mod $mod {
				use super::*;
				#[derive(Debug, Clone, PartialEq, Eq, Hash)]
				pub struct $oper {
					lhs: Option<Tree>,
					rhs: Option<Tree>,
				src: Source
				}
				lazy_static!{
					pub static ref REGEX: Regex = regex!(concat!("\\A(?:", $regex, ")"));
				}

				impl ::obj::attrs::HasDefaultAttrs for $oper {
					fn default_attrs() -> &'static ::obj::attrs::DefaultAttrs { &DEFAULT_ATTRS }
				}

				lazy_static!{
					static ref DEFAULT_ATTRS: ::obj::attrs::DefaultAttrs = {
						use std::borrow::Borrow;
						use obj::attrs::{DefaultAttrs, HasDefaultAttrs};
						use obj::{object::QObj, classes::boundfn::RustFn};
						let mut m = DefaultAttrs::new();
						m.extend(QObj::default_attrs());
						m.insert("()".into(), RustFn(concat!("QOper(", $attr, ")"), |obj: &QObject, _, env| {
							match expect_qobj!(obj, Oper) {
								$qopers::$variant(oper) => oper.execute(env),
								other => panic!(concat!("Called ", stringify!($oper), ".`()` with an invalid qobject: {:?}"), obj)
							}
						}));
						m
					};
				}

				impl From<$oper> for QOper {
					fn from(oper: $oper) -> QOper {
						QOper::$variant(oper)
					}
				}

				impl Display for $oper {
					#[inline]
					fn fmt(&self, f: &mut Formatter) -> fmt::Result {
						Display::fmt(&$attr, f)
					}
				}

				impl $oper {
					pub fn from_tree(tree: &Tree) -> $oper {
						debug_assert!(REGEX.is_match(tree.oper.try_as_str().unwrap()), concat!(stringify!($oper), ": invalid oper `{}` supplied (`", $attr, "` expected)"), tree.oper.try_as_str().unwrap());
						$oper{
							lhs: tree.lhs().map(Tree::clone),
							rhs: tree.rhs().map(Tree::clone),
							src: tree.oper.src.clone()
						}
					}
					$($impls)*
				}
			}
		)*
	}
}

macro_rules! lhs {
	($val:expr, $attr:expr) => ( $val.lhs.as_ref().expect(concat!("lhs is needed for `", $attr, "`")) );
}
macro_rules! rhs {
	($val:expr, $attr:expr) => ( $val.rhs.as_ref().expect(concat!("rhs is needed for `", $attr, "`")) );
}

macro_rules! binaryoper {
	($attr:expr) => {
		pub fn execute(&self, env: &Environment) -> Result {
			let lhs = lhs!(self, $attr).execute(env)?;
			let rhs = rhs!(self, $attr).execute(env)?;
			lhs.call_attr($attr, &[&rhs], env)
		}
	}
}
macro_rules! binaryassign {
	($attr:expr) => {
		pub fn execute(&self, env: &Environment) -> Result {
			let lhs = lhs!(self, $attr);
			let rhs = rhs!(self, $attr).execute(env)?;
			Ok(env.set(lhs.to_qvar(env)?, lhs.execute(env)?.call_attr($attr, &[&rhs], env)?))
		}
	}
}

macro_rules! assignoper {
	($attr:expr, $reversed:expr) => {
		pub fn execute(&self, env: &Environment) -> Result {
			let (lhs, rhs) = match $reversed {
				false => (lhs!(self, $attr), rhs!(self, $attr)),
				true  => (rhs!(self, $attr), lhs!(self, $attr))
			};

			Ok(env.set(lhs.to_qvar(env)?, rhs.execute(env)?))
		}
	}
}

// }

define_opers!{
	pub enum QOper {
		Add(mod add { struct QAdd("+", r"\+"); binaryoper!("+"); }),
		Sub(mod sub { struct QSub("-",   "-"); binaryoper!("-"); }),
		Mul(mod mul { struct QMul("*", r"\*"); binaryoper!("*"); }),
		Div(mod div { struct QDiv("/",   "/"); binaryoper!("/"); }),
		Pow(mod pow { struct QPow("^", r"\^|pow\b|\*\*"); binaryoper!("^"); }),
		Mod(mod mod_{ struct QMod("%", r"%|mod\b"); binaryoper!("%"); }),

		AddI(mod addi { struct QAddI("+=", r"\+="); binaryassign!("+"); }),
		SubI(mod subi { struct QSubI("-=",   "-="); binaryassign!("-"); }),
		MulI(mod muli { struct QMulI("*=", r"\*="); binaryassign!("*"); }),
		DivI(mod divi { struct QDivI("/=",   "/="); binaryassign!("/"); }),
		PowI(mod powi { struct QPowI("^=", r"(?:\^|pow|\*\*)="); binaryassign!("^"); }),
		ModI(mod modi { struct QModI("%=", "(?:%|mod)="); binaryassign!("%"); }),


		// token O_NOT(oper unary true QNot, noti::REGEX, Unary);
		Or(mod or {
			struct QOr("||", r"\|\||or\b");
			pub fn execute(&self, env: &Environment) -> Result {
				let lhs = self.lhs.as_ref().expect("lhs is needed for `or`").execute(env)?;
				if lhs.as_bool(env)?.to_bool() {
					Ok(lhs)
				} else {
					self.rhs.as_ref().expect("rhs is needed for `or` when lhs is null").execute(env)
				}
			}
		}),
		And(mod and {
			struct QAnd("&&",  r"&&|and\b");
			pub fn execute(&self, env: &Environment) -> Result {
				let lhs = self.lhs.as_ref().expect("lhs is needed for `and`").execute(env)?;
				if lhs.as_bool(env)?.to_bool() {
					self.rhs.as_ref().expect("rhs is needed for `and` when lhs is null").execute(env)
				} else {
					Ok(lhs)
				}
			}
		}),
		
		Lt(mod lt { struct QLt( "<",  "<"); binaryoper!("<"); }),
		Gt(mod gt { struct QGt( ">",  ">"); binaryoper!(">"); }),
		Le(mod le { struct QLe("<=", "<=|≤"); binaryoper!("<="); }),
		Ge(mod ge { struct QGe(">=", ">=|≥"); binaryoper!(">="); }),
		Cmp(mod cmp { struct QCmp("<=>", "<=>"); binaryoper!("<=>"); }),

		Eq(mod eq { struct QEq("==", "=="); binaryoper!("=="); }),
		Ne(mod ne { struct QNe("!=", "!=|≠"); binaryoper!("!="); }),
		
		Assign(mod assign { struct QAssign("=", "="); assignoper!("=", false); }),
		AssignL(mod assignl { struct QAssignL("<-", "<-"); assignoper!("=", false); }),
		AssignR(mod assignr { struct QAssignR("->", "->"); assignoper!("=", true); }),
		
		// token O_NOT(oper unary true QBwNeg, bw_neg::REGEX, Unary);
		BwOr(mod bw_or { struct QBwOr("|", r"\|"); binaryoper!("|"); }),
		BwAnd(mod bw_and { struct QBwAnd("&", "&"); binaryoper!("&"); }),
		BwXor(mod bw_xor { struct QBwXor("^^", r"\^\^|xor\b"); binaryoper!("^^"); }),
		BwLs(mod bw_ls { struct QBwLs("<<", "<<"); binaryoper!("<<"); }),
		BwRs(mod bw_rs { struct QBwRs(">>", ">>"); binaryoper!(">>"); }),
		
		BwOrI(mod bw_ori { struct QBwOrI("|=", r"\|="); binaryassign!("|"); }),
		BwAndI(mod bw_andi { struct QBwAndI("&=", "&="); binaryassign!("&"); }),
		BwXorI(mod bw_xori { struct QBwXorI("^^=", r"(?:\^\^|xor)="); binaryassign!("^"); }),
		BwLsI(mod bw_lsi { struct QBwLsI("<<=", "<<="); binaryassign!("<<"); }),
		BwRsI(mod bw_rsi { struct QBwRsI(">>=", ">>="); binaryassign!(">>"); }),



		Exists(mod exists {
			struct QQuestion("?", "\\?");
			pub fn execute(&self, env: &Environment) -> Result {
				match (&self.lhs, &self.rhs) {
					(Some(ref var), None) | (None, Some(ref var)) => Ok(env.has(&var.to_qvar(env)?).into()),
					(Some(ref lhs), Some(ref rhs)) if rhs.oper.try_as_str() == Some(":") => 
						if lhs.execute(env)?.as_bool(env)?.to_bool() {
							rhs.lhs().expect("LHS is needed in COND ? LHS : RHS ").execute(env)
						} else {
							rhs.rhs().expect("RHS is needed in COND ? LHS : RHS ").execute(env)
						}
					(lhs, rhs) => panic!("invalid lhs, rhs encountered: {:?}, {:?}", lhs, rhs)
				}
			}
		}),

		Comma(mod comma {
			struct QComma(",", ",");
			pub fn execute(&self, env: &Environment) -> Result {
				let mut body = Vec::new();
				let mut lhs = match self.lhs {
					Some(ref lhs) => lhs,
					None => panic!("a left hand side is needed for `,`") // ie `foo(,)` is not allowed; maybe allow in future?
				};

				if lhs.oper.try_as_str().map(|s| s == ",").unwrap_or(false) {
					let lhs = lhs.execute(env)?.as_list(env)?.into();
					body = lhs;
				} else {
					body.push(lhs.execute(env)?);
				}
				if let Some(ref rhs) = self.rhs {
					body.push(rhs.execute(env)?);
				}
				Ok(QList::from(body).into())
			}
		}),

		LineEnd(mod line_end {
			struct QLineEnd(";", ";");
			pub fn execute(&self, env: &Environment) -> Result {
				if let Some(ref lhs) = self.lhs {
					lhs.execute(env)?;
				}
				if let Some(ref rhs) = self.rhs {
					rhs.execute(env)
				} else {
					Ok(QNull.into())
				}
			}
		}),
		// token EXISTS(oper binary QQuestion, exists::REGEX, Unary)

		Accessor(mod accessor {
			struct QAccessor(".", r"\.");
			pub fn execute(&self, env: &Environment) -> Result {
				let lhs = self.lhs.as_ref().expect("lhs is needed for `.`").execute(env)?;
				let rhs = self.rhs.as_ref().expect("rhs is needed for `.`").to_qvar(env)?;
				lhs.call_attr(".", &[&rhs], env)
			}
		})
	}
}

















