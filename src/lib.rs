#![allow(unused)]
#![feature(unsize, coerce_unsized, core_intrinsics, never_type)]

#[macro_use]
mod util;

mod object;
mod map;
mod shared;
mod env;
mod err;

pub fn _test(){
	macro_rules! n {
		($obj:expr) => (Object::new_number($obj).as_any())
	}
	macro_rules! t {
		($obj:expr) => (Object::new_text_str($obj).as_any())
	}
	use object::Object;

	let text = Object::new_text_str("abc").as_any();
	let num1 = Object::new_number(1.0).as_any();
	let num2 = Object::new_number(2.0).as_any();


	println!("{:?}", text.call_attr("[]=", &[&n!(1.0), &n!(1.0), &t!("")]));
	// let num2 = object::Object::new_number(000.123);
	// use crate::map::Map;

	// println!("{:?}", num1.call_attr("==", &[&object::Object::new_number(456.0).as_any()]));
	// println!("{:?}", num1.call_attr("===", &[&object::Object::new_number(456.0).as_any()]));
	// // println!("{:#?}", num1._map().read().unwrap().get(&object::Object::new_variable("==").as_any()));

	// let res = num1.call_attr("+", &[&num2.as_any()]).unwrap();
	// println!("{:?}", res);
}