use parse::{Parsable, Stream, Token};
use env::{Environment, Executable};

use std::ops::{Deref, DerefMut};
use std::hash::{Hash, Hasher};
use std::fmt::{self, Display, Formatter};
use obj::{AnyShared, SharedObject, Error, types::IntoObject};
use super::shared::{self, Offset::*};
use super::block;

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct List(Vec<AnyShared>);

impl Parsable for List {
	fn parse(stream: &mut Stream) -> Option<Token> {
		let (raw, data) = block::parse_block('[', ']', stream)?;
		Some(Token::new_env(raw, Default::default(), move |env| {
			let env_new = env.new_stack();
			env_new.execute(data.into_iter())?;
			env.push(env_new.stack().clone());
			Ok(())
		}))

		// if stream.as_str().starts_with(']') {
		// 	stream.eof = true;
		// 	return None;
		// }

		// if !stream.as_str().starts_with('[') {
		// 	return None;
		// }

		// None

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

impl List {
	#[inline]
	pub fn new(vec: Vec<AnyShared>) -> Self {
		List(vec)
	}

	pub fn is_empty(&self) -> bool {
		self.0.is_empty()
	}

	pub fn __get(&self, pos: usize) -> Option<AnyShared> {
		self.0.get(pos).cloned()
	}
}

impl From<Vec<AnyShared>> for List {
	#[inline]
	fn from(vec: Vec<AnyShared>) -> List {
		List::new(vec)
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
	type List, with self attr ele env;
	for<Var> { ele.data.id_str() } {

		"@list" => fn(this, {
			Ok(this.read().duplicate())
		}),

		"@bool" => fn(this, {
			Ok((!this.read().data.is_empty()).into_object())
		}),

		"@map" => fn(this, {
			Ok(this.read()
			       .data
			       .iter()
			       .enumerate()
			       .map(|(i, o)| (i.into_anyshared(), o.clone()))
			       .collect::<::map::ObjMap>()
			       .into_object()
			)
		}),

		"len" => fn(this, {
			Ok(this.read().data.len().into_object())
		}),

		"count" => fn(this, (ele) {
			let ref ele = *ele.read();
			let ref data = this.read().data;
			Ok(data.iter().filter(|obj| &*obj.read() == ele).count().into_object())
		}),

		"has?" => fn(this, env, (ele) {
			let ref ele = *ele.read();
			let ref data = this.read().data;
			Ok(data.iter().any(|obj| &*obj.read() == ele).into_object())
		}),

		"get" => fn(this, env, (start/*; end*/) {
			let end = None;
			let ref data = this.read().data;
			let start = start.read_into_num(env)?;
			let end_num_res = end.unwrap_or_else(Object::null).read_into_num(env);
			let start = start - Number::from(1);
			let end_num_res = end_num_res.map(|e| - Number::from(1));
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
		}),

		"set" => fn(this, env, (pos, ele) {
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
		}),

		"del" => fn(this, env, (pos) {
			let pos = pos.read_into_num(env)?;
			let ref mut data = this.write().data;
			match shared::offset(data.len(), pos)? {
				Valid(n) => Ok(data.remove(n)),
				OutOfBounds(_) | Underflow(_) => Ok(Object::null())
			}
		}),

		"push" => fn(this, (ele) {
			this.write().data.push(ele.clone());
			Ok(ele)
		}),

		"pop" => fn(this, {
			Ok(this.write().data.pop().unwrap_or_else(Object::null))
		}),

		"+" => fn(this, env, args, {
			this.read().duplicate().read_call(&"+=".into_anyshared(), args, env)
		}),

		"+=" => fn(this, env, (other) {
			let other = other.read_into_list(env)?;
			this.write().data.extend_from_slice(&other);
			Ok(this)
		}),

		"-" => fn(this, env, args, {
			this.read().duplicate().read_call(&"-=".into_anyshared(), args, env)
		}),

		"-=" => fn(this, env, (other) {
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
		}),

		"bitor" => fn(this, env, args, {
			this.read().duplicate().read_call(&"bitor=".into_anyshared(), args, env)
		}),

		"bitor=" => fn(this, env, (other) {
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
		}),

		"bitand" => fn(this, env, args, {
			this.read().duplicate().read_call(&"bitand".into_anyshared(), args, env)
		}),

		"bitand=" => fn(this, env, (other) {
			let other = other.read_into_list(env)?;
			this.write().data.retain(|x| other.contains(x));
			Ok(this)
		}),

		"bitxor" => fn(this, env, args, {
			this.read().duplicate().read_call(&"bitxor".into_anyshared(), args, env)
		}),

		"bitxor=" => fn(this, env, (other) {
			let other = other.read_into_list(env)?;
			{
				let ref mut data = this.write().data;
				data.retain(|x| !other.contains(x));
				let o = other.0.iter().filter(|e| !data.contains(e)).cloned().collect::<Vec<_>>();
				data.extend_from_slice(&o);
			}
			Ok(this)
		}),

		"uniq" => fn(this, env, args, {
			this.read().duplicate().read_call(&"uniq!".into_anyshared(), args, env)
		}),

		"uniq!" => fn(this, {
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
		}),

		_ => None ()
	}

	for<Number> { ele.data } {
		num => eval {
		// 	unimplemented!("hi")
		// }fn(this, env, args, {
			let num_up = num.into_anyshared();
			Some(self.attrs.get(&"get".into_anyshared(), env)
				.expect("`get` doesnt exist for List?")
				.read_call(&"()".into_anyshared(), &[num_up], env))
		}
	}

	_ => eval {
		any::get_default(self, attr.clone(), env)
	}
}
