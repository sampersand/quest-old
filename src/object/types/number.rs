use std::fmt::{self, Display, Formatter};
use std::hash::{Hash, Hasher};
use crate::object::Object;
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
}

impl Object<Number> {
	pub fn new_number(num: f64) -> Object<Number> {
		Object::new(Number::new(num))
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
		let rhs_ref = getarg!(args[0]: Number)?;
		let lhs = num.data().read().expect(concat!("num read error in Number::", stringify!($oper)));
		let rhs = rhs_ref.data().read().expect(concat!("rhs read error in Number::", stringify!($oper)));
		Ok(Object::new_number(**lhs $oper **rhs))
	}};
	(logic $oper:tt) => { |num, args| {
		let rhs_ref = getarg!(args[0]: Number)?;
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

	"+" => f64_func!(math +),
	"-" => f64_func!(math -),
	"*" => f64_func!(math *),
	"/" => f64_func!(math /),
	"%" => f64_func!(math %),
	"**" => |num, args| {
		let rhs_ref = getarg!(args[0]: Number)?;
		let lhs = num.data().read().expect("num read error in Number::**");
		let rhs = rhs_ref.data().read().expect("rhs read error in Number::**");
		Ok(Object::new_number(lhs.powf(**rhs)))
	},
	"==" => f64_func!(logic ==),
	"!=" => f64_func!(logic !=),
	"<=" => f64_func!(logic <=),
	"<" => f64_func!(logic <),
	">=" => f64_func!(logic >=),
	">" => f64_func!(logic >),
	"-@" => |num, _| Ok(Object::new_number(-**num.data().read().expect("read error in Number::-@"))),
	"+@" => |num, _| Ok(num.duplicate())
}

// fn number_add(num: &Object<Number>, args: &[&AnyObject]) -> Result<AnyObject> {
// 	let arg = args.get(0).unwrap().downcast::<Number>().unwrap();
// 	Ok(Object::new_number(num.data().as_ref() + arg.data().as_ref()))
// }

// lazy_static::lazy_static! {
// 	pub static ref NUMBER_MAP: Shared<dyn Map> = Shared::new({
// 		let mut map = HashMap::<AnyObject, AnyObject>::new();
// 		map.insert(Object::new_variable("+"), Object::new_named_rustfn("+", number_add));
// 		map
// 	});
// }

// impl Type for  Number {
// 	fn get_type_map() -> Shared<dyn Map> {
// 		NUMBER_MAP.clone()
// 	}
// }


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
}