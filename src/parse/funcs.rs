use parse::{Stream, Parsable, impls};

use obj::{self, SharedObject, QObject};
use std::{fs, io};
use env::Environment;

pub fn parse_file(file: &str, env: &Environment) -> io::Result<Vec<SharedObject>> {
	let data = fs::read_to_string(file)?;
	let mut stream = Stream::from_file(file, &data);
	Ok(parse_stream(&mut Stream::from_file(file, &data)))
}

fn parse_stream(stream: &mut Stream) -> Vec<SharedObject> {
	use obj::Id;
	use obj::classes::{oper::Oper, num::Number, null::Null};
	use self::impls::*;

	let mut objects = Vec::new();

	macro_rules! try_parse {
		($($ty:ty)*) => {
			$(
				if let Some(object) = <$ty>::try_parse(stream) {
					objects.push(QObject::from(object).make_shared() as _);
					continue
				}
			)*
		}
	}

	while !stream.as_str().is_empty() {

		if Whitespace::try_parse(stream).is_some() || Comment::try_parse(stream).is_some() {
			continue
		}

		try_parse!(Null bool String Number Oper Id);
		panic!("No objects could match the stream: {}", stream.as_str());
	}
	objects
}