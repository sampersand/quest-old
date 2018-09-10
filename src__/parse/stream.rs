use obj::AnyShared;
use env::Environment;

use std::ops::{Deref, DerefMut};
use parse::{ParserFn, Token};
use std::str::Chars;
use std::path::Path;
use std::fmt::{self, Debug, Formatter};

#[derive(Clone, Default)]
pub struct ParserFnVec(Vec<ParserFn>);

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Stream<'a> {
	parsers: ParserFnVec,
	data: &'a str,
	path: Option<&'a Path>,
	pub eof: bool,
	line: usize
}

#[derive(Debug)]
pub struct StreamChars<'a> {
	chars: Chars<'a>,
	eof: &'a bool,
	offset: usize,
	prev: Option<char>,
}

impl<'a> Stream<'a> {
	pub fn from_str(data: &'a str, parsers: Vec<ParserFn>) -> Stream<'a> {
		Stream { parsers: ParserFnVec(parsers), data, ..Default::default() }
	}

	pub fn from_path(path: &'a Path, data: &'a str, parsers: Vec<ParserFn>) -> Stream<'a> {
		Stream { parsers: ParserFnVec(parsers), data, ..Default::default() }
	}

	pub fn as_str(&self) -> &str {
		self.as_ref()
	}

	pub fn as_str_raw(&self) -> &str {
		self.data
	}


	pub fn offset_by_char(&mut self, c: char) -> &str{
		self.offset_to(c.len_utf8())
	}

	pub fn offset_by(&mut self, s: &str) -> &str {
		assert!(self.as_str().starts_with(s));
		self.offset_to(s.len())
	}

	pub fn offset_to(&mut self, amnt: usize) -> &str {
		let (old, new) = self.data.split_at(amnt);
		self.data = new;
		self.line += old.matches('\n').count();
		if self.data.is_empty() {
			self.eof = true;
		}
		old
	}

	pub fn chars(&mut self) -> StreamChars {
		StreamChars {
			offset: 0,
			prev: None,
			eof: &self.eof,
			chars: self.data.chars()
		}
	}
}

impl<'a> Iterator for Stream<'a> {
	type Item = Token;
	fn next(&mut self) -> Option<Token> {
		let parsers = self.parsers.clone();

		for parser in parsers.iter() {
			if let Some(result) = (parser)(self) {
				return Some(result);
			}

			if self.eof { 
				return None;
			}
		}

		panic!("No rules could find a token at the stream: {:#?}", self)
	}
}

impl<'a> StreamChars<'a> {
	pub fn rollback(&mut self) {
		self.offset -= self.prev.expect("`prev` is needed to rollback").len_utf8();
	}

	pub fn offset(&self) -> usize {
		self.offset
	}

	pub fn prev(&self) -> Option<char> {
		self.prev.clone()
	}
}

impl<'a> Iterator for StreamChars<'a> {
	type Item = char;
	fn next(&mut self) -> Option<char> {
		if *self.eof {
			return None;
		}

		let c = self.chars.next()?;
		self.offset += c.len_utf8();
		self.prev = Some(c);
		Some(c)
	}
}

impl<'a> AsRef<str> for Stream<'a> {
	#[inline]
	fn as_ref(&self) -> &str {
		if self.eof {
			""
		} else {
			&self.data
		}
	}
}

impl Deref for ParserFnVec {
	type Target = Vec<ParserFn>;

	fn deref(&self) -> &Vec<ParserFn> {
		&self.0
	}
}

impl DerefMut for ParserFnVec {
	fn deref_mut(&mut self) -> &mut Vec<ParserFn> {
		&mut self.0
	}
}

impl Eq for ParserFnVec {}
impl PartialEq for ParserFnVec {
	fn eq(&self, other: &ParserFnVec) -> bool {
		if self.len() != other.len() {
			return false;
		}

		(0..self.len()).all(|i| self[i] as usize != other[i] as usize)
	}
}

impl Debug for ParserFnVec {
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		struct FnPtr(usize);

		impl Debug for FnPtr {
			fn fmt(&self, f: &mut Formatter) -> fmt::Result {
				write!(f, "{:p}", self.0 as *const ())
			}
		}

		f.debug_list().entries(self.0.iter().map(|x| FnPtr(*x as usize))).finish()

	}
}