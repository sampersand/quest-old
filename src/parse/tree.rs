use env::Environment;
use obj::{QObject, Result};
use parse::TokenMatch;
use std::fmt::{self, Debug, Display, Formatter};

use std::hash::{Hash, Hasher};

#[derive(Clone)]
pub struct Tree {
	pub oper: TokenMatch,
	lhs: Option<Box<Tree>>,
	rhs: Option<Box<Tree>>,
}

impl Tree {
	pub fn new(oper: TokenMatch, lhs: Option<Tree>, rhs: Option<Tree>) -> Tree {
		Tree { oper, lhs: lhs.map(Box::new), rhs: rhs.map(Box::new) }
	}

	pub fn is_singular(&self) -> bool {
		self.lhs.is_none() && self.rhs.is_none()
	}

	pub fn execute(&self, env: &Environment) -> Result {
		self.oper.token.create_qobject(self, env)
	}

	pub fn to_qvar(&self, env: &Environment) -> Result {
		self.oper.token.to_qvar(self, env)
	}
}

impl Tree {
	pub fn lhs(&self) -> Option<&Tree> {
		self.lhs.as_ref().map(AsRef::as_ref)
	}
	pub fn rhs(&self) -> Option<&Tree> {
		self.rhs.as_ref().map(AsRef::as_ref)
	}

	pub fn try_from_vec(mut matches: Vec<TokenMatch>) -> Option<Tree> {
		if matches.is_empty() {
			return None;
		}
		let (oper_pos, _) = matches.iter().enumerate().max_by_key(|(_, k)| k.token).expect("no key found?");
		let rhs = matches.split_off(oper_pos + 1);
		let oper = matches.pop().unwrap();
		matches.shrink_to_fit();
		Some(Tree::new(oper, Tree::try_from_vec(matches), Tree::try_from_vec(rhs)))
	}
}

impl Eq for Tree {}
impl PartialEq for Tree {
	fn eq(&self, other: &Tree) -> bool {
		self.oper == other.oper && self.lhs == other.lhs && self.rhs == other.rhs
	}
}

impl Hash for Tree {
	fn hash<H: Hasher>(&self, h: &mut H) {
		self.oper.hash(h);
		self.lhs.hash(h);
		self.rhs.hash(h);
	}
}

impl Display for Tree {
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		Display::fmt(&self.oper.src, f)
	}
}

impl Debug for Tree {
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		let mut t = f.debug_struct("Tree");
		t.field("oper", &self.oper);
		if let Some(ref lhs) = self.lhs {
			t.field("lhs", &lhs);
		}
		if let Some(ref rhs) = self.rhs {
			t.field("rhs", &rhs);
		}
		// ignore the srcation
		t.finish()
	}
}

