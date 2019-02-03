#![allow(unused)]

#![feature(unsize, coerce_unsized, core_intrinsics, never_type)]
mod object;
mod map;
mod shared;
mod env;
mod err;
mod util;

pub fn _test(){
	let num1 = object::Object::new_number(456.0);
	let num2 = object::Object::new_number(000.123);
	use crate::map::Map;

	println!("{:?}", num1.call_attr("==", &[&object::Object::new_number(456.0).as_any()]));
	println!("{:?}", num1.call_attr("===", &[&object::Object::new_number(456.0).as_any()]));
	// println!("{:#?}", num1._map().read().unwrap().get(&object::Object::new_variable("==").as_any()));

	let res = num1.call_attr("+", &[&num2.as_any()]).unwrap();
	println!("{:?}", res);
}