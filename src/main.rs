extern crate quest;

fn main() {
	let x = quest::parse_str(" ", None);
	println!("{:?}", x);
}