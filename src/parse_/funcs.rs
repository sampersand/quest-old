use parse::{Stream, Parsable, Tree, impls};

use obj::{self, AnyObject, QObject};
use std::{fs, io};
use env::{Binding, Environment};

pub fn parse_str(data: &str, bind: Binding) -> Tree {
	parse_stream(&mut Stream::from_str(data), env)
}

fn foo(env: &Environment) {
	let o: AnyObject = obj::SharedObject::from(false) as _;
	println!("{}", o.call_attr("@text", &[], env).unwrap().downcast::<String>().unwrap());
	panic!()
}

pub fn parse_file(file: &str, env: &Environment) -> io::Result<Tree> {
	foo(env);
	let data = fs::read_to_string(file)?;
	let data = data.splitn(2, "\n__EOF__\n").next().unwrap().splitn(2, "\n__END__\n").next().unwrap(); // these prematurely end the file

	Ok(parse_stream(&mut Stream::from_file(file, &data), env))
}

pub fn parse_stream(stream: &mut Stream, env: &Environment) -> Tree {
	use obj::Id;
	use obj::classes::*;
	use self::impls::*;

	let mut objects: Vec<AnyObject> = Vec::new();

	macro_rules! try_parse {
		($($ty:ty)*) => {
			$(
				if let Some(object) = <$ty>::try_parse(stream) {
					objects.push(object);
					continue
				}
			)*
		}
	}

	while !stream.as_str().is_empty() {

		if Whitespace::try_parse(stream).is_some() || Comment::try_parse(stream).is_some() {
			continue
		}

		if LParen::try_parse(stream).is_some() {
			break
		}

		try_parse!(QNull QBool QText QNum QOper QVar);
		panic!("No objects could match the stream: {}", stream.as_str());
	}
	Tree::from(objects)
}