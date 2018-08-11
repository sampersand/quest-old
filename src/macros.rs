macro_rules! regex {
	($regex:expr) => (Regex::new($regex).expect(concat!("Invalid internal regex `", $regex, "` encountered")));
}

macro_rules! assert_ge {
	($lhs:expr, $rhs:expr $(, $descr:expr)*) => {
		assert!($lhs >= $rhs $(,$descr)*)
	}
}

macro_rules! argcount {
	() => (0);
	($arg:tt $($other:tt)*) => (1 + argcount!($($other)*));
}

macro_rules! assert_args_len {
	($args:expr, $min_len:expr, $func:expr) => {{
		let len = $args.len();
		assert_ge!(len, $min_len, concat!("Invalid args length ({} < {}) for `", $func, "`"), len, $min_len)
	}}
}

macro_rules! expect_id {
	($obj:expr) => { expect_qobj!($obj, Var).as_id() }
}

macro_rules! expect_qobj {
($obj:expr, $var:ident) => {{
		use std::ops::Deref;
		match $obj.class().deref() {
			::obj::Classes::$var(val) => val,
			other => panic!(concat!("Unexpected variant `{:?}` found; ", stringify!($var), " expected"), other)
		}
	}}
}
