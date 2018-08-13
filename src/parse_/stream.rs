use parse::token::Token;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Stream<'a> {
	data: &'a str,
	file: &'a str,
	line: usize
}

impl<'a> Stream<'a> {
	pub fn from_str(data: &'a str) -> Stream<'a> {
		Stream {
			data,
			file: "<evald string>",
			line: 0
		}
	}
	pub fn from_file(file: &'a str, data: &'a str) -> Stream<'a> {
		Stream { data, file, line: 0 }
	}

	pub fn as_str(&self) -> &'a str {
		self.data
	}

	pub fn advance_by(&mut self, token: &Token) {
		assert!(self.data.starts_with(token.as_str()), "Attempted to advance by a token that we dont have!");
		self.data = &self.data[token.len()..];
	}
}

impl<'a> From<&'a str> for Stream<'a> {
	#[inline]
	fn from(s: &'a str) -> Stream<'a> {
		Stream::from_str(s)
	}
}