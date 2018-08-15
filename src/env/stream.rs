use env::Parsable;

use std::path::Path;
use std::sync::Arc;
use std::str::pattern::Pattern;

type Parser = Arc<dyn Parsable>;

#[derive(Debug, Clone)]
pub struct Stream<'a> {
	parsers: Vec<Parser>,
	data: &'a str,
	file: Option<&'a Path>,
	line: usize
}

fn default_parsers() -> Vec<Parser> {
	use env::parse;
	vec![
		Arc::new(parse::Whitespace) as _,
		Arc::new(parse::Comment) as _,
	]
}

impl<'a> Stream<'a> {
	pub(super) fn new(data: &'a str, file: Option<&'a Path>, parsers: Option<Vec<Parser>>) -> Self {
		Stream { data, file, parsers: parsers.unwrap_or_else(|| default_parsers()), line: 0 }
	}

	pub(super) fn parsers(&self) -> impl Iterator<Item=&Parser> {
		self.parsers.iter()
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