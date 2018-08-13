use obj::classes::{QuestClass, DefaultAttrs};
use obj::{Id, classes};

use parse::{Stream, Parsable};

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub struct Whitespace;

#[derive(Debug)]
pub struct Nothing;

impl QuestClass for Nothing {
	fn default_attrs() -> &'static DefaultAttrs<Self> { unreachable!("Cannot convert Nothing to a QuestClass"); }
}

impl Parsable for Whitespace {
	type Value = Nothing;

	fn try_parse(stream: &mut Stream) -> Option<Nothing> {
		stream.try_get(regex!(r"\A\s+")).and(Some(Nothing))
	}
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub struct Comment;

impl Parsable for Comment {
	type Value = Nothing;

	fn try_parse(stream: &mut Stream) -> Option<Nothing> {
		stream.try_get(regex!(r"(?m)\A(//|#).*$")).and(Some(Nothing))

	}
}


