use parse::{Parsable, Stream, ParseResult};
use env::Environment;
use obj::{Object, AnyShared, SharedObject, types::IntoObject};
use std::ops::{Deref, DerefMut};
use std::fmt::{self, Debug, Display, Formatter};

use obj::Id;
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Var(Id);

impl Parsable for Id {
	fn parse(stream: &mut Stream, env: &Environment) -> ParseResult {
		let mut offset = 0;
		let mut eval = true;
		{
			let mut chars = stream.chars();

			offset = match chars.next()? {
				c if c.is_alphabetic() => c.len_utf8(),
				c @ '_' | c @ '@' | c @ '$' => c.len_utf8(),
				'`' => { eval = false; '`'.len_utf8() },
				_ => return None
			};

			for chr in chars {
				if chr.is_alphanumeric() || chr == '_' || chr == '?' || chr == '!' {
					offset += chr.len_utf8();
				} else {
					break;
				}
			}
		}

		let id = Id::from_nonstatic_str(stream.offset_to(offset));

		let func = move || {
			env.get(&(id.into_object() as AnyShared)).unwrap_or_else(Object::null)
		};

		if eval {
			Some(Box::new(func))
		} else {
			Some(Box::new(id))
		}
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

impl_type! {
	for Var, with self attr;

	fn "@text" (this) {
		Ok(this.read().data.0.try_as_str().expect("All Var Ids should have `str`s associated with them?").into_object())
	}

	fn "?" (this) env, {
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

