use std::fmt::{self, Debug, Display, Formatter};
use std::ops;

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

macro_rules! const_concat {
	($($arg:expr),*) => {{
		::lazy_static::lazy_static!{
			static ref VAL: &'static str = {
				let mut x = String::new();
				$(x.push_str(&$arg);)*
				Box::leak(x.into_boxed_str())
			};
		}
		&*VAL
	}}
}

#[cfg(test)]
macro_rules! matches {
	($a:expr, $b:pat) => (if let $b = $a { true } else { false })
}

#[cfg(test)]
macro_rules! assert_matches {
	($lhs:expr, $rhs:pat) => ({
		let lhs = $lhs;
		assert!(matches!(lhs, $rhs), "{:?} doesn't match the pattern", $lhs);
	})
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum IndexError {
	// zero isn't allowed because indexing starts from 1, and negative values start from -1.
	// thus zero is undefined
	ZeroPassed,
	StartOutOfBounds,
	StartBiggerThanEnd,
}

pub fn get_index(start: isize, end: Option<isize>, len: usize) -> Result<ops::Range<usize>, IndexError> {
	enum OffsetError {
		ZeroPassed, OutOfBounds
	}

	fn offset(pos: isize, len: usize, decrement: bool) -> Result<usize, OffsetError> {
		use std::cmp::Ordering;

		let abs_pos = pos.checked_abs().map(|x| x as usize).unwrap_or(!(!std::isize::MIN as usize));

		if len < abs_pos {
			if pos == 0 {
				return Err(OffsetError::ZeroPassed);
			}
			return Err(OffsetError::OutOfBounds)
		}

		match pos.cmp(&0) {
			// indexing starts at 1 in quest, and thus 0 is not allowed.
			Ordering::Equal => Err(OffsetError::ZeroPassed),
			// Since Rust starts indexing at 0, we need to subtract one from numbers to get there
			// However, negative numbers already start at one less than the end, so there's no need to
			// subtract from them.
			Ordering::Greater if decrement => {
				debug_assert!(len >= abs_pos);
				debug_assert!(0 <= pos - 1);
				Ok(abs_pos - 1)
			},
			Ordering::Greater => Ok(abs_pos),

			Ordering::Less => {
				debug_assert!(len >= abs_pos);
				Ok(len - abs_pos)
			}
		}
	}

	let start = match offset(start, len, true) {
		Ok(start) => start,
		Err(OffsetError::ZeroPassed) => return Err(IndexError::ZeroPassed),
		Err(OffsetError::OutOfBounds) => return Err(IndexError::StartOutOfBounds),
	};

	let end = if let Some(end) = end {
		match offset(end, len, false) {
			Ok(end) => end,
			Err(OffsetError::ZeroPassed) => return Err(IndexError::ZeroPassed),
			Err(OffsetError::OutOfBounds) => len // if end is out of bounds, we upto the end
		}
	} else {
		start
	};

	if start > end {
		Err(IndexError::StartBiggerThanEnd)
	} else {
		Ok(start..end)
	}
}

#[cfg(test)]
mod tests {
	use super::{IndexError, get_index};

	const U_MAX: usize = std::usize::MAX;
	const I_MAX: isize = std::isize::MAX;
	const I_MIN: isize = std::isize::MIN;

	// TODO: START OUT OF BOUNDS

	#[test]
	fn get_index_test() {
		fn abs(inp: isize) -> usize {
			if inp < 0 {
				if inp == I_MIN { 
					!(!I_MIN as usize)
				} else {
					inp.abs() as usize
				}
			} else {
				inp as usize
			}
		}

		for start in &[0, 1, -1, 2, -2, I_MAX, I_MAX - 1, I_MIN, I_MIN + 1] {
			let start_abs = abs(*start);
			for end in &[None, Some(0), Some(1), Some(-1), Some(2), Some(-2), Some(I_MAX), Some(I_MAX - 1), Some(I_MIN), Some(I_MIN + 1)] {

				for len in &[0, 1, 2, I_MAX as usize, I_MAX as usize - 1, I_MAX as usize + 1, U_MAX, U_MAX - 1] {
					let (start, end, len) = (*start, *end, *len);
					let (o_start, o_end, o_len) = (start, end, len);

					let res = get_index(start, end, len);

					if start == 0 {
						assert_eq!(res, Err(IndexError::ZeroPassed), "get_index({}, {:?}, {})", o_start, o_end, o_len);
						continue;
					} else if len < start_abs {
						assert_eq!(res, Err(IndexError::StartOutOfBounds), "get_index({}, {:?}, {})", o_start, o_end, o_len);
						continue;
					}

					let start = if start < 0 {
						(len - start_abs) as usize
					} else {
						(start - 1) as usize
					};

					if end == Some(0) {
						assert_eq!(res, Err(IndexError::ZeroPassed), "get_index({}, {:?}, {})", o_start, o_end, o_len);
						continue;
					}

					let end = if let Some(end) = end {
						if end < 0 {
							let abs_end = abs(end);
							if abs_end > len {
								len
							} else {
								(len - abs_end) as usize
							}
						} else if end as usize > len {
							len 
						} else {
							(end) as usize
						}
					} else {
						start
					};

					if start > end {
						assert_eq!(res, Err(IndexError::StartBiggerThanEnd), "get_index({}, {:?}, {})", o_start, o_end, o_len);
					} else {
						assert_eq!(res, Ok(start..end), "get_index({}, {:?}, {})", o_start, o_end, o_len);
					}
				}
			}
		}
	}

	// Zero isn't allowed in indexing, so it'll raise an error
	// #[test]
	// fn get_index_no_end_zero() {
	// 	/* Negative values should be `len + start ..len + start + 1` */
	// 	assert_eq!(0, -0); // just to make sure
	// 	assert_eq!(get_index(0, None, 0), Err(ZeroPassed));
	// 	assert_eq!(get_index(0, None, 1), Err(ZeroPassed));
	// 	assert_eq!(get_index(0, None, 129), Err(ZeroPassed));
	// 	assert_eq!(get_index(0, None, 129), Err(ZeroPassed));
	// 	assert_eq!(get_index(0, None, U_MAX), Err(ZeroPassed));
	// }

	// /* Positive values should be `start..start + 1` */
	// #[test]
	// fn get_index_no_end_positive() {
	// 	assert_eq!(get_index(1, None, 1), Ok(0..=0));
	// 	assert_eq!(get_index(1, None, 2), Ok(0..=0));
	// 	assert_eq!(get_index(1, None, 3), Ok(0..=0));
	// 	assert_eq!(get_index(1, None, 12394), Ok(0..=0));
	// 	assert_eq!(get_index(1, None, U_MAX), Ok(0..=0));
	// 	assert_eq!(get_index(1, None, 0), Err(StartOutOfBounds));

	// 	assert_eq!(get_index(2, None, 2), Ok(1..=1));
	// 	assert_eq!(get_index(2, None, 3), Ok(1..=1));
	// 	assert_eq!(get_index(2, None, 123981), Ok(1..=1));
	// 	assert_eq!(get_index(2, None, U_MAX), Ok(1..=1));
	// 	assert_eq!(get_index(2, None, 1), Err(StartOutOfBounds));
	// 	assert_eq!(get_index(2, None, 0), Err(StartOutOfBounds));

	// 	assert_eq!(get_index(9, None, 9), Ok(8..=8));
	// 	assert_eq!(get_index(9, None, 12), Ok(8..=8));
	// 	assert_eq!(get_index(9, None, U_MAX), Ok(8..=8));
	// 	assert_eq!(get_index(9, None, 8), Err(StartOutOfBounds));

	// 	assert_eq!(get_index(100_001, None, U_MAX), Ok(100_000..=100_000));
	// 	assert_eq!(get_index(100_001, None, 100_002), Ok(100_000..=100_000));
	// 	assert_eq!(get_index(100_001, None, 100_001), Ok(100_000..=100_000));
	// 	assert_eq!(get_index(100_001, None, 100_000), Err(StartOutOfBounds));
	// 	assert_eq!(get_index(100_000, None, 12_391), Err(StartOutOfBounds));
	// 	assert_eq!(get_index(100_000, None, 0), Err(StartOutOfBounds));

	// 	assert_eq!(get_index(I_MAX, None, I_MAX_U-1), Err(StartOutOfBounds));
	// 	assert_eq!(get_index(I_MAX, None, I_MAX_U), Ok(I_MAX_U-1..=I_MAX_U-1));
	// 	assert_eq!(get_index(I_MAX, None, U_MAX), Ok(I_MAX_U-1..=I_MAX_U-1));
	// 	assert_eq!(get_index(I_MAX, None, 0), Err(StartOutOfBounds));
	// }

	// #[test]
	// fn get_index_len_0() {
	// 	macro_rules! assert_start_out_of_bounds {
	// 		($($start:expr),*) => {
	// 			$(
	// 				assert_eq!(get_index($start, None, 0), Err(StartOutOfBounds));
	// 				assert_eq!(get_index($start, Some(1), 0), Err(StartOutOfBounds));
	// 				assert_eq!(get_index($start, Some(2), 0), Err(StartOutOfBounds));
	// 				assert_eq!(get_index($start, Some(-1), 0), Err(StartOutOfBounds));
	// 				assert_eq!(get_index($start, Some(-2), 0), Err(StartOutOfBounds));
	// 				assert_eq!(get_index($start, Some(I_MAX), 0), Err(StartOutOfBounds));
	// 				assert_eq!(get_index($start, Some(I_MAX-1), 0), Err(StartOutOfBounds));
	// 				assert_eq!(get_index($start, Some(I_MIN), 0), Err(StartOutOfBounds));
	// 				assert_eq!(get_index($start, Some(I_MIN+1), 0), Err(StartOutOfBounds));
	// 			)*
	// 		}
	// 	}

	// 	assert_start_out_of_bounds!(1, 2, -1, -2, I_MAX, I_MAX-1, I_MIN, I_MIN+1);
	// }

	// #[test]
	// fn get_index_len_1() {
	// 	assert_eq!(get_index(1, None, 1), Ok(0..=0));
	// 	assert_eq!(get_index(2, None, 1), Err(StartOutOfBounds));
	// 	assert_eq!(get_index(-1, None, 1), Ok(0..=0));
	// 	assert_eq!(get_index(-2, None, 1), Err(StartOutOfBounds));
	// 	assert_eq!(get_index(I_MAX, None, 1), Err(StartOutOfBounds));
	// 	assert_eq!(get_index(I_MAX-1, None, 1), Err(StartOutOfBounds));
	// 	assert_eq!(get_index(I_MIN, None, 1), Err(StartOutOfBounds));
	// 	assert_eq!(get_index(I_MIN+1, None, 1), Err(StartOutOfBounds));

	// 	assert_eq!(get_index(1, Some(1), 1), Ok(0..=0));
	// 	assert_eq!(get_index(2, Some(1), 1), Err(StartOutOfBounds));
	// 	assert_eq!(get_index(-1, Some(1), 1), Ok(0..=0));
	// 	assert_eq!(get_index(-2, Some(1), 1), Err(StartOutOfBounds));
	// 	assert_eq!(get_index(I_MAX, Some(1), 1), Err(StartOutOfBounds));
	// 	assert_eq!(get_index(I_MAX-1, Some(1), 1), Err(StartOutOfBounds));
	// 	assert_eq!(get_index(I_MIN, Some(1), 1), Err(StartOutOfBounds));
	// 	assert_eq!(get_index(I_MIN+1, Some(1), 1), Err(StartOutOfBounds));

	// 	assert_eq!(get_index(1, Some(2), 1), Ok(0..=0));
	// 	assert_eq!(get_index(2, Some(2), 1), Err(StartOutOfBounds));
	// 	assert_eq!(get_index(-1, Some(2), 1), Ok(0..=0));
	// 	assert_eq!(get_index(-2, Some(2), 1), Err(StartOutOfBounds));
	// 	assert_eq!(get_index(I_MAX, Some(2), 1), Err(StartOutOfBounds));
	// 	assert_eq!(get_index(I_MAX-1, Some(2), 1), Err(StartOutOfBounds));
	// 	assert_eq!(get_index(I_MIN, Some(2), 1), Err(StartOutOfBounds));
	// 	assert_eq!(get_index(I_MIN+1, Some(2), 1), Err(StartOutOfBounds));

	// 	assert_eq!(get_index(1, Some(-1), 1), Ok(0..=0));
	// 	assert_eq!(get_index(2, Some(-1), 1), Err(StartOutOfBounds));
	// 	assert_eq!(get_index(-1, Some(-1), 1), Ok(0..=0));
	// 	assert_eq!(get_index(-2, Some(-1), 1), Err(StartOutOfBounds));
	// 	assert_eq!(get_index(I_MAX, Some(-1), 1), Err(StartOutOfBounds));
	// 	assert_eq!(get_index(I_MAX-1, Some(-1), 1), Err(StartOutOfBounds));
	// 	assert_eq!(get_index(I_MIN, Some(-1), 1), Err(StartOutOfBounds));
	// 	assert_eq!(get_index(I_MIN+1, Some(-1), 1), Err(StartOutOfBounds));

	// 	assert_eq!(get_index(1, Some(-2), 1), Ok(0..=0));
	// 	assert_eq!(get_index(2, Some(-2), 1), Err(StartOutOfBounds));
	// 	assert_eq!(get_index(-1, Some(-2), 1), Ok(0..=0));
	// 	assert_eq!(get_index(-2, Some(-2), 1), Err(StartOutOfBounds));
	// 	assert_eq!(get_index(I_MAX, Some(-2), 1), Err(StartOutOfBounds));
	// 	assert_eq!(get_index(I_MAX-1, Some(-2), 1), Err(StartOutOfBounds));
	// 	assert_eq!(get_index(I_MIN, Some(-2), 1), Err(StartOutOfBounds));
	// 	assert_eq!(get_index(I_MIN+1, Some(-2), 1), Err(StartOutOfBounds));

	// 	assert_eq!(get_index(1, Some(I_MAX), 1), Ok(0..=0));
	// 	assert_eq!(get_index(2, Some(I_MAX), 1), Err(StartOutOfBounds));
	// 	assert_eq!(get_index(-1, Some(I_MAX), 1), Ok(0..=0));
	// 	assert_eq!(get_index(-2, Some(I_MAX), 1), Err(StartOutOfBounds));
	// 	assert_eq!(get_index(I_MAX, Some(I_MAX), 1), Err(StartOutOfBounds));
	// 	assert_eq!(get_index(I_MAX-1, Some(I_MAX), 1), Err(StartOutOfBounds));
	// 	assert_eq!(get_index(I_MIN, Some(I_MAX), 1), Err(StartOutOfBounds));
	// 	assert_eq!(get_index(I_MIN+1, Some(I_MAX), 1), Err(StartOutOfBounds));

	// 	assert_eq!(get_index(1, Some(I_MAX-1), 1), Ok(0..=0));
	// 	assert_eq!(get_index(2, Some(I_MAX-1), 1), Err(StartOutOfBounds));
	// 	assert_eq!(get_index(-1, Some(I_MAX-1), 1), Ok(0..=0));
	// 	assert_eq!(get_index(-2, Some(I_MAX-1), 1), Err(StartOutOfBounds));
	// 	assert_eq!(get_index(I_MAX, Some(I_MAX-1), 1), Err(StartOutOfBounds));
	// 	assert_eq!(get_index(I_MAX-1, Some(I_MAX-1), 1), Err(StartOutOfBounds));
	// 	assert_eq!(get_index(I_MIN, Some(I_MAX-1), 1), Err(StartOutOfBounds));
	// 	assert_eq!(get_index(I_MIN+1, Some(I_MAX-1), 1), Err(StartOutOfBounds));


	// }
	// /*
	// 	// len=2
	// 	assert_eq!(get_index(1, None, 1), Ok(0..=0));
	// 	assert_eq!(get_index(2, None, 1), Ok(1..=1));
	// 	assert_eq!(get_index(-1, None, 1), Ok(0..=0));
	// 	assert_eq!(get_index(-2, None, 1), Err(StartOutOfBounds);
	// 	assert_eq!(get_index(I_MAX, None, 1), Err(StartOutOfBounds));
	// 	assert_eq!(get_index(I_MAX-1, None, 1), Err(StartOutOfBounds));
	// 	assert_eq!(get_index(I_MIN, None, 1), Err(StartOutOfBounds));
	// 	assert_eq!(get_index(I_MIN-1, None, 1), Err(StartOutOfBounds));


	// 	assert_eq!(get_index(1, None, U_MAX), Ok(0..=0));
	// 	assert_eq!(get_index(2, None, U_MAX), Ok(1..=1));
	// 	assert_eq!(get_index(-1, None, U_MAX), Ok(U_MAX-1..=U_MAX-1));
	// 	assert_eq!(get_index(-2, None, U_MAX), Ok(U_MAX-2..=U_MAX-2));
	// 	assert_eq!(get_index(I_MAX, None, U_MAX), Ok(U_MAX-I_MAX_U-1..=U_MAX-I_MAX_U-1));
	// 	assert_eq!(get_index(I_MAX-1, None, U_MAX), Ok(U_MAX-I_MAX_U-2..=U_MAX-I_MAX_U-2));
	// 	assert_eq!(get_index(I_MIN, None, U_MAX), Ok(U_MAX-I_MIN_U_POSITIVE..=U_MAX-I_MIN_U_POSITIVE));
	// 	assert_eq!(get_index(I_MIN-1, None, U_MAX), Ok(U_MAX-I_MIN_U_POSITIVE-1..=U_MAX-I_MIN_U_POSITIVE-1));
	// }*/

	// #[test]
	// fn get_index_no_end_negative() {
	// 	assert_eq!(get_index(-1, None, 0), Err(StartOutOfBounds));
	// 	assert_eq!(get_index(-1, None, 1), Ok(0..=0));
	// 	assert_eq!(get_index(-1, None, 2), Ok(1..=1));
	// 	assert_eq!(get_index(-1, None, 100_000), Ok(99_999..=99_999));
	// 	assert_eq!(get_index(-1, None, U_MAX), Ok(U_MAX-1..=U_MAX-1));

	// 	assert_eq!(get_index(-2, None, 3), Ok(1..=1));
	// 	assert_eq!(get_index(-2, None, 2), Ok(0..=0));
	// 	assert_eq!(get_index(-2, None, 1), Err(StartOutOfBounds));
	// 	assert_eq!(get_index(-2, None, 0), Err(StartOutOfBounds));
	// 	assert_eq!(get_index(-2, None, 0x_12_34_56), Ok(0x12_34_54..=0x12_34_54));
	// 	assert_eq!(get_index(-2, None, U_MAX), Ok(U_MAX-2..=U_MAX-2));

	// 	assert_eq!(get_index(-3, None, 3), Ok(0..=0));

	// 	assert_eq!(get_index(-990, None, 100_000), Ok(99_010..=99_010));
	// 	assert_eq!(get_index(-99_999, None, 100_000), Ok(1..=1));
	// 	assert_eq!(get_index(-100_000, None, 100_000), Ok(0..=0));
	// 	assert_eq!(get_index(-100_001, None, 100_000), Err(StartOutOfBounds));
	// 	assert_eq!(get_index(-100_001, None, U_MAX), Ok(U_MAX-100_001..=U_MAX-100_001));
	// }


// 	#[test]
// 	fn get_index_noend() -> Result<(), IndexError> {
// 		assert_eq!(0..1, get_index(1, None, 1)?);
// 		assert_eq!(0..1, get_index(1, None, 2)?);
// 		assert_eq!(0..1, get_index(1, None, 99)?);
// 		assert_eq!(1..2, get_index(2, None, 3)?);

// 		assert_eq!(0..1, get_index(1, None, 0)?);
// 		Ok(())
// 	}
// 	#[test]
// 	fn get_index_indexing() -> Result<(), IndexError> {
// 		// unimplemented!()
// 		Ok(())
// // pub fn get_index(mut start: isize, end: Option<isize>, len: usize) -> Result<Range<usize>, IndexError> {

// 	}
}







