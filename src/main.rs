#![allow(unused)]
use quest::object::{TypedObject, IntoObject};
use quest::*;

fn main() {
    simple_logger::init().unwrap();

    let twenty = 20i32.into_object();
    let fifteen = 15i32.into_object();

    let mut parent = twenty.get_attr("@parent").unwrap();
    parent.set_attr("()", parent.get_attr("*").unwrap());

    println!("{:?}", twenty.call_attr("()", &[&fifteen]));
    println!("{:?}", fifteen.call_attr("()", &[&twenty]));
    println!("{:?}", twenty.get_attr("()"));
    println!("{:?}", true.into_object().get_attr("()"));
    // println!("{:?}", twenty.call_attr("+", &[&fifteen]));
    // println!("{:?}", twenty.call_attr("@bool", &[]));
}