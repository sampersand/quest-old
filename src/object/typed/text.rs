use std::fmt::{self, Debug, Display, Formatter};
use lazy_static::lazy_static;
use crate::{Error, Result};

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct Text(String);

impl Text {
	pub fn new<T: Into<String>>(data: T) -> Text {
		Text(data.into())
	}
}

impl Display for Text {
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		Display::fmt(&self.0, f)
	}
}

impl Debug for Text {
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		write!(f, "Text({:?})", self.0)
	}
}



impl_typed_conversion!(Text, String);
impl_typed_object!(Text, new_text, downcast_text, is_text);
impl_quest_conversion!("@text" (as_text_obj is_text) (into_text downcast_text) -> Text);

fn exec_shell(cmd: String) -> Result<String> {
	use libc::{popen, pclose, fgets, fread, ferror, c_char, c_int, c_void};
	use std::ffi::{CString, CStr};
	use std::io::{self, ErrorKind};
	use std::mem::size_of;

	const CAPACITY: usize = 1024;
	const READ: *const c_char = b"r\0".as_ptr() as *const c_char;

	unsafe fn shutdown(cmd: *mut c_char, fp: Option<*mut libc::FILE>) -> Result<()> {
		CString::from_raw(cmd);
		if let Some(fp) = fp {
			if pclose(fp) == -1 {
				return Err(Error::IoError(io::Error::new(ErrorKind::Other, "pclose failed")));
			}
		}
		Ok(())
	}


	let cmd = CString::new(cmd)
		.map_err(|_| Error::IoError(io::Error::new(
			ErrorKind::InvalidInput,
			"command contained a null (\\0) byte"
		)))?
		.into_raw();

	let fp;

	unsafe {
		fp = popen(cmd, READ);

		if fp == (0 as _) {
			// if the allocation fails, then we dont get an error code.
			// im not sure how to fix that, oh well
			shutdown(cmd, None)?;
			return Err(Error::IoError(io::Error::last_os_error()));
		}
	}

	let mut result = String::new();
	let mut buf = Vec::<c_char>::with_capacity(CAPACITY);
	let ptr = buf.as_mut_ptr();

	unsafe {
		while fgets(ptr, CAPACITY as c_int - 1, fp) != (0 as _) {
			match CStr::from_ptr(ptr).to_str() {
				Ok(buf_str) => result.push_str(buf_str),
				Err(err) => {
					shutdown(cmd, Some(fp))?;
					return Err(Error::IoError(io::Error::new(ErrorKind::InvalidData, err)))
				}
			}
		}

		if ferror(fp) != 0 {
			shutdown(cmd, Some(fp))?;
			return Err(Error::IoError(io::Error::last_os_error()));
		}

		assert_ne!(libc::feof(fp), 0);
		shutdown(cmd, Some(fp))?
	};

	Ok(result)
}


impl_type! { for Text, downcast_fn=downcast_text;
	fn "@text" (this) {
		this.into_object()
	}

	fn "@var" (this) {
		super::Variable::from_string(this.0).into_object()
	}

	fn "@bool" (this) {
		(!this.0.is_empty()).into_object()
	}

	fn "@num" (this) {
		use crate::parse::{ParseFromStr, ParseOk};
		match super::Number::from_str(&this.0) {
			Ok(ParseOk::NotFound) => Object::new_null(),
			Ok(ParseOk::Found(num, _)) => num.into_object(),
			Err(err) => return Err(crate::Error::Boxed(Box::new(err)))
		}
	}

	fn "==" (this, rhs) {
		(this == rhs.into_text()?).into_object()
	}

	fn "()" (this) {
		exec_shell(this.0)?.into_object()
	}

	fn "eval" (_this) { todo!("this will be evaluate, possibly with new env"); }

	fn "+" (this, rhs) {
		let mut this = this;
		this.0.push_str(&rhs.into_text()?.0);
		this.into_object()
	}

	fn "*" (this, rhs) {
		let lim = *rhs.into_num()?.into_integer().as_ref() as isize;
		if lim < 0 {
			return Ok("".to_string().into_object());
		}

		let mut new = String::with_capacity(this.0.len() * (lim as usize));
		for _ in 0..lim {
			new.push_str(&this.0);
		}

		new.into_object()
	}

	fn "len" (this) {
		this.0.len().into_object()
	}

	fn "get" (_this, _index) { todo!() }
	fn "set" (_this, _index, _val) { todo!() }
	fn "has" (_this, _index) { todo!() }
	fn "del" (_this, _index) { todo!() }
}

#[cfg(test)]
mod rust_tests {
	use super::Text;
	use crate::object::IntoObject;

	#[test]
	fn to_string_conversion_works(){
		assert_eq!(String::from("where is the sun"), "where is the sun".to_string());
	}

	#[test]
	fn empty_string() {
		assert_eq!(String::from(Text::new("")), "".to_string());
	}

	#[test]
	fn preserves_contents() {
		assert_eq!(String::from(Text::new("my contents")), "my contents".to_string());
	}

	#[test]
	fn create_string_object() {
		assert_eq!("hi friend".to_string().into_object().downcast_text().unwrap(), Text::new("hi friend"));
	}

	#[test]
	fn is_text() {
		assert!("lol".to_string().into_object().is_text());
	}

	#[test]
	fn object_eql_works() {
		assert_eq!("hi there".to_string().into_object(), "hi there".to_string().into_object());
	}
}


#[cfg(test)]
#[allow(unused)]
mod quest_tests {
	use super::Text;
	use crate::object::{IntoObject};

	macro_rules! text {
		($x:expr) => ( Text::new($x) );
	}


	#[test]
	fn at_text() {
		// assert_eq!(make!(text "fooey").into_text().unwrap(), text!("fooey"));
	}

	#[test]
	fn at_var() {
		// assert_eq!(tobj!("fooey").into_var().unwrap(), text!("fooey"));
	}


// impl_type! { for Text, downcast_fn=downcast_text;
// 	fn "@text" (this) {
// 		this.into_object()
// 	}

// 	fn "@var" (this) {
// 		super::var::Variable::from_string(this.0).into_object()
// 	}

// 	fn "@bool" (this) {
// 		(!this.0.is_empty()).into_object()
// 	}

// 	fn "@num" (_this) { todo!() }

// 	fn "==" (this, rhs) {
// 		(this == rhs.into_text()?).into_object()
// 	}

// 	fn "()" (_this) { todo!("this will be a shell command"); }
// 	fn "eval" (_this) { todo!("this will be evaluate, possibly with new env"); }

// 	fn "+" (this, rhs) {
// 		let mut this = this;
// 		this.0.push_str(&rhs.into_text()?.0);
// 		this.into_object()
// 	}

// 	fn "*" (this, rhs) {
// 		let lim = *rhs.into_num()?.into_integer().as_ref() as isize;
// 		if lim < 0 {
// 			return Ok("".to_string().into_object());
// 		}

// 		let mut new = String::with_capacity(this.0.len() * (lim as usize));
// 		for _ in 0..lim {
// 			new.push_str(&this.0);
// 		}

// 		new.into_object()
// 	}

// 	fn "len" (this) {
// 		this.0.len().into_object()
// 	}

// 	fn "get" (_this, _index) { todo!() }
// 	fn "set" (_this, _index, _val) { todo!() }
// 	fn "has" (_this, _index) { todo!() }
// 	fn "del" (_this, _index) { todo!() }
// }
}






















