use std::error::Error;
use crate::Object;
use std::ops::Try;

#[derive(Debug)]
pub enum Result<T=Object> {
	Restart, // for things like whitespace and comments
	Ok(T),
	Err(Box<dyn Error>),
	Eof, // for things like __END__
	None
}

// impl Result<T> {
// 	pub fn try_from<T>(opt: Option<std::result::Result<T, Error>>) -> std::result::Result<T, Self> {
// 		match opt {
// 			None => Err(Result::None),
// 			Some(Ok(obj)) => Ok(obj),
// 			Some(Err(err)) => Err(Result::Err(err)),
// 		}
// 	}
// }


// impl Try for Result<T> {
// 	type Ok
// }