#![allow(unused)]
use quest::*;

fn main() {
    simple_logger::init().unwrap();

    let ref three = 3.into_object();
    let ref four = 4.into_object();
    let ref twenty = 20i32.into_object();
    let ref fifteen = 15i32.into_object();
    let ref r#true = true.into_object();
    let ref r#false = false.into_object();
    let ref plus = "+".into_object();
    let ref foo = "foo".into_object();
    let ref samp = "samp".to_string().into_object();

    let ref r#if = quest::__BUILTINS_MAP.get(&"if".into_object()).unwrap();
    
    foo.call_attr("=", &[r#false]).unwrap();
    let ref bar = 
        r#if.call_attr("()", &[&foo.call_attr("()", &[]).unwrap(), twenty, fifteen])
            .unwrap()
            .call_attr("*", &[three])
            .unwrap();

    println!("{:?}", bar);

    println!("{:?}", samp.call_attr("*", &[three]));

    // println!("{:?}", r#if.call_attr("()", &[truth, twenty, fifteen]));

    // println!("{:?}", foo.call_attr("()", &[]));
    // println!("{:?}", quest::Environment::current());

    // println!("{:#?}", twenty.call(plus, &[fifteen]));
    // println!("{:?}", truth.call(plus, &[fifteen]));

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