use parse::token::Token;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Stream<'a> {
	data: &'a str,
	file: &'a str,
	line: usize
}

impl<'a> Stream<'a> {
	pub fn from_file(file: &'a str, data: &'a str) -> Stream<'a> {
		Stream { file, data, line: 0 }
	}

	pub fn as_str(&self) -> &'a str {
		self.data
	}

	pub fn advance_by(&mut self, len: usize) {
		assert!(self.data.len() >= len, "Attempted to advance too far! ({} > {})", len, self.data.len());
		self.data = &self.data[len..];
	}
}