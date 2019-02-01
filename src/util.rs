use std::fmt::{self, Debug, Display, Formatter};
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