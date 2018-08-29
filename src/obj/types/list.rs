use std::ops::{Deref, DerefMut};
use std::hash::{Hash, Hasher};
use std::fmt::{self, Display, Formatter};
use obj::{AnyShared, SharedObject, types::IntoObject};
use super::shared::{self, Offset::*};

#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
pub struct List(Vec<AnyShared>);


impl From<Vec<AnyShared>> for List {
	#[inline]
	fn from(vec: Vec<AnyShared>) -> List {
		List(vec)
	}
}

impl IntoObject for Vec<AnyShared> {
	type Type = List;
	#[inline]
	fn into_object(self) -> SharedObject<List> {
		List::from(self).into_object()
	}
}

impl<'a> IntoObject for &'a [AnyShared] {
	type Type = List;
	#[inline]
	fn into_object(self) -> SharedObject<List> {
		List::from(self.to_vec()).into_object()
	}
}

impl Display for List {
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		write!(f, "[{}]", self.0.iter().map(ToString::to_string).collect::<Vec<_>>().join(", "))
	}
}

impl Deref for List {
	type Target = Vec<AnyShared>;

	#[inline]
	fn deref(&self) -> &Vec<AnyShared> {
		&self.0
	}
}

impl DerefMut for List {
	#[inline]
	fn deref_mut(&mut self) -> &mut Vec<AnyShared> {
		&mut self.0
	}
}

impl_type! {
	for List, with self attr;

	fn "@bool" (this) {
		Ok((!this.is_empty()).into_object())
	}

	fn "len" (this) {
		Ok(this.len().into_object())
	}

	fn "has" (this, ele) {
		Ok(this.0.iter().any(|obj| *obj.read() == *ele).into_object())
	}

	fn "get" (this, pos) env, {
		match shared::offset(this.data.len(), pos.attrs.into_num(env)?)? {
			Valid(n) => Ok(this[n].clone()),
			OutOfBounds(_) | Underflow(_) => unimplemented!("TODO: error for out of bounds"),
		}
	}

	fn "set" (mut this, pos, shared ele) env, {
		match shared::offset(this.data.len(), pos.attrs.into_num(env)?)? {
			Valid(n) => this[n] = ele.clone(),
			OutOfBounds(1) => this.push(ele.clone()),
			OutOfBounds(_) | Underflow(_) => unimplemented!("TODO: error for out of bounds"),
		};
		Ok(ele.clone())
	}

	fn "del" (mut this, pos) env, {
		match shared::offset(this.data.len(), pos.attrs.into_num(env)?)? {
			Valid(n) => Ok(this.remove(n)),
			OutOfBounds(_) | Underflow(_) => unimplemented!("TODO: error for out of bounds")
		}
	}

	fn "[]" (this, start; end = Object::null()) env, {
		let start = start.attrs.into_num(env)?;
		let len = this.data.len();

		let start_off = shared::offset(len, start)?;
		match end.attrs.into_num(env) {
			Ok(end) => match (start_off, shared::offset(len, end)?) {
				(Valid(s), Valid(e)) if s < e => Ok(this[s..e].into_object()), // begin < end
				(Valid(_), Valid(_)) => Ok(vec![].into_object()), // begin >= end

				(Valid(s), OutOfBounds(_)) => Ok(this[s..].into_object()), 
				_ => Ok(Object::null()) // everything else is nil
			},

			Err(Error::MissingAttr { .. }) => match start_off {
				Valid(s) => Ok(this[s].clone()),
				OutOfBounds(_) | Underflow(_) => Ok(Object::null())
			},
			Err(other) => Err(other)
		}
	}

	fn "[]=" (mut this, pos, shared ele) env, {
		match shared::offset(this.data.len(), pos.attrs.into_num(env)?)? {
			Valid(n) => this[n] = ele.clone(),
			OutOfBounds(n) => {
				this.reserve(n);
				for _ in 0..(n - 1) {
					this.push(Object::null())
				}
				this.push(ele.clone());
			},
			Underflow(_) => unimplemented!("TODO: error for out of bounds"),
		};
		Ok(ele.clone())
	}

	fn "[]~" (mut this, pos) env, {
		match shared::offset(this.data.len(), pos.attrs.into_num(env)?)? {
			Valid(n) => Ok(this.remove(n)),
			OutOfBounds(_) | Underflow(_) => Ok(Object::null())
		}
	}

	fn "push" (mut this, shared ele) {
		this.data.push(ele.clone());
		Ok(ele.clone())
	}

	fn "pop" (mut this) {
		Ok(this.data.pop().unwrap_or_else(|| Null.into_object()))
	}

	fn "<<" (shared this) env, args, {
		this.read_call("push", args, env)
	}

	fn _ (_) {
		any::get_default_attr(self, attr)
	}
}






