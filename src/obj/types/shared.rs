use obj::types::{Number, Sign};
use obj::{Result, Error};
use std::ops::{Index, Range};

pub enum Offset {
	Valid(usize),
	OutOfBounds(usize), // how much over it is
	Underflow(usize) // how much under it is
}

pub fn offset(len: usize, pos: Number) -> Result<Offset> {
	let sign = pos.sign();
	let pos = pos.to_integer().expect("pos wasnt an integer <TODO: Make this an error>").abs() as usize;

	Ok(match sign {
		Sign::Positive if pos == 0 => Offset::Valid(0),
		Sign::Negative if pos == 0 => Offset::Valid(len - 1),
		Sign::Positive if pos < len => Offset::Valid(pos),
		Sign::Negative if pos < len => Offset::Valid(len - pos),
		Sign::Positive => Offset::OutOfBounds(pos - len),
		Sign::Negative => Offset::Underflow(pos - len)
	})
}