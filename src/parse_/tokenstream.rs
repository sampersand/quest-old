use parse::{Tokenizer, Stream, Token};

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
		let token = self.tokens.iter()
			.filter_map(|tokenizer| tokenizer.parse(self.stream))
			.rev()
			.max_by(|x, y| x.len().cmp(&y.len()))
			.unwrap_or_else(|| panic!("No valid tokens found at:\n===START===\n{}\n====END====\n", self.stream.as_str()));
		self.stream.advance_by(&token);
		Some(token)
	}
}
