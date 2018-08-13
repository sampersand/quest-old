use std::fmt::Debug;
use parse::stream::Stream;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Paren { Curly, Round, Square }

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Token<'a> {
	Whitespace, // any whitespace that's encountered; `comment` will also return whitespace
	Eof, // stop parsing (eg __EOF__)

	Block(Paren, Vec<Token<'a>>),
	Literal(&'a str), // this is anythign that's not one of the above--variables, numbers, text, and operators
}

pub trait Tokenizer : Debug + 'static {
	fn next_token<'a>(&self, stream: &mut Stream<'a>) -> Option<Token<'a>>;
}

#[derive(Debug)]
pub struct TokenStream<'a, 'b> {
	tokens: &'a [&'a dyn Tokenizer],
	stream: &'b mut Stream<'b>
}

impl<'a, 'b> TokenStream<'a, 'b> {
	pub fn new(tokens: &'a [&'a dyn Tokenizer], stream: &'b mut Stream<'b>) -> TokenStream<'a, 'b> {
		TokenStream { tokens, stream }
	}
}

impl<'a, 'b> Iterator for TokenStream<'a, 'b> {
	type Item = Token<'b>;
	fn next(&mut self) -> Option<Token<'b>> {
		self.tokens.iter().find_map(|tokenizer| tokenizer.next_token(self.stream))
	}
}
