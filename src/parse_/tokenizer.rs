use regex::Regex;
use std::fmt::Debug;
use parse::{Stream, Token};

pub trait Tokenizer : Debug + 'static {
	fn parse<'a>(&self, stream: &Stream<'a>) -> Option<Token<'a>>;
}

macro_rules! tokenizers {
	(use ident $data:ident; $($name:ident($regex:expr) $body:block)*) => {
		pub const DEFAULT: &'static [&'static dyn Tokenizer] = &[$(&$name as _),*];

		$(
			#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
			pub struct $name;
			impl Tokenizer for $name {
				fn parse<'a>(&self, stream: &Stream<'a>) -> Option<Token<'a>> {
					static_regex!(REGEX = concat!("\\A(?:", $regex, ")"));
					if let Some(m) = REGEX.find(stream.as_str()) {
						if m.start() == 0 {
							use parse::token::{Token::*, Literal::*, Paren::*};
							let $data = m.as_str();
							return Some($body);
						}
					}
					None
				}
			}
		)*
	}
}

tokenizers!{ use ident data;
	Number(r"(?i)(0(x[a-f\d]+|o[0-7]+|b[01]+|d\d+))|[-+]?\d+(\.\d+)?(e[-+]?\d+)?") { Literal(Number, data) }
	Text(r#"("(\\.|[^"])*")|('(\\.|[^'])*')|(`(\\.|[^`])*`)"#) { Literal(Text, data) }
	Variable(r#"\$\W|(\$\w|[a-zA-Z_])\w*[!?]?\b"#) { Literal(Variable, data) }
	Whitespace(r"\s+"){ Whitespace(data) }
	Parens(r"[\[\](){}]") { match data {
		"(" => OpenParen(Round),
		"[" => OpenParen(Square),
		"{" => OpenParen(Curly),
		")" => CloseParen(Round),
		"]" => CloseParen(Square),
		"}" => CloseParen(Curly),
		other => unreachable!("Unrecognized paren encountered: `{}`", other)
	}}
	Operators(r"(?x)
		[!?~]|not\b|                        # Unary operators
		([-+*/^%]|\*\*)=?|(mod|pow)(=|\b)|  # Math operators (and their assign equivalents)
		\+\+|\-\-|                          # Increment and Decrement
		(and|or)\b|                         # Logical And and OR
		[<>]=?|==|!=|<=>|                   # Comparison operators
		=|<-|->|										# Assignment operatokrs
		(bitand|bitor|bitxor|)(=|\b)|       # Bitwise `bitand`, `bitor`, `bitxor`
		<<=?|>>=?|                          # Shift left and right
		[;,.]|                              # Deliminators
	") {
		match data {
			
		}
	}
}


















