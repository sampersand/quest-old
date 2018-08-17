use parse::Parser;
use std::str::pattern::Pattern;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Stream<'a> {
	parsers: Vec<Parser>,
	data: &'a str,
	file: &'a str,
	line: usize
}

impl<'a> Stream<'a> {
	pub fn from_str(data: &'a str) -> Stream<'a> {
		Stream {
			parsers: unimplemented!(),
			data,
			file: "<evald string>",
			line: 0
		}
	}

	pub fn from_file(file: &'a str, data: &'a str) -> Stream<'a> {
		Stream { parsers: unimplemented!(), file, data, line: 0 }
	}

	pub fn as_str(&self) -> &'a str {
		self.data
	}

	pub fn try_get<P: Pattern<'a>>(&mut self, pat: P) -> Option<&'a str> {
		let (pos, val) = self.data.match_indices(pat).next()?;
		if pos == 0 {
			self.data = &self.data[val.len()..];
			self.line += val.matches('\n').count();
			Some(val)
		} else {
			None
		}
	}
}

impl<'a> From<&'a str> for Stream<'a> {
	#[inline]
	fn from(s: &'a str) -> Stream<'a> {
		Stream::from_str(s)
	}
}