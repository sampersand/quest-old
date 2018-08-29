mod err;
mod id;
mod object;
mod attrs;

pub mod types;

pub use self::id::Id;
pub use self::types::Type;
pub use self::object::Object;
pub use self::err::{Error, Result};

use std::any::Any;
use shared::{Shared, Weak};

pub type AnyObject = Object<dyn Any>;
pub type WeakObject = Weak<AnyObject>;
pub type SharedObject<T> = Shared<Object<T>>;
pub type AnyShared = Shared<AnyObject>;

pub type SharedResult<T> = Result<SharedObject<T>>;
pub type AnyResult = Result<AnyShared>;

pub fn _foo(){
	let ref mut env = ::env::Environment;
	use self::types::IntoObject;

	let text = "this is a test".into_anyobject();
	let num = 2i32.into_anyobject();

	let getter = text.read().attrs.get("[]").unwrap();
	text.write().attrs.set("()".into_object(), getter);
	

	let list = vec![text.clone(), num.clone(), false.into_anyobject()].into_anyobject();

	let map = {
		use std::collections::HashMap;
		let mut h = HashMap::new();
		h.insert("hello".into_anyobject(), "world".into_anyobject());
		h.into_anyobject()
	};

	println!("{:?}", text.read_call("()", &[&num], env));
	println!("{}", list);
	println!("{}", list.read_call("[]", &[&1i32.into_anyobject()], env).unwrap());
	list.read_call("[]=", &[&1i32.into_anyobject(), &"a".into_anyobject()], env).unwrap();
	list.read_call("[]~", &[&0i32.into_anyobject()], env).unwrap();
	println!("{}", list);

	println!("{}", map);
	println!("{}", map.read_call("[]", &[&"hello".into_anyobject()], env).unwrap());
	map.read_call("[]=", &[&"johnny".into_anyobject(),
		&vec!["appleseed".into_anyobject(), "boy".into_anyobject()].into_anyobject()], env).unwrap();
	println!("{}", map);
	println!("{}", map.read_call("[]", &[&"johnny".into_anyobject()], env).unwrap()
							.read_call("[]", &[&1i32.into_anyobject()], env).unwrap());
}











