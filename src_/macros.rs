macro_rules! regex {
	($regex:expr) => {{
		use regex::Regex;
		use std::ops::Deref;
		lazy_static!{
			static ref REGEX: Regex = Regex::new($regex).expect(concat!("Invalid internal regex `", $regex, "` encountered"));
		}
		REGEX.deref()
	}}
}

macro_rules! argcount {
	() => (0);
	($arg:tt $($other:tt)*) => (1 + argcount!($($other)*));
}


macro_rules! fn_struct {
	(pub struct $struct:ident(pub $ty:ty);) => {
		#[derive(Clone, Copy)]
		pub struct $struct(pub $ty);
		fn_struct!(_inner $struct $ty);
	};
	(struct $struct:ident(pub $ty:ty);) => {
		#[derive(Clone, Copy)]
		struct $struct(pub $ty);
		fn_struct!(_inner $struct $ty);
	};
	(_inner $struct:ident $ty:ty) => {
		impl From<$ty> for $struct {
			#[inline]
			fn from(func: $ty) -> $struct {
				$struct(func)
			}
		}


		impl Eq for $struct {}
		impl PartialEq for $struct {
			fn eq(&self, other: &$struct) -> bool {
				self.0 as usize == other.0 as usize
			}
		}
		
		impl ::std::fmt::Debug for $struct {
			fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
				write!(f, concat!(stringify!($struct), "({:p})"), &self.0)
			}
		}
		
	}
}
