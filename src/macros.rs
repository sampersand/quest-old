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
