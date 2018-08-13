use parse::{self, Parsable, Stream, Tree};
use obj::{AnyObject, SharedObject};

use std::fmt::{self, Display, Formatter};	

pub type QBlock = SharedObject<Tree>;

impl Parsable for QBlock {
	type Value = QBlock;

	fn try_parse(stream: &mut Stream) -> Option<QBlock> {
		if let Some(paren) = stream.try_get(regex!(r"\A[{(\[]")) {
			// todo: check to make sure the last paren is the correct one?
			Some(Tree::from(parse::parse_stream(stream)).into())
		} else {
			None
		}
	}
}

define_attrs! { for QBlock;
	use QObject<Tree>;

	fn "{}" (this) with env args obj {
		unimplemented!("TODO: local call qblock");
		Ok(obj.clone())
	}

	fn "()" (this) with env args obj {
		unimplemented!("TODO: call qblock");
		Ok(obj.clone())
	}
}

// fn execute(tree: Option<&Tree>, args: &[&QObject__], env: &Environment__) -> Result_ {
// 	if let Some(tree) = tree {
// 		env.set_arguments(args);
// 		match tree.execute(&env) {
// 			Ok(thing) => Ok(thing),
// 			Err(Exception_::Old(Exception__::Return(0, Some(val)))) => Ok(::obj_::QObject_::Old(val)),
// 			Err(Exception_::Old(Exception__::Return(0, None))) => Ok(::obj_::QObject_::Old(().into())),
// 			Err(Exception_::Old(Exception__::Return(i, ret_val))) => Err(Exception__::Return(i - 1, ret_val).into()),
// 			other => other
// 		}
// 	} else {
// 		Ok(::obj_::QObject_::Old(().into())) // aka we have an empty tree
// 	}
// }