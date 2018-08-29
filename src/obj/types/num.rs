use obj::{Result, SharedObject, types::IntoObject};
use std::fmt::{self, Display, Formatter};
use std::ops::{
	Add, Sub, Mul, Div, Rem, Neg,
	BitAnd, BitOr, BitXor, Shr, Shl,
	AddAssign, SubAssign, MulAssign, DivAssign, RemAssign,
};
use std::cmp::{PartialOrd, Ord, Ordering};

pub type Unsigned = u32;
pub type Signed = i32;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Sign {
	Positive,
	Negative
}

use self::Sign::*;

impl Default for Sign {
	#[inline]
	fn default() -> Self {
		Positive
	}
}


impl IntoObject for usize {
	type Type = Number;
	fn into_object(self) -> SharedObject<Number> {
		Number::from_integer(self as Signed).into_object()
	}
}


impl IntoObject for Signed {
	type Type = Number;
	fn into_object(self) -> SharedObject<Number> {
		Number::from_integer(self).into_object()
	}
}

impl Mul<Unsigned> for Sign {
	type Output = Signed;
	fn mul(self, rhs: Unsigned) -> Signed {
		match self {
			Positive =>   rhs as Signed,
			Negative => -(rhs as Signed)
		}
	}
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub struct Number {
	sign: Sign,
	whole: Unsigned,
	frac: Unsigned
}

const ONE: Number = Number { sign: Positive, whole: 1, frac: 0 };
const ZERO: Number = Number { sign: Positive, whole: 0, frac: 0 };
const PI: Number = Number { sign: Positive, whole: 3, frac: -1i32 as u32 };
const E: Number = Number { sign: Positive, whole: 3, frac: -1i32 as u32 };

const NEG_ONE: Number = Number { sign: Negative, whole: 1, frac: 0 };
const NEG_ZERO: Number = Number { sign: Negative, whole: 0, frac: 0 };

impl Number {
	#[inline(always)]
	pub fn one() -> Number { ONE }

	#[inline(always)]
	pub fn zero() -> Number { ZERO }

	#[inline(always)]
	pub fn neg_one() -> Number { NEG_ONE }

	#[inline(always)]
	pub fn neg_zero() -> Number { NEG_ZERO }

	#[inline(always)]
	pub fn pi() -> Number { PI }

	#[inline(always)]
	pub fn e() -> Number { E }
}

impl Number {
	#[inline]
	pub fn new(sign: Sign, whole: Unsigned, frac: Unsigned) -> Self {
		Number { sign, whole, frac }
	}

	pub fn from_integer(whole: Signed) -> Self {
		if whole < 0 {
			Number::new(Negative, -whole as Unsigned, 0)
		} else {
			Number::new(Positive, whole as Unsigned, 0)
		}
	}

	#[inline]
	pub fn is_integer(&self) -> bool {
		self.frac == 0
	}

	#[inline]
	pub fn is_zero(&self) -> bool {
		self.whole == 0 && self.frac == 0
	}

	#[inline]
	pub fn is_positive(&self) -> bool {
		self.sign == Positive
	}

	#[inline]
	pub fn is_negative(&self) -> bool {
		!self.is_positive()
	}

	#[inline]
	pub fn sign(&self) -> Sign {
		self.sign
	}


	#[inline]
	pub fn to_integer(self) -> Option<Signed> {
		if self.is_integer() {
			Some(self.sign * self.whole)
		} else {
			None
		}
	}

	pub fn recip(self) -> Number {
		unimplemented!("TODO: recip for numbers");
		Number { sign: self.sign, whole: self.frac, frac: self.whole }
	}


	pub fn pow(self, other: Number) -> Number {
		unimplemented!("TODO: pow for numbers")
	}
}

impl Add for Number {
	type Output = Number;
	fn add(self, rhs: Number) -> Number {
		if !self.is_integer() || !rhs.is_integer() {
			unimplemented!("todo: add with fracs ({}, {})", self, rhs);
		}

		match (self.sign, rhs.sign) {
			(Positive, Positive) => Number::new(Positive, self.whole + rhs.whole, 0),
			(Positive, Negative) if self.whole >= rhs.whole => Number::new(Positive, self.whole - rhs.whole, 0), // `4-4` will be `+0`
			(Positive, Negative) => Number::new(Negative, rhs.whole - self.whole, 0),
			(Negative, Positive) if self.whole > rhs.whole => Number::new(Negative, self.whole - rhs.whole, 0), // `4-4` will be `+0`
			(Negative, Positive) => Number::new(Positive, rhs.whole - self.whole, 0), // `4-4` will be `+0`
			(Negative, Negative) => Number::new(Negative, self.whole + rhs.whole, 0)
		}
	}
}

impl AddAssign for Number {
	fn add_assign(&mut self, rhs: Number) {
		unimplemented!("TODO: add assign")
	}
}

impl Neg for Number {
	type Output = Number;
	fn neg(self) -> Number {
		match self.sign {
			Positive => Number { sign: Negative, ..self },
			Negative => Number { sign: Positive, ..self },
		}
	}
}

impl Sub for Number {
	type Output = Number;
	fn sub(self, rhs: Number) -> Number {
		self + -rhs
	}
}

impl SubAssign for Number {
	fn sub_assign(&mut self, rhs: Number) {
		unimplemented!("TODO: sub assign")
	}
}

impl Mul for Number {
	type Output = Number;
	fn mul(self, rhs: Number) -> Number {
		if !self.is_integer() || !rhs.is_integer() {
			unimplemented!("todo: mul with fracs ({}, {})", self, rhs);
		}

		let sign = match (self.sign, rhs.sign) {
			(Positive, Positive) | (Negative, Negative) => Positive,
			_ => Negative
		};

		Number {
			sign,
			whole: self.whole * rhs.whole,
			frac: 0
		}
	}
}

impl MulAssign for Number {
	fn mul_assign(&mut self, rhs: Number) {
		unimplemented!("TODO: mul assign")
	}
}

impl Div for Number {
	type Output = Number;
	fn div(self, rhs: Number) -> Number {
		self * rhs.recip()
	}
}

impl DivAssign for Number {
	fn div_assign(&mut self, rhs: Number) {
		unimplemented!("TODO: div assign")
	}
}

impl Rem for Number {
	type Output = Number;
	fn rem(self, rhs: Number) -> Number {
		unimplemented!("TODO: Modulo")
	}
}

impl RemAssign for Number {
	fn rem_assign(&mut self, rhs: Number) {
		unimplemented!("TODO: rem assign")
	}
}

impl BitAnd for Number {
	type Output = Option<Number>;
	fn bitand(self, rhs: Number) -> Option<Number> {
		Some(Number::from_integer(self.to_integer()? & rhs.to_integer()?))
	}
}

impl BitOr for Number {
	type Output = Option<Number>;
	fn bitor(self, rhs: Number) -> Option<Number> {
		Some(Number::from_integer(self.to_integer()? | rhs.to_integer()?))
	}
}

impl BitXor for Number {
	type Output = Option<Number>;
	fn bitxor(self, rhs: Number) -> Option<Number> {
		Some(Number::from_integer(self.to_integer()? ^ rhs.to_integer()?))
	}
}

impl Shl for Number {
	type Output = Option<Number>;
	fn shl(self, rhs: Number) -> Option<Number> {
		Some(Number::from_integer(self.to_integer()? << rhs.to_integer()?))
	}
}

impl Shr for Number {
	type Output = Option<Number>;
	fn shr(self, rhs: Number) -> Option<Number> {
		Some(Number::from_integer(self.to_integer()? >> rhs.to_integer()?))
	}
}



impl Ord for Number {
	fn cmp(&self, num: &Number) -> Ordering {
		if self.is_zero() && num.is_zero() {
			return Ordering::Equal; // so we dont get `-0 != +0`
		}

		match (self.sign, num.sign) {
			(Negative, Positive) => Ordering::Less,
			(Positive, Negative) => Ordering::Greater,
			(Positive, Positive) if self.whole == num.whole => self.frac.cmp(&num.frac),
			(Positive, Positive) => self.whole.cmp(&num.whole),
			(Negative, Negative) if self.whole == num.whole => match self.frac.cmp(&num.frac) {
				Ordering::Less => Ordering::Greater,
				Ordering::Equal => Ordering::Equal,
				Ordering::Greater => Ordering::Less
			},
			(Negative, Negative) => self.whole.cmp(&num.whole)
		}
	}
}

impl PartialOrd for Number {
	fn partial_cmp(&self, num: &Number) -> Option<Ordering> {
		return Some(self.cmp(num))
	}
}

impl PartialEq<Signed> for Number {
	fn eq(&self, whole: &Signed) -> bool {
		*self == Number::from_integer(*whole)
	}
}

impl PartialOrd<Signed> for Number {
	fn partial_cmp(&self, whole: &Signed) -> Option<Ordering> {
		self.partial_cmp(&Number::from_integer(*whole))
	}
}

impl Display for Number {
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		if f.alternate() {
			return write!(f, "{:}{}.{}", self.sign, self.whole, self.frac)
		}

		write!(f, "{}{}", self.sign, self.whole)?;

		if !self.is_integer() {
			write!(f, ".{}", self.frac)
		} else {
			Ok(())
		}
	}
}

impl Display for Sign {
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		match self {
			Negative => write!(f, "-"),
			Positive if f.alternate() => write!(f, "+"),
			Positive => Ok(())
		}
	}
}

macro_rules! binary_oper {
	(assign $oper:tt $this:ident $other:ident $env:ident) => {{
		$this.data $oper $other.attrs.into_num($env)?;
		Ok($this.upgrade())
	}};

	($oper:tt $this:ident $other:ident $env:ident) => (Ok(($this.data $oper $other.attrs.into_num($env)?).into_object()))
}


macro_rules! bit_oper {
	(assign $oper:tt $this:ident $other:ident $env:ident) => ({
		$this.data = ($this.data $oper $other.attrs.into_num($env)?)
			.ok_or_else(|| 
				Error::BadArguments {
					args: vec![$this.upgrade(), $other.upgrade()],
					descr: concat!("whole numbers are needed for `", stringify!($oper), "`")
				}
			)?;
		Ok($this.upgrade())
	});

	($oper:tt $this:ident $other:ident $env:ident) => (
		($this.data $oper $other.attrs.into_num($env)?)
			.map(IntoObject::into_anyobject)
			.ok_or_else(|| 
				Error::BadArguments {
					args: vec![$this.upgrade(), $other.upgrade()],
					descr: concat!("whole numbers are needed for `", stringify!($oper), "`")
				}
			)
	)
}


impl_type! {
	for Number, with self attr;

	fn "@bool" (this) {
		Ok((!this.data.is_zero()).into_object())
	}

	fn "@num" (this) {
		Ok(this.duplicate())
	}

	fn "()" (shared this) env, args, {
		this.read_call("*", args, env)
	}

	fn "+"  (this, rhs) env, { binary_oper!(+ this rhs env) }
	fn "-"  (this, rhs) env, { binary_oper!(- this rhs env) }
	fn "*"  (this, rhs) env, { binary_oper!(* this rhs env) }
	fn "/"  (this, rhs) env, { binary_oper!(/ this rhs env) }
	fn "%"  (this, rhs) env, { binary_oper!(% this rhs env) }
	fn "**"  (this, rhs) env, { Ok(this.data.pow(rhs.attrs.into_num(env)?).into_object()) }

	fn "+="  (mut this, rhs) env, { binary_oper!(assign += this rhs env) }
	fn "-="  (mut this, rhs) env, { binary_oper!(assign -= this rhs env) }
	fn "*="  (mut this, rhs) env, { binary_oper!(assign *= this rhs env) }
	fn "/="  (mut this, rhs) env, { binary_oper!(assign /= this rhs env) }
	fn "%="  (mut this, rhs) env, { binary_oper!(assign %= this rhs env) }
	fn "**="  (mut this, rhs) env, {
		this.data = this.data.pow(rhs.attrs.into_num(env)?);
		Ok(this.upgrade())
	}

	fn "&" (this, rhs) env, { bit_oper!(& this rhs env) }
	fn "|" (this, rhs) env, { bit_oper!(| this rhs env) }
	fn "^" (this, rhs) env, { bit_oper!(^ this rhs env) }
	fn "<<" (this, rhs) env, { bit_oper!(<< this rhs env) }
	fn ">>" (this, rhs) env, { bit_oper!(>> this rhs env) }

	fn "&=" (mut this, rhs) env, { bit_oper!(assign & this rhs env) }
	fn "|=" (mut this, rhs) env, { bit_oper!(assign | this rhs env) }
	fn "^=" (mut this, rhs) env, { bit_oper!(assign ^ this rhs env) }
	fn "<<=" (mut this, rhs) env, { bit_oper!(assign << this rhs env) }
	fn ">>=" (mut this, rhs) env, { bit_oper!(assign >> this rhs env) }



	fn "<"  (this, rhs) env, { binary_oper!(< this rhs env) }
	fn "<=" (this, rhs) env, { binary_oper!(<= this rhs env) }
	fn ">"  (this, rhs) env, { binary_oper!(> this rhs env) }
	fn ">=" (this, rhs) env, { binary_oper!(>= this rhs env) }
	fn "==" (this, rhs) env, { binary_oper!(== this rhs env) }
	fn "!=" (this, rhs) env, { binary_oper!(!= this rhs env) }

	fn "<=>" (this, rhs) env, {
		Ok(match this.data.cmp(&rhs.attrs.into_num(env)?) {
			Ordering::Less => Number::neg_one(),
			Ordering::Equal => Number::zero(),
			Ordering::Greater => Number::one(),
		}.into_object())
	}

	fn _ (_) {
		any::get_default_attr(self, attr)
	}
}


#[cfg(test)]
mod test {
	use super::*;

	fn s(sign: Sign, amnt: Unsigned) -> Number {
		Number::new(sign, amnt, 0)
	}

	fn i(amnt: Signed) -> Number {
		Number::from_integer(amnt)
	}

	#[test]
	fn creation(){
		assert_eq!(Number::new(Positive, 43, 12841), Number { sign: Positive, whole: 43, frac: 12841 });
		assert_eq!(Number::new(Negative, 43, 12841), Number { sign: Negative, whole: 43, frac: 12841 });
		assert_eq!(Number::from_integer(0), Number { sign: Positive, whole: 0, frac: 0 });
		assert_eq!(Number::from_integer(-0), Number { sign: Positive, whole: 0, frac: 0 });
		assert_eq!(Number::from_integer(54), Number { sign: Positive, whole: 54, frac: 0 });
		assert_eq!(Number::from_integer(-192351), Number { sign: Negative, whole: 192351, frac: 0 });
	}

	#[test]
	fn add() {
		assert_eq!(i(4) + i(3), i(7));
		assert_eq!(i(7) + i(-3), i(4));
		assert_eq!(i(7) + i(-0), i(7));
		assert_eq!(i(5) + i(-5), s(Positive, 0));
		assert_eq!(i(-5) + i(5), s(Positive, 0));
		assert_eq!(i(5) + i(-12), i(-7));
		assert_eq!(i(-5) + i(-12), i(-17));
	}
}






















