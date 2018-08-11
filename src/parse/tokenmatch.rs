use parse::stream::Source;
use parse::{Tree, Token};
use env::Environment;
use obj::QObject;
use std::cmp::Ordering;


#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum MatchData {
	Text(String, usize),
	Block(Vec<TokenMatch>, usize),
	NoToken(usize), // token len
	Eof(usize), // token length
}

use self::MatchData::*;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TokenMatch {
	pub(super) data: MatchData,
	pub(super) token: &'static Token,
	pub src: Source
}

impl TokenMatch {
	pub fn new(data: MatchData, token: &'static Token, src: Source) -> TokenMatch {
		assert!(data.len() != 0 || data.is_eof(), "Cannot have an empty TokenMatch");
		TokenMatch { data, token, src }
	}

	#[inline]
	pub fn try_as_str(&self) -> Option<&str> {
		self.data.try_as_str()
	}
}

impl MatchData {
	pub fn len(&self) -> usize {
		match self {
			MatchData::Text(_, len) => *len,
			MatchData::Block(_, len) => *len,
			MatchData::NoToken(len) => *len,
			MatchData::Eof(_) => 0
		}
	}

	fn cmp_len(&self) -> usize {
		match self {
			MatchData::Text(_, len) => *len,
			MatchData::Block(_, len) => *len,
			MatchData::NoToken(len) => *len,
			MatchData::Eof(len) => *len
		}
	}

	pub fn try_as_block(&self) -> Option<&[TokenMatch]> {
		if let MatchData::Block(v, _) = self {
			Some(v.as_slice())
		} else {
			None
		}
	}

	pub fn try_as_str(&self) -> Option<&str> {
		if let MatchData::Text(text, _) = self {
			Some(text)
		} else {
			None
		}
	}

	pub fn is_eof(&self) -> bool {
		if let MatchData::Eof(_) = self {
			true
		} else {
			false
		}
	}

	pub fn is_notoken(&self) -> bool {
		if let MatchData::NoToken(_) = self {
			true
		} else {
			false
		}
	}
}

impl PartialOrd for MatchData {
	fn partial_cmp(&self, rhs: &MatchData) -> Option<Ordering> {
		self.cmp_len().partial_cmp(&rhs.cmp_len())
	}	
}

impl Ord for MatchData {
	fn cmp(&self, other: &MatchData) -> Ordering {
		self.cmp_len().cmp(&other.cmp_len())
	}
}


