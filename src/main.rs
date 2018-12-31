#![allow(unused)]
use quest::object::{TypedObject, IntoObject};
use quest::*;

fn main() {
    simple_logger::init().unwrap();

    let ref twenty = 20i32.into_object();
    let ref fifteen = 15i32.into_object();
    let ref truth = true.into_object();
    let ref plus = "+".into_object();

    println!("{:#?}", 
        truth.call(plus, &[fifteen])
    );

    // println!("{:#?}", parent.get_attr("@parent"));
    // basic.set_attr("()", parent.get_attr("*").unwrap());
    // // basic.set_attr("bool", basic.get_attr("@bool").unwrap());
    // println!("{:?}", fifteen.call_attr("__id__", &[]));

    // println!("{:?}", twenty.call_attr("()", &[&fifteen]));
    // println!("{:?}", fifteen.call_attr("()", &[&twenty]));
    // println!("{:?}", twenty.get_attr("()"));
    // println!("{:?}", true.into_object().get_attr("()"));
    // println!("{:?}", true.into_object().call_attr("()", &[&fifteen]));
    // println!("{:?}", twenty.call_attr("+", &[&fifteen]));
    // println!("{:?}", twenty.call_attr("@bool", &[]));
}