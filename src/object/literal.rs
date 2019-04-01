use std::sync::RwLock;
use std::{collections::HashSet, borrow::Borrow};

use lazy_static::lazy_static;
use std::fmt::{self, Display, Formatter};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Literal(&'static str);

impl Literal {
	// not public because we dont want random static literals being created
	const fn new(literal: &'static str) -> Literal {
		Literal(literal)
	}

	#[cfg(test)]
	pub const fn new_testing(literal: &'static str) -> Literal {
		Literal::new(literal)
	}
}

lazy_static! {
	static ref LITERALS: RwLock<HashSet<Literal>> = RwLock::new(HashSet::new());
}

impl AsRef<str> for Literal {
	fn as_ref(&self) -> &str {
		self.0
	}
}

impl Borrow<str> for Literal {
	fn borrow(&self) -> &str {
		self.0
	}
}

impl From<String> for Literal {
	fn from(string: String) -> Literal {
		let literals = LITERALS.read().expect("LITERALS is poisoned");

		if let Some(literal) = literals.get(string.as_str()) {
			return *literal;
		}

		drop(literals);
		let mut literals = LITERALS.write().expect("LITERALS is poisoned");

		if let Some(literal) = literals.get(string.as_str()) { // double check
			return *literal;
		}

		let literal = Literal::new(Box::leak(string.into_boxed_str()));
		literals.insert(literal);
		literal
	}
}

impl PartialEq<&'_ str> for Literal {
	fn eq(&self, other: &&'_ str) -> bool {
		&self.0 == other
	}
}

impl Display for Literal {
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		write!(f, "{}", self.0)
	}
}

pub mod consts {
	use super::Literal;

	/* Conversions */
	pub const AT_TEXT: Literal = Literal::new("@text");
	pub const AT_BOOL: Literal = Literal::new("@bool");
	pub const AT_NUM: Literal = Literal::new("@num");
	pub const AT_LIST: Literal = Literal::new("@list");
	pub const AT_MAP: Literal = Literal::new("@map");
	pub const AT_VAR: Literal = Literal::new("@var");

	/* Normal Operators */
	// Equality
	pub const STRICT_EQL: Literal = Literal::new("===");
	pub const STRICT_NEQ: Literal = Literal::new("!==");
	pub const EQL: Literal = Literal::new("==");
	pub const NEQ: Literal = Literal::new("!=");
		
		// Comparison
	pub const LTH: Literal = Literal::new("<");
	pub const LEQ: Literal = Literal::new("<=");
	pub const GTH: Literal = Literal::new(">");
	pub const GEQ: Literal = Literal::new(">=");
	pub const CMP: Literal = Literal::new("<=>");
		
		// Logical
	pub const NOT: Literal = Literal::new("!");
	pub const AND: Literal = Literal::new("and");
	pub const OR: Literal = Literal::new("or");

		// Bitwise
	pub const B_XOR: Literal = Literal::new("^");
	pub const B_AND: Literal = Literal::new("&");
	pub const B_OR: Literal = Literal::new("|");
	pub const B_LSH: Literal = Literal::new("<<");
	pub const B_RSH: Literal = Literal::new(">>");
	pub const B_NOT: Literal = Literal::new("~");

		// Mathematical
	pub const ADD: Literal = Literal::new("+");
	pub const SUB: Literal = Literal::new("-");
	pub const MUL: Literal = Literal::new("*");
	pub const DIV: Literal = Literal::new("/");
	pub const POW: Literal = Literal::new("**");
	pub const MOD: Literal = Literal::new("%");

	pub const POS: Literal = Literal::new("@+");
	pub const NEG: Literal = Literal::new("@-");

	/* Assignment Operators */
	pub const ASSIGN: Literal = Literal::new("=");
	pub const ARROW_LEFT: Literal = Literal::new("<-");
	pub const ARROW_RIGHT: Literal = Literal::new("->");

	pub const ADD_ASSIGN: Literal = Literal::new("+=");
	pub const SUB_ASSIGN: Literal = Literal::new("-=");
	pub const MUL_ASSIGN: Literal = Literal::new("*=");
	pub const DIV_ASSIGN: Literal = Literal::new("/=");
	pub const MOD_ASSIGN: Literal = Literal::new("%=");
	pub const POW_ASSIGN: Literal = Literal::new("**=");
	pub const B_XOR_ASSIGN: Literal = Literal::new("^=");
	pub const B_AND_ASSIGN: Literal = Literal::new("&=");
	pub const B_OR_ASSIGN: Literal = Literal::new("|=");

	/* Misc Operators */
	pub const COMMA: Literal = Literal::new(",");
	pub const ENDLINE: Literal = Literal::new(";");
	pub const CALL: Literal = Literal::new("()");

	/* Indexing and Calling */
	pub const INDEX: Literal = Literal::new("[]");
	pub const INDEX_ASSIGN: Literal = Literal::new("[]=");
	pub const INDEX_DELETE: Literal = Literal::new("[]~");
	pub const INDEX_HAS: Literal = Literal::new("[]?");

	pub const ATTR_GET: Literal = Literal::new(".");
	pub const ATTR_SET: Literal = Literal::new(".=");
	pub const ATTR_DEL: Literal = Literal::new(".~");
	pub const ATTR_HAS: Literal = Literal::new(".?");

	pub const COLON_COLON: Literal = Literal::new("::");


	/* Literals */
	pub const L_ID: Literal = Literal::new("__id__");
	pub const L_MAP: Literal = Literal::new("__map__");
	pub const L_ENV: Literal = Literal::new("__env__");

	pub const L_CLONE: Literal = Literal::new("clone");
	pub const L_LEN: Literal = Literal::new("len");
	pub const L_EVAL: Literal = Literal::new("eval");
	pub const L_PARENT: Literal = Literal::new("parent")
;
	pub const L_PARSER: Literal = Literal::new("parser");
	pub const L_STACK: Literal = Literal::new("stack");
	pub const L_POP: Literal = Literal::new("pop");
	pub const L_PUSH: Literal = Literal::new("push");


}