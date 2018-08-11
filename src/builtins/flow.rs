use obj::{Exception, classes::QNull};
use env::Environment;
use std::process;
use builtins;

builtins!{
	fn IF(args, env) {
		assert!(args.len() > 0, "`if` requires a condition");
		let cond = args[0].as_bool(env)?.to_bool();
		if let Some(obj) = args.get(1 + !cond as usize) {
			if obj.is_block(){
				obj.call_local(&[], env)
			} else {
				Ok((*obj).clone())
			}
		} else {
			Ok(QNull.into())
		}
	}

	fn WHILE(args, env){
		assert!(args.len() >= 2, "`while` needs condition and body");
		let cond = args[0];
		let body = args[1];
		let mut result = None;

		while cond.call_local(&[], env)?.as_bool(env)?.to_bool() {
			result = Some(body.call_local(&[], env)?);
		}

		Ok(result.unwrap_or_else(|| QNull.into()))
	}

	fn RETURN(args, env){
		Err(match args.len() {
			0 => Exception::Return(1, None),
			1 => Exception::Return(1, Some(args[0].clone())),
			_ => Exception::Return(args[1].as_num(env)?.into(), Some(args[0].clone()))
		})
	}

	fn EXIT(args, env){
		if let Some(message) = args.get(0) {
			if !message.is_null() {
				builtins::io::DISP.call_from_null(&[message], env);
			}
		}

		if let Some(exitstatus) = args.get(1) {
			process::exit(exitstatus.as_num(env)?.try_to_i32().expect("integer required for exit status"));
		}  else {
			process::exit(1);
		}
	}
}