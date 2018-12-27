use parse::{Parsable, Stream, Token};
use env::Environment;
use obj::{Result, AnyShared, SharedObject, types::IntoObject};
use std::fmt::{self, Display, Formatter};
use std::ops::{
	Add, Sub, Mul, Div, Rem, Neg,
	BitAnd, BitOr, BitXor, Shr, Shl,
	AddAssign, SubAssign, MulAssign, DivAssign, RemAssign,
	BitAndAssign, BitOrAssign, BitXorAssign, ShrAssign, ShlAssign,
};

use std::cmp::{PartialOrd, Ord, Ordering};

type Unsigned = u32;
pub type Integer = i32;

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
		Number::from(self as Integer).into_object()
	}
}

impl IntoObject for Integer {
	type Type = Number;
	fn into_object(self) -> SharedObject<Number> {
		Number::from(self).into_object()
	}
}

impl Mul<Unsigned> for Sign {
	type Output = Integer;
	fn mul(self, rhs: Unsigned) -> Integer {
		match self {
			Positive =>   rhs as Integer,
			Negative => -(rhs as Integer)
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

impl From<Integer> for Number {
	fn from(whole: Integer) -> Self {
		if whole < 0 {
			Number::new(Negative, -whole as Unsigned, 0)
		} else {
			Number::new(Positive, whole as Unsigned, 0)
		}
	}
}

impl Number {
	#[inline]
	pub fn new(sign: Sign, whole: Unsigned, frac: Unsigned) -> Self {
		Number { sign, whole, frac }
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
	pub fn is_over_zero(&self) -> bool {
		self.is_positive() && !self.is_zero()
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
	pub fn to_integer(self) -> Option<Integer> {
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

	pub fn pow_assign(&mut self, other: Number) {
		*self = self.pow(other)
	}
}

impl Number {
	pub fn parse_str<C: Iterator<Item=char>>(mut chars: C) -> Option<(Number, usize)> {
		let mut num = Number::default();
		let mut offset = 0;
		let mut digit = chars.next()?;
		if digit == '-' || digit == '+' {
			debug_assert_eq!(num.sign, Positive); // sign defaults to positive
			if digit == '-' {
				num.sign = Negative;
			}
			offset += digit.len_utf8();
			// digit = chars.by_ref().skip_while(|c| c.is_whitespace()).next()?;
			digit = chars.next()?; // we can only have a `-` or `+` flush against a number. TODO: fix this?
		}
		offset += digit.len_utf8();
		num.whole = digit.to_digit(10)?; // if its not a digit, this returns early

		for chr in chars {
			if let Some(digit) = chr.to_digit(10) {
				num.whole = num.whole * 10 + digit;
				offset += chr.len_utf8();
			} else {
				break
			}
		}
		Some((num, offset))
	}
}
impl Parsable for Number {
	fn parse(stream: &mut Stream) -> Option<Token> {
		let (num, offset) = Number::parse_str(stream.chars())?;



		Some(Token::new_literal(stream.offset_to(offset), Default::default(), move || num.into_object()))
	}
}

/*
impl Parsable for Number {
	fn parse(stream: &mut Stream) -> Option<Token> {
		let mut sign = None;
		let mut whole = 0;
		let mut frac = 0;
		let mut offset = 0;

		{
			let mut chars = stream.chars();

			match chars.next()? {
				'-' => sign = Some(Negative),
				'+' => sign = Some(Positive),
				digit if digit.is_digit(10) => whole = digit.to_digit(10).unwrap(),
				_ => return None
			}

			if sign.is_some() {
				for c in &mut chars {
					if c.is_whitespace() {
						continue;
					}

					if let Some(digit) = c.to_digit(10) {
						whole = digit;
						break
					}

					return None; // this is a `+/-` without a digit following it
				}
			}

			let mut has_decimal = false;
			while let Some(digit) = chars.next().and_then(|c| c.to_digit(10)) {
				whole = whole * 10 + digit;
			}

			if chars.prev() == Some('.') {
				while let Some(digit) = chars.next().and_then(|c| c.to_digit(10)) {
					frac = frac * 10 + digit;
				}
			}

			if chars.prev() == Some('e') || chars.prev() == Some('E') {
				unimplemented!("TODO: scientific notation");
			}
			offset = chars.offset();
		}

		let text = stream.offset_to(offset);
		let num = Number::new(sign.unwrap_or(Positive), whole, frac);

		Some(Token::new(text, move |_: &Environment| num.into_object() as AnyShared))
	}
}
*/

impl Neg for Number {
	type Output = Number;
	fn neg(self) -> Number {
		match self.sign {
			Positive => Number { sign: Negative, ..self },
			Negative => Number { sign: Positive, ..self },
		}
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
		*self = *self + rhs;
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
		*self = *self - rhs;
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
		*self = *self * rhs;
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
		*self = *self / rhs;
	}
}

impl Rem for Number {
	type Output = Number;
	fn rem(mut self, rhs: Number) -> Number {
		if !rhs.is_over_zero() {
			panic!("Can't modulo negative numbers");
		}

		if let (Some(this), Some(rhs)) = (self.to_integer(), rhs.to_integer()) {
			return Number::from(this % rhs);
		}

		panic!("Can only modulo integers");

		while (self - rhs).is_over_zero() {
			self -= rhs;
		}

		self
	}
}

impl RemAssign for Number {
	fn rem_assign(&mut self, rhs: Number) {
		*self = *self % rhs;
	}
}

impl BitAnd for Number {
	type Output = Option<Number>;
	fn bitand(self, rhs: Number) -> Option<Number> {
		Some(Number::from(self.to_integer()? & rhs.to_integer()?))
	}
}

impl BitOr for Number {
	type Output = Option<Number>;
	fn bitor(self, rhs: Number) -> Option<Number> {
		Some(Number::from(self.to_integer()? | rhs.to_integer()?))
	}
}

impl BitXor for Number {
	type Output = Option<Number>;
	fn bitxor(self, rhs: Number) -> Option<Number> {
		Some(Number::from(self.to_integer()? ^ rhs.to_integer()?))
	}
}

impl Shl for Number {
	type Output = Option<Number>;
	fn shl(self, rhs: Number) -> Option<Number> {
		Some(Number::from(self.to_integer()? << rhs.to_integer()?))
	}
}

impl Shr for Number {
	type Output = Option<Number>;
	fn shr(self, rhs: Number) -> Option<Number> {
		Some(Number::from(self.to_integer()? >> rhs.to_integer()?))
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

impl PartialEq<Integer> for Number {
	fn eq(&self, whole: &Integer) -> bool {
		*self == Number::from(*whole)
	}
}

impl PartialOrd<Integer> for Number {
	fn partial_cmp(&self, whole: &Integer) -> Option<Ordering> {
		self.partial_cmp(&Number::from(*whole))
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
	(assign $oper:ident $this:ident $other:ident $env:ident) => {{
		let other = $other.read_into_num($env)?;
		$this.write().data.$oper(other);
		Ok($this.clone())
	}};

	($oper:ident $this:ident $other:ident $env:ident) => {{
		let other = $other.read_into_num($env)?;
		Ok($this.read().data.$oper(other).into_object())
	}}
}


macro_rules! bit_oper {
	(assign $oper:ident $this:ident $other:ident $env:ident) => ({
		let other = $other.read_into_num($env)?;
		let val = $this.read().data.$oper(other)
			.ok_or_else(|| 
				Error::BadArguments {
					args: vec![$this.clone(), $other.clone()],
					descr: concat!("whole numbers are needed for `", stringify!($oper), "`")
				}
			)?;
		$this.write().data = val;
		Ok($this.clone())
	});

	($oper:tt $this:ident $other:ident $env:ident) => {{
		let other = $other.read_into_num($env);
		if let Ok(other) = $other.read_into_num($env) {
			if let (Some(this), Some(other)) = ($this.read().data.to_integer(), other.to_integer()) {
				return Ok(this.$oper(other).into_object());
			}
		}
		Err(Error::BadArguments {
			args: vec![$this.clone(), $other.clone()],
			descr: concat!("whole numbers are needed for `", stringify!($oper), "`")
		})
	}}
}


__impl_type! {
	for Number, with self attr;

	fn "@bool" (this) {
		Ok((!this.read().data.is_zero()).into_object())
	}

	fn "@num" (this) {
		Ok(this.read().duplicate())
	}

	fn "()" (this) env, args, {
		this.read_call(&"*".into_anyshared(), args, env)
	}

	fn "+"  (this, rhs) env, { binary_oper!(add this rhs env) }
	fn "-"  (this, rhs) env, { binary_oper!(sub this rhs env) }
	fn "*"  (this, rhs) env, { binary_oper!(mul this rhs env) }
	fn "/"  (this, rhs) env, { binary_oper!(div this rhs env) }
	fn "%"  (this, rhs) env, { binary_oper!(rem this rhs env) }
	fn "^" (this, rhs) env, { binary_oper!(pow this rhs env) }

	fn "+="  (this, rhs) env, { binary_oper!(assign add_assign this rhs env) }
	fn "-="  (this, rhs) env, { binary_oper!(assign sub_assign this rhs env) }
	fn "*="  (this, rhs) env, { binary_oper!(assign mul_assign this rhs env) }
	fn "/="  (this, rhs) env, { binary_oper!(assign div_assign this rhs env) }
	fn "%="  (this, rhs) env, { binary_oper!(assign rem_assign this rhs env) }
	fn "6=" (this, rhs) env, { binary_oper!(assign pow_assign this rhs env) }

	fn "bitand" (this, rhs) env, { bit_oper!(bitand this rhs env) }
	fn "bitor" (this, rhs) env, { bit_oper!(bitor this rhs env) }
	fn "bitxor" (this, rhs) env, { bit_oper!(bitxor this rhs env) }
	fn "<<" (this, rhs) env, { bit_oper!(shl this rhs env) }
	fn ">>" (this, rhs) env, { bit_oper!(shr this rhs env) }

	fn "&=" (this, rhs) env, { bit_oper!(assign bitand this rhs env) }
	fn "|=" (this, rhs) env, { bit_oper!(assign bitor this rhs env) }
	fn "^=" (this, rhs) env, { bit_oper!(assign bitxor this rhs env) }
	fn "<<=" (this, rhs) env, { bit_oper!(assign shl this rhs env) }
	fn ">>=" (this, rhs) env, { bit_oper!(assign shr this rhs env) }

	fn "<"  (this, rhs) env, { let other = rhs.read_into_num(env)?; Ok((this.read().data <  other).into_object()) }
	fn "<=" (this, rhs) env, { let other = rhs.read_into_num(env)?; Ok((this.read().data <= other).into_object()) }
	fn ">"  (this, rhs) env, { let other = rhs.read_into_num(env)?; Ok((this.read().data >  other).into_object()) }
	fn ">=" (this, rhs) env, { let other = rhs.read_into_num(env)?; Ok((this.read().data >= other).into_object()) }
	fn "==" (this, rhs) env, { let other = rhs.read_into_num(env)?; Ok((this.read().data == other).into_object()) }

	fn "@++" (this) {
		let prev = this.read().data;
		this.write().data += Number::one();
		Ok(prev.into_object())
	}

	fn "++@" (this) {
		this.write().data += Number::one();
		Ok(this.read().duplicate())
	}


	fn "@--" (this) {
		let prev = this.read().data;
		this.write().data -= Number::one();
		Ok(prev.into_object())
	}

	fn "--@" (this) {
		this.write().data -= Number::one();
		Ok(this.read().duplicate())
	}

	fn "<=>" (this, rhs) env, {
		let rhs = rhs.read_into_num(env)?;
		Ok(match this.read().data.cmp(&rhs) {
			Ordering::Less => Number::neg_one(),
			Ordering::Equal => Number::zero(),
			Ordering::Greater => Number::one(),
		}.into_object())
	}

/*
	fn "." (this, rhs) env, {
		if let Ok(rhs) = rhs.read_into_num(env) {
			let num = Number { frac: rhs.whole, ..this.read().data };
			Ok(num.into_object() as AnyShared)
		} else {
			unimplemented!("TODO: attrs for num");
			// Ok(any::__get_default(self, rhs).map(|x| x as AnyShared).unwrap_or(Object::null))
		}
	}
*/

	fn _ () {
		any::__get_default(self, attr)
	}
}


#[cfg(test)]
mod test {
	use super::*;

	fn s(sign: Sign, amnt: Unsigned) -> Number {
		Number::new(sign, amnt, 0)
	}

	fn i(amnt: Integer) -> Number {
		Number::from(amnt)
	}

	#[test]
	fn creation(){
		assert_eq!(Number::new(Positive, 43, 12841), Number { sign: Positive, whole: 43, frac: 12841 });
		assert_eq!(Number::new(Negative, 43, 12841), Number { sign: Negative, whole: 43, frac: 12841 });
		assert_eq!(Number::from(0), Number { sign: Positive, whole: 0, frac: 0 });
		assert_eq!(Number::from(-0), Number { sign: Positive, whole: 0, frac: 0 });
		assert_eq!(Number::from(54), Number { sign: Positive, whole: 54, frac: 0 });
		assert_eq!(Number::from(-192351), Number { sign: Negative, whole: 192351, frac: 0 });
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






















