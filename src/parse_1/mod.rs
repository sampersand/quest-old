mod stream;
mod token;
mod tokenizers;

use self::stream::Stream;
use self::token::{Token, Tokenizer, TokenStream};


use std::{fs, io};
pub fn parse_str() -> String {
	unimplemented!()
}

pub fn parse_file(file: &str, env: &::env::Environment) -> ::std::io::Result<()> {
	let data = fs::read_to_string(file)?;
	let mut stream = Stream::from_file(file, &data);
	let mut tokens = TokenStream::new(tokenizer::DEFAULT, &mut stream);
	
	println!("{:?}", tokens.collect::<Vec<_>>());
	Ok(())
}