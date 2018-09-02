use parse::{Parsable, Stream, ParseResult};
use env::Environment;
use obj::AnyShared;

pub struct Oper;

impl Parsable for Oper {
	fn parse(stream: &mut Stream, env: &Environment) -> ParseResult {
		None
	}
}
