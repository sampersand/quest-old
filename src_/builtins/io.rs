use obj_::classes_::{QNull, QList};
use obj_::QObject_;
use env_::Environment__;
use std::io::{self, BufRead};

builtins!{
	fn DISP(args, env) {
		println!("{}", args.iter().map(|arg| arg.as_text(env).expect("`@text` is required for disp").as_str().to_owned()).collect::<Vec<_>>().join(" "));

		if args.len() == 1 {
			Ok(QObject_::Old(args[0].clone()))
		} else {
			Ok(QObject_::Old(QList::new(args.iter().map(|x| (*x).clone()).collect()).into()))
		}
	}
	fn PROMPT(args, env) {
		if let Some(message) = args.first() {
			DISP.call_from_null(&[message], env);
		}

		let mut input = String::new();
		let mut stdin = io::stdin().read_line(&mut input).expect("Unable to read line to input");
		if input.chars().last() == Some('\n') {
			assert_eq!(input.pop(), Some('\n'));
			if input.chars().last() == Some('\r') {
				assert_eq!(input.pop(), Some('\r'));
			}
		}
		Ok(QObject_::Old(input.into()))
	}
}