use crate::{Shared, Object};
use crate::parse::{self, Parsable, Parser};
use crate::parse::parsable::Named;

pub(super) struct ForcedEof; 

named!(ForcedEof);

impl Parsable for ForcedEof {
	fn try_parse(parser: &Shared<Parser>) -> parse::Result<Object> {
		// let mut is_eof;
		let data = parser.read();
		let ref_data = data.as_ref();
		
		let is_eof = ref_data.starts_with("__END__") || ref_data.starts_with("__EOF__");

		drop(ref_data);
		drop(data);

		if is_eof {
			debug!(target: "parser", "Forced eof parsed. chars={:?}", parser.read().beginning());
			parse::Result::Eof
		} else {
			trace!(target: "parser", "No forced eof found. stream={:?}", parser.read().beginning());
			parse::Result::None
		}
	}
}