use sync::SpinRwLock;
use std::hash::{Hash, Hasher};
use std::ops::{Deref, DerefMut};
use std::sync::Arc;
use obj::{Id, classes, attrs::*, object::Classes};
use env::Environment;
use std::fmt::{self, Display, Formatter};

#[derive(Debug)] // might eq and Partial Eq be because of ID?
pub struct QObj {
	id: Id,
	class: SpinRwLock<Classes>,
	pub(super) attrs: Attributes,
}

impl Eq for QObj {}
impl PartialEq for QObj {
	fn eq(&self, other: &QObj) -> bool {
		*self.class.read() == *other.class.read()
	}
}

impl Hash for QObj {
	fn hash<H: Hasher>(&self, hasher: &mut H) {
		self.class.read().hash(hasher);
	}
}

impl QObj {
	pub fn new<O: HasDefaultAttrs + Into<Classes>>(obj: O) -> QObj {
		QObj {
			id: Id::next(),
			class: SpinRwLock::new(obj.into()),
			attrs: Attributes::new(O::default_attrs())
		}
	}

	#[inline]
	pub fn id(&self) -> Id {
		self.id
	}

	#[inline]
	pub fn class<'a>(&'a self) -> impl Deref<Target=Classes> + 'a {
		self.class.read()
	}

	#[inline]
	pub fn class_mut<'a>(&'a self) -> impl DerefMut<Target=Classes> + 'a {
		self.class.write()
	}

	#[inline]
	pub fn into_class(self) -> Classes {
		self.class.into_inner()
	}
}

impl Clone for QObj {
	fn clone(&self) -> Self {
		QObj {
			id: Id::next(),
			attrs: self.attrs.clone(),
			class: self.class.try_clone().expect("unable to clone qobj")
		}
	}
}

impl Display for QObj {
	#[inline]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		Display::fmt(&*self.class.read(), f)
	}
}

default_attrs! { for QObj;
	fn "@text" (this) {
		this.to_string().into()
	}

	fn "@bool" (this) {
		true.into()
	}

	fn "clone" (this) {
		this.clone().into()
	}

	fn "==" (this, rhs) {
		(this == rhs.deref()).into()
	}

	fn "!=" (this, rhs) {
		(this != rhs.deref()).into()
	}

	fn "." (_this, attr) with env vars obj {
		obj.get_attr(attr.clone()).unwrap_or_else(|| ().into())
	}
}

// __default_attrs! {
// 	"@text" => |obj, _, _| classes::QText::new(obj).into(),
// 	"@bool" => |_, _, _| classes::QBool::new(true).into(),
// 	"clone" => |obj, _, _| QObj::clone(&obj).into(),
// 	"." => |obj, args, env| obj.get_attr(args[0].as_var(env).expect("var is currently required to index").as_id()).unwrap_or_else(|| ().into()),//QObj::default_attrs()[&Id::from("get_attr")].call(obj, args, env),
// 	"get_attr" => |obj, args, _| {
// 		assert_args_len!(args, 1, "get_attr");
// 		let val = match args[0].class {
// 			Classes::Var(id) => obj.get_attr(id.as_id()),
// 			_ => obj.get_attr(args[0].clone())
// 		};
// 		match val {
// 			Some(val) => val,
// 			None => {
// 				eprintln!("Attribute `{}` doesn't exist for `{:?}`", args[0], obj);
// 				classes::QNull.into()
// 			}
// 		}
// 	},
// 	"set_attr" => |obj, args, _| {
// 		assert_args_len!(args, 2, "set_attr");
// 		obj.set_attr(expect_id!(args[0]), args[1].clone()).unwrap_or_else(|| classes::QNull.into())
// 	},
// 	"del_attr" => |obj, args, _| {
// 		assert_args_len!(args, 1, "del_attr");
// 		obj.del_attr(expect_id!(args[0])).unwrap_or_else(|| classes::QNull.into())
// 	},
// 	"attr?" => |obj, args, _| {
// 		assert_args_len!(args, 1, "del_attr");
// 		classes::QBool::new(obj.has_attr(expect_id!(args[0]))).into()

// 	}
// }