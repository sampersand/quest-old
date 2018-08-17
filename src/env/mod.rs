mod binding;
mod stream;
pub mod parse;

use self::stream::Stream;
pub use self::binding::Binding;


#[derive(Debug)]
pub struct Environment<'a> {
	pub stream: Stream<'a>,
	pub binding: Binding
}
