use std::ops::Deref;
use std::cmp::min;
use std::fmt::{self, Display, Debug, Formatter};

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct Stream<'a> {
	data: &'a str,
	src: Source
}

impl<'a> Stream<'a> {
	pub fn from_file(file: &str, data: &'a str) -> Stream<'a> {
		Stream::from_source(file.to_string(), data)
	}
	pub fn from_str(data: &'a str) -> Stream<'a> {
		Stream::from_source(format!("<data from '{}'>", &data[0..min(data.len(), 10)]), data)
	}
	pub fn from_source(source: String, data: &'a str) -> Stream<'a> {
		Stream { data, src: Source { source, line: 0, col: 0 }}
	}
}

impl<'a> Stream<'a> {
	pub(super)fn get_src(&self) -> Source {
		self.src.clone()
	}

	pub(super) fn offset_by(&mut self, amnt: usize) {
		// todo: fix this
		if self.data[..amnt].contains('\n') {
			self.src.line += self.data[0..amnt].lines().count();
			self.src.col = self.data[0..amnt].lines().last().unwrap().len();
		} else {
			self.src.col += amnt;
		}

		self.data = &self.data[amnt..];
	}
}

impl<'a> Deref for Stream<'a> {
	type Target = str;
	fn deref(&self) -> &str {
		&self.data
	}
}

impl<'a> Debug for Stream<'a> {
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		write!(f, "Stream({}, '{}')", self.src, self.data)
	}
}


impl<'a> Display for Stream<'a> {
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		Display::fmt(&self.data, f)
	}
}


#[derive(Clone, PartialEq, Eq, Hash)]
pub struct Source {
	source: String,
	line: usize,
	col: usize
}

impl Display for Source {
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		write!(f, "{}:{}", self.source, self.line)
	}
}

impl Debug for Source {
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		write!(f, "Source({:?}, {}, {})", self.source, self.line, self.col)
	}
}