use crate::Shared;
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

		if is_eof {
			debug!(target: "parser", "Forced eof found. chars={:?}", parser.read().beginning());
			ParseResult::Eof
		} else {
			trace!(target: "parser", "No forced eof found. stream={:?}", parser.read().beginning());
			ParseResult::None
		}
	}
}