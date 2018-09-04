#![feature(fn_traits)]
#![allow(unused)]
extern crate quest;

extern crate log;
extern crate simple_logger;
#[macro_use]
mod tmp;
use std::{path::Path, fs};
use self::tmp::*;
use quest::{*, obj::{*, types::*}, parse::*};

fn main() {
	simple_logger::init_with_level(log::Level::Trace).unwrap();
	let ref mut env = quest::Environment::new();

	let path = Path::new("code/guess.qs");
	let data = fs::read_to_string(path).unwrap();
	let parsers = parse::default_parsers();{}
	let mut stream = Stream::from_path(path, &data, parsers);

	env.execute_stream(stream).expect("couldn't exec");

	println!("{:#?}", env.pop());//.unwrap().read_call(&("()".into_object() as AnyShared), &[], env));
}
/*
	macro_rules! call {
		($thing:expr, $fn:expr $(,$args:expr)*) => {
			$thing.read_call(&var($fn), &[$($args),*], env)?
		}
	}

	println!("{}", call!(text("127.b0.a0.a1"), "count", text(".a")));
	Ok(())
	// unimplemented!()
	// let mut binding = quest::Binding::default();

	// let mut result = binding.parse_file("code/test.qs", None).expect("cant read file");
	// println!("====[ Result ]===\n{:#?}", result.as_slice());
}

// pub fn _fib(num: AnyShared, env: &mut env::Environment) -> AnyResult {
// 	use self::types::*;
// 	num.read_call("-=", &[&Number::from(2).into_anyobject()], env)?;

// 	let vec = vec![Number::zero().into_anyobject(), Number::one().into_anyobject()].into_anyobject();

// 	fn gt(num: &AnyShared, env: &mut ::env::Environment) -> Result<bool> {
// 		let x = num.read_call(">", &[&Number::zero().into_anyobject()], env)?;
// 		let r = x.read();
// 		r.attrs.into_bool(env)
// 	}

// 	while gt(&num, env)? {
// 		let penult = vec.read_call("[]", &[&Number::neg_one().into_anyobject()], env)?;
// 		let ult = vec.read_call("[]", &[&Number::neg_zero().into_anyobject()], env)?;
// 		println!("penult: {:?}, ult: {:?}", penult, ult);
// 		vec.read_call("<<", &[&penult.read_call("+", &[&ult], env)?], env)?;
// 		num.read_call("--@", &[], env)?;
// 	}

// 	Ok(vec)
// }

// pub fn _foo(){
// 	let ref mut env = ::env::Environment::default();
// 	use self::types::*;

// 	let var = Id::from("name").into_anyobject();
// 	let name = Text::from("sam").into_anyobject();
// 	env.set(var.clone(), name);

// 	println!("{:?}", var.read_call("()", &[], env));
// 	println!("{:?}", var.read_call("~", &[], env));
// 	println!("{:?}", var.read_call("()", &[], env));
// 	println!("{:?}", var.read_call("@text", &[], env));
// }

// fn __() {
// 	let ref mut env = ::env::Environment::default();
// 	use self::types::IntoObject;

// 	println!("{}", _fib(10.into_anyobject(), env).unwrap());

// 	let text = "this is a test".into_anyobject();
// 	let num = 2i32.into_anyobject();

// 	let getter = text.read().attrs.get("[]").unwrap();
// 	text.write().attrs.set("()".into_object(), getter);
	

// 	let list = vec![text.clone(), num.clone(), false.into_anyobject()].into_anyobject();

// 	let map = {
// 		use std::collections::HashMap;
// 		let mut h = HashMap::new();
// 		h.insert("hello".into_anyobject(), "world".into_anyobject());
// 		h.into_anyobject()
// 	};

// 	println!("{:?}", text.read_call("()", &[&num], env));
// 	println!("{}", list);
// 	println!("{}", list.read_call("[]", &[&1i32.into_anyobject()], env).unwrap());
// 	list.read_call("[]=", &[&1i32.into_anyobject(), &"a".into_anyobject()], env).unwrap();
// 	list.read_call("[]~", &[&0i32.into_anyobject()], env).unwrap();
// 	println!("{}", list);

// 	println!("{}", map);
// 	println!("{}", map.read_call("[]", &[&"hello".into_anyobject()], env).unwrap());
// 	map.read_call("[]=", &[&"johnny".into_anyobject(),
// 		&vec!["appleseed".into_anyobject(), "boy".into_anyobject()].into_anyobject()], env).unwrap();
// 	println!("{:?}", map.read_call("@text", &[], env));
// 	println!("{}", map.read_call("[]", &[&"johnny".into_anyobject()], env).unwrap()
// 							.read_call("[]", &[&1i32.into_anyobject()], env).unwrap());
// }

*/