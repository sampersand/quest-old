#![allow(unused)]
extern crate quest;

extern crate log;
extern crate simple_logger;

fn main(){
	simple_logger::init_with_level(log::Level::Trace).unwrap();
	let env = quest::Environment::default();
	quest::_foo();
	// unimplemented!()
	// let mut binding = quest::Binding::default();

	// let mut result = binding.parse_file("code/test.qs", None).expect("cant read file");
	// println!("====[ Result ]===\n{:#?}", result.as_slice());
}




