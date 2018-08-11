use obj::{QObject, Result, Exception};
use parse::Tree;
use env::Environment;
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

fn execute(tree: Option<&Tree>, args: &[&QObject], env: &Environment) -> Result {
	if let Some(tree) = tree {
		env.set_arguments(args);
		match tree.execute(&env) {
			Ok(val) => Ok(val),
			Err(Exception::Return(0, Some(val))) => Ok(val),
			Err(Exception::Return(0, None)) => Ok(().into()),
			Err(Exception::Return(i, ret_val)) => Err(Exception::Return(i - 1, ret_val)),
			other => other
		}
	} else {
		Ok(().into()) // aka we have an empty tree
	}
}

default_attrs! { for QBlock, with variant Block;
	use QObj;

	fn "{}" (this) with env args {
		execute(this.0.as_ref(), args, env)
	}

	fn "()" (this) with env args {
		let mut envv = env.clone_for_call();
		envv.bind(env);
		execute(this.0.as_ref(), args, &envv)
	}
}







