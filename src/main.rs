#![allow(unused)]
extern crate quest;

extern crate simple_logger;

fn main(){
	let ref env = quest::Environment::default();
	simple_logger::init().unwrap();

	let result = quest::parse_file("code/test.qs", env).expect("cant read file");
	println!("====[ Result ]===\n{:#?}", result);
}









