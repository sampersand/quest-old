use std::error::Error as ErrorTrait;
use std::fmt::{self, Display, Formatter};
use obj::AnyShared;

#[derive(Debug, Clone)]
pub enum Error {
	MissingAttr { // an attribute couldn't be found
		obj: AnyShared,
		attr: AnyShared
	},
	InvalidAttrResult { // an attribute returned an invalid value (eg `@num` returning text)
		obj: AnyShared,
		attr: AnyShared,
		res: AnyShared,
	},
	BadArguments {
		args: Vec<AnyShared>, 
		descr: &'static str
	},
	Return {
		levels: usize,
		obj: AnyShared
	}
}

use self::Error::*;

#[must_use]
pub type Result<T> = ::std::result::Result<T, Error>;

impl Display for Error {
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		match self {
			MissingAttr { obj, attr } => write!(f, "Object `{}` is missing attribute `{}`", obj, attr),
			InvalidAttrResult { obj, attr, res } => write!(f, "Method `{}` on object `{}` returned an invalid value: `{}`", attr, obj, res),
			BadArguments { args, descr } => write!(f, "Bad arguments supplied ({}): {}", args.iter().map(ToString::to_string).collect::<Vec<_>>().join(", "), descr),
			Return { levels, obj } => write!(f, "Returning `{}` levels with obj: {}", levels, obj),
		}
	}
}

impl ErrorTrait for Error {
	fn description(&self) -> &str {
		match self {
			MissingAttr { .. } => "an attribute is missing",
			InvalidAttrResult { .. } => "an attrinbteu returned an invalid value",
			BadArguments { descr, .. } => descr,
			Return { .. } => "returning from a method",
		}
	}
}

impl Eq for Error {}
impl PartialEq for Error {
	fn eq(&self, other: &Error) -> bool {
		match (self, other) {
			(MissingAttr { obj: s_o, attr: s_a }, MissingAttr { obj: o_o, attr: o_a }) => s_o == o_o && s_a == o_a, 
			(InvalidAttrResult { obj: s_o, attr: s_a, res: s_r }, InvalidAttrResult { obj: o_o, attr: o_a, res: o_r }) => s_o == o_o && s_a == o_a && s_r == o_r,
			(BadArguments { args: s_a, descr: s_d }, BadArguments { args: o_a, descr: o_d }) => s_a == o_a && s_d == o_d,
			(Return { levels: s_l, obj: s_o }, Return { levels: o_l, obj: o_o }) => s_l == o_l && s_o == o_o,
			_ => false
		}
	}
}