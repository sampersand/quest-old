use std::fmt::{self, Display, Formatter};
use std::hash::{Hash, Hasher};
use crate::object::{Object, AnyObject};
use crate::err::Result;
use std::ops::Deref;

#[derive(Debug, PartialEq, PartialOrd, Clone, Copy, Default)]
pub struct Number(f64);

impl Number {
	#[inline]
	pub fn new(num: f64) -> Number {
		Number(num)
	}

	#[allow(unused)] // this isn't working perfectly
	fn _from_whole_decimal(whole: i32, decimal: u32) -> Number {
		let decimal_digits = (decimal as f64).log10().ceil();
		let whole = whole as f64;
		let decimal = (decimal as f64) * 10f64.powf(-decimal_digits);

		if whole.is_sign_negative() {
			Number(whole - decimal)
		} else {
			Number(whole + decimal)
		}
	}

	pub fn parse_str(text: &str) -> Result<Number> {
		// BadArgument
		unimplemented!("Number::parse_str")
	}
	// "@num" => |obj, _| Number::parse_str(&obj.data().read().expect("read err in Text::@bool")).map(Object::new),

}

impl Object<Number> {
	pub fn new_number(num: f64) -> Object<Number> {
		Object::new(Number::new(num))
	}
}

impl AnyObject {
	pub fn to_number(&self) -> Result<Object<Number>> {
		self.call_attr("@num", &[])?
			.downcast_or_err::<Number>()
	}
}


impl From<f64> for Number {
	fn from(num: f64) -> Number {
		Number::new(num)
	}
}

impl From<Number> for f64 {
	fn from(num: Number) -> f64 {
		num.0
	}
}

impl AsRef<f64> for Number {
	fn as_ref(&self) -> &f64 {
		&self.0
	}
}

impl Deref for Number {
	type Target = f64;
	fn deref(&self) -> &f64 {
		&self.0
	}
}

impl Hash for Number {
	fn hash<H: Hasher>(&self, h: &mut H) {
		(self.0 as u64).hash(h);
	}
}

impl Display for Number {
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		Display::fmt(&self.0, f)
	}
}

macro_rules! f64_func {
	(math $oper:tt) => { |num, args| {
		let rhs_ref = getarg!(args[0] @ to_number)?;
		let lhs = num.data().read().expect(concat!("num read error in Number::", stringify!($oper)));
		let rhs = rhs_ref.data().read().expect(concat!("rhs read error in Number::", stringify!($oper)));
		Ok(Object::new_number(**lhs $oper **rhs))
	}};
	(logic $oper:tt) => { |num, args| {
		let rhs_ref = getarg!(args[0] @ to_number)?;
		let lhs = num.data().read().expect(concat!("num read error in Number::", stringify!($oper)));
		let rhs = rhs_ref.data().read().expect(concat!("rhs read error in Number::", stringify!($oper)));
		Ok(Object::new_boolean(**lhs $oper **rhs))
	}};

	(integer $oper:tt) => {
		unimplemented!()
	}
}

impl_type! { for Number;
	"@bool" => |num, _| Ok(Object::new_boolean(*num.data().read().expect("read error in Number::@bool").as_ref() != 0.0)),
	"@num" => |num, _| Ok(Object::new_number(**num.data().read().expect("read error in Number::@num"))),
	"@text" => |num, _| Ok(Object::new_text(num.data().read().expect("read error in Number::@text").as_ref().to_string())),

	"+" => f64_func!(math +),
	"-" => f64_func!(math -),
	"*" => f64_func!(math *),
	"/" => f64_func!(math /),
	"%" => f64_func!(math %),
	"**" => |obj, args| {
		let rhs_ref = getarg!(args[0] @ to_number)?;
		let lhs = obj.data().read().expect("obj read error in Number::**");
		let rhs = rhs_ref.data().read().expect("rhs read error in Number::**");
		Ok(Object::new_number(lhs.powf(**rhs)))
	},

	"==" => f64_func!(logic ==),
	"!=" => f64_func!(logic !=),
	"<=" => f64_func!(logic <=),
	"<" => f64_func!(logic <),
	">=" => f64_func!(logic >=),
	">" => f64_func!(logic >),
	"<=>" => |obj, args| {
		use std::cmp::{Ordering, Ord};
		let rhs_ref = getarg!(args[0] @ to_number)?;
		let lhs = **obj.data().read().expect("obj read error in Number::**");
		let rhs = rhs_ref.data().read().expect("rhs read error in Number::**");

		Ok(Object::new_number(match lhs.partial_cmp(&*rhs) {
			None => std::f64::NAN,
			Some(Ordering::Less) => -1.0,
			Some(Ordering::Equal) => 0.0,
			Some(Ordering::Greater) => 1.0
		}))
	},

	"-@" => |obj, _| Ok(Object::new_number(-**obj.data().read().expect("read error in Number::-@"))),
	"+@" => |obj, _| Ok(obj.duplicate()),
}

#[cfg(test)]
mod fn_tests {
	use super::*;
	use crate::object::types::{Boolean, Text};
	use crate::err::Error;
	use std::f64::{INFINITY, NEG_INFINITY, NAN, consts::{PI, E}};

	macro_rules! n {
		($num:expr) => (Object::new_number($num).as_any())
	}

	macro_rules! assert_num_call_eq {
		($attr:tt $type:ty; $(($obj:expr, $args:tt) => $expected:expr),*) => {
			$(
				assert_eq!(**n!($obj).call_attr($attr, &$args)?.downcast_or_err::<$type>()?.data().read().unwrap(), $expected);
			)*
		}
	}

	#[test]
	fn at_bool() -> Result<()> {
		assert_num_call_eq!("@bool" Boolean;
			(0.0, []) => false,
			(-0.0, []) => false,
			(13.4, []) => true,
			(INFINITY, []) => true,
			(PI, []) => true,
			(E, []) => true,
			(-123.0, []) => true,
			(12e49, [&n!(34.0)]) => true //  ensure extra args are ignored
		);
		Ok(())
	}

	#[test]
	fn at_text() -> Result<()> {
		assert_num_call_eq!("@text" Text;
			(0.0, []) => *"0",
			(1.0, []) => *"1",
			(-1.0, []) => *"-1",
			(123.4, []) => *"123.4",
			(-1.23, []) => *"-1.23",
			(NAN, []) => *"NaN",
			(INFINITY, []) => *"inf",
			(NEG_INFINITY, []) => *"-inf",
			(-999.0, [&n!(12.0)]) => *"-999" // ensure extra args are ignored
		);

		// Note: There isn't a specified way large numbers (eg `1e9`) will be displayed
		// Also of note: There isn't a specified length of characters for `E` or `PI`.
		Ok(())
	}

	#[test]
	fn at_num() -> Result<()> {
		assert_num_call_eq!("@num" Number; 
			(13.4, []) => 13.4,
			(INFINITY, []) => INFINITY,
			(PI, []) => PI,
			(E, []) => E,
			(-123.0, []) => -123.0,
			(12.0, [&n!(34.0)]) => 12.0 //  ensure extra args are ignored
		);

		// make sure that it acutally duplicates the map
		let obj = Object::new_number(12.45);
		let dup = obj.call_attr("@num", &[])?.downcast_or_err::<Number>()?;
		assert_eq!(*obj.data().read().unwrap(), *dup.data().read().unwrap());
		assert!(!obj._map_only_for_testing().ptr_eq(dup._map_only_for_testing()));
		Ok(())
	}

	#[test]
	fn add() -> Result<()> {
		assert_num_call_eq!("+" Number;
			(13.4, [&n!(-4.0)]) => 9.4,
			(PI, [&n!(PI)]) => 2f64 * PI,
			(E, [&n!(E)]) => 2f64 * E,
			(8e9, [&n!(1e9), &n!(PI)]) => 9e9 // ensure extra args are ignored
		);

		assert!(n!(NAN).call_attr("+", &[&n!(NAN)])?.downcast_or_err::<Number>()?.data().read().unwrap().is_nan());
		assert!(n!(INFINITY).call_attr("+", &[&n!(INFINITY)])?.downcast_or_err::<Number>()?.data().read().unwrap().is_infinite());
		assert!(n!(INFINITY).call_attr("+", &[&n!(NEG_INFINITY)])?.downcast_or_err::<Number>()?.data().read().unwrap().is_nan());

		// check to see if too few args are passed it handles it right
		match n!(4.0).call_attr("+", &[]).unwrap_err() {
			Error::MissingArgument { pos: 0, .. } => {},
			_ => panic!()
		}

		Ok(())
	}
	

	#[test]
	fn sub() -> Result<()> {
		assert_num_call_eq!("-" Number;
			(13.4, [&n!(-4.0)]) => 17.4,
			(PI, [&n!(PI)]) => 0.0,
			(E, [&n!(PI)]) => E - PI,
			(9e9, [&n!(1e9), &n!(PI)]) => 8e9 // ensure extra args are ignored
		);

		assert!(n!(NAN).call_attr("-", &[&n!(NAN)])?.downcast_or_err::<Number>()?.data().read().unwrap().is_nan());
		assert!(n!(INFINITY).call_attr("-", &[&n!(INFINITY)])?.downcast_or_err::<Number>()?.data().read().unwrap().is_nan());
		assert!(n!(INFINITY).call_attr("-", &[&n!(NEG_INFINITY)])?.downcast_or_err::<Number>()?.data().read().unwrap().is_infinite());

		// check to see if too few args are passed it handles it right
		match n!(4.0).call_attr("-", &[]).unwrap_err() {
			Error::MissingArgument { pos: 0, .. } => {},
			_ => panic!()
		}

		Ok(())
	}
	
	#[test]
	fn mul() -> Result<()> {
		assert_num_call_eq!("*" Number;
			(13.4, [&n!(-4.0)]) => -53.6,
			(PI, [&n!(PI)]) => PI * PI,
			(E, [&n!(-1e-4)]) => E * -1e-4,
			(9e3, [&n!(8e3), &n!(PI)]) => 7.2e7 // ensure extra args are ignored
		);


		assert!(n!(NAN).call_attr("*", &[&n!(NAN)])?.downcast_or_err::<Number>()?.data().read().unwrap().is_nan());
		assert!(n!(INFINITY).call_attr("*", &[&n!(INFINITY)])?.downcast_or_err::<Number>()?.data().read().unwrap().is_infinite());
		assert!(n!(INFINITY).call_attr("*", &[&n!(NEG_INFINITY)])?.downcast_or_err::<Number>()?.data().read().unwrap().is_infinite());

		// check to see if too few args are passed it handles it right
		match n!(4.0).call_attr("*", &[]).unwrap_err() {
			Error::MissingArgument { pos: 0, .. } => {},
			_ => panic!()
		}

		Ok(())
	}
	

	#[test]
	fn div() -> Result<()> {
		assert_num_call_eq!("/" Number;
			(13.4, [&n!(-4.0)]) => -3.35,
			(PI, [&n!(E)]) => PI / E,
			(9e7, [&n!(-8e-2)]) => -1.125e9,
			(4.0, [&n!(1.0), &n!(PI)]) => 4.0 // ensure extra args are ignored
		);

		// make sure to test for negative stuff here
		assert!(n!(1.0).call_attr("/", &[&n!(0.0)])?.downcast_or_err::<Number>()?.data().read().unwrap().is_infinite());

		assert!(n!(NAN).call_attr("/", &[&n!(NAN)])?.downcast_or_err::<Number>()?.data().read().unwrap().is_nan());
		assert!(n!(INFINITY).call_attr("/", &[&n!(INFINITY)])?.downcast_or_err::<Number>()?.data().read().unwrap().is_nan());
		assert!(n!(INFINITY).call_attr("/", &[&n!(NEG_INFINITY)])?.downcast_or_err::<Number>()?.data().read().unwrap().is_nan());

		// check to see if too few args are passed it handles it right
		match n!(4.0).call_attr("/", &[]).unwrap_err() {
			Error::MissingArgument { pos: 0, .. } => {},
			_ => panic!()
		}

		Ok(())
	}
	

	#[test]
	fn r#mod() -> Result<()> {
		// Note: Rust implements negative modulos differently than other languages:
		// n % d == n - (n/d).to_integer() * d
		assert_num_call_eq!("%" Number;
			(13.5, [&n!(-4.0)]) => 1.5, 
			(13.4, [&n!(3.1)]) => 1.0,
			(PI, [&n!(E)]) => PI % E,
			(9e19, [&n!(9.0)]) => 0.0,
			(-1234.0, [&n!(39.0), &n!(PI)]) => -25.0 // ensure extra args are ignored
		);

		assert!(n!(1.0).call_attr("%", &[&n!(0.0)])?.downcast_or_err::<Number>()?.data().read().unwrap().is_nan());

		assert!(n!(NAN).call_attr("%", &[&n!(NAN)])?.downcast_or_err::<Number>()?.data().read().unwrap().is_nan());
		assert!(n!(INFINITY).call_attr("%", &[&n!(INFINITY)])?.downcast_or_err::<Number>()?.data().read().unwrap().is_nan());
		assert!(n!(INFINITY).call_attr("%", &[&n!(NEG_INFINITY)])?.downcast_or_err::<Number>()?.data().read().unwrap().is_nan());

		// check to see if too few args are passed it handles it right
		match n!(4.0).call_attr("%", &[]).unwrap_err() {
			Error::MissingArgument { pos: 0, .. } => {},
			_ => panic!()
		}

		Ok(())
	}
	
	#[test]
	fn pow() -> Result<()> {
		assert_num_call_eq!("**" Number;
			(13.5, [&n!(4.0)]) => 33215.0625, 
			(64.0, [&n!(0.5)]) => 8.0,
			(-0.05, [&n!(-1.0)]) => -20.0,
			(9e9, [&n!(2.0)]) => 8.1e19,
			(NAN, [&n!(0.0)]) => 1.0,
			(INFINITY, [&n!(0.0)]) => 1.0,
			(1234.0, [&n!(NEG_INFINITY)]) => 0.0,
			(1234.0, [&n!(INFINITY)]) => INFINITY,
			(12.0, [&n!(0.0), &n!(PI)]) => 1.0 // ensure extra args are ignored
		);

		assert!(n!(NAN).call_attr("**", &[&n!(NAN)])?.downcast_or_err::<Number>()?.data().read().unwrap().is_nan());
		assert!(n!(NAN).call_attr("**", &[&n!(INFINITY)])?.downcast_or_err::<Number>()?.data().read().unwrap().is_nan());
		assert!(n!(NEG_INFINITY).call_attr("**", &[&n!(NAN)])?.downcast_or_err::<Number>()?.data().read().unwrap().is_nan());

		// check to see if too few args are passed it handles it right
		match n!(4.0).call_attr("**", &[]).unwrap_err() {
			Error::MissingArgument { pos: 0, .. } => {},
			_ => panic!()
		}

		Ok(())
	}


	#[test]
	fn equality() -> Result<()> {
		assert_num_call_eq!("==" Boolean;
			(13.5, [&n!(13.5)]) => true, 
			(-123.0, [&n!(-123.0)]) => true,
			(-0.0, [&n!(0.0)]) => true,
			(9.123e9, [&n!(-9.123e9)]) => false,
			(INFINITY, [&n!(INFINITY)]) => true,
			(INFINITY, [&n!(NEG_INFINITY)]) => false,
			(NAN, [&n!(NAN)]) => false,
			(1.0, [&n!(1.0), &n!(2.0)]) => true // ensure extra args are ignored
		);

		// check to see if too few args are passed it handles it right
		match n!(4.0).call_attr("==", &[]).unwrap_err() {
			Error::MissingArgument { pos: 0, .. } => {},
			_ => panic!()
		}

		Ok(())
	}

	#[test]
	fn not_equal() -> Result<()> {
		assert_num_call_eq!("!=" Boolean;
			(13.5, [&n!(13.5)]) => false, 
			(-123.0, [&n!(-123.0)]) => false,
			(-0.0, [&n!(0.0)]) => false,
			(9.123e9, [&n!(-9.123e9)]) => true,
			(INFINITY, [&n!(INFINITY)]) => false,
			(INFINITY, [&n!(NEG_INFINITY)]) => true,
			(NAN, [&n!(NAN)]) => true,
			(1.0, [&n!(1.0), &n!(2.0)]) => false // ensure extra args are ignored
		);

		// check to see if too few args are passed it handles it right
		match n!(4.0).call_attr("==", &[]).unwrap_err() {
			Error::MissingArgument { pos: 0, .. } => {},
			_ => panic!()
		}

		Ok(())
	}

	#[test]
	fn cmp() -> Result<()> {
		assert_num_call_eq!("<=>" Number;
			(13.5, [&n!(4.0)]) => 1.0, 
			(0.5, [&n!(64.0)]) => -1.0,
			(-0.05, [&n!(-1.0)]) => 1.0,
			(2.0, [&n!(9e9)]) => -1.0,
			(9e9, [&n!(9e9)]) => 0.0,
			(NEG_INFINITY, [&n!(INFINITY)]) => -1.0,
			(1.0, [&n!(0.0), &n!(PI)]) => 1.0 // ensure extra args are ignored
		);

		assert!(n!(NAN).call_attr("<=>", &[&n!(9.0)])?.downcast_or_err::<Number>()?.data().read().unwrap().is_nan());
		assert!(n!(NAN).call_attr("<=>", &[&n!(NAN)])?.downcast_or_err::<Number>()?.data().read().unwrap().is_nan());
		assert!(n!(NEG_INFINITY).call_attr("<=>", &[&n!(NAN)])?.downcast_or_err::<Number>()?.data().read().unwrap().is_nan());


		// check to see if too few args are passed it handles it right
		match n!(4.0).call_attr("<=>", &[]).unwrap_err() {
			Error::MissingArgument { pos: 0, .. } => {},
			_ => panic!()
		}

		Ok(())
	}

	#[test]
	fn less_than() -> Result<()> {
		assert_num_call_eq!("<" Boolean;
			(13.5, [&n!(4.0)]) => false, 
			(0.5, [&n!(64.0)]) => true,
			(-0.05, [&n!(-1.0)]) => false,
			(2.0, [&n!(9e9)]) => true,
			(9e9, [&n!(9e9)]) => false,
			(NAN, [&n!(9.0)]) => false,
			(NAN, [&n!(NAN)]) => false,
			(NEG_INFINITY, [&n!(NAN)]) => false,
			(NEG_INFINITY, [&n!(INFINITY)]) => true,
			(1.0, [&n!(0.0), &n!(PI)]) => false // ensure extra args are ignored
		);

		// check to see if too few args are passed it handles it right
		match n!(4.0).call_attr("<", &[]).unwrap_err() {
			Error::MissingArgument { pos: 0, .. } => {},
			_ => panic!()
		}

		Ok(())
	}

	#[test]
	fn less_than_eq() -> Result<()> {
		assert_num_call_eq!("<=" Boolean;
			(13.5, [&n!(4.0)]) => false, 
			(0.5, [&n!(64.0)]) => true,
			(-0.05, [&n!(-1.0)]) => false,
			(2.0, [&n!(9e9)]) => true,
			(9e9, [&n!(9e9)]) => true,
			(NAN, [&n!(9.0)]) => false,
			(NAN, [&n!(NAN)]) => false,
			(NEG_INFINITY, [&n!(NAN)]) => false,
			(NEG_INFINITY, [&n!(INFINITY)]) => true,
			(NEG_INFINITY, [&n!(NEG_INFINITY)]) => true,
			(1.0, [&n!(1.0), &n!(-PI)]) => true // ensure extra args are ignored
		);

		// check to see if too few args are passed it handles it right
		match n!(4.0).call_attr("<=", &[]).unwrap_err() {
			Error::MissingArgument { pos: 0, .. } => {},
			_ => panic!()
		}

		Ok(())
	}


	#[test]
	fn greater_than() -> Result<()> {
		assert_num_call_eq!(">" Boolean;
			(13.5, [&n!(4.0)]) => true, 
			(0.5, [&n!(64.0)]) => false,
			(-0.05, [&n!(-1.0)]) => true,
			(9e9, [&n!(2.0)]) => true,
			(9e9, [&n!(9e9)]) => false,
			(NAN, [&n!(9.0)]) => false,
			(NAN, [&n!(NAN)]) => false,
			(NEG_INFINITY, [&n!(NAN)]) => false,
			(NEG_INFINITY, [&n!(INFINITY)]) => false,
			(1.0, [&n!(0.0), &n!(PI)]) => true // ensure extra args are ignored
		);

		// check to see if too few args are passed it handles it right
		match n!(4.0).call_attr(">", &[]).unwrap_err() {
			Error::MissingArgument { pos: 0, .. } => {},
			_ => panic!()
		}

		Ok(())
	}

	#[test]
	fn greater_than_eq() -> Result<()> {
		assert_num_call_eq!(">=" Boolean;
			(13.5, [&n!(4.0)]) => true, 
			(0.5, [&n!(64.0)]) => false,
			(-0.05, [&n!(-1.0)]) => true,
			(9e9, [&n!(2.0)]) => true,
			(9e9, [&n!(9e9)]) => true,
			(NAN, [&n!(9.0)]) => false,
			(NAN, [&n!(NAN)]) => false,
			(NEG_INFINITY, [&n!(NAN)]) => false,
			(NEG_INFINITY, [&n!(INFINITY)]) => false,
			(1.0, [&n!(0.0), &n!(PI)]) => true // ensure extra args are ignored
		);

		// check to see if too few args are passed it handles it right
		match n!(4.0).call_attr(">=", &[]).unwrap_err() {
			Error::MissingArgument { pos: 0, .. } => {},
			_ => panic!()
		}

		Ok(())
	}

	#[test]
	fn neg_unary() -> Result<()> {
		assert_num_call_eq!("-@" Number;
			(13.5, []) => -13.5, 
			(-PI, []) => PI,
			(0.0, []) => 0.0,
			(9e9, []) => -9e9,
			(NEG_INFINITY, []) => INFINITY,
			(INFINITY, []) => NEG_INFINITY,
			(1.0, [&n!(PI)]) => -1.0 // ensure extra args are ignored
		);

		assert!(n!(NAN).call_attr("-@", &[])?.downcast_or_err::<Number>()?.data().read().unwrap().is_nan());

		Ok(())
	}

	#[test]
	fn pos_unary() -> Result<()> {
		assert_num_call_eq!("+@" Number;
			(13.5, []) => 13.5, 
			(-PI, []) => -PI,
			(0.0, []) => 0.0,
			(9e9, []) => 9e9,
			(NEG_INFINITY, []) => NEG_INFINITY,
			(INFINITY, []) => INFINITY,
			(1.0, [&n!(PI)]) => 1.0 // ensure extra args are ignored
		);

		assert!(n!(NAN).call_attr("+@", &[])?.downcast_or_err::<Number>()?.data().read().unwrap().is_nan());

		Ok(())
	}
}


#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn new() {
		macro_rules! assert_new_asref_eq {
			($($val:expr),*) => {
				$( assert_eq!(Number::new($val).as_ref(), &$val); )*
			};
		}

		use std::f64::{*, consts::*};

		assert!(Number::new(NAN).as_ref().is_nan());
		assert!(Number::new(NEG_INFINITY).as_ref().is_infinite());
		assert!(Number::new(INFINITY).as_ref().is_infinite());

		assert_new_asref_eq!{
			0.0, -1.0, 1.0, 123491.0,
			INFINITY, NEG_INFINITY, EPSILON,
			MIN, MIN_POSITIVE, MAX,
			E, FRAC_1_PI, FRAC_2_PI, FRAC_1_SQRT_2, FRAC_2_SQRT_PI,
			FRAC_PI_2, FRAC_PI_3, FRAC_PI_4, FRAC_PI_6, FRAC_PI_8,
			LN_2, LN_10, LOG2_E, LOG10_E, PI, SQRT_2
		};

	}

	#[ignore]
	#[test]
	fn _from_whole_decimal() {
		assert_eq!(Number::_from_whole_decimal(12, 34).as_ref(), &12.34);
		assert_eq!(Number::_from_whole_decimal(-12, 34).as_ref(), &-12.34);
		assert_eq!(Number::_from_whole_decimal(0, 1234).as_ref(), &0.1234);
		assert_eq!(Number::_from_whole_decimal(1234, 0).as_ref(), &1234.0);
		assert_eq!(Number::_from_whole_decimal(-1234, 0).as_ref(), &-1234.0);
		assert_eq!(Number::_from_whole_decimal(-99999999, 99999999).as_ref(), &-99999999.99999999);
	}


	#[test]
	fn equality() {
		assert_eq!(Number::new(0.0), Number::new(0.0));
		assert_eq!(Number::new(123.456), Number::new(123.456));
	}

	#[test]
	fn new_number() {
		assert_eq!(Object::new(Number::new(123.456)), Object::new_number(123.456));
	}

	#[test]
	fn to_number() -> Result<()> {
		assert_eq!(**Object::new_number(1234.0).as_any().to_number()?.data().read().unwrap(), 1234.0);
		assert_eq!(**Object::new_number(1.0).as_any().to_number()?.data().read().unwrap(), 1.0);
		assert!(Object::new_number(std::f64::INFINITY).as_any().to_number()?.data().read().unwrap().is_infinite());
		assert!(Object::new_number(std::f64::NAN).as_any().to_number()?.data().read().unwrap().is_nan());
		
		Ok(())
	}

}