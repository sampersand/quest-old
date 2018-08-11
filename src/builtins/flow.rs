use obj::classes::QNull;
use env::Environment;
use std::process;
use builtins;

builtins!{
	fn IF(args, env) {
		assert!(args.len() > 0, "`if` requires a condition");
		let cond = args[0].as_bool(env).expect("`@bool` required for if condition").to_bool();
		if let Some(obj) = args.get(1 + !cond as usize) {
			if obj.is_block(){
				obj.call(&[], env)
			} else {
				(*obj).clone()
			}
		} else {
			QNull.into()
		}
	}

	fn WHILE(args, env){
		assert!(args.len() >= 2, "`while` needs condition and body");
		let cond = args[0];
		let body = args[1];
		let mut result = None;

		while cond.call(&[], env).as_bool(env).expect("condition doesn't respond to `@bool`").to_bool() {
			result = Some(body.call(&[], env));
		}

		result.unwrap_or_else(|| QNull.into())
	}

	fn RETURN(args, env){
		unimplemented!("return")
	}

	fn EXIT(args, env){
		if let Some(message) = args.get(0) {
			if !message.is_null() {
				builtins::io::DISP.call_from_null(&[message], env);
			}
		}

		if let Some(exitstatus) = args.get(1) {
			process::exit(exitstatus.as_num(env).expect("`@num` requried for exit status").try_to_i32().expect("integer required for exit status"));
		}  else {
			process::exit(1);
		}
	}
}