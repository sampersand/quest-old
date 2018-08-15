use env::{Stream, Environment};
use shared::SharedMap;

use obj::{Id, AnyObject};
use std::{io, fs, path::Path};

#[derive(Debug, Clone)]
pub struct Binding<'a> {
	locals: SharedMap<Id, AnyObject>,
	globals: SharedMap<Id, AnyObject>,
	caller: Option<&'a Binding<'a>>
}

impl<'bind> Default for Binding<'bind> {
	fn default() -> Self {
		Binding {
			locals: SharedMap::default(),
			globals: {
				info!(target: "todo", "TODO: default globals");
				SharedMap::default()
			},
			caller: None,
		}
	}
}

impl<'a> Binding<'a> {
	pub fn new(locals: SharedMap<Id, AnyObject>, globals: SharedMap<Id, AnyObject>) -> Self{
		Binding { locals, globals, caller: None }
	}

	pub fn parse_str(self, data: &str) -> AnyObject {
		Environment {
			binding: self,
			stream: Stream::new(data, None, None),
			stack: Default::default()
		}.parse()
	}

	pub fn parse_file<F: AsRef<Path>>(self, file: F) -> io::Result<AnyObject> {
		let file = file.as_ref();

		let data = fs::read_to_string(file)?;
		let data = data.splitn(2, "\n__EOF__\n").next().unwrap().splitn(2, "\n__END__\n").next().unwrap(); // these prematurely end the file

		Ok(Environment {
			binding: self,
			stream: Stream::new(data, Some(file), None),
			stack: Default::default()
		}.parse())
	}
}

impl<'a> Eq for Binding<'a> {}
impl<'a> PartialEq for Binding<'a> {
	fn eq(&self, other: &Binding<'a>) -> bool {
		if self as *const Binding<'a> == other as *const Binding<'a> {
			return true;
		}

		let other_locals = other.locals.read();

		for (key, val) in self.locals.read().iter() {
			match other_locals.get(key) {
				// Some(oval) if oval == val => {},
				// _ => return false
				_ => unimplemented!()
			}
		}

		drop(other_locals);

		if cfg!(debug_assertions) {
			let other_globals = other.globals.read();
			for (key, val) in self.globals.read().iter() {
				match other_globals.get(key) {
					// Some(oval) if oval == val => {},
					// _ => return false
					_ => unimplemented!()
				}
			}
		}

		true // we dont care if caller matches
	}
}