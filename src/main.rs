// #![feature(fn_traits)]
// #![allow(unused)]
// extern crate quest;

// extern crate log;
// extern crate simple_logger;

// use std::{path::Path, fs};
// use self::tmp::*;
// use quest::{*, obj::{*, types::*}, parse::*};

// fn main() -> ::std::result::Result<(), String> {
// 	simple_logger::init_with_level(log::Level::Trace).unwrap();
// 	let ref mut env = quest::Environment::new();

// 	let path = Path::new("code/test.qs");
// 	let data = fs::read_to_string(path).unwrap();
// 	let parsers = parse::default_parsers();{}
// 	let mut stream = Stream::from_path(path, &data, parsers);

// 	env.execute_stream(stream).map_err(|err| err.to_string())?;
// 	Ok(())
// }

extern crate quest;

fn main() {
	quest::env::init(quest::Binding::empty());
	let obj = quest::obj::Object::new(true);

	println!("{:#?}", obj);
}