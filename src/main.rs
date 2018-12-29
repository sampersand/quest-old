#![allow(unused)]
use quest::object::TypedObject;
use quest::*;

fn main() {
    simple_logger::init().unwrap();
	let o = TypedObject::new_num(123);
	let o = o.objectify();
	println!("{:#?}", o.call(&TypedObject::new_var("@text").objectify(), &[]));
}