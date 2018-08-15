mod binding;
mod stream;
mod stack;
mod parse;

pub use self::binding::Binding;

use self::stack::Stack;
use self::stream::Stream;

#[derive(Debug, Clone)]
struct Environment<'stream, 'bind: 'stream> {
	binding: Binding<'bind>,
	stack: Stack,
	stream: Stream<'stream>,
}

use obj::AnyObject;
use std::fmt::Debug;

#[derive(Debug, Clone)]
enum Token {
	Object(AnyObject),
	NoObject,
	Eof
}

trait Parsable : Debug {
	fn try_parse(&self, env: &mut Environment) -> Option<Token>;
}

impl<'stream, 'bind: 'stream> Environment<'stream, 'bind> {
	fn parse(mut self) -> AnyObject { // we consume because `Stream` should be consumed by the end
		debug_assert!(self.stack.is_empty(), "didn't start with an empty stack?");
		'outer: while !self.stream.as_str().is_empty() {
			let stream = self.stream.clone(); // we have to clone in case the parser modifies the parsers whilst the stream is being run
			for parser in stream.parsers() {
				match parser.try_parse(&mut self) {
					Some(Token::Object(obj)) => {
						self.stack.handle(obj);
						continue 'outer; // we've found an object, retry thru parsers
					}
					Some(Token::NoObject) => continue 'outer, // we found whitespace, so just restart iter search
					Some(Token::Eof) => break 'outer, // we've hit EOF, so prematurely end the while loop
					None => continue // if we cant find anything, look for the next token
				}

				// if we reach here, we've exhausted all parsers, and none of them worked
				panic!("No tokens found for {:#?}", self.stream); 
			}

			// try_parse!(QNull QBool QText QNum QOper QVar);
			panic!("No objects could match the stream: {}", self.stream.as_str());
		}

		self.stack.finish()
	}
}

