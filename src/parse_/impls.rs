use obj::classes::{QuestClass, GetDefault, HasDefault};
use parse::{Stream, Parsable};
use shared::RawShared;

#[derive(Debug)]
pub struct Nothing;

impl QuestClass for Nothing {
	const GET_DEFAULTS: GetDefault = |_, _| unreachable!("`Nothing` became a quest class?");
	const HAS_DEFAULTS: HasDefault = |_| unreachable!("`Nothing` became a QuestClass?");
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub struct Whitespace;

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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LParen { Round, Square, Curl }

impl Parsable for LParen {
	type Value = Nothing;

	fn try_parse(stream: &mut Stream) -> Option<Nothing> {
		stream.try_get(regex!(r"\A[})\]]")).and(Some(Nothing))
	}
}

