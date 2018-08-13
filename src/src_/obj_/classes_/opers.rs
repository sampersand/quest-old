use regex::Regex;
use parse::{Tree, Source};
use std::fmt::{self, Display, Formatter};
use env_::Environment__;
use obj_::{QObject__, Result_, classes_::{QNull, QList}};


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

				impl ::obj_::attrs::HasDefaultAttrs for $oper {
					fn default_attrs() -> &'static ::obj_::attrs::DefaultAttrs { &DEFAULT_ATTRS }
				}

				lazy_static!{
					static ref DEFAULT_ATTRS: ::obj_::attrs::DefaultAttrs = {
						use std::borrow::Borrow;
						use obj_::attrs::{DefaultAttrs, HasDefaultAttrs};
						use obj_::{object::QObj, classes_::boundfn::RustFn};
						let mut m = DefaultAttrs::new();
						m.extend(QObj::default_attrs());
						m.insert("()".into(), RustFn(concat!("QOper(", $attr, ")"), |obj: &QObject__, _, env| {
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
		pub fn execute(&self, env: &Environment__) -> Result_ {
			let lhs = lhs!(self, $attr).execute(env)?.unwrap_old();
			let rhs = rhs!(self, $attr).execute(env)?.unwrap_old();
			lhs.call_attr($attr, &[&rhs], env)
		}
	}
}
macro_rules! binaryassign {
	($attr:expr) => {
		pub fn execute(&self, env: &Environment__) -> Result_ {
			let lhs = lhs!(self, $attr);
			let rhs = rhs!(self, $attr).execute(env)?.unwrap_old();
			Ok(env.assign(lhs.to_qvar(env)?.unwrap_old(), lhs.execute(env)?.unwrap_old().call_attr($attr, &[&rhs], env)?.unwrap_old()).into())

			// Ok(obj_::object::qobject::QObject_::Old(env.assign(match ::std::ops::Try::into_result(lhs.to_qvar(env)) {
   //             ::std::result::Result::Err(err) =>
   //                 #[allow(unreachable_code)]
   //                 return ::std::ops::Try::from_error(::std::convert::From::from(err)),
   //             ::std::result::Result::Ok(val) =>
   //                 #[allow(unreachable_code)]
   //                 val,
   //         }.unwrap_old(),
   //         match ::std::ops::Try::into_result(match ::std::ops::Try::into_result(lhs.aexecute(env))
   //                                                {
   //                                                ::std::result::Result::Err(err)
   //                                                =>
   //                                                    #[allow(unreachable_code)]
   //                                                    return ::std::ops::Try::from_error(::std::convert::From::from(err)),
   //                                                ::std::result::Result::Ok(val)
   //                                                =>
   //                                                    #[allow(unreachable_code)]
   //                                                    val,
   //                                            }.unwrap_old().call_attr("+",
   //                                                                     &[&rhs],
   //                                                                     env)) {
   //             ::std::result::Result::Err(err) =>
   //                 #[allow(unreachable_code)]
   //                 return ::std::ops::Try::from_error(::std::convert::From::from(err)),
   //             ::std::result::Result::Ok(val) =>
   //                 #[allow(unreachable_code)]
   //                 val,
   //         }.unwrap_old())))
		}
	}
}

macro_rules! assignoper {
	($attr:expr, $reversed:expr) => {
		pub fn execute(&self, env: &Environment__) -> Result_ {
			let (lhs, rhs) = match $reversed {
				false => (lhs!(self, $attr), rhs!(self, $attr)),
				true  => (rhs!(self, $attr), lhs!(self, $attr))
			};

			Ok(env.assign(lhs.to_qvar(env)?.unwrap_old(), rhs.execute(env)?.unwrap_old()).into())
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
			pub fn execute(&self, env: &Environment__) -> Result_ {
				let lhs = self.lhs.as_ref().expect("lhs is needed for `or`").execute(env)?.unwrap_old();
				if lhs.as_bool(env)?.to_bool() {
					Ok(lhs.old())
				} else {
					self.rhs.as_ref().expect("rhs is needed for `or` when lhs is null").execute(env)
				}
			}
		}),
		And(mod and {
			struct QAnd("&&",  r"&&|and\b");
			pub fn execute(&self, env: &Environment__) -> Result_ {
				let lhs = self.lhs.as_ref().expect("lhs is needed for `and`").execute(env)?.unwrap_old();
				if lhs.as_bool(env)?.to_bool() {
					self.rhs.as_ref().expect("rhs is needed for `and` when lhs is null").execute(env)
				} else {
					Ok(lhs.old())
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
			pub fn execute(&self, env: &Environment__) -> Result_ {
				match (&self.lhs, &self.rhs) {
					(Some(ref var), None) | (None, Some(ref var)) => Ok(::obj_::QObject_::Old(env.has(&var.to_qvar(env)?.unwrap_old()).into())),
					(Some(ref lhs), Some(ref rhs)) if rhs.oper.try_as_str() == Some(":") => 
						if lhs.execute(env)?.unwrap_old().as_bool(env)?.to_bool() {
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
			pub fn execute(&self, env: &Environment__) -> Result_ {
				let mut body = Vec::new();
				let mut lhs = match self.lhs {
					Some(ref lhs) => lhs,
					None => panic!("a left hand side is needed for `,`") // ie `foo(,)` is not allowed; maybe allow in future?
				};

				if lhs.oper.try_as_str().map(|s| s == ",").unwrap_or(false) {
					let lhs = lhs.execute(env)?.unwrap_old().as_list(env)?.into();
					body = lhs;
				} else {
					body.push(lhs.execute(env)?.unwrap_old());
				}
				if let Some(ref rhs) = self.rhs {
					body.push(rhs.execute(env)?.unwrap_old());
				}
				Ok(::obj_::QObject_::Old(QList::from(body).into()))
			}
		}),

		LineEnd(mod line_end {
			struct QLineEnd(";", ";");
			pub fn execute(&self, env: &Environment__) -> Result_ {
				if let Some(ref lhs) = self.lhs {
					lhs.execute(env)?.unwrap_old();
				}
				if let Some(ref rhs) = self.rhs {
					rhs.execute(env)
				} else {
					Ok(::obj_::QObject_::Old(QNull.into()))
				}
			}
		}),
		// token EXISTS(oper binary QQuestion, exists::REGEX, Unary)

		Accessor(mod accessor {
			struct QAccessor(".", r"\.");
			pub fn execute(&self, env: &Environment__) -> Result_ {
				let lhs = self.lhs.as_ref().expect("lhs is needed for `.`").execute(env)?.unwrap_old();
				let rhs = self.rhs.as_ref().expect("rhs is needed for `.`").to_qvar(env)?.unwrap_old();
				lhs.call_attr(".", &[&rhs], env)
			}
		})
	}
}

















