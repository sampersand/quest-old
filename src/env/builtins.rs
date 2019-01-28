use lazy_static::lazy_static;
use crate::{Object, Error, Environment};
use std::io::{self, Write, Read};

// to make it easier on my eyes
macro_rules! builtins {
	($($args:tt)*) => {
		lazy_static! { 
			pub static ref BUILTINS_MAP: Object = Object::new(crate::collections::ParentalMap::new_mapped(
				|| crate::object::typed::PRISTINE_MAP.clone(),
				function_map!(prefix="Builtins", downcast_fn=__error, $($args)*)
			));
		}
	}
}

builtins! {
	fn "if" (@cond, if_true; if_false=Object::new_null()) {
		if cond.into_bool()?.into_inner() {
			if_true.clone()
		} else {
			if_false
		}
	}

	fn "while" (@cond, body) {
		while cond.call_attr("()", &[])?.into_bool()?.into_inner() {
			match body.call_attr("()", &[]) {
				Ok(_) => {},
				Err(crate::Error::NothingToReturn) => {},
				Err(other) => return Err(other)
			}
		}
		Object::new_null()
	}

	fn "loop" (@body) {
		loop {
			body.call_attr("()", &[])?;
		}
	}

	fn "switch" (@case, body) {
		body.call_attr("()", &[])?.call_attr("[]", &[case])?
	}

	fn "return" (@which) args {
		return Err(crate::Error::Return {
			env: which.into_env()?,
			obj: args.get(1).cloned().cloned()
		})
		// return Err(crate::Error::)
		// let obj = args.get(1)
		// println!("{:?}", args);
		// todo!();
	} // exit === return

	fn "import" (@_file) { todo!(); }

	fn "disp" (_) args {
		let sep = Environment::current()
			.get_attr("sep")
			.and_then(|x| x.into_text().ok())
			.map(|x| x.as_ref().clone())
			.unwrap_or_else(|| String::from(" "));

		let end = Environment::current()
			.get_attr("end")
			.and_then(|x| x.into_text().ok())
			.map(|x| x.as_ref().clone())
			.unwrap_or_else(|| String::from("\n"));

		let v = args
			.iter()
			.map(|x| x.into_text().map(|x| x.into()))
			.collect::<::std::result::Result<Vec<String>, Error>>()?;
		io::stdout().write(v.join(&sep).as_ref()).map_err(Error::IoError)?;
		io::stdout().write(end.as_ref()).map_err(Error::IoError)?;

		if args.len() == 1 {
			(*args[0]).clone()
		} else {
			args.iter().map(|x| (*x).clone()).collect::<Vec<_>>().into_object()
		}
	}

	fn "input" (@;prompt=Object::new_null()) {
		if !prompt.is_null() {
			io::stdout()
				.write(prompt.into_text()?.as_ref().as_ref())
				.map_err(Error::IoError)?;
		}
		let mut buffer = String::new();
		io::stdin().read_to_string(&mut buffer).map_err(Error::IoError)?;
		buffer.into_object()
	}
	
	fn "rand" (_) args {
		if args.len() == 0 {
			return Ok(rand::random::<f64>().into_object())
		}
		todo!()
	}
}