use obj_::{QObject__, Result_, Exception__, Exception_};
use parse::Tree;
use env_::Environment__;
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

fn execute(tree: Option<&Tree>, args: &[&QObject__], env: &Environment__) -> Result_ {
	if let Some(tree) = tree {
		env.set_arguments(args);
		match tree.execute(&env) {
			Ok(thing) => Ok(thing),
			Err(Exception_::Old(Exception__::Return(0, Some(val)))) => Ok(::obj_::QObject_::Old(val)),
			Err(Exception_::Old(Exception__::Return(0, None))) => Ok(::obj_::QObject_::Old(().into())),
			Err(Exception_::Old(Exception__::Return(i, ret_val))) => Err(Exception__::Return(i - 1, ret_val).into()),
			other => other
		}
	} else {
		Ok(::obj_::QObject_::Old(().into())) // aka we have an empty tree
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







