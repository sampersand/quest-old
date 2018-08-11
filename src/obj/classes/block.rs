use obj::classes::QNull;
use parse::Tree;
use std::fmt::{self, Display, Formatter};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct QBlock(Option<Tree>);

impl From<Option<Tree>> for QBlock {
	#[inline]
	fn from(inp: Option<Tree>) -> QBlock {
		QBlock::new(inp)
	}
}

impl QBlock {
	pub fn new(tree: Option<Tree>) -> QBlock {
		QBlock(tree)
	}
}

impl Display for QBlock {
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		if let Some(ref tree) = self.0 {
			Display::fmt(&tree, f)
		} else {
			write!(f, "<empty tree>")
		}
	}
}

default_attrs! { for QBlock, with variant Block;
	use QObj;

	fn "{}" (this) with env args {
		if let Some(tree) = this.0.as_ref() {
			env.set_arguments(args);
			tree.execute(&env)
		} else {
			().into()
		}
	}

	fn "()" (this) with env args {
		if let Some(tree) = this.0.as_ref() {
			tree.execute(&env.clone_for_call(args))
		} else {
			().into()
		}
	}
}


