#![allow(unused)]
use quest::object;
use quest::*;

fn main() {
    simple_logger::init().unwrap();
	let o = quest::object::TypedObject::new_num(123);
	let o = o.objectify();
	println!("{:#?}", o);
}