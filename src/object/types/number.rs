use std::fmt::{self, Display, Formatter};
use std::hash::{Hash, Hasher};
use crate::object::{Object, AnyObject};
use crate::err::{Result, Error};
use std::ops::Deref;
use super::quest_funcs::{
	AT_BOOL, AT_NUM, AT_TEXT,
	ADD, SUB, MUL, DIV, MOD, POW,
	EQL, NEQ, LTH, GTH, LEQ, GEQ, CMP,
	POS, NEG
};


#[derive(Debug, PartialEq, PartialOrd, Clone, Copy, Default)]
pub struct Number(f64);

impl Number {
	#[inline]
	pub fn new(num: f64) -> Number {
		Number(num)
	}

	#[allow(unused)] // this isn't working properly
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
		use std::str::FromStr;
		// todo: parse_str
		// this is very temporary.
		if text.is_empty() {
			return Ok(Number::new(0.0))
		}

		f64::from_str(text).map(Number::new).map_err(|_| Error::BadArgument {
			pos: 0, arg: Object::new_number(9.0).as_any(), msg: ""
		})
	}

	pub fn to_integer(&self) -> isize {
		// TODO: make tests here
		self.0 as isize
	}
}

impl Object<Number> {
	pub fn new_number(num: f64) -> Object<Number> {
		Object::new(Number::new(num))
	}
}

impl AnyObject {
	pub fn to_number(&self) -> Result<Object<Number>> {
		self.call_attr(AT_NUM, &[])?
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
	(math $oper:tt $name:ident) => { |num, args| {
		let rhs_obj = getarg!(args[0] @ to_number)?;
		let lhs = num.data().read().expect(const_concat!("num read error in Number::", $name));
		let rhs = rhs_obj.data().read().expect(const_concat!("rhs read error in Number::", $name));
		Ok(Object::new_number(**lhs $oper **rhs))
	}};

	(logic $oper:tt $name:ident) => { |num, args| {
		let rhs_obj = getarg!(args[0] @ to_number)?;
		let lhs = num.data().read().expect(const_concat!("num read error in Number::", $name));
		let rhs = rhs_obj.data().read().expect(const_concat!("rhs read error in Number::", $name));
		Ok(Object::new_boolean(**lhs $oper **rhs))
	}};

	(integer $oper:tt) => {
		unimplemented!()
	}
}

impl_type! { for Number;
	AT_BOOL => |num, _| Ok(Object::new_boolean(*num.data().read().expect(const_concat!("read error in Number::", AT_BOOL)).as_ref() != 0.0)),
	AT_NUM => |num, _| Ok(num.duplicate()),
	AT_TEXT => |num, _| Ok(Object::new_text(num.data().read().expect(const_concat!("read error in Number::", AT_TEXT)).as_ref().to_string())),

	ADD => f64_func!(math + ADD),
	SUB => f64_func!(math - SUB),
	MUL => f64_func!(math * MUL),
	DIV => f64_func!(math / DIV),
	MOD => f64_func!(math % MOD),
	POW => |obj, args| {
		let rhs_obj = getarg!(args[0] @ to_number)?;
		let lhs = obj.data().read().expect(const_concat!("obj read error in Number::", POW));
		let rhs = rhs_obj.data().read().expect(const_concat!("rhs read error in Number::", POW));
		Ok(Object::new_number(lhs.powf(**rhs)))
	},

	EQL => f64_func!(logic == EQL),
	NEQ => f64_func!(logic != NEQ),
	LTH => f64_func!(logic < LTH),
	GTH => f64_func!(logic > GTH),
	LEQ => f64_func!(logic <= LEQ),
	GEQ => f64_func!(logic >= GEQ),
	CMP => |obj, args| {
		use std::cmp::{Ordering, Ord};
		let rhs_obj = getarg!(args[0] @ to_number)?;
		let lhs = **obj.data().read().expect(const_concat!("obj read error in Number::", CMP));
		let rhs = rhs_obj.data().read().expect(const_concat!("rhs read error in Number::", CMP));

		Ok(match lhs.partial_cmp(&*rhs) {
			None => Object::new_null(),
			Some(Ordering::Less) => Object::new_number(-1.0),
			Some(Ordering::Equal) => Object::new_number(0.0),
			Some(Ordering::Greater) => Object::new_number(1.0)
		})
	},

	NEG => |obj, _| Ok(Object::new_number(-**obj.data().read().expect(const_concat!("read error in Number::", NEG)))),
	POS => |obj, _| Ok(obj.duplicate()), // maybe making this an absolute value might be interesting
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
		assert_num_call_eq!(AT_BOOL Boolean;
			(0.0, []) => false,
			(-0.0, []) => false,
			(13.4, []) => true,
			(INFINITY, []) => true,
			(PI, []) => true,
			(E, []) => true,
			(-123.0, []) => true,
			(12e49, [&n!(34.0)]) => true // ensure extra args are ignored
		);

		Ok(())
	}

	#[test]
	fn at_text() -> Result<()> {
		assert_num_call_eq!(AT_TEXT Text;
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
		assert_num_call_eq!(AT_NUM Number; 
			(13.4, []) => 13.4,
			(INFINITY, []) => INFINITY,
			(PI, []) => PI,
			(E, []) => E,
			(-123.0, []) => -123.0,
			(12.0, [&n!(34.0)]) => 12.0 // ensure extra args are ignored
		);

		// make sure that it acutally duplicates the map
		let obj = Object::new_number(12.45);
		let dup = obj.as_any().call_attr(AT_NUM, &[])?.downcast_or_err::<Number>()?;
		assert_eq!(*obj.data().read().unwrap(), *dup.data().read().unwrap());
		assert!(!obj._map_only_for_testing().ptr_eq(dup._map_only_for_testing()));
		Ok(())
	}

	#[test]
	fn add() -> Result<()> {
		assert_num_call_eq!(ADD Number;
			(13.4, [&n!(-4.0)]) => 9.4,
			(PI, [&n!(PI)]) => 2f64 * PI,
			(E, [&n!(E)]) => 2f64 * E,
			(8e9, [&n!(1e9), &n!(PI)]) => 9e9 // ensure extra args are ignored
		);

		assert!(n!(NAN).call_attr(ADD, &[&n!(NAN)])?.downcast_or_err::<Number>()?.data().read().unwrap().is_nan());
		assert!(n!(INFINITY).call_attr(ADD, &[&n!(INFINITY)])?.downcast_or_err::<Number>()?.data().read().unwrap().is_infinite());
		assert!(n!(INFINITY).call_attr(ADD, &[&n!(NEG_INFINITY)])?.downcast_or_err::<Number>()?.data().read().unwrap().is_nan());

		assert_param_missing!(n!(4.0).call_attr(ADD, &[]));

		Ok(())
	}
	

	#[test]
	fn sub() -> Result<()> {
		assert_num_call_eq!(SUB Number;
			(13.4, [&n!(-4.0)]) => 17.4,
			(PI, [&n!(PI)]) => 0.0,
			(E, [&n!(PI)]) => E - PI,
			(9e9, [&n!(1e9), &n!(PI)]) => 8e9 // ensure extra args are ignored
		);

		assert!(n!(NAN).call_attr(SUB, &[&n!(NAN)])?.downcast_or_err::<Number>()?.data().read().unwrap().is_nan());
		assert!(n!(INFINITY).call_attr(SUB, &[&n!(INFINITY)])?.downcast_or_err::<Number>()?.data().read().unwrap().is_nan());
		assert!(n!(INFINITY).call_attr(SUB, &[&n!(NEG_INFINITY)])?.downcast_or_err::<Number>()?.data().read().unwrap().is_infinite());

		assert_param_missing!(n!(4.0).call_attr(SUB, &[]));

		Ok(())
	}
	
	#[test]
	fn mul() -> Result<()> {
		assert_num_call_eq!(MUL Number;
			(13.4, [&n!(-4.0)]) => -53.6,
			(PI, [&n!(PI)]) => PI * PI,
			(E, [&n!(-1e-4)]) => E * -1e-4,
			(9e3, [&n!(8e3), &n!(PI)]) => 7.2e7 // ensure extra args are ignored
		);


		assert!(n!(NAN).call_attr(MUL, &[&n!(NAN)])?.downcast_or_err::<Number>()?.data().read().unwrap().is_nan());
		assert!(n!(INFINITY).call_attr(MUL, &[&n!(INFINITY)])?.downcast_or_err::<Number>()?.data().read().unwrap().is_infinite());
		assert!(n!(INFINITY).call_attr(MUL, &[&n!(NEG_INFINITY)])?.downcast_or_err::<Number>()?.data().read().unwrap().is_infinite());

		assert_param_missing!(n!(4.0).call_attr(MUL, &[]));

		Ok(())
	}
	

	#[test]
	fn div() -> Result<()> {
		assert_num_call_eq!(DIV Number;
			(13.4, [&n!(-4.0)]) => -3.35,
			(PI, [&n!(E)]) => PI / E,
			(9e7, [&n!(-8e-2)]) => -1.125e9,
			(4.0, [&n!(1.0), &n!(PI)]) => 4.0 // ensure extra args are ignored
		);

		// make sure to test for negative stuff here
		assert!(n!(1.0).call_attr(DIV, &[&n!(0.0)])?.downcast_or_err::<Number>()?.data().read().unwrap().is_infinite());

		assert!(n!(NAN).call_attr(DIV, &[&n!(NAN)])?.downcast_or_err::<Number>()?.data().read().unwrap().is_nan());
		assert!(n!(INFINITY).call_attr(DIV, &[&n!(INFINITY)])?.downcast_or_err::<Number>()?.data().read().unwrap().is_nan());
		assert!(n!(INFINITY).call_attr(DIV, &[&n!(NEG_INFINITY)])?.downcast_or_err::<Number>()?.data().read().unwrap().is_nan());

		assert_param_missing!(n!(4.0).call_attr(DIV, &[]));

		Ok(())
	}
	

	#[test]
	fn r#mod() -> Result<()> {
		// Note: Rust implements negative modulos differently than other languages:
		// n % d == n - (n/d).to_integer() * d
		assert_num_call_eq!(MOD Number;
			(13.5, [&n!(-4.0)]) => 1.5, 
			(13.4, [&n!(3.1)]) => 1.0,
			(PI, [&n!(E)]) => PI % E,
			(9e19, [&n!(9.0)]) => 0.0,
			(-1234.0, [&n!(39.0), &n!(PI)]) => -25.0 // ensure extra args are ignored
		);

		assert!(n!(1.0).call_attr(MOD, &[&n!(0.0)])?.downcast_or_err::<Number>()?.data().read().unwrap().is_nan());

		assert!(n!(NAN).call_attr(MOD, &[&n!(NAN)])?.downcast_or_err::<Number>()?.data().read().unwrap().is_nan());
		assert!(n!(INFINITY).call_attr(MOD, &[&n!(INFINITY)])?.downcast_or_err::<Number>()?.data().read().unwrap().is_nan());
		assert!(n!(INFINITY).call_attr(MOD, &[&n!(NEG_INFINITY)])?.downcast_or_err::<Number>()?.data().read().unwrap().is_nan());

		assert_param_missing!(n!(4.0).call_attr(MOD, &[]));

		Ok(())
	}
	
	#[test]
	fn pow() -> Result<()> {
		assert_num_call_eq!(POW Number;
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

		assert!(n!(NAN).call_attr(POW, &[&n!(NAN)])?.downcast_or_err::<Number>()?.data().read().unwrap().is_nan());
		assert!(n!(NAN).call_attr(POW, &[&n!(INFINITY)])?.downcast_or_err::<Number>()?.data().read().unwrap().is_nan());
		assert!(n!(NEG_INFINITY).call_attr(POW, &[&n!(NAN)])?.downcast_or_err::<Number>()?.data().read().unwrap().is_nan());

		assert_param_missing!(n!(4.0).call_attr(POW, &[]));

		Ok(())
	}


	#[test]
	fn eql() -> Result<()> {
		assert_num_call_eq!(EQL Boolean;
			(13.5, [&n!(13.5)]) => true, 
			(-123.0, [&n!(-123.0)]) => true,
			(-0.0, [&n!(0.0)]) => true,
			(9.123e9, [&n!(-9.123e9)]) => false,
			(INFINITY, [&n!(INFINITY)]) => true,
			(INFINITY, [&n!(NEG_INFINITY)]) => false,
			(NAN, [&n!(NAN)]) => false,
			(1.0, [&n!(1.0), &n!(2.0)]) => true // ensure extra args are ignored
		);

		assert_param_missing!(n!(4.0).call_attr(EQL, &[]));

		Ok(())
	}

	#[test]
	fn neq() -> Result<()> {
		assert_num_call_eq!(NEQ Boolean;
			(13.5, [&n!(13.5)]) => false, 
			(-123.0, [&n!(-123.0)]) => false,
			(-0.0, [&n!(0.0)]) => false,
			(9.123e9, [&n!(-9.123e9)]) => true,
			(INFINITY, [&n!(INFINITY)]) => false,
			(INFINITY, [&n!(NEG_INFINITY)]) => true,
			(NAN, [&n!(NAN)]) => true,
			(1.0, [&n!(1.0), &n!(2.0)]) => false // ensure extra args are ignored
		);

		assert_param_missing!(n!(4.0).call_attr(NEQ, &[]));

		Ok(())
	}

	#[test]
	fn cmp() -> Result<()> {
		assert_num_call_eq!(CMP Number;
			(13.5, [&n!(4.0)]) => 1.0, 
			(0.5, [&n!(64.0)]) => -1.0,
			(-0.05, [&n!(-1.0)]) => 1.0,
			(2.0, [&n!(9e9)]) => -1.0,
			(9e9, [&n!(9e9)]) => 0.0,
			(NEG_INFINITY, [&n!(INFINITY)]) => -1.0,
			(1.0, [&n!(0.0), &n!(PI)]) => 1.0 // ensure extra args are ignored
		);

		assert!(n!(NAN).call_attr(CMP, &[&n!(9.0)])?.is_null());
		assert!(n!(NAN).call_attr(CMP, &[&n!(NAN)])?.is_null());
		assert!(n!(NEG_INFINITY).call_attr(CMP, &[&n!(NAN)])?.is_null());


		assert_param_missing!(n!(4.0).call_attr(CMP, &[]));

		Ok(())
	}

	#[test]
	fn lth() -> Result<()> {
		assert_num_call_eq!(LTH Boolean;
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

		assert_param_missing!(n!(4.0).call_attr(LTH, &[]));

		Ok(())
	}

	#[test]
	fn leq() -> Result<()> {
		assert_num_call_eq!(LEQ Boolean;
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

		assert_param_missing!(n!(4.0).call_attr(LEQ, &[]));

		Ok(())
	}


	#[test]
	fn gth() -> Result<()> {
		assert_num_call_eq!(GTH Boolean;
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

		assert_param_missing!(n!(4.0).call_attr(GTH, &[]));

		Ok(())
	}

	#[test]
	fn geq() -> Result<()> {
		assert_num_call_eq!(GEQ Boolean;
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

		assert_param_missing!(n!(4.0).call_attr(GEQ, &[]));

		Ok(())
	}

	#[test]
	fn neg() -> Result<()> {
		assert_num_call_eq!(NEG Number;
			(13.5, []) => -13.5, 
			(-PI, []) => PI,
			(0.0, []) => 0.0,
			(9e9, []) => -9e9,
			(NEG_INFINITY, []) => INFINITY,
			(INFINITY, []) => NEG_INFINITY,
			(1.0, [&n!(PI)]) => -1.0 // ensure extra args are ignored
		);

		assert!(n!(NAN).call_attr(NEG, &[])?.downcast_or_err::<Number>()?.data().read().unwrap().is_nan());

		Ok(())
	}

	#[test]
	fn pos() -> Result<()> {
		assert_num_call_eq!(POS Number;
			(13.5, []) => 13.5, 
			(-PI, []) => -PI,
			(0.0, []) => 0.0,
			(9e9, []) => 9e9,
			(NEG_INFINITY, []) => NEG_INFINITY,
			(INFINITY, []) => INFINITY,
			(1.0, [&n!(PI)]) => 1.0 // ensure extra args are ignored
		);

		assert!(n!(NAN).call_attr(POS, &[])?.downcast_or_err::<Number>()?.data().read().unwrap().is_nan());


		// make sure that it acutally duplicates the map
		let obj = Object::new_number(12.45);
		let dup = obj.as_any().call_attr(AT_NUM, &[])?.downcast_or_err::<Number>()?;
		assert_eq!(*obj.data().read().unwrap(), *dup.data().read().unwrap());
		assert!(!obj._map_only_for_testing().ptr_eq(dup._map_only_for_testing()));
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