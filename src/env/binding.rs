use env::{Environment, Stream, parse::Precedence};
use shared::{Shared, SharedMap};
use obj::{Id, AnyObject};

use std::{io, fs};
use std::{path::Path, any::Any};
use std::fmt::{self, Debug, Display, Formatter};

type ObjectMap = SharedMap<Id, AnyObject>;
type SharedBinding = Shared<Binding>;


#[derive(Clone, Copy)]
struct Executor(fn(&dyn Any, &mut Binding));

#[derive(Debug, Clone, Default)]
struct Stack {
	objs: Vec<AnyObject>,
	opers: Vec<(AnyObject, Precedence, Executor)>
}

#[derive(Debug)]
pub struct Binding {
	locals: ObjectMap,
	globals: ObjectMap,
	stack: Stack,
	caller: Option<SharedBinding>
}

impl Default for Binding {
	fn default() -> Self {
		info!("TODO: defaults for globals");
		Binding {
			locals: ObjectMap::default(),
			globals: ObjectMap::default(),
			stack: Stack::default(),
			caller: None
		}
	}
}

impl Binding {
	pub fn as_slice(&self) -> &[AnyObject] {
		self.stack.objs.as_slice()
	}

	pub fn as_map(&self) -> &ObjectMap {
		&self.locals
	}


	pub fn pop(&mut self) -> Option<AnyObject> {
		debug_assert!(self.stack.opers.is_empty());
		self.stack.objs.pop()
	}

	pub(super) fn handle(&mut self, obj: AnyObject, precedence: Precedence) {
		if precedence == Precedence::Literal {
			self.stack.objs.push(obj);
			return;
		}

		while let Some((oper, oprecedence, oexec)) = self.stack.opers.pop() {
			if precedence <= oprecedence {
				self.stack.opers.push((oper, precedence, oexec));
				break
			} else {
				unimplemented!("oper1: {:?}", oper);
			}
		}

		self.stack.opers.push((obj, precedence, unimplemented!()));
	}
	pub(super) fn finish(mut self) -> Binding {
		while let Some((oper, _, exec)) = self.stack.opers.pop() {
			(exec.0)(oper, &mut self);
		}
		self
	}
}

#[cfg(debug_assertions)]
impl Binding {
	pub(super) fn stack_is_empty(&self) -> bool {
		self.stack.opers.is_empty() && self.stack.objs.is_empty()
	}
}


impl Binding {
	pub fn parse_str(self, data: &str, caller: Option<SharedBinding>) -> Binding {
		self.parse_stream(Stream::new(data, None, None), caller)
	}

	pub fn parse_file<F: AsRef<Path>>(self, file: F, caller: Option<SharedBinding>) -> io::Result<Binding> {
		let file = file.as_ref();

		let data = fs::read_to_string(file)?;
		let data = data.splitn(2, "\n__EOF__\n").next().unwrap().splitn(2, "\n__END__\n").next().unwrap(); // these prematurely end the file

		Ok(self.parse_stream(Stream::new(data, Some(file), None), caller))
	}

	fn parse_stream(mut self, stream: Stream, caller: Option<SharedBinding>) -> Binding {
		if cfg!(debug_assertions) && caller.is_some() {
			assert!(self.caller.is_none(), "parsed a string and attempted to override an older stream ({:#?})", self);
		}
		self.caller = caller;
		Environment { binding: self, stream }.parse()
	}
}

impl Display for Binding {
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		// write!(f, "{}", self.)
		unimplemented!()
	}
}


impl Debug for Executor {
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		f.debug_tuple("Executor").field(&(self.0 as usize)).finish()
	}
}

impl Eq for Executor {}
impl PartialEq for Executor {
	fn eq(&self, other: &Executor) -> bool {
		self.0 as usize == other.0 as usize
	}
}


