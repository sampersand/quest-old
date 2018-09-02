use parse::{Parsable, Stream, ParseResult};
use env::Environment;

use std::ops::{Deref, DerefMut};
use std::hash::{Hash, Hasher};
use std::fmt::{self, Display, Formatter};
use obj::{AnyShared, SharedObject, types::IntoObject};
use super::shared::{self, Offset::*};

#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
pub struct Block(Vec<AnyShared>);

impl Parsable for Block {
	fn parse(stream: &mut Stream, env: &Environment) -> ParseResult {
		if stream.starts_with(']') {
			stream.eof = true;
			return None;
		}

		if !stream.starts_with('[') {
			return None;
		}
		None

// {
// 	1 + 2 // }
// }

// 		stream.offset_by("[");
// 		let env = env.

// 		// let mut c
// 		// if !stream.starts_with('[')
// 		unimplemented!()
	}
}
/*

impl Block {
	#[inline]
	pub fn new(vec: Vec<AnyShared>) -> Self {
		Block(vec)
	}
}

impl From<Vec<AnyShared>> for Block {
	#[inline]
	fn from(vec: Vec<AnyShared>) -> Block {
		Block::new(vec)
	}
}

impl IntoObject for Vec<AnyShared> {
	type Type = Block;
	#[inline]
	fn into_object(self) -> SharedObject<Block> {
		Block::from(self).into_object()
	}
}

impl<'a> IntoObject for &'a [AnyShared] {
	type Type = Block;
	#[inline]
	fn into_object(self) -> SharedObject<Block> {
		Block::from(self.to_vec()).into_object()
	}
}

impl Display for Block {
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		write!(f, "[{}]", self.0.iter().map(ToString::to_string).collect::<Vec<_>>().join(", "))
	}
}

impl Deref for Block {
	type Target = Vec<AnyShared>;

	#[inline]
	fn deref(&self) -> &Vec<AnyShared> {
		&self.0
	}
}

impl DerefMut for Block {
	#[inline]
	fn deref_mut(&mut self) -> &mut Vec<AnyShared> {
		&mut self.0
	}
}

impl_type! {
	for Block, with self attr;

	fn "@bool" (this) {
		Ok((!this.read().data.is_empty()).into_object())
	}

	fn "@map" (this) {
		Ok(this.read()
		       .data
		       .iter()
		       .enumerate()
		       .map(|(i, o)| (i.into_anyobject(), o.clone()))
		       .collect::<::std::collections::HashMap<AnyShared, AnyShared>>()
		       .into_anyobject()
		)
	}

	fn "len" (this) {
		Ok(this.read().data.len().into_object())
	}

	fn "count" (this, ele) {
		let ref ele = *ele.read();
		let ref data = this.read().data;
		Ok(data.iter().filter(|obj| &*obj.read() == ele).count().into_object())
	}

	fn "has?" (this, ele) env, {
		let ref ele = *ele.read();
		let ref data = this.read().data;
		Ok(data.iter().any(|obj| &*obj.read() == ele).into_object())
	}

	fn "get" (this, start; end) env, {
		let ref data = this.read().data;
		let start = start.read_into_num(env)?;
		let end_num_res = end.unwrap_or_else(Object::null).read_into_num(env);
		let len = data.len();

		let start_off = shared::offset(len, start)?;
		match end_num_res {
			Ok(end) => match (start_off, shared::offset(len, end)?) {
				(Valid(s), Valid(e)) if s < e => Ok(data[s..e].into_object()), // begin < end
				(Valid(_), Valid(_)) => Ok(vec![].into_object()), // begin >= end

				(Valid(s), OutOfBounds(_)) => Ok(data[s..].into_object()), 
				_ => Ok(Object::null()) // everything else is nil
			},

			Err(Error::MissingAttr { .. }) => match start_off {
				Valid(s) => Ok(data[s].clone()),
				OutOfBounds(_) | Underflow(_) => Ok(Object::null())
			},
			Err(other) => Err(other)
		}
	}

	fn "set" (this, pos, ele) env, {
		let pos = pos.read_into_num(env)?;
		let ref mut data = this.write().data;
		match shared::offset(data.len(), pos)? {
			Valid(n) => {
				let old = data[n].clone();
				data[n] = ele;
				Ok(old)
			},
			OutOfBounds(n) => {
				data.reserve(n);
				for _ in 0..(n - 1) {
					data.push(Object::null())
				}
				data.push(ele);
				Ok(Object::null())
			},
			Underflow(_) => unimplemented!("TODO: error for out of bounds"),
		}
	}

	fn "del" (this, pos) env, {
		let pos = pos.read_into_num(env)?;
		let ref mut data = this.write().data;
		match shared::offset(data.len(), pos)? {
			Valid(n) => Ok(data.remove(n)),
			OutOfBounds(_) | Underflow(_) => Ok(Object::null())
		}
	}

	fn "push" (this, ele) {
		this.write().data.push(ele.clone());
		Ok(ele)
	}

	fn "pop" (this) {
		Ok(this.write().data.pop().unwrap_or_else(Object::null))
	}

	fn "<<" (this) env, args, {
		this.read_call(&"push".into_anyobject(), args, env)
	}

	fn "+" (this) env, args, {
		this.read().duplicate().read_call(&"+".into_anyobject(), args, env)
	}

	fn "+=" (this, other) env, {
		let other = other.read_into_list(env)?;
		this.write().data.extend_from_slice(&other);
		Ok(this)
	}

	fn "-" (this) env, args, {
		this.read().duplicate().read_call(&"+".into_anyobject(), args, env)
	}

	fn "-=" (this, other) env, {
		let other = other.read_into_list(env)?;
		{
			let ref mut data = this.write().data;
			for ele in other.iter() {
				if let Some(ind) = data.iter().position(|x| x == ele) {
					data.remove(ind);
				}
			}
		}
		Ok(this)
	}

	fn "|" (this) env, args, {
		this.read().duplicate().read_call(&"|".into_anyobject(), args, env)
	}

	fn "|=" (this, other) env, {
		let other = other.read_into_list(env)?;
		{
			let ref mut data = this.write().data;
			data.reserve(other.len());
			for ele in other.0.into_iter() {
				if !data.contains(&ele) {
					data.push(ele)
				}
			}
			data.shrink_to_fit();
		}
		Ok(this)
	}

	fn "&" (this) env, args, {
		this.read().duplicate().read_call(&"&".into_anyobject(), args, env)
	}

	fn "&=" (this, other) env, {
		let other = other.read_into_list(env)?;
		this.write().data.retain(|x| other.contains(x));
		Ok(this)
	}

	fn "^" (this) env, args, {
		this.read().duplicate().read_call(&"^".into_anyobject(), args, env)
	}

	fn "^=" (this, other) env, {
		let other = other.read_into_list(env)?;
		{
			let ref mut data = this.write().data;
			data.retain(|x| !other.contains(x));
			let o = other.0.iter().filter(|e| !data.contains(e)).map(Clone::clone).collect::<Vec<_>>();
			data.extend_from_slice(&o);
		}
		Ok(this)
	}

	fn "uniq" (this) env, args, {
		this.read().duplicate().read_call(&"uniq!".into_anyobject(), args, env)
	}

	fn "uniq!" (this) env, args, {
		{
			let ref mut data = this.write().data;
			let mut i = 0;
			while i < data.len() {
				if data[i..].iter().find(|&x| x == &data[i]).is_some() {
					data.remove(i);
				} else {
					i += 1;
				}
			}
		}
		Ok(this)
	}


	fn _ () {
		any::get_default_attr(self, attr)
	}
}




*/