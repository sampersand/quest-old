extern crate quest;

fn main() {
	let x = quest::parse_str("foo", None);
	println!("{:?}", x);
}