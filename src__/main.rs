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

fn main() -> ::std::result::Result<(), String> {
	simple_logger::init_with_level(log::Level::Trace).unwrap();
	let ref mut env = quest::Environment::new();

	let path = Path::new("code/test.qs");
	let data = fs::read_to_string(path).unwrap();
	let parsers = parse::default_parsers();{}
	let mut stream = Stream::from_path(path, &data, parsers);

	env.execute_stream(stream).map_err(|err| err.to_string())?;
	Ok(())
	// println!("{:#?}", env.pop());//.unwrap().read_call(&("()".into_object() as AnyShared), &[], env));
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
// 	num.read_call("-=", &[&Number::from(2).into_anyshared()], env)?;

// 	let vec = vec![Number::zero().into_anyshared(), Number::one().into_anyshared()].into_anyshared();

// 	fn gt(num: &AnyShared, env: &mut ::env::Environment) -> Result<bool> {
// 		let x = num.read_call(">", &[&Number::zero().into_anyshared()], env)?;
// 		let r = x.read();
// 		r.attrs.into_bool(env)
// 	}

// 	while gt(&num, env)? {
// 		let penult = vec.read_call("[]", &[&Number::neg_one().into_anyshared()], env)?;
// 		let ult = vec.read_call("[]", &[&Number::neg_zero().into_anyshared()], env)?;
// 		println!("penult: {:?}, ult: {:?}", penult, ult);
// 		vec.read_call("<<", &[&penult.read_call("+", &[&ult], env)?], env)?;
// 		num.read_call("--@", &[], env)?;
// 	}

// 	Ok(vec)
// }

// pub fn _foo(){
// 	let ref mut env = ::env::Environment::default();
// 	use self::types::*;

// 	let var = Id::from("name").into_anyshared();
// 	let name = Text::from("sam").into_anyshared();
// 	env.set(var.clone(), name);

// 	println!("{:?}", var.read_call("()", &[], env));
// 	println!("{:?}", var.read_call("~", &[], env));
// 	println!("{:?}", var.read_call("()", &[], env));
// 	println!("{:?}", var.read_call("@text", &[], env));
// }

// fn __() {
// 	let ref mut env = ::env::Environment::default();
// 	use self::types::IntoObject;

// 	println!("{}", _fib(10.into_anyshared(), env).unwrap());

// 	let text = "this is a test".into_anyshared();
// 	let num = 2i32.into_anyshared();

// 	let getter = text.read().attrs.get("[]").unwrap();
// 	text.write().attrs.set("()".into_object(), getter);
	

// 	let list = vec![text.clone(), num.clone(), false.into_anyshared()].into_anyshared();

// 	let map = {
// 		use std::collections::HashMap;
// 		let mut h = HashMap::new();
// 		h.insert("hello".into_anyshared(), "world".into_anyshared());
// 		h.into_anyshared()
// 	};

// 	println!("{:?}", text.read_call("()", &[&num], env));
// 	println!("{}", list);
// 	println!("{}", list.read_call("[]", &[&1i32.into_anyshared()], env).unwrap());
// 	list.read_call("[]=", &[&1i32.into_anyshared(), &"a".into_anyshared()], env).unwrap();
// 	list.read_call("[]~", &[&0i32.into_anyshared()], env).unwrap();
// 	println!("{}", list);

// 	println!("{}", map);
// 	println!("{}", map.read_call("[]", &[&"hello".into_anyshared()], env).unwrap());
// 	map.read_call("[]=", &[&"johnny".into_anyshared(),
// 		&vec!["appleseed".into_anyshared(), "boy".into_anyshared()].into_anyshared()], env).unwrap();
// 	println!("{:?}", map.read_call("@text", &[], env));
// 	println!("{}", map.read_call("[]", &[&"johnny".into_anyshared()], env).unwrap()
// 							.read_call("[]", &[&1i32.into_anyshared()], env).unwrap());
// }

*/