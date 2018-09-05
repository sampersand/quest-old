use parse::{Parsable, Stream, Token};
use env::{Environment, Executable};
use obj::{Object, AnyShared, SharedObject, types::IntoObject};
use std::ops::{Deref, DerefMut};
use std::fmt::{self, Debug, Display, Formatter};

use obj::Id;
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Var(Id);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Missing(Id);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct RawVar; // this is _just_ for implementing Parsable for "`" vars

impl Var {
	pub fn id_str(&self) -> &'static str {
		self.0.try_as_str().expect("no str associated with variable?")
	}
}

impl Parsable for RawVar {
	fn parse(stream: &mut Stream) -> Option<Token> {
		let mut offset;
		{
			let mut chars = stream.chars();
			if chars.next()? != '`' {
				return None;
			}
			offset = '`'.len_utf8();
			loop {
				let chr = chars.next()?;
				offset += chr.len_utf8();
				match chr {
					'\\' => offset += chars.next()?.len_utf8(),
					'`' => break,
				_ => { /* we already added the offset here */ }
				}
			}
		}
		let text = stream.offset_to(offset);
		let id = text[1..text.len() - 1].to_string();
		Some(Token::new_literal(text, Default::default(), move || Id::from_nonstatic_str(&id).into_object()))
	}
}
impl Parsable for Var {
	fn parse(stream: &mut Stream) -> Option<Token> {
		let mut offset;
		{
			let mut chars = stream.chars();

			let mut is_special = false;
			let mut c = chars.next()?;
			offset = c.len_utf8();
			match c {
				'@' | '$'  => is_special = true,
				'_' => {},
				_ if c.is_alphabetic() => {},
				_ => return None
			}

			if is_special {
				offset += chars.next()?.len_utf8();
			}

			for c in chars {
				match c {
					_ if c.is_alphanumeric() => offset += c.len_utf8(),
					'_' | '?' | '!' => offset += c.len_utf8(),
					_ => break // if we find anything else, we're done
				}
			}
		}

		let text = stream.offset_to(offset);
		let id = text.to_string();

		Some(Token::new_env(text, Default::default(), move |env| {
			let id = Id::from_nonstatic_str(&id);
		env.push(env.get(&id.into_anyshared()).unwrap_or_else(|| Missing(id).into_anyshared()));
			Ok(())
		}))
	}
}


impl From<Id> for Var {
	#[inline]
	fn from(id: Id) -> Var {
		Var(id)
	}
}

impl IntoObject for &'static str {
	type Type = Var;
	fn into_object(self) -> SharedObject<Var> {
		Id::from(self).into_object()
	}
}


impl IntoObject for Id {
	type Type = Var;
	fn into_object(self) -> SharedObject<Var> {
		Var::from(self).into_object()
	}
}

impl Deref for Var {
	type Target = Id;

	#[inline]
	fn deref(&self) -> &Id {
		&self.0
	}
}

impl DerefMut for Var {
	#[inline]
	fn deref_mut(&mut self) -> &mut Id {
		&mut self.0
	}
}

impl Display for Var {
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		Display::fmt(&self.0, f)
	}
}

impl Display for Missing {
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		write!(f, "<missing {}>", self.0)
	}
}

impl From<Missing> for Var {
	fn from(missing: Missing) -> Var {
		Var::from(missing.0)
	}
}

impl_type! {
	for Missing, with self attr;

	fn "@bool" (_) {
		Ok(false.into_object())
	}

	fn "=" (this, val) env, {
		env.set(Var::from(this.read().data.0).into_object(), val.clone());
		Ok(val.clone())
	}

	fn _ () {
		any::get_default_attr(self, attr)
	}
}

impl_type! {
	for Var, with self attr;

	fn "@text" (this) {
		Ok(this.read().data.id_str().into_object())
	}

	fn "@bool" (this) env, {
		Ok(env.has(&(this.clone() as AnyShared)).into_object())
	}

	fn "=" (this, val) env, {
		env.set(this.clone(), val.clone());
		Ok(val.clone())
	}

	fn "~" (this) env, {
		Ok(env.del(&(this.clone() as AnyShared)).unwrap_or_else(Object::null))
	}

	fn "()" (this) env, {
		// todo: remove the `clone` here and instead cast
		Ok(env.get(&(this.clone() as AnyShared)).unwrap_or_else(Object::null))
	}

	fn _ () {
		any::get_default_attr(self, attr)
	}
}

