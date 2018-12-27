macro_rules! __impl_type_pat_inner {
	($_fn:expr, $_args:ident $_pos:expr; [] []) => {};

	($fn:expr, $args:ident $pos:expr; [] [$opt:ident $(; $oopt:ident)*]) => {
		let $opt = $args.get($pos).map(|x| (*x).clone());
		__impl_type_pat_inner!($fn, $args $pos + 1; [] [$($oopt)*])
	};

	($fn:expr, $args:ident $pos:expr; [$req:ident $($oreq:ident)*] $opts:tt) => {
		let $req = $args.get($pos).expect(concat!("Missing expected arg `", stringify!($req), "` for `", $fn, "`")).clone();
		__impl_type_pat_inner!($fn, $args $pos + 1; [$($oreq)*] $opts)
	};
}

macro_rules! __impl_type_pat {
	($_fn:expr, $_self:ident () $body:block) => { $body };
	($_fn:expr, $_self:ident (_) $body:block) => { Some(Ok(BoundFnOld::bind_void(|_, _| $body).into_anyshared())) };

	($fn:expr, $self:ident $fnargs:tt $body:block) => {
		__impl_type_pat!($fn, $self $fnargs $body _);
	};

	($fn:expr, $self:ident $fnargs:tt $body:block $env:pat) => {
		__impl_type_pat!($fn, $self $fnargs $body $env, _args);
	};


	($_fn:expr, $self:ident ($this:pat) $body:block $env:pat, $args:pat) => {
		Some(Ok($self.upgrade().bind_to__(|$this, $args, $env| $body).into_anyshared()))
	};

	($fn:expr, $self:ident ($this:ident $(, $req:ident)* $(; $opt:ident)*) $body:block $env:pat, $args:ident) => {
		Some(Ok($self.upgrade().bind_to__(|$this, $args, $env| {
			__impl_type_pat_inner!($fn, $args 0; [$($req)*] [$($opt)*]);
			$body
		}).into_anyshared()))
	};
}

macro_rules! __impl_type {
	(defaults fn $fn_name:ident<$T:ident>($obj:ident, $attr:ident) where {$($params:tt)*};
		$(static ref $static:ident: $static_ty:ty = $static_body:expr;)*
		$(fn $fn:tt $args:tt $($vars:pat,)* $body:block)*
	) => {
		pub fn $fn_name<$T: 'static>($obj: &::obj::Object<$T>, $attr: AnyShared) -> Option<::obj::AnyResult>
			where $($params)* {
			lazy_static! {
				$(
					static ref $static: $static_ty = $static_body;
				)*
			}

			use obj::{*, types::*};
			let attr = {
					$attr.read().downcast_ref::<Var>().map(|x| x.data)
						.or_else(|| $attr.read().downcast_ref::<Missing>().map(|x| x.data.into()))?
						.try_as_str().expect("bad data str")
			};

			match attr {
				$(
					$fn => __impl_type_pat!(concat!(stringify!($type), ".", $fn), $obj $args $body $($vars),*)
				),*
			}
		}
	};

	(for $type:ty, with $self:ident $attr:ident; 
		$(static ref $static:ident: $static_ty:ty = $static_body:expr;)*
		$(fn $fn:tt $args:tt $($vars:pat,)* $body:block)*
	) => {
		lazy_static! {
			$(
				static ref $static: $static_ty = $static_body;
			)*
		}


		// impl ::obj::types::IntoObject for $type {
		// 	type Type = Self;
		// 	fn into_object(self) -> ::obj::SharedObject<Self> {
		// 		::obj::Object::new(self).into()
		// 	}
		// }

		impl $crate::obj::types::HasDefaults for $crate::obj::Object<$type> {
			fn get_default($self: &Self, $attr: $crate::obj::AnyShared, _: &$crate::env::Environment) -> Option<$crate::obj::types::AnyResult> {
				use obj::{*, types::*};
				let attr = {
						$attr.read().downcast_ref::<Var>().map(|x| x.data)
							.or_else(|| $attr.read().downcast_ref::<Missing>().map(|x| x.data.into()))
							.map(|x| x.try_as_str().expect("bad data str"))
				};

				match attr {
					$(
						Some($fn) => __impl_type_pat!(concat!(stringify!($type), ".", $fn), $self $args $body $($vars),*)
					,)*
					None => None//unimplemented!("cant do stuff")
				}
			}
		}
	}
}

#[macro_use]
mod macros;

mod bool;
mod text;
mod any;
mod boundfn_old;
mod boundfn;
mod null;
mod num;
mod shared;
mod map;
mod list;
mod var;
mod block;
mod env;


pub use self::boundfn_old::BoundFnOld;
pub use self::boundfn::BoundFn;
pub use self::num::{Number, Sign, Integer};
pub use self::map::Map;
pub use self::list::List;
pub use self::null::Null;
pub use self::text::Text;
pub use self::var::{Var, RawVar, Missing};
pub use self::block::{Block, BlockExec};

use std::fmt::{Debug, Display};
use std::hash::Hash;
use env::Environment;
use obj::{AnyShared, AnyResult, SharedObject, Object};

pub trait HasDefaults {
	fn get_default(&self, attr: AnyShared, env: &Environment) -> Option<AnyResult> {
		self.get_default_var(attr.read().downcast_ref::<Var>()?.data.id_str(), env)
	}

	fn get_default_var(&self, attr: &str, env: &Environment) -> Option<AnyResult> {
		None
	}
}

pub trait Type : Debug + Display + Eq + Send + Sync + 'static where Object<Self>: HasDefaults {}
// pub trait Type : Debug + Display + Eq + Send + Sync + 'static where Object<Self>: HasDefaults {}

impl<T: Debug + Display + Eq + Send + Sync + 'static> Type for T where Object<T>: HasDefaults {}

pub trait IntoObject : Sized {
	type Type : Send + Sync + 'static + ?Sized;
	fn into_object(self) -> SharedObject<Self::Type>;
	fn into_anyshared(self) -> AnyShared where Self::Type: Sized {
		self.into_object() as AnyShared
	}
}

impl<T: Type> IntoObject for T where Object<T>: HasDefaults {
	type Type = Self;
	fn into_object(self) -> SharedObject<Self> {
		Object::new(self)
	}
}

impl<T: Send + Sync + 'static + ?Sized> IntoObject for SharedObject<T> {
	type Type = T;
	fn into_object(self) -> Self {
		self
	}
}
