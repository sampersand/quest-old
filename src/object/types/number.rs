use std::fmt::{self, Display, Formatter};
use std::hash::{Hash, Hasher};
use crate::object::{Object, AnyObject};
use crate::err::{Result, Error};
use std::ops::Deref;
use super::quest_funcs::{
	AT_BOOL, AT_NUM, AT_TEXT,
	CALL,
	ADD, SUB, MUL, DIV, MOD, POW,
	EQL, NEQ, LTH, GTH, LEQ, GEQ, CMP,
	POS, NEG
};

type Inner = f64;

#[derive(Debug, PartialEq, PartialOrd, Clone, Copy, Default)]
pub struct Number(Inner);

mod consts {
	use super::Inner;
	pub use std::f64::{self as inner_mod, consts as inner_consts};

	pub const INF: Inner = inner_mod::INFINITY;
	pub const NEG_INF: Inner = inner_mod::NEG_INFINITY;
	pub const NAN: Inner = inner_mod::NAN;
	pub const PI: Inner = inner_consts::PI;
	pub const E: Inner = inner_consts::E;
}

impl Number {
	#[inline]
	pub fn new(num: Inner) -> Number {
		Number(num)
	}

	#[allow(unused)] // this isn't working properly
	fn _from_whole_decimal(whole: i32, decimal: u32) -> Number {
		let decimal_digits = (decimal as Inner).log10().ceil();
		let whole = whole as Inner;
		let decimal = (decimal as Inner) * 10f64.powf(-decimal_digits);

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

		Inner::from_str(text).map(Number::new).map_err(|_| Error::BadArgument {
			pos: 0, arg: Object::new_number(9.0).as_any(), msg: ""
		})
	}

	pub fn to_integer(&self) -> isize {
		// TODO: make tests here
		self.0 as isize
	}
}

impl Object<Number> {
	pub fn new_number(num: Inner) -> Object<Number> {
		Object::new(Number::new(num))
	}
}

impl AnyObject {
	pub fn to_number(&self) -> Result<Object<Number>> {
		self.call_attr(AT_NUM, &[])?.downcast_or_err::<Number>()
	}
}


impl From<Inner> for Number {
	fn from(num: Inner) -> Number {
		Number::new(num)
	}
}

impl From<Number> for Inner {
	fn from(num: Number) -> Inner {
		num.0
	}
}

impl PartialEq<Inner> for Object<Number> {
	fn eq(&self, rhs: &Inner) -> bool {
		self.data().read().expect("read error in Object<Number>::eq<Inner>").as_ref() == rhs
	}
}

impl Object<Number> {
	pub fn is_nan(&self) -> bool {
		self.data().read().expect("read err in Object<Number>::is_nan").is_nan()
	}

	pub fn is_infinite(&self) -> bool {
		self.data().read().expect("read err in Object<Number>::is_infinite").is_infinite()
	}
}

impl AsRef<Inner> for Number {
	fn as_ref(&self) -> &Inner {
		&self.0
	}
}

impl Deref for Number {
	type Target = Inner;
	fn deref(&self) -> &Inner {
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
		if self.is_nan() {
			write!(f, "NaN")
		} else if self.is_infinite() {
			if self.is_sign_negative() {
				write!(f, "-inf")
			} else {
				write!(f, "inf")
			}
		} else {
			Display::fmt(&self.0, f)
		}
	}
}

mod funcs {
	use super::Number;
	use crate::err::Result;
	use crate::object::{Object, AnyObject};
	use crate::object::types::{quest_funcs, Boolean, Text};

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

	pub fn at_bool(num: &Object<Number>) -> Object<Boolean> {
		Object::new_boolean(!(*num == 0.0 || num.is_nan()))
	}

	pub fn at_num(num: &Object<Number>) -> Object<Number> {
		num.duplicate()
	}

	pub fn at_text(num: &Object<Number>) -> Object<Text> {
		Object::new_text(num.data().read().expect(const_concat!("read error in Number::", quest_funcs::AT_TEXT)).as_ref().to_string())
	}


	pub fn call(num: &Object<Number>, args: &[&AnyObject]) -> Result<AnyObject> {
		num.as_any().call_attr(quest_funcs::MUL, args)
	}

	pub fn add(lhs: &Object<Number>, rhs: &Object<Number>) -> Object<Number> {
		let lhs = **lhs.data().read().expect(data_err![read in Number, quest_funcs::ADD]);
		let rhs = **rhs.data().read().expect(data_err![read in Number, quest_funcs::ADD]);
		Object::new_number(lhs + rhs)
	}

	pub fn sub(lhs: &Object<Number>, rhs: &Object<Number>) -> Object<Number> {
		let lhs = **lhs.data().read().expect(data_err![read in Number, quest_funcs::SUB]);
		let rhs = **rhs.data().read().expect(data_err![read in Number, quest_funcs::SUB]);
		Object::new_number(lhs - rhs)
	}

	pub fn mul(lhs: &Object<Number>, rhs: &Object<Number>) -> Object<Number> {
		let lhs = **lhs.data().read().expect(data_err![read in Number, quest_funcs::MUL]);
		let rhs = **rhs.data().read().expect(data_err![read in Number, quest_funcs::MUL]);
		Object::new_number(lhs * rhs)
	}

	pub fn div(lhs: &Object<Number>, rhs: &Object<Number>) -> Object<Number> {
		let lhs = **lhs.data().read().expect(data_err![read in Number, quest_funcs::DIV]);
		let rhs = **rhs.data().read().expect(data_err![read in Number, quest_funcs::DIV]);
		Object::new_number(lhs / rhs)
	}

	pub fn r#mod(lhs: &Object<Number>, rhs: &Object<Number>) -> Object<Number> {
		let lhs = **lhs.data().read().expect(data_err![read in Number, quest_funcs::MOD]);
		let rhs = **rhs.data().read().expect(data_err![read in Number, quest_funcs::MOD]);
		Object::new_number(lhs % rhs)
	}

	pub fn pow(lhs: &Object<Number>, rhs: &Object<Number>) -> Object<Number> {
		let lhs = **lhs.data().read().expect(data_err![read in Number, quest_funcs::POW]);
		let rhs = **rhs.data().read().expect(data_err![read in Number, quest_funcs::POW]);
		Object::new_number(lhs.powf(rhs))
	}


	pub fn eql(lhs: &Object<Number>, rhs: &Object<Number>) -> Object<Boolean> {
		let lhs = **lhs.data().read().expect(data_err![read in Number, quest_funcs::EQL]);
		let rhs = **rhs.data().read().expect(data_err![read in Number, quest_funcs::EQL]);
		Object::new_boolean(lhs == rhs)
	}

	pub fn neq(lhs: &Object<Number>, rhs: &Object<Number>) -> Object<Boolean> {
		let lhs = **lhs.data().read().expect(data_err![read in Number, quest_funcs::NEQ]);
		let rhs = **rhs.data().read().expect(data_err![read in Number, quest_funcs::NEQ]);
		Object::new_boolean(lhs != rhs)
	}

	pub fn lth(lhs: &Object<Number>, rhs: &Object<Number>) -> Object<Boolean> {
		let lhs = **lhs.data().read().expect(data_err![read in Number, quest_funcs::LTH]);
		let rhs = **rhs.data().read().expect(data_err![read in Number, quest_funcs::LTH]);
		Object::new_boolean(lhs < rhs)
	}
	pub fn gth(lhs: &Object<Number>, rhs: &Object<Number>) -> Object<Boolean> {
		let lhs = **lhs.data().read().expect(data_err![read in Number, quest_funcs::GTH]);
		let rhs = **rhs.data().read().expect(data_err![read in Number, quest_funcs::GTH]);
		Object::new_boolean(lhs > rhs)
	}
	pub fn leq(lhs: &Object<Number>, rhs: &Object<Number>) -> Object<Boolean> {
		let lhs = **lhs.data().read().expect(data_err![read in Number, quest_funcs::LEQ]);
		let rhs = **rhs.data().read().expect(data_err![read in Number, quest_funcs::LEQ]);
		Object::new_boolean(lhs <= rhs)
	}
	pub fn geq(lhs: &Object<Number>, rhs: &Object<Number>) -> Object<Boolean> {
		let lhs = **lhs.data().read().expect(data_err![read in Number, quest_funcs::GEQ]);
		let rhs = **rhs.data().read().expect(data_err![read in Number, quest_funcs::GEQ]);
		Object::new_boolean(lhs >= rhs)
	}

	pub fn cmp(lhs: &Object<Number>, rhs: &Object<Number>) -> AnyObject {
		use std::cmp::{Ord, Ordering};

		let lhs = **lhs.data().read().expect(data_err![read in Number, quest_funcs::CMP]);
		let rhs = **rhs.data().read().expect(data_err![read in Number, quest_funcs::CMP]);
		match lhs.partial_cmp(&rhs) {
			None => Object::new_null(),
			Some(Ordering::Less) => Object::new_number(-1.0),
			Some(Ordering::Equal) => Object::new_number(0.0),
			Some(Ordering::Greater) => Object::new_number(1.0)
		}
	}


	pub fn pos(num: &Object<Number>) -> Object<Number> {
		Object::new_number(num.data().read().expect(data_err![read in Number, quest_funcs::POS]).abs())
	}

	pub fn neg(num: &Object<Number>) -> Object<Number> {
		Object::new_number(-num.data().read().expect(data_err![read in Number, quest_funcs::POS]).as_ref())
	}

}

impl_type! { for Number;
	AT_BOOL => |n, _| Ok(funcs::at_bool(n)),
	AT_NUM => |n, _| Ok(funcs::at_num(n)),
	AT_TEXT => |n, _| Ok(funcs::at_text(n)),

	CALL => funcs::call,

	ADD => |n, a| Ok(funcs::add(n, &getarg!(a[0] @ to_number)?)),
	SUB => |n, a| Ok(funcs::sub(n, &getarg!(a[0] @ to_number)?)),
	MUL => |n, a| Ok(funcs::mul(n, &getarg!(a[0] @ to_number)?)),
	DIV => |n, a| Ok(funcs::div(n, &getarg!(a[0] @ to_number)?)),
	MOD => |n, a| Ok(funcs::r#mod(n, &getarg!(a[0] @ to_number)?)),
	POW => |n, a| Ok(funcs::pow(n, &getarg!(a[0] @ to_number)?)),

	EQL => |n, a| Ok(funcs::eql(n, &getarg!(a[0] @ to_number)?)),
	NEQ => |n, a| Ok(funcs::neq(n, &getarg!(a[0] @ to_number)?)),
	LTH => |n, a| Ok(funcs::lth(n, &getarg!(a[0] @ to_number)?)),
	GTH => |n, a| Ok(funcs::gth(n, &getarg!(a[0] @ to_number)?)),
	LEQ => |n, a| Ok(funcs::leq(n, &getarg!(a[0] @ to_number)?)),
	GEQ => |n, a| Ok(funcs::geq(n, &getarg!(a[0] @ to_number)?)),
	CMP => |n, a| Ok(funcs::cmp(n, &getarg!(a[0] @ to_number)?)),

	POS => |n, _| Ok(funcs::pos(n)),
	NEG => |n, _| Ok(funcs::neg(n)),
}
#[cfg(test)]
mod fn_tests {
	use super::{funcs, consts::*};
	use crate::object::Object;

	macro_rules! n {
		($num:expr) => (Object::new_number($num as f64))
	}

	#[test]
	fn at_bool() {
		assert_eq!(funcs::at_bool(&n![0.0]), false);
		assert_eq!(funcs::at_bool(&n![-0.0]), false);
		assert_eq!(funcs::at_bool(&n![NAN]), false);
		assert_eq!(funcs::at_bool(&n![13.4]), true);
		assert_eq!(funcs::at_bool(&n![INF]), true);
		assert_eq!(funcs::at_bool(&n![PI]), true);
		assert_eq!(funcs::at_bool(&n![E]), true);
		assert_eq!(funcs::at_bool(&n![-123.0]), true);
		assert_eq!(funcs::at_bool(&n![12e49]), true);
	}

	#[test]
	fn at_text() {
		assert_eq!(funcs::at_text(&n![0.0]), "0");
		assert_eq!(funcs::at_text(&n![1.0]), "1");
		assert_eq!(funcs::at_text(&n![-1.0]), "-1");
		assert_eq!(funcs::at_text(&n![123.4]), "123.4");
		assert_eq!(funcs::at_text(&n![-1.23]), "-1.23");
		assert_eq!(funcs::at_text(&n![NAN]), "NaN");
		assert_eq!(funcs::at_text(&n![INF]), "inf");
		assert_eq!(funcs::at_text(&n![NEG_INF]), "-inf");
		assert_eq!(funcs::at_text(&n![-999.0]), "-999");

		// Note: There isn't a specified way large numbers (eg `1e9`) will be displayed
		// Also of note: There isn't a specified length of characters for `E` or `PI`, so those are ignored
	}

	#[test]
	fn at_num() {
		assert_eq!(funcs::at_num(&n![13.4]), 13.4);
		assert_eq!(funcs::at_num(&n![INF]), INF);
		assert_eq!(funcs::at_num(&n![PI]), PI);
		assert_eq!(funcs::at_num(&n![E]), E);
		assert_eq!(funcs::at_num(&n![-123.0]), -123.0);
		assert_eq!(funcs::at_num(&n![12.0]), 12.0);

		let n = Object::new_number(3.0); // best number ever
		assert_obj_duplicated!(n, funcs::at_num(&n));
	}


	#[test]
	fn add() {
		assert_eq!(funcs::add(&n![13.4], &n![-4.0]), 9.4);
		assert_eq!(funcs::add(&n![PI], &n![PI]), 2f64 * PI);
		assert_eq!(funcs::add(&n![E], &n![E]), 2f64 * E);
		assert_eq!(funcs::add(&n![8e9], &n![1e9]), 9e9);

		assert!(funcs::add(&n![NAN], &n![NAN]).is_nan());
		assert!(funcs::add(&n![NAN], &n![123.0]).is_nan());
		assert!(funcs::add(&n![123.0], &n![NAN]).is_nan());
		assert!(funcs::add(&n![NEG_INF], &n![NAN]).is_nan());
		assert!(funcs::add(&n![NEG_INF], &n![INF]).is_nan());

		assert_eq!(funcs::add(&n![NEG_INF], &n![NEG_INF]), NEG_INF);
		assert_eq!(funcs::add(&n![INF], &n![INF]), INF);
	}
	
	#[test]
	fn sub() {
		assert_eq!(funcs::sub(&n![13.4], &n![-4.0]), 17.4);
		assert_eq!(funcs::sub(&n![PI], &n![PI]), 0.0);
		assert_eq!(funcs::sub(&n![E], &n![PI]), E - PI);
		assert_eq!(funcs::sub(&n![9e9], &n![1e9]), 8e9);

		assert!(funcs::sub(&n![NAN], &n![NAN]).is_nan());
		assert!(funcs::sub(&n![INF], &n![NAN]).is_nan());
		assert!(funcs::sub(&n![INF], &n![INF]).is_nan());
		assert!(funcs::sub(&n![INF], &n![NEG_INF]).is_infinite());
	}

	#[test]
	fn mul() {
		assert_eq!(funcs::mul(&n![13.4], &n![-4.0]), -53.6);
		assert_eq!(funcs::mul(&n![PI], &n![PI]), PI * PI);
		assert_eq!(funcs::mul(&n![E], &n![-1e-4]), E * -1e-4);
		assert_eq!(funcs::mul(&n![9e3], &n![8e3]), 7.2e7);

		assert!(funcs::mul(&n![NAN], &n![NAN]).is_nan());
		assert!(funcs::mul(&n![INF], &n![INF]).is_infinite());
		assert!(funcs::mul(&n![NEG_INF], &n![INF]).is_infinite());
		assert!(funcs::mul(&n![NEG_INF], &n![NAN]).is_nan());
	}

	#[test]
	fn div() {
		assert_eq!(funcs::div(&n![13.4], &n![-4.0]), -3.35);
		assert_eq!(funcs::div(&n![PI], &n![E]), PI / E);
		assert_eq!(funcs::div(&n![9e7], &n![-8e-2]), -1.125e9);
		assert_eq!(funcs::div(&n![4.0], &n![1.0]), 4.0);

		assert!(funcs::div(&n![1.0], &n![0.0]).is_infinite());
		assert!(funcs::div(&n![NAN], &n![NAN]).is_nan());
		assert!(funcs::div(&n![INF], &n![INF]).is_nan());
		assert!(funcs::div(&n![INF], &n![NEG_INF]).is_nan());
	}

	#[test]
	fn r#mod() {
		// Note: Rust implements negative modulos differently than other languages:
		// n % d == n - (n/d).to_integer() * d
		// this is especially important for negative numbers

		assert_eq!(funcs::r#mod(&n![13.5], &n![-4.0]), 1.5);
		assert_eq!(funcs::r#mod(&n![13.4], &n![3.1]), 1.0);
		assert_eq!(funcs::r#mod(&n![PI], &n![E]), PI % E);
		assert_eq!(funcs::r#mod(&n![9e19], &n![9.0]), 0.0);
		assert_eq!(funcs::r#mod(&n![-1234.0], &n![39.0]), -25.0);

		assert!(funcs::r#mod(&n![1.0], &n![0.0]).is_nan());
		assert!(funcs::r#mod(&n![NAN], &n![NAN]).is_nan());
		assert!(funcs::r#mod(&n![INF], &n![INF]).is_nan());
		assert!(funcs::r#mod(&n![NEG_INF], &n![INF]).is_nan());
	}

	#[test]
	fn pow() {
		assert_eq!(funcs::pow(&n![13.5], &n![4.0]), 33215.0625);
		assert_eq!(funcs::pow(&n![64.0], &n![0.5]), 8.0);
		assert_eq!(funcs::pow(&n![-0.05], &n![-1.0]), -20.0);
		assert_eq!(funcs::pow(&n![9e9], &n![2.0]), 8.1e19);
		assert_eq!(funcs::pow(&n![NAN], &n![0.0]), 1.0);
		assert_eq!(funcs::pow(&n![INF], &n![0.0]), 1.0);
		assert_eq!(funcs::pow(&n![1234.0], &n![NEG_INF]), 0.0);
		assert_eq!(funcs::pow(&n![1234.0], &n![INF]), INF);
		assert_eq!(funcs::pow(&n![12.0], &n![0.0]), 1.0);

		assert_eq!(funcs::pow(&n![INF], &n![INF]), INF);
		assert_eq!(funcs::pow(&n![NEG_INF], &n![INF]), INF);
		assert_eq!(funcs::pow(&n![INF], &n![NEG_INF]), 0.0);
		assert_eq!(funcs::pow(&n![NEG_INF], &n![NEG_INF]), 0.0);

		assert!(funcs::pow(&n![NAN], &n![NAN]).is_nan());
		assert!(funcs::pow(&n![NAN], &n![INF]).is_nan());
		assert!(funcs::pow(&n![NAN], &n![NEG_INF]).is_nan());
	}

	#[test]
	fn eql() {
		assert_eq!(funcs::eql(&n![13.5], &n![13.5]), true);
		assert_eq!(funcs::eql(&n![-123.0], &n![-123.0]), true);
		assert_eq!(funcs::eql(&n![123.0], &n![-123.0]), false);
		assert_eq!(funcs::eql(&n![-0.0], &n![0.0]), true);
		assert_eq!(funcs::eql(&n![9.123e9], &n![-9.123e9]), false);

		assert_eq!(funcs::eql(&n![-1.0], &n![-2.0]), false);
		assert_eq!(funcs::eql(&n![-1.0], &n![-1.0]),  true);
		assert_eq!(funcs::eql(&n![-1.0], &n![ 0.0]), false);
		assert_eq!(funcs::eql(&n![-1.0], &n![ 1.0]), false);
		assert_eq!(funcs::eql(&n![ 0.0], &n![-1.0]), false);
		assert_eq!(funcs::eql(&n![ 0.0], &n![ 0.0]),  true);
		assert_eq!(funcs::eql(&n![ 0.0], &n![ 1.0]), false);
		assert_eq!(funcs::eql(&n![ 1.0], &n![-1.0]), false);
		assert_eq!(funcs::eql(&n![ 1.0], &n![ 0.0]), false);
		assert_eq!(funcs::eql(&n![ 1.0], &n![ 1.0]),  true);
		assert_eq!(funcs::eql(&n![ 1.0], &n![ 2.0]), false);

		assert_eq!(funcs::eql(&n![NEG_INF], &n![NEG_INF+1.0]), true);
		assert_eq!(funcs::eql(&n![INF], &n![INF-1.0]), true);

		assert_eq!(funcs::eql(&n![NAN], &n![NEG_INF]), false);
		assert_eq!(funcs::eql(&n![NAN], &n![INF]), false);
		assert_eq!(funcs::eql(&n![NAN], &n![NAN]), false);
		assert_eq!(funcs::eql(&n![NAN], &n![0.0]), false);
		assert_eq!(funcs::eql(&n![NAN], &n![-1.0]), false);
		assert_eq!(funcs::eql(&n![NAN], &n![1.0]), false);

		assert_eq!(funcs::eql(&n![NEG_INF], &n![NEG_INF]), true);
		assert_eq!(funcs::eql(&n![NEG_INF], &n![INF]), false);
		assert_eq!(funcs::eql(&n![NEG_INF], &n![NAN]), false);
		assert_eq!(funcs::eql(&n![NEG_INF], &n![0.0]), false);
		assert_eq!(funcs::eql(&n![NEG_INF], &n![-1.0]), false);
		assert_eq!(funcs::eql(&n![NEG_INF], &n![1.0]), false);

		assert_eq!(funcs::eql(&n![INF], &n![NEG_INF]), false);
		assert_eq!(funcs::eql(&n![INF], &n![INF]), true);
		assert_eq!(funcs::eql(&n![INF], &n![NAN]), false);
		assert_eq!(funcs::eql(&n![INF], &n![0.0]), false);
		assert_eq!(funcs::eql(&n![INF], &n![-1.0]), false);
		assert_eq!(funcs::eql(&n![INF], &n![1.0]), false);
	}

	#[test]
	fn neq() {
		assert_eq!(funcs::neq(&n![13.5], &n![13.5]), false);
		assert_eq!(funcs::neq(&n![-123.0], &n![-123.0]), false);
		assert_eq!(funcs::neq(&n![123.0], &n![-123.0]), true);
		assert_eq!(funcs::neq(&n![-0.0], &n![0.0]), false);
		assert_eq!(funcs::neq(&n![9.123e9], &n![-9.123e9]), true);

		assert_eq!(funcs::neq(&n![-1.0], &n![-2.0]),  true);
		assert_eq!(funcs::neq(&n![-1.0], &n![-1.0]), false);
		assert_eq!(funcs::neq(&n![-1.0], &n![ 0.0]),  true);
		assert_eq!(funcs::neq(&n![-1.0], &n![ 1.0]),  true);
		assert_eq!(funcs::neq(&n![ 0.0], &n![-1.0]),  true);
		assert_eq!(funcs::neq(&n![ 0.0], &n![ 0.0]), false);
		assert_eq!(funcs::neq(&n![ 0.0], &n![ 1.0]),  true);
		assert_eq!(funcs::neq(&n![ 1.0], &n![-1.0]),  true);
		assert_eq!(funcs::neq(&n![ 1.0], &n![ 0.0]),  true);
		assert_eq!(funcs::neq(&n![ 1.0], &n![ 1.0]), false);
		assert_eq!(funcs::neq(&n![ 1.0], &n![ 2.0]),  true);

		assert_eq!(funcs::neq(&n![NAN], &n![NEG_INF]), true);
		assert_eq!(funcs::neq(&n![NAN], &n![INF]), true);
		assert_eq!(funcs::neq(&n![NAN], &n![NAN]), true);
		assert_eq!(funcs::neq(&n![NAN], &n![0.0]), true);
		assert_eq!(funcs::neq(&n![NAN], &n![-1.0]), true);
		assert_eq!(funcs::neq(&n![NAN], &n![1.0]), true);

		assert_eq!(funcs::neq(&n![NEG_INF], &n![NEG_INF]), false);
		assert_eq!(funcs::neq(&n![NEG_INF], &n![INF]), true);
		assert_eq!(funcs::neq(&n![NEG_INF], &n![NAN]), true);
		assert_eq!(funcs::neq(&n![NEG_INF], &n![0.0]), true);
		assert_eq!(funcs::neq(&n![NEG_INF], &n![-1.0]), true);
		assert_eq!(funcs::neq(&n![NEG_INF], &n![1.0]), true);

		assert_eq!(funcs::neq(&n![INF], &n![NEG_INF]), true);
		assert_eq!(funcs::neq(&n![INF], &n![INF]), false);
		assert_eq!(funcs::neq(&n![INF], &n![NAN]), true);
		assert_eq!(funcs::neq(&n![INF], &n![0.0]), true);
		assert_eq!(funcs::neq(&n![INF], &n![-1.0]), true);
		assert_eq!(funcs::neq(&n![INF], &n![1.0]), true);
	}

	#[test]
	fn lth() {
		assert_eq!(funcs::lth(&n![13.5], &n![4.0]), false);
		assert_eq!(funcs::lth(&n![0.5], &n![64.0]), true);
		assert_eq!(funcs::lth(&n![-0.05], &n![-1.0]), false);
		assert_eq!(funcs::lth(&n![2.0], &n![9e9]), true);
		assert_eq!(funcs::lth(&n![9e9], &n![9e9]), false);

		assert_eq!(funcs::lth(&n![-1.0], &n![-2.0]), false);
		assert_eq!(funcs::lth(&n![-1.0], &n![-1.0]), false);
		assert_eq!(funcs::lth(&n![-1.0], &n![ 0.0]),  true);
		assert_eq!(funcs::lth(&n![-1.0], &n![ 1.0]),  true);
		assert_eq!(funcs::lth(&n![ 0.0], &n![-1.0]), false);
		assert_eq!(funcs::lth(&n![ 0.0], &n![ 0.0]), false);
		assert_eq!(funcs::lth(&n![ 0.0], &n![ 1.0]),  true);
		assert_eq!(funcs::lth(&n![ 1.0], &n![-1.0]), false);
		assert_eq!(funcs::lth(&n![ 1.0], &n![ 0.0]), false);
		assert_eq!(funcs::lth(&n![ 1.0], &n![ 1.0]), false);
		assert_eq!(funcs::lth(&n![ 1.0], &n![ 2.0]),  true);

		assert_eq!(funcs::lth(&n![NAN], &n![NEG_INF]), false);
		assert_eq!(funcs::lth(&n![NAN], &n![INF]), false);
		assert_eq!(funcs::lth(&n![NAN], &n![NAN]), false);
		assert_eq!(funcs::lth(&n![NAN], &n![0.0]), false);
		assert_eq!(funcs::lth(&n![NAN], &n![-1.0]), false);
		assert_eq!(funcs::lth(&n![NAN], &n![1.0]), false);

		assert_eq!(funcs::lth(&n![NEG_INF], &n![NEG_INF]), false);
		assert_eq!(funcs::lth(&n![NEG_INF], &n![INF]), true);
		assert_eq!(funcs::lth(&n![NEG_INF], &n![NAN]), false);
		assert_eq!(funcs::lth(&n![NEG_INF], &n![0.0]), true);
		assert_eq!(funcs::lth(&n![NEG_INF], &n![-1.0]), true);
		assert_eq!(funcs::lth(&n![NEG_INF], &n![1.0]), true);

		assert_eq!(funcs::lth(&n![INF], &n![NEG_INF]), false);
		assert_eq!(funcs::lth(&n![INF], &n![INF]), false);
		assert_eq!(funcs::lth(&n![INF], &n![NAN]), false);
		assert_eq!(funcs::lth(&n![INF], &n![0.0]), false);
		assert_eq!(funcs::lth(&n![INF], &n![-1.0]), false);
		assert_eq!(funcs::lth(&n![INF], &n![1.0]), false);
	}

	#[test]
	fn leq() {
		assert_eq!(funcs::leq(&n![13.5], &n![4.0]), false);
		assert_eq!(funcs::leq(&n![0.5], &n![64.0]), true);
		assert_eq!(funcs::leq(&n![-0.05], &n![-1.0]), false);
		assert_eq!(funcs::leq(&n![2.0], &n![9e9]), true);
		assert_eq!(funcs::leq(&n![9e9], &n![9e9]), true);

		assert_eq!(funcs::leq(&n![-1.0], &n![-2.0]), false);
		assert_eq!(funcs::leq(&n![-1.0], &n![-1.0]),  true);
		assert_eq!(funcs::leq(&n![-1.0], &n![ 0.0]),  true);
		assert_eq!(funcs::leq(&n![-1.0], &n![ 1.0]),  true);
		assert_eq!(funcs::leq(&n![ 0.0], &n![-1.0]), false);
		assert_eq!(funcs::leq(&n![ 0.0], &n![ 0.0]),  true);
		assert_eq!(funcs::leq(&n![ 0.0], &n![ 1.0]),  true);
		assert_eq!(funcs::leq(&n![ 1.0], &n![-1.0]), false);
		assert_eq!(funcs::leq(&n![ 1.0], &n![ 0.0]), false);
		assert_eq!(funcs::leq(&n![ 1.0], &n![ 1.0]),  true);
		assert_eq!(funcs::leq(&n![ 1.0], &n![ 2.0]),  true);

		assert_eq!(funcs::leq(&n![NAN], &n![NEG_INF]), false);
		assert_eq!(funcs::leq(&n![NAN], &n![INF]), false);
		assert_eq!(funcs::leq(&n![NAN], &n![NAN]), false);
		assert_eq!(funcs::leq(&n![NAN], &n![0.0]), false);
		assert_eq!(funcs::leq(&n![NAN], &n![-1.0]), false);
		assert_eq!(funcs::leq(&n![NAN], &n![1.0]), false);

		assert_eq!(funcs::leq(&n![NEG_INF], &n![NEG_INF]), true);
		assert_eq!(funcs::leq(&n![NEG_INF], &n![INF]), true);
		assert_eq!(funcs::leq(&n![NEG_INF], &n![NAN]), false);
		assert_eq!(funcs::leq(&n![NEG_INF], &n![0.0]), true);
		assert_eq!(funcs::leq(&n![NEG_INF], &n![-1.0]), true);
		assert_eq!(funcs::leq(&n![NEG_INF], &n![1.0]), true);

		assert_eq!(funcs::leq(&n![INF], &n![NEG_INF]), false);
		assert_eq!(funcs::leq(&n![INF], &n![INF]), true);
		assert_eq!(funcs::leq(&n![INF], &n![NAN]), false);
		assert_eq!(funcs::leq(&n![INF], &n![0.0]), false);
		assert_eq!(funcs::leq(&n![INF], &n![-1.0]), false);
		assert_eq!(funcs::leq(&n![INF], &n![1.0]), false);
	}

	#[test]
	fn gth() {
		assert_eq!(funcs::gth(&n![13.5], &n![4.0]), true);
		assert_eq!(funcs::gth(&n![0.5], &n![64.0]), false);
		assert_eq!(funcs::gth(&n![-0.05], &n![-1.0]), true);
		assert_eq!(funcs::gth(&n![2.0], &n![9e9]), false);
		assert_eq!(funcs::gth(&n![9e9], &n![9e9]), false);

		assert_eq!(funcs::gth(&n![-1.0], &n![-2.0]),  true);
		assert_eq!(funcs::gth(&n![-1.0], &n![-1.0]), false);
		assert_eq!(funcs::gth(&n![-1.0], &n![ 0.0]), false);
		assert_eq!(funcs::gth(&n![-1.0], &n![ 1.0]), false);
		assert_eq!(funcs::gth(&n![ 0.0], &n![-1.0]),  true);
		assert_eq!(funcs::gth(&n![ 0.0], &n![ 0.0]), false);
		assert_eq!(funcs::gth(&n![ 0.0], &n![ 1.0]), false);
		assert_eq!(funcs::gth(&n![ 1.0], &n![-1.0]),  true);
		assert_eq!(funcs::gth(&n![ 1.0], &n![ 0.0]),  true);
		assert_eq!(funcs::gth(&n![ 1.0], &n![ 1.0]), false);
		assert_eq!(funcs::gth(&n![ 1.0], &n![ 2.0]), false);

		assert_eq!(funcs::gth(&n![NAN], &n![NEG_INF]), false);
		assert_eq!(funcs::gth(&n![NAN], &n![INF]), false);
		assert_eq!(funcs::gth(&n![NAN], &n![NAN]), false);
		assert_eq!(funcs::gth(&n![NAN], &n![0.0]), false);
		assert_eq!(funcs::gth(&n![NAN], &n![-1.0]), false);
		assert_eq!(funcs::gth(&n![NAN], &n![1.0]), false);

		assert_eq!(funcs::gth(&n![NEG_INF], &n![NEG_INF]), false);
		assert_eq!(funcs::gth(&n![NEG_INF], &n![INF]), false);
		assert_eq!(funcs::gth(&n![NEG_INF], &n![NAN]), false);
		assert_eq!(funcs::gth(&n![NEG_INF], &n![0.0]), false);
		assert_eq!(funcs::gth(&n![NEG_INF], &n![-1.0]), false);
		assert_eq!(funcs::gth(&n![NEG_INF], &n![1.0]), false);

		assert_eq!(funcs::gth(&n![INF], &n![NEG_INF]), true);
		assert_eq!(funcs::gth(&n![INF], &n![INF]), false);
		assert_eq!(funcs::gth(&n![INF], &n![NAN]), false);
		assert_eq!(funcs::gth(&n![INF], &n![0.0]), true);
		assert_eq!(funcs::gth(&n![INF], &n![-1.0]), true);
		assert_eq!(funcs::gth(&n![INF], &n![1.0]), true);
	}

	#[test]
	fn geq() {
		assert_eq!(funcs::geq(&n![13.5], &n![4.0]), true);
		assert_eq!(funcs::geq(&n![0.5], &n![64.0]), false);
		assert_eq!(funcs::geq(&n![-0.05], &n![-1.0]), true);
		assert_eq!(funcs::geq(&n![2.0], &n![9e9]), false);
		assert_eq!(funcs::geq(&n![9e9], &n![9e9]), true);

		assert_eq!(funcs::geq(&n![-1.0], &n![-2.0]),  true);
		assert_eq!(funcs::geq(&n![-1.0], &n![-1.0]),  true);
		assert_eq!(funcs::geq(&n![-1.0], &n![ 0.0]), false);
		assert_eq!(funcs::geq(&n![-1.0], &n![ 1.0]), false);
		assert_eq!(funcs::geq(&n![ 0.0], &n![-1.0]),  true);
		assert_eq!(funcs::geq(&n![ 0.0], &n![ 0.0]),  true);
		assert_eq!(funcs::geq(&n![ 0.0], &n![ 1.0]), false);
		assert_eq!(funcs::geq(&n![ 1.0], &n![-1.0]),  true);
		assert_eq!(funcs::geq(&n![ 1.0], &n![ 0.0]),  true);
		assert_eq!(funcs::geq(&n![ 1.0], &n![ 1.0]),  true);
		assert_eq!(funcs::geq(&n![ 1.0], &n![ 2.0]), false);

		assert_eq!(funcs::geq(&n![NAN], &n![NEG_INF]), false);
		assert_eq!(funcs::geq(&n![NAN], &n![NEG_INF+1.0]), false);
		assert_eq!(funcs::geq(&n![NAN], &n![INF]), false);
		assert_eq!(funcs::geq(&n![NAN], &n![NAN]), false);
		assert_eq!(funcs::geq(&n![NAN], &n![0.0]), false);
		assert_eq!(funcs::geq(&n![NAN], &n![-1.0]), false);
		assert_eq!(funcs::geq(&n![NAN], &n![1.0]), false);

		assert_eq!(funcs::geq(&n![NEG_INF], &n![NEG_INF]), true);
		assert_eq!(funcs::geq(&n![NEG_INF], &n![INF]), false);
		assert_eq!(funcs::geq(&n![NEG_INF], &n![NAN]), false);
		assert_eq!(funcs::geq(&n![NEG_INF], &n![0.0]), false);
		assert_eq!(funcs::geq(&n![NEG_INF], &n![-1.0]), false);
		assert_eq!(funcs::geq(&n![NEG_INF], &n![1.0]), false);

		assert_eq!(funcs::geq(&n![INF], &n![NEG_INF]), true);
		assert_eq!(funcs::geq(&n![INF], &n![INF]), true);
		assert_eq!(funcs::geq(&n![INF], &n![NAN]), false);
		assert_eq!(funcs::geq(&n![INF], &n![0.0]), true);
		assert_eq!(funcs::geq(&n![INF], &n![-1.0]), true);
		assert_eq!(funcs::geq(&n![INF], &n![1.0]), true);
	}

	#[test]
	fn cmp() {
		use super::Number;
		assert_eq!(funcs::cmp(&n![13.5], &n![4.0]).downcast::<Number>().unwrap(), 1.0);
		assert_eq!(funcs::cmp(&n![0.5], &n![64.0]).downcast::<Number>().unwrap(), -1.0);
		assert_eq!(funcs::cmp(&n![-0.05], &n![-1.0]).downcast::<Number>().unwrap(), 1.0);
		assert_eq!(funcs::cmp(&n![2.0], &n![9e9]).downcast::<Number>().unwrap(), -1.0);
		assert_eq!(funcs::cmp(&n![9e9], &n![9e9]).downcast::<Number>().unwrap(), 0.0);

		assert_eq!(funcs::cmp(&n![-1.0], &n![-1.0]).downcast::<Number>().unwrap(),  0.0);
		assert_eq!(funcs::cmp(&n![-1.0], &n![ 0.0]).downcast::<Number>().unwrap(), -1.0);
		assert_eq!(funcs::cmp(&n![-1.0], &n![ 1.0]).downcast::<Number>().unwrap(), -1.0);
		assert_eq!(funcs::cmp(&n![ 0.0], &n![-1.0]).downcast::<Number>().unwrap(),  1.0);
		assert_eq!(funcs::cmp(&n![ 0.0], &n![ 0.0]).downcast::<Number>().unwrap(),  0.0);
		assert_eq!(funcs::cmp(&n![ 0.0], &n![ 1.0]).downcast::<Number>().unwrap(), -1.0);
		assert_eq!(funcs::cmp(&n![ 1.0], &n![-1.0]).downcast::<Number>().unwrap(),  1.0);
		assert_eq!(funcs::cmp(&n![ 1.0], &n![ 0.0]).downcast::<Number>().unwrap(),  1.0);
		assert_eq!(funcs::cmp(&n![ 1.0], &n![ 1.0]).downcast::<Number>().unwrap(),  0.0);

		assert!(funcs::cmp(&n![NAN], &n![NEG_INF]).is_null());
		assert!(funcs::cmp(&n![NAN], &n![INF]).is_null());
		assert!(funcs::cmp(&n![NAN], &n![NAN]).is_null());
		assert!(funcs::cmp(&n![NAN], &n![0.0]).is_null());
		assert!(funcs::cmp(&n![NAN], &n![-1.0]).is_null());
		assert!(funcs::cmp(&n![NAN], &n![1.0]).is_null());

		assert_eq!(funcs::cmp(&n![NEG_INF], &n![NEG_INF]).downcast::<Number>().unwrap(), 0.0);
		assert_eq!(funcs::cmp(&n![NEG_INF], &n![INF]).downcast::<Number>().unwrap(), -1.0);
		assert!(funcs::cmp(&n![NEG_INF], &n![NAN]).is_null());
		assert_eq!(funcs::cmp(&n![NEG_INF], &n![0.0]).downcast::<Number>().unwrap(), -1.0);
		assert_eq!(funcs::cmp(&n![NEG_INF], &n![-1.0]).downcast::<Number>().unwrap(), -1.0);
		assert_eq!(funcs::cmp(&n![NEG_INF], &n![1.0]).downcast::<Number>().unwrap(), -1.0);

		assert_eq!(funcs::cmp(&n![INF], &n![NEG_INF]).downcast::<Number>().unwrap(), 1.0);
		assert_eq!(funcs::cmp(&n![INF], &n![INF]).downcast::<Number>().unwrap(), 0.0);
		assert!(funcs::cmp(&n![INF], &n![NAN]).is_null());
		assert_eq!(funcs::cmp(&n![INF], &n![0.0]).downcast::<Number>().unwrap(), 1.0);
		assert_eq!(funcs::cmp(&n![INF], &n![-1.0]).downcast::<Number>().unwrap(), 1.0);
		assert_eq!(funcs::cmp(&n![INF], &n![1.0]).downcast::<Number>().unwrap(), 1.0);
	}

	#[test]
	fn pos() {
		assert_eq!(funcs::pos(&n![-9e9]), 9e9);
		assert_eq!(funcs::pos(&n![-2.0]), 2.0);
		assert_eq!(funcs::pos(&n![-1.0]), 1.0);
		assert_eq!(funcs::pos(&n![-0.5]), 0.5);
		assert_eq!(funcs::pos(&n![ 0.0]), 0.0);
		assert_eq!(funcs::pos(&n![ 1.0]), 1.0);
		assert_eq!(funcs::pos(&n![ 2.0]), 2.0);
		assert!(funcs::pos(&n![NAN]).is_nan());
		assert_eq!(funcs::pos(&n![NEG_INF]), INF);
		assert_eq!(funcs::pos(&n![INF]), INF);

		let n = Object::new_number(3.14);
		assert_obj_duplicated!(n, funcs::pos(&n));
	}
}


#[cfg(test)]
mod integration {
	use super::{funcs, consts::*, Number};
	use crate::object::Object;
	use crate::object::types::{Boolean, Text};
	use crate::object::types::quest_funcs::{
		AT_BOOL, AT_TEXT, AT_NUM,
		ADD, SUB, MUL, DIV, MOD, POW, 
		EQL, NEQ, LTH, LEQ, GTH, GEQ, CMP,
		POS, NEG
	};
	use crate::err::Result;

	macro_rules! _n_ {
		($num:expr) => (Object::new_number($num as f64))
	}

	macro_rules! n {
		($num:expr) => (Object::new_number($num as f64).as_any())
	}

	macro_rules! assert_call_eq {
		(SINGLE; $attr:ident $func:ident $ret:ty { $($n:tt)* }) => {
			{
				$({
					let ref n = Object::new_number($n);
					assert_eq!(n.as_any().call_attr($attr, &[])?.downcast_or_err::<$ret>()?, funcs::$func(n));
				})*
			}
		};

		(DOUBLE; $attr:ident $func:ident $ret:ty { $first:tt $firstarg:tt $(,$n:tt $narg:tt)* }) => {
			{
				let ref first = Object::new_number($first);
				let ref firstarg = Object::new_number($firstarg);
				assert_eq!(first.as_any().call_attr($attr, &[&firstarg.as_any()])?.downcast_or_err::<$ret>()?, funcs::$func(first, firstarg));
				assert_param_missing!(first.as_any().call_attr($attr, &[]));

				$({
					let ref n = Object::new_number($n);
					let ref a = Object::new_number($narg);
					assert_eq!(n.as_any().call_attr($attr, &[&a.as_any()])?.downcast_or_err::<$ret>()?, funcs::$func(n, a));
				})*
			}
		}
	}
	macro_rules! assert_call_nan {
		(DOUBLE; $attr:ident $func:ident { $($n:tt $narg:tt),* }) => ({
			$(
				assert!(Object::new_number($n)
					.as_any()
					.call_attr($attr, &[&Object::new_number($narg).as_any()])?
					.downcast_or_err::<Number>()?
					.is_nan()
				);
			)*
		})
	}


	#[test]
	fn at_bool() -> Result<()> {
		assert_call_eq!(SINGLE; AT_BOOL at_bool Boolean {
			0.0 (-0.0) NAN 13.4 INF PI E (-123.0) 12e49
		});
		Ok(())
	}

	#[test]
	fn at_text() -> Result<()> {
		assert_call_eq!(SINGLE; AT_TEXT at_text Text {
			0.0 1.0 (-1.0) 12.34 (-1.23) NAN INF NEG_INF (-990.0)
		});
		Ok(())
	}

	#[test]
	fn at_num() -> Result<()> {
		assert_call_eq!(SINGLE; AT_NUM at_num Number {
			13.4 INF PI E (-123.0) 12.0
		});
		Ok(())
	}


	#[test]
	fn add() -> Result<()> {
		assert_call_eq!(DOUBLE; ADD add Number {
			13.4 (-4.0), PI PI, E E, 8e9 1e9,
			NEG_INF NEG_INF, INF INF
		});

		assert_call_nan!(DOUBLE; ADD add {
			NAN NAN, 123.0 NAN, NEG_INF NAN,
			INF NEG_INF, NEG_INF INF
		});

		Ok(())
	}
	
	#[test]
	fn sub() -> Result<()> {
		assert_call_eq!(DOUBLE; SUB sub Number {
			13.4 (-4.0), PI PI, E PI, 9e9 1e9,
			INF NEG_INF, NEG_INF INF
		});

		assert_call_nan!(DOUBLE; SUB sub {
			NAN NAN, 123.0 NAN, NEG_INF NAN,
			INF INF, NEG_INF NEG_INF
		});

		Ok(())
	}

	#[test]
	fn mul() -> Result<()> {
		assert_call_eq!(DOUBLE; MUL mul Number {
			13.4 (-4.0), PI PI, E (-1e-4), 9e3 8e3,
			INF INF, INF NEG_INF, NEG_INF INF, NEG_INF NEG_INF
		});

		assert_call_nan!(DOUBLE; MUL mul {
			NAN NAN, 123.0 NAN, INF NAN
		});

		Ok(())
	}

	#[test]
	fn div() -> Result<()> {
		assert_call_eq!(DOUBLE; DIV div Number {
			13.4 (-4.0), PI E, 9e7 (-8e-2), 4.0 1.0,
			1.0 0.0
		});

		assert_call_nan!(DOUBLE; DIV div {
			NAN NAN, 123.0 NAN, INF NAN,
			INF INF, INF NEG_INF, NEG_INF INF, NEG_INF NEG_INF
		});
		Ok(())
	}

	#[test]
	fn r#mod() -> Result<()> {
		// Note: Rust implements negative modulos differently than other languages:
		// n % d == n - (n/d).to_integer() * d
		// this is especially important for negative numbers
		assert_call_eq!(DOUBLE; MOD r#mod Number {
			13.5 (-4.0), 13.4 3.1, PI E, 9e19 9.0, (-1234.0) 39.0
		});

		assert_call_nan!(DOUBLE; MOD r#mod {
			NAN NAN, 123.0 NAN, INF NAN, 1.0 0.0,
			INF INF, INF NEG_INF, NEG_INF INF, NEG_INF NEG_INF
		});

		Ok(())
	}

	#[test]
	fn pow() -> Result<()> {
		assert_call_eq!(DOUBLE; POW pow Number {
			13.5 4.0, 64.0 0.5, (-0.05) (-1.0),
			9e9 2.0, NAN 0.0, INF 0.0, 12.0 (-2.0), 64.0 0.5,
			1234.0 NEG_INF, 1234.0 INF, 12.0 0.0,
			INF INF, NEG_INF INF, INF NEG_INF, NEG_INF NEG_INF
		});

		assert_call_nan!(DOUBLE; POW pow {
			NAN NAN, 123.0 NAN, NAN INF, NAN NEG_INF,
			INF NAN, NEG_INF NAN
		});

		Ok(())
	}

	#[test]
	fn eql() -> Result<()> {
		assert_eq!(funcs::eql(&_n_![13.5], &_n_![13.5]), true);
		assert_eq!(funcs::eql(&_n_![-123.0], &_n_![-123.0]), true);
		assert_eq!(funcs::eql(&_n_![123.0], &_n_![-123.0]), false);
		assert_eq!(funcs::eql(&_n_![-0.0], &_n_![0.0]), true);
		assert_eq!(funcs::eql(&_n_![9.123e9], &_n_![-9.123e9]), false);

		assert_eq!(funcs::eql(&_n_![-1.0], &_n_![-2.0]), false);
		assert_eq!(funcs::eql(&_n_![-1.0], &_n_![-1.0]),  true);
		assert_eq!(funcs::eql(&_n_![-1.0], &_n_![ 0.0]), false);
		assert_eq!(funcs::eql(&_n_![-1.0], &_n_![ 1.0]), false);
		assert_eq!(funcs::eql(&_n_![ 0.0], &_n_![-1.0]), false);
		assert_eq!(funcs::eql(&_n_![ 0.0], &_n_![ 0.0]),  true);
		assert_eq!(funcs::eql(&_n_![ 0.0], &_n_![ 1.0]), false);
		assert_eq!(funcs::eql(&_n_![ 1.0], &_n_![-1.0]), false);
		assert_eq!(funcs::eql(&_n_![ 1.0], &_n_![ 0.0]), false);
		assert_eq!(funcs::eql(&_n_![ 1.0], &_n_![ 1.0]),  true);
		assert_eq!(funcs::eql(&_n_![ 1.0], &_n_![ 2.0]), false);

		assert_eq!(funcs::eql(&_n_![NEG_INF], &_n_![NEG_INF+1.0]), true);
		assert_eq!(funcs::eql(&_n_![INF], &_n_![INF-1.0]), true);

		assert_eq!(funcs::eql(&_n_![NAN], &_n_![NEG_INF]), false);
		assert_eq!(funcs::eql(&_n_![NAN], &_n_![INF]), false);
		assert_eq!(funcs::eql(&_n_![NAN], &_n_![NAN]), false);
		assert_eq!(funcs::eql(&_n_![NAN], &_n_![0.0]), false);
		assert_eq!(funcs::eql(&_n_![NAN], &_n_![-1.0]), false);
		assert_eq!(funcs::eql(&_n_![NAN], &_n_![1.0]), false);

		assert_eq!(funcs::eql(&_n_![NEG_INF], &_n_![NEG_INF]), true);
		assert_eq!(funcs::eql(&_n_![NEG_INF], &_n_![INF]), false);
		assert_eq!(funcs::eql(&_n_![NEG_INF], &_n_![NAN]), false);
		assert_eq!(funcs::eql(&_n_![NEG_INF], &_n_![0.0]), false);
		assert_eq!(funcs::eql(&_n_![NEG_INF], &_n_![-1.0]), false);
		assert_eq!(funcs::eql(&_n_![NEG_INF], &_n_![1.0]), false);

		assert_eq!(funcs::eql(&_n_![INF], &_n_![NEG_INF]), false);
		assert_eq!(funcs::eql(&_n_![INF], &_n_![INF]), true);
		assert_eq!(funcs::eql(&_n_![INF], &_n_![NAN]), false);
		assert_eq!(funcs::eql(&_n_![INF], &_n_![0.0]), false);
		assert_eq!(funcs::eql(&_n_![INF], &_n_![-1.0]), false);
		assert_eq!(funcs::eql(&_n_![INF], &_n_![1.0]), false);
		Ok(())
	}

	#[test]
	fn neq() -> Result<()> {
		assert_eq!(funcs::neq(&_n_![13.5], &_n_![13.5]), false);
		assert_eq!(funcs::neq(&_n_![-123.0], &_n_![-123.0]), false);
		assert_eq!(funcs::neq(&_n_![123.0], &_n_![-123.0]), true);
		assert_eq!(funcs::neq(&_n_![-0.0], &_n_![0.0]), false);
		assert_eq!(funcs::neq(&_n_![9.123e9], &_n_![-9.123e9]), true);

		assert_eq!(funcs::neq(&_n_![-1.0], &_n_![-2.0]),  true);
		assert_eq!(funcs::neq(&_n_![-1.0], &_n_![-1.0]), false);
		assert_eq!(funcs::neq(&_n_![-1.0], &_n_![ 0.0]),  true);
		assert_eq!(funcs::neq(&_n_![-1.0], &_n_![ 1.0]),  true);
		assert_eq!(funcs::neq(&_n_![ 0.0], &_n_![-1.0]),  true);
		assert_eq!(funcs::neq(&_n_![ 0.0], &_n_![ 0.0]), false);
		assert_eq!(funcs::neq(&_n_![ 0.0], &_n_![ 1.0]),  true);
		assert_eq!(funcs::neq(&_n_![ 1.0], &_n_![-1.0]),  true);
		assert_eq!(funcs::neq(&_n_![ 1.0], &_n_![ 0.0]),  true);
		assert_eq!(funcs::neq(&_n_![ 1.0], &_n_![ 1.0]), false);
		assert_eq!(funcs::neq(&_n_![ 1.0], &_n_![ 2.0]),  true);

		assert_eq!(funcs::neq(&_n_![NAN], &_n_![NEG_INF]), true);
		assert_eq!(funcs::neq(&_n_![NAN], &_n_![INF]), true);
		assert_eq!(funcs::neq(&_n_![NAN], &_n_![NAN]), true);
		assert_eq!(funcs::neq(&_n_![NAN], &_n_![0.0]), true);
		assert_eq!(funcs::neq(&_n_![NAN], &_n_![-1.0]), true);
		assert_eq!(funcs::neq(&_n_![NAN], &_n_![1.0]), true);

		assert_eq!(funcs::neq(&_n_![NEG_INF], &_n_![NEG_INF]), false);
		assert_eq!(funcs::neq(&_n_![NEG_INF], &_n_![INF]), true);
		assert_eq!(funcs::neq(&_n_![NEG_INF], &_n_![NAN]), true);
		assert_eq!(funcs::neq(&_n_![NEG_INF], &_n_![0.0]), true);
		assert_eq!(funcs::neq(&_n_![NEG_INF], &_n_![-1.0]), true);
		assert_eq!(funcs::neq(&_n_![NEG_INF], &_n_![1.0]), true);

		assert_eq!(funcs::neq(&_n_![INF], &_n_![NEG_INF]), true);
		assert_eq!(funcs::neq(&_n_![INF], &_n_![INF]), false);
		assert_eq!(funcs::neq(&_n_![INF], &_n_![NAN]), true);
		assert_eq!(funcs::neq(&_n_![INF], &_n_![0.0]), true);
		assert_eq!(funcs::neq(&_n_![INF], &_n_![-1.0]), true);
		assert_eq!(funcs::neq(&_n_![INF], &_n_![1.0]), true);
		Ok(())
	}

	#[test]
	fn lth() -> Result<()> {
		assert_eq!(funcs::lth(&_n_![13.5], &_n_![4.0]), false);
		assert_eq!(funcs::lth(&_n_![0.5], &_n_![64.0]), true);
		assert_eq!(funcs::lth(&_n_![-0.05], &_n_![-1.0]), false);
		assert_eq!(funcs::lth(&_n_![2.0], &_n_![9e9]), true);
		assert_eq!(funcs::lth(&_n_![9e9], &_n_![9e9]), false);

		assert_eq!(funcs::lth(&_n_![-1.0], &_n_![-2.0]), false);
		assert_eq!(funcs::lth(&_n_![-1.0], &_n_![-1.0]), false);
		assert_eq!(funcs::lth(&_n_![-1.0], &_n_![ 0.0]),  true);
		assert_eq!(funcs::lth(&_n_![-1.0], &_n_![ 1.0]),  true);
		assert_eq!(funcs::lth(&_n_![ 0.0], &_n_![-1.0]), false);
		assert_eq!(funcs::lth(&_n_![ 0.0], &_n_![ 0.0]), false);
		assert_eq!(funcs::lth(&_n_![ 0.0], &_n_![ 1.0]),  true);
		assert_eq!(funcs::lth(&_n_![ 1.0], &_n_![-1.0]), false);
		assert_eq!(funcs::lth(&_n_![ 1.0], &_n_![ 0.0]), false);
		assert_eq!(funcs::lth(&_n_![ 1.0], &_n_![ 1.0]), false);
		assert_eq!(funcs::lth(&_n_![ 1.0], &_n_![ 2.0]),  true);

		assert_eq!(funcs::lth(&_n_![NAN], &_n_![NEG_INF]), false);
		assert_eq!(funcs::lth(&_n_![NAN], &_n_![INF]), false);
		assert_eq!(funcs::lth(&_n_![NAN], &_n_![NAN]), false);
		assert_eq!(funcs::lth(&_n_![NAN], &_n_![0.0]), false);
		assert_eq!(funcs::lth(&_n_![NAN], &_n_![-1.0]), false);
		assert_eq!(funcs::lth(&_n_![NAN], &_n_![1.0]), false);

		assert_eq!(funcs::lth(&_n_![NEG_INF], &_n_![NEG_INF]), false);
		assert_eq!(funcs::lth(&_n_![NEG_INF], &_n_![INF]), true);
		assert_eq!(funcs::lth(&_n_![NEG_INF], &_n_![NAN]), false);
		assert_eq!(funcs::lth(&_n_![NEG_INF], &_n_![0.0]), true);
		assert_eq!(funcs::lth(&_n_![NEG_INF], &_n_![-1.0]), true);
		assert_eq!(funcs::lth(&_n_![NEG_INF], &_n_![1.0]), true);

		assert_eq!(funcs::lth(&_n_![INF], &_n_![NEG_INF]), false);
		assert_eq!(funcs::lth(&_n_![INF], &_n_![INF]), false);
		assert_eq!(funcs::lth(&_n_![INF], &_n_![NAN]), false);
		assert_eq!(funcs::lth(&_n_![INF], &_n_![0.0]), false);
		assert_eq!(funcs::lth(&_n_![INF], &_n_![-1.0]), false);
		assert_eq!(funcs::lth(&_n_![INF], &_n_![1.0]), false);
		Ok(())
	}

	#[test]
	fn leq() -> Result<()> {
		assert_eq!(funcs::leq(&_n_![13.5], &_n_![4.0]), false);
		assert_eq!(funcs::leq(&_n_![0.5], &_n_![64.0]), true);
		assert_eq!(funcs::leq(&_n_![-0.05], &_n_![-1.0]), false);
		assert_eq!(funcs::leq(&_n_![2.0], &_n_![9e9]), true);
		assert_eq!(funcs::leq(&_n_![9e9], &_n_![9e9]), true);

		assert_eq!(funcs::leq(&_n_![-1.0], &_n_![-2.0]), false);
		assert_eq!(funcs::leq(&_n_![-1.0], &_n_![-1.0]),  true);
		assert_eq!(funcs::leq(&_n_![-1.0], &_n_![ 0.0]),  true);
		assert_eq!(funcs::leq(&_n_![-1.0], &_n_![ 1.0]),  true);
		assert_eq!(funcs::leq(&_n_![ 0.0], &_n_![-1.0]), false);
		assert_eq!(funcs::leq(&_n_![ 0.0], &_n_![ 0.0]),  true);
		assert_eq!(funcs::leq(&_n_![ 0.0], &_n_![ 1.0]),  true);
		assert_eq!(funcs::leq(&_n_![ 1.0], &_n_![-1.0]), false);
		assert_eq!(funcs::leq(&_n_![ 1.0], &_n_![ 0.0]), false);
		assert_eq!(funcs::leq(&_n_![ 1.0], &_n_![ 1.0]),  true);
		assert_eq!(funcs::leq(&_n_![ 1.0], &_n_![ 2.0]),  true);

		assert_eq!(funcs::leq(&_n_![NAN], &_n_![NEG_INF]), false);
		assert_eq!(funcs::leq(&_n_![NAN], &_n_![INF]), false);
		assert_eq!(funcs::leq(&_n_![NAN], &_n_![NAN]), false);
		assert_eq!(funcs::leq(&_n_![NAN], &_n_![0.0]), false);
		assert_eq!(funcs::leq(&_n_![NAN], &_n_![-1.0]), false);
		assert_eq!(funcs::leq(&_n_![NAN], &_n_![1.0]), false);

		assert_eq!(funcs::leq(&_n_![NEG_INF], &_n_![NEG_INF]), true);
		assert_eq!(funcs::leq(&_n_![NEG_INF], &_n_![INF]), true);
		assert_eq!(funcs::leq(&_n_![NEG_INF], &_n_![NAN]), false);
		assert_eq!(funcs::leq(&_n_![NEG_INF], &_n_![0.0]), true);
		assert_eq!(funcs::leq(&_n_![NEG_INF], &_n_![-1.0]), true);
		assert_eq!(funcs::leq(&_n_![NEG_INF], &_n_![1.0]), true);

		assert_eq!(funcs::leq(&_n_![INF], &_n_![NEG_INF]), false);
		assert_eq!(funcs::leq(&_n_![INF], &_n_![INF]), true);
		assert_eq!(funcs::leq(&_n_![INF], &_n_![NAN]), false);
		assert_eq!(funcs::leq(&_n_![INF], &_n_![0.0]), false);
		assert_eq!(funcs::leq(&_n_![INF], &_n_![-1.0]), false);
		assert_eq!(funcs::leq(&_n_![INF], &_n_![1.0]), false);
	Ok(())
	}

	#[test]
	fn gth() -> Result<()> {
		assert_eq!(funcs::gth(&_n_![13.5], &_n_![4.0]), true);
		assert_eq!(funcs::gth(&_n_![0.5], &_n_![64.0]), false);
		assert_eq!(funcs::gth(&_n_![-0.05], &_n_![-1.0]), true);
		assert_eq!(funcs::gth(&_n_![2.0], &_n_![9e9]), false);
		assert_eq!(funcs::gth(&_n_![9e9], &_n_![9e9]), false);

		assert_eq!(funcs::gth(&_n_![-1.0], &_n_![-2.0]),  true);
		assert_eq!(funcs::gth(&_n_![-1.0], &_n_![-1.0]), false);
		assert_eq!(funcs::gth(&_n_![-1.0], &_n_![ 0.0]), false);
		assert_eq!(funcs::gth(&_n_![-1.0], &_n_![ 1.0]), false);
		assert_eq!(funcs::gth(&_n_![ 0.0], &_n_![-1.0]),  true);
		assert_eq!(funcs::gth(&_n_![ 0.0], &_n_![ 0.0]), false);
		assert_eq!(funcs::gth(&_n_![ 0.0], &_n_![ 1.0]), false);
		assert_eq!(funcs::gth(&_n_![ 1.0], &_n_![-1.0]),  true);
		assert_eq!(funcs::gth(&_n_![ 1.0], &_n_![ 0.0]),  true);
		assert_eq!(funcs::gth(&_n_![ 1.0], &_n_![ 1.0]), false);
		assert_eq!(funcs::gth(&_n_![ 1.0], &_n_![ 2.0]), false);

		assert_eq!(funcs::gth(&_n_![NAN], &_n_![NEG_INF]), false);
		assert_eq!(funcs::gth(&_n_![NAN], &_n_![INF]), false);
		assert_eq!(funcs::gth(&_n_![NAN], &_n_![NAN]), false);
		assert_eq!(funcs::gth(&_n_![NAN], &_n_![0.0]), false);
		assert_eq!(funcs::gth(&_n_![NAN], &_n_![-1.0]), false);
		assert_eq!(funcs::gth(&_n_![NAN], &_n_![1.0]), false);

		assert_eq!(funcs::gth(&_n_![NEG_INF], &_n_![NEG_INF]), false);
		assert_eq!(funcs::gth(&_n_![NEG_INF], &_n_![INF]), false);
		assert_eq!(funcs::gth(&_n_![NEG_INF], &_n_![NAN]), false);
		assert_eq!(funcs::gth(&_n_![NEG_INF], &_n_![0.0]), false);
		assert_eq!(funcs::gth(&_n_![NEG_INF], &_n_![-1.0]), false);
		assert_eq!(funcs::gth(&_n_![NEG_INF], &_n_![1.0]), false);

		assert_eq!(funcs::gth(&_n_![INF], &_n_![NEG_INF]), true);
		assert_eq!(funcs::gth(&_n_![INF], &_n_![INF]), false);
		assert_eq!(funcs::gth(&_n_![INF], &_n_![NAN]), false);
		assert_eq!(funcs::gth(&_n_![INF], &_n_![0.0]), true);
		assert_eq!(funcs::gth(&_n_![INF], &_n_![-1.0]), true);
		assert_eq!(funcs::gth(&_n_![INF], &_n_![1.0]), true);
		Ok(())
	}

	#[test]
	fn geq() -> Result<()> {
		assert_eq!(funcs::geq(&_n_![13.5], &_n_![4.0]), true);
		assert_eq!(funcs::geq(&_n_![0.5], &_n_![64.0]), false);
		assert_eq!(funcs::geq(&_n_![-0.05], &_n_![-1.0]), true);
		assert_eq!(funcs::geq(&_n_![2.0], &_n_![9e9]), false);
		assert_eq!(funcs::geq(&_n_![9e9], &_n_![9e9]), true);

		assert_eq!(funcs::geq(&_n_![-1.0], &_n_![-2.0]),  true);
		assert_eq!(funcs::geq(&_n_![-1.0], &_n_![-1.0]),  true);
		assert_eq!(funcs::geq(&_n_![-1.0], &_n_![ 0.0]), false);
		assert_eq!(funcs::geq(&_n_![-1.0], &_n_![ 1.0]), false);
		assert_eq!(funcs::geq(&_n_![ 0.0], &_n_![-1.0]),  true);
		assert_eq!(funcs::geq(&_n_![ 0.0], &_n_![ 0.0]),  true);
		assert_eq!(funcs::geq(&_n_![ 0.0], &_n_![ 1.0]), false);
		assert_eq!(funcs::geq(&_n_![ 1.0], &_n_![-1.0]),  true);
		assert_eq!(funcs::geq(&_n_![ 1.0], &_n_![ 0.0]),  true);
		assert_eq!(funcs::geq(&_n_![ 1.0], &_n_![ 1.0]),  true);
		assert_eq!(funcs::geq(&_n_![ 1.0], &_n_![ 2.0]), false);

		assert_eq!(funcs::geq(&_n_![NAN], &_n_![NEG_INF]), false);
		assert_eq!(funcs::geq(&_n_![NAN], &_n_![NEG_INF+1.0]), false);
		assert_eq!(funcs::geq(&_n_![NAN], &_n_![INF]), false);
		assert_eq!(funcs::geq(&_n_![NAN], &_n_![NAN]), false);
		assert_eq!(funcs::geq(&_n_![NAN], &_n_![0.0]), false);
		assert_eq!(funcs::geq(&_n_![NAN], &_n_![-1.0]), false);
		assert_eq!(funcs::geq(&_n_![NAN], &_n_![1.0]), false);

		assert_eq!(funcs::geq(&_n_![NEG_INF], &_n_![NEG_INF]), true);
		assert_eq!(funcs::geq(&_n_![NEG_INF], &_n_![INF]), false);
		assert_eq!(funcs::geq(&_n_![NEG_INF], &_n_![NAN]), false);
		assert_eq!(funcs::geq(&_n_![NEG_INF], &_n_![0.0]), false);
		assert_eq!(funcs::geq(&_n_![NEG_INF], &_n_![-1.0]), false);
		assert_eq!(funcs::geq(&_n_![NEG_INF], &_n_![1.0]), false);

		assert_eq!(funcs::geq(&_n_![INF], &_n_![NEG_INF]), true);
		assert_eq!(funcs::geq(&_n_![INF], &_n_![INF]), true);
		assert_eq!(funcs::geq(&_n_![INF], &_n_![NAN]), false);
		assert_eq!(funcs::geq(&_n_![INF], &_n_![0.0]), true);
		assert_eq!(funcs::geq(&_n_![INF], &_n_![-1.0]), true);
		assert_eq!(funcs::geq(&_n_![INF], &_n_![1.0]), true);
		Ok(())
	}

	#[test]
	fn cmp() -> Result<()> {
		use super::Number;
		assert_eq!(funcs::cmp(&_n_![13.5], &_n_![4.0]).downcast::<Number>().unwrap(), 1.0);
		assert_eq!(funcs::cmp(&_n_![0.5], &_n_![64.0]).downcast::<Number>().unwrap(), -1.0);
		assert_eq!(funcs::cmp(&_n_![-0.05], &_n_![-1.0]).downcast::<Number>().unwrap(), 1.0);
		assert_eq!(funcs::cmp(&_n_![2.0], &_n_![9e9]).downcast::<Number>().unwrap(), -1.0);
		assert_eq!(funcs::cmp(&_n_![9e9], &_n_![9e9]).downcast::<Number>().unwrap(), 0.0);

		assert_eq!(funcs::cmp(&_n_![-1.0], &_n_![-1.0]).downcast::<Number>().unwrap(),  0.0);
		assert_eq!(funcs::cmp(&_n_![-1.0], &_n_![ 0.0]).downcast::<Number>().unwrap(), -1.0);
		assert_eq!(funcs::cmp(&_n_![-1.0], &_n_![ 1.0]).downcast::<Number>().unwrap(), -1.0);
		assert_eq!(funcs::cmp(&_n_![ 0.0], &_n_![-1.0]).downcast::<Number>().unwrap(),  1.0);
		assert_eq!(funcs::cmp(&_n_![ 0.0], &_n_![ 0.0]).downcast::<Number>().unwrap(),  0.0);
		assert_eq!(funcs::cmp(&_n_![ 0.0], &_n_![ 1.0]).downcast::<Number>().unwrap(), -1.0);
		assert_eq!(funcs::cmp(&_n_![ 1.0], &_n_![-1.0]).downcast::<Number>().unwrap(),  1.0);
		assert_eq!(funcs::cmp(&_n_![ 1.0], &_n_![ 0.0]).downcast::<Number>().unwrap(),  1.0);
		assert_eq!(funcs::cmp(&_n_![ 1.0], &_n_![ 1.0]).downcast::<Number>().unwrap(),  0.0);

		assert!(funcs::cmp(&_n_![NAN], &_n_![NEG_INF]).is_null());
		assert!(funcs::cmp(&_n_![NAN], &_n_![INF]).is_null());
		assert!(funcs::cmp(&_n_![NAN], &_n_![NAN]).is_null());
		assert!(funcs::cmp(&_n_![NAN], &_n_![0.0]).is_null());
		assert!(funcs::cmp(&_n_![NAN], &_n_![-1.0]).is_null());
		assert!(funcs::cmp(&_n_![NAN], &_n_![1.0]).is_null());

		assert_eq!(funcs::cmp(&_n_![NEG_INF], &_n_![NEG_INF]).downcast::<Number>().unwrap(), 0.0);
		assert_eq!(funcs::cmp(&_n_![NEG_INF], &_n_![INF]).downcast::<Number>().unwrap(), -1.0);
		assert!(funcs::cmp(&_n_![NEG_INF], &_n_![NAN]).is_null());
		assert_eq!(funcs::cmp(&_n_![NEG_INF], &_n_![0.0]).downcast::<Number>().unwrap(), -1.0);
		assert_eq!(funcs::cmp(&_n_![NEG_INF], &_n_![-1.0]).downcast::<Number>().unwrap(), -1.0);
		assert_eq!(funcs::cmp(&_n_![NEG_INF], &_n_![1.0]).downcast::<Number>().unwrap(), -1.0);

		assert_eq!(funcs::cmp(&_n_![INF], &_n_![NEG_INF]).downcast::<Number>().unwrap(), 1.0);
		assert_eq!(funcs::cmp(&_n_![INF], &_n_![INF]).downcast::<Number>().unwrap(), 0.0);
		assert!(funcs::cmp(&_n_![INF], &_n_![NAN]).is_null());
		assert_eq!(funcs::cmp(&_n_![INF], &_n_![0.0]).downcast::<Number>().unwrap(), 1.0);
		assert_eq!(funcs::cmp(&_n_![INF], &_n_![-1.0]).downcast::<Number>().unwrap(), 1.0);
		assert_eq!(funcs::cmp(&_n_![INF], &_n_![1.0]).downcast::<Number>().unwrap(), 1.0);
		Ok(())
	}

	#[test]
	fn pos() -> Result<()> {
		assert_eq!(funcs::pos(&_n_![-9e9]), 9e9);
		assert_eq!(funcs::pos(&_n_![-2.0]), 2.0);
		assert_eq!(funcs::pos(&_n_![-1.0]), 1.0);
		assert_eq!(funcs::pos(&_n_![-0.5]), 0.5);
		assert_eq!(funcs::pos(&_n_![ 0.0]), 0.0);
		assert_eq!(funcs::pos(&_n_![ 1.0]), 1.0);
		assert_eq!(funcs::pos(&_n_![ 2.0]), 2.0);
		assert!(funcs::pos(&_n_![NAN]).is_nan());
		assert_eq!(funcs::pos(&_n_![NEG_INF]), INF);
		assert_eq!(funcs::pos(&_n_![INF]), INF);

		let n = Object::new_number(3.14);
		assert_obj_duplicated!(n, funcs::pos(&n));
		Ok(())
	}
}
/*
#[cfg(test)]
mod fn_tests_ {
	use super::*;
	use crate::object::types::{Boolean, Text};
	use crate::err::Error;
	use std::f64::{INF, NEG_INF, NAN, consts::{PI, E}};

	macro_rules! n {
		($num:expr) => (Object::new_number($num).as_any())
	}

	macro_rules! assert_num_call_eq {
		($attr:tt $type:ty; $(($obj:expr, $args:tt) => $expected:expr),*) => {
			$(
				assert_eq!(*n!($obj).call_attr($attr, &$args)?.downcast_or_err::<$type>()?.unwrap_data(), $expected);
			)*
		}
	}

	#[test]
	fn at_bool() -> Result<()> {
		assert_num_call_eq!(AT_BOOL Boolean;
			(0.0, []) => false,
			(-0.0, []) => false,
			(NAN, []) => false,
			(13.4, []) => true,
			(INF, []) => true,
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
			(INF, []) => *"inf",
			(NEG_INF, []) => *"-inf",
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
			(INF, []) => INF,
			(PI, []) => PI,
			(E, []) => E,
			(-123.0, []) => -123.0,
			(12.0, [&n!(34.0)]) => 12.0 // ensure extra args are ignored
		);

		// make sure that it acutally duplicates the map
		let obj = Object::new_number(12.45);
		let dup = obj.as_any().call_attr(AT_NUM, &[])?.downcast_or_err::<Number>()?;
		assert_eq!(obj.unwrap_data(), dup.unwrap_data());
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

		assert!(n!(NAN).call_attr(ADD, &[&n!(NAN)])?.downcast_or_err::<Number>()?.unwrap_data().is_nan());
		assert!(n!(INF).call_attr(ADD, &[&n!(INF)])?.downcast_or_err::<Number>()?.unwrap_data().is_infinite());
		assert!(n!(INF).call_attr(ADD, &[&n!(NEG_INF)])?.downcast_or_err::<Number>()?.unwrap_data().is_nan());

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

		assert!(n!(NAN).call_attr(SUB, &[&n!(NAN)])?.downcast_or_err::<Number>()?.unwrap_data().is_nan());
		assert!(n!(INF).call_attr(SUB, &[&n!(INF)])?.downcast_or_err::<Number>()?.unwrap_data().is_nan());
		assert!(n!(INF).call_attr(SUB, &[&n!(NEG_INF)])?.downcast_or_err::<Number>()?.unwrap_data().is_infinite());

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


		assert!(n!(NAN).call_attr(MUL, &[&n!(NAN)])?.downcast_or_err::<Number>()?.unwrap_data().is_nan());
		assert!(n!(INF).call_attr(MUL, &[&n!(INF)])?.downcast_or_err::<Number>()?.unwrap_data().is_infinite());
		assert!(n!(INF).call_attr(MUL, &[&n!(NEG_INF)])?.downcast_or_err::<Number>()?.unwrap_data().is_infinite());

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
		assert!(n!(1.0).call_attr(DIV, &[&n!(0.0)])?.downcast_or_err::<Number>()?.unwrap_data().is_infinite());

		assert!(n!(NAN).call_attr(DIV, &[&n!(NAN)])?.downcast_or_err::<Number>()?.unwrap_data().is_nan());
		assert!(n!(INF).call_attr(DIV, &[&n!(INF)])?.downcast_or_err::<Number>()?.unwrap_data().is_nan());
		assert!(n!(INF).call_attr(DIV, &[&n!(NEG_INF)])?.downcast_or_err::<Number>()?.unwrap_data().is_nan());

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

		assert!(n!(1.0).call_attr(MOD, &[&n!(0.0)])?.downcast_or_err::<Number>()?.unwrap_data().is_nan());

		assert!(n!(NAN).call_attr(MOD, &[&n!(NAN)])?.downcast_or_err::<Number>()?.unwrap_data().is_nan());
		assert!(n!(INF).call_attr(MOD, &[&n!(INF)])?.downcast_or_err::<Number>()?.unwrap_data().is_nan());
		assert!(n!(INF).call_attr(MOD, &[&n!(NEG_INF)])?.downcast_or_err::<Number>()?.unwrap_data().is_nan());

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
			(INF, [&n!(0.0)]) => 1.0,
			(1234.0, [&n!(NEG_INF)]) => 0.0,
			(1234.0, [&n!(INF)]) => INF,
			(12.0, [&n!(0.0), &n!(PI)]) => 1.0 // ensure extra args are ignored
		);

		assert!(n!(NAN).call_attr(POW, &[&n!(NAN)])?.downcast_or_err::<Number>()?.unwrap_data().is_nan());
		assert!(n!(NAN).call_attr(POW, &[&n!(INF)])?.downcast_or_err::<Number>()?.unwrap_data().is_nan());
		assert!(n!(NEG_INF).call_attr(POW, &[&n!(NAN)])?.downcast_or_err::<Number>()?.unwrap_data().is_nan());

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
			(INF, [&n!(INF)]) => true,
			(INF, [&n!(NEG_INF)]) => false,
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
			(INF, [&n!(INF)]) => false,
			(INF, [&n!(NEG_INF)]) => true,
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
			(NEG_INF, [&n!(INF)]) => -1.0,
			(1.0, [&n!(0.0), &n!(PI)]) => 1.0 // ensure extra args are ignored
		);

		assert!(n!(NAN).call_attr(CMP, &[&n!(9.0)])?.is_null());
		assert!(n!(NAN).call_attr(CMP, &[&n!(NAN)])?.is_null());
		assert!(n!(NEG_INF).call_attr(CMP, &[&n!(NAN)])?.is_null());


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
			(NEG_INF, [&n!(NAN)]) => false,
			(NEG_INF, [&n!(INF)]) => true,
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
			(NEG_INF, [&n!(NAN)]) => false,
			(NEG_INF, [&n!(INF)]) => true,
			(NEG_INF, [&n!(NEG_INF)]) => true,
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
			(NEG_INF, [&n!(NAN)]) => false,
			(NEG_INF, [&n!(INF)]) => false,
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
			(NEG_INF, [&n!(NAN)]) => false,
			(NEG_INF, [&n!(INF)]) => false,
			(1.0, [&n!(0.0), &n!(PI)]) => true // ensure extra args are ignored
		);

		assert_param_missing!(n!(4.0).call_attr(GEQ, &[]));

		Ok(())
	}

	#[test]
	fn pos() -> Result<()> {
		assert_num_call_eq!(POS Number;
			(13.5, []) => 13.5, 
			(-PI, []) => PI,
			(0.0, []) => 0.0,
			(9e9, []) => 9e9,
			(NEG_INF, []) => INF,
			(INF, []) => INF,
			(1.0, [&n!(-PI)]) => 1.0 // ensure extra args are ignored
		);

		assert!(n!(NAN).call_attr(POS, &[])?.downcast_or_err::<Number>()?.unwrap_data().is_nan());


		// make sure that it acutally duplicates the map
		let obj = Object::new_number(12.45);
		let dup = obj.as_any().call_attr(AT_NUM, &[])?.downcast_or_err::<Number>()?;
		assert_obj_duplicated!(obj, dup);

		Ok(())
	}

	#[test]
	fn neg() -> Result<()> {
		assert_num_call_eq!(NEG Number;
			(13.5, []) => -13.5, 
			(-PI, []) => PI,
			(0.0, []) => 0.0,
			(9e9, []) => -9e9,
			(NEG_INF, []) => INF,
			(INF, []) => NEG_INF,
			(1.0, [&n!(PI)]) => -1.0 // ensure extra args are ignored
		);

		assert!(n!(NAN).call_attr(NEG, &[])?.downcast_or_err::<Number>()?.unwrap_data().is_nan());

		Ok(())
	}
}*/


#[cfg(test)]
mod tests {
	use super::{*, consts::{*, inner_mod}};

	#[test]
	fn new() {
		macro_rules! assert_new_asref_eq {
			($($val:expr),*) => {
				$( assert_eq!(Number::new($val).as_ref(), &$val); )*
			};
		}

		assert!(Number::new(NAN).as_ref().is_nan());
		assert!(Number::new(NEG_INF).as_ref().is_infinite());
		assert!(Number::new(INF).as_ref().is_infinite());

		assert_new_asref_eq!{
			0.0, -1.0, 1.0, 123491.0,
			INF, NEG_INF, E, PI, 
			inner_mod::EPSILON, inner_mod::MIN, inner_mod::MIN_POSITIVE, inner_mod::MAX,
			inner_consts::FRAC_1_PI, inner_consts::FRAC_2_PI, inner_consts::FRAC_1_SQRT_2, inner_consts::FRAC_2_SQRT_PI,
			inner_consts::FRAC_PI_2, inner_consts::FRAC_PI_3, inner_consts::FRAC_PI_4, inner_consts::FRAC_PI_6,
			inner_consts::FRAC_PI_8, inner_consts::LN_2, inner_consts::LN_10, inner_consts::LOG2_E,
			inner_consts::LOG10_E, inner_consts::SQRT_2
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
		assert_eq!(*Object::new_number(1234.0).as_any().to_number()?.unwrap_data(), 1234.0);
		assert_eq!(*Object::new_number(1.0).as_any().to_number()?.unwrap_data(), 1.0);
		assert!(Object::new_number(INF).as_any().to_number()?.unwrap_data().is_infinite());
		assert!(Object::new_number(NAN).as_any().to_number()?.unwrap_data().is_nan());
		
		Ok(())
	}

}