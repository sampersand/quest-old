use std::str::Chars;

#[derive(Debug)]
pub struct CharIter<'a> {
	iter: Chars<'a>,
	chars_count: usize,
}

impl CharIter<'_> {
	#[inline]
	pub fn chars_count(&self) -> usize {
		self.chars_count
	}

	#[inline]
	pub fn unwind(&mut self) {
		self.unwind_n(1);
	}

	#[inline]
	pub fn unwind_n(&mut self, amnt: usize) {
		self.chars_count -= amnt;
	}
}

impl<'a> From<&'a str> for CharIter<'a> {
	fn from(inp: &'a str) -> CharIter<'a> {
		CharIter {
			iter: inp.chars(),
			chars_count: 0
		}
	}
}

impl Iterator for CharIter<'_> {
	type Item = char;
	fn next(&mut self) -> Option<char> {
		let c = self.iter.next()?;
		self.chars_count += c.len_utf8();
		Some(c)
	}
}