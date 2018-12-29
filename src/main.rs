#![allow(unused)]
use quest::object::{TypedObject, IntoObject};
use quest::*;

fn main() {
    simple_logger::init().unwrap();
	let o = 123_i32.into_object();
	let t = o.call(&"@text".into_object(), &[]);
	println!("{:?}", t);
	println!("{:#?}", t.unwrap().call(&"@var".into_object(), &[]));
}