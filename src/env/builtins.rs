use obj::{Object, AnyShared, Result, types::{IntoObject, BoundFn}};
use std::collections::HashMap;
use parse::Token;
use std::io::{self, BufRead};
use rand;


pub fn disp_fn() -> BoundFn {
	BoundFn::bind_void(|args, env| {
		if args.len() != 0 {
			print!("{}", args[0].read_into_text(env)?.as_str());
			for arg in &args[1..] {
				print!(" {}", arg.read_into_text(env)?.as_str());
			}
		}
		println!();
		Ok(args.into_object())
	})
}

pub fn if_fn() -> BoundFn {
	BoundFn::bind_void(|args, env| {
		let cond = args.get(0).expect("`cond` needed for if statements");
		let if_true = args.get(1).expect("`if_true` needed for if statements");
		let if_false = args.get(2);

		if cond.read_into_bool(env)? == true {
			Ok(if_true.clone())
		} else if let Some(if_false) = if_false {
			Ok(if_false.clone())
		} else {
			Ok(Object::null())
		}
	})
}

pub fn while_fn() -> BoundFn {
	BoundFn::bind_void(|args, env| {
		let cond = args.get(0).expect("`cond` needed for while statements");
		let body = args.get(1).expect("`body` needed for while statements");

		let mut last = Object::null();
		while cond.read_call(&("()".into_object() as AnyShared), &[], env)?.read_into_bool(env)? == true {
			last = body.read_call(&("()".into_object() as AnyShared), &[], env)?;
		}
		Ok(last)
	})
}


pub fn rand_fn() -> BoundFn {
	BoundFn::bind_void(|args, env| {
		let rand = (rand::random::<u32>() & (::std::u32::MAX >> 1)) as i32;

		Ok(rand.into_object() as AnyShared)
	})
}


pub fn prompt_fn() -> BoundFn {
	BoundFn::bind_void(|args, env| {
		disp_fn().call_bound(args, env)?;
		let stdin = io::stdin();
		let res = stdin.lock().lines().next()
			.transpose()
			.expect("io error encountered when prompting!")
			.map(|x| x.into_object() as AnyShared)
			.unwrap_or_else(Object::null);
		Ok(res)
	})
}
