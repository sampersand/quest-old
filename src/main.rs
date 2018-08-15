#![allow(unused)]
extern crate quest;

extern crate log;
extern crate simple_logger;

fn main(){
	let mut binding = quest::Binding::default();
	simple_logger::init_with_level(log::Level::Trace).unwrap();

	let result = binding.parse_file("code/test.qs").expect("cant read file");
	println!("====[ Result ]===\n{:#?}", result);
}









