use std::fmt::{self, Debug, Display, Formatter};
use std::ops::Range;

pub struct PtrFormatter(pub usize);

impl Debug for PtrFormatter {
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		write!(f, "{:p}", self.0 as *const ())
	}
}

impl Display for PtrFormatter {
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		write!(f, "{:p}", self.0 as *const ())
	}
}


pub enum IndexError {
	ZeroPassed,
	StartTooBig,
	StartBiggerThanEnd
}

pub fn get_index(mut start: isize, end: Option<isize>, len: usize) -> Result<Range<usize>, IndexError> {
	if start == 0 || end == Some(0) {
		return Err(IndexError::ZeroPassed);
	}

	if start.is_positive() {
		// since indexing starts at 1 in quest, we need to bring it down to 0 to work with rust
		// we don't subtract for negative numbers because they already are one below
		start -= 1;
	}

	// note that we don't decrement the end; this is because "1..2" should give the first two elements,
	// and is transformed into "0..2" and therefore works

	if start.is_negative() {
		if len < start.abs() as usize { // our starting point is too large
			return Err(IndexError::StartTooBig);
		} else {
			start += len as isize; // make it wrap around
		}
	}

	let start = start as usize; // remove the mutability; start is also now guaranteed to be positive or zero
	let mut end = end.unwrap_or(start as isize + 1);

	if end.is_negative() {
		if len < end.abs() as usize { // our ending point is too large
			return Ok(0..start);
		} else {
			end += len as isize + 1; // we add one for the same reason why we don't subtract one when `end` is positive
		}
	}

	let end = end as usize; // remove the mutability; end is also now guaranteed to be positive or zero

	if end < start { // this is invalid, so return null
		Err(IndexError::StartBiggerThanEnd)
	} else {
		Ok(start..end)
	}
}
