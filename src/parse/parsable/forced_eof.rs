use crate::{Shared, Result};
use crate::parse::{Parsable, ParseResult, Parser};

pub(super) struct ForcedEof; 

impl Parsable for ForcedEof {
	const NAME: &'static str = "ForcedEof";
	fn try_parse(parser: &Shared<Parser>) -> ParseResult {
		// let mut is_eof;
		let data = parser.read();
		let ref_data = data.as_ref();
		
		let is_eof = ref_data.starts_with("__END__") || ref_data.starts_with("__EOF__");

		drop(ref_data);
		drop(data);

		if !is_eof {
			trace!(target: "parser", "No forcedeof parsed for {:?}", parser.read().beginning());
			ParseResult::None
		} else {
			// let mut parser = parser.write();
			// parser.advance(parser.as_ref().len());
			// debug_assert!(parser.as_ref().is_empty(), "forced eof, but length wasn't 0 at the end?");

			debug!(target: "parser", "Forced eof found for {:?}", parser.read().beginning());
			ParseResult::Eof
		}
	}
}