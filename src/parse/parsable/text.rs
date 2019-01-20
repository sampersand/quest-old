use crate::{Shared, Error, Object, IntoObject};
use crate::parse::{self, Parsable, Parser};
pub use crate::object::typed::Text;

named!(Text);

impl Parsable for Text {
	fn try_parse(parser: &Shared<Parser>) -> parse::Result<Object> {
		let parser_read = parser.read();
		let mut chars = parser_read.as_ref().chars();
		let mut count = 0;

		macro_rules! next {
			() => ({ count += 1; chars.next() })
		}

		let quote = match next!() {
			Some(quote @ '\'') | Some(quote @ '\"') => quote,
			_ => {
				trace!(target: "parser", "No text found. stream={:?}", parser_read.beginning());;
				return parse::Result::None
			}
		};

		debug_assert!(quote == '\'' || quote == '\"');

		let mut text = String::new();

		macro_rules! parse_err {
			($msg:expr) => ({
				warn!(target: "parser", concat!("Invalid text encountered (", $msg, ")"));
				parse::Result::Err(Box::new(Error::ParserError {
					msg: $msg,
					parser: { drop(chars); drop(parser_read); parser.clone() }
				}))
			})
		}

		loop {
			match next!() {
				None => return parse_err!("Unterminated string found"),
				Some(chr) if chr == quote => break,
				Some('\\') => match next!() {
					None => return parse_err!("Lonely `\\` found"),
					Some(chr @ '\"') | Some(chr @ '\'') | Some(chr @ '\\') => text.push(chr),
					Some('n') => text.push('\n'),
					Some('t') => text.push('\t'),
					Some('r') => text.push('\r'),
					Some('0') => text.push('\0'),
					Some('x') => match next!().and_then(|f| next!().map(|s| (f, s))) {
						None => return parse_err!("Unfinished `\\x` found"),
						Some((x, y)) => match (x.to_digit(16), y.to_digit(16)) {
							(Some(x), Some(y)) => text.push((x * 0x10 + y) as u8 as char),
							_ => return parse_err!("Invalid `\\x` escape code found")
						}
					},
					Some('u') => unimplemented!("TODO: `\\u`"),
					Some('U') => unimplemented!("TODO: `\\U`"),
					Some(other) => return parse_err!("Unknown escape code found")
				},
				Some(other) => text.push(other)
			}
		}

		drop(parser_read);

		let mut res = parser.write().advance(count - 1);

		debug_assert!(res.chars().next().unwrap() == quote, res);
		debug_assert!(res.chars().last().unwrap() == quote, res);
		debug_assert!(res.chars().count() >= 2, res);

		debug!(target: "parser", "Text parsed. chars={:?}", res);

		parse::Result::Ok(text.into_object())

		// if data.is_empty() || (data[0] != '\'' && data[0] != '\"') {
		// }

		// let quote = data[0].a;

		// let is_eof = ref_data.starts_with("__END__") || ref_data.starts_with("__EOF__");

		// drop(ref_data);
		// drop(data);

		// let number = Text::from_str(parser.read().as_ref());

		// if let Some((number, index)) = number {
		// 	let mut parser = parser.write();
		// 	let res = parser.advance(index);
		// 	debug_assert_eq!(number, Text::from_str(&res).unwrap().0);
		// 	debug!(target: "parser", "Text parsed. chars={:?}", res);
		// 	parse::Result::Ok(number.into_object())
		// } else {
		// 	trace!(target: "parser", "No number found. stream={:?}", parser.read().beginning());
		// 	parse::Result::None
		// }
	}
}