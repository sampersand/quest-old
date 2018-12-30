#![allow(unused)]
use quest::object::{TypedObject, IntoObject};
use quest::*;

fn main() {
    simple_logger::init().unwrap();

    let twenty = 20i32.into_object();
    let fifteen = 15i32.into_object();

    println!("{:?}", twenty.call_attr("+", &[&fifteen]));
    println!("{:?}", twenty.call_attr("@bool", &[]));
}