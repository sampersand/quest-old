macro_rules! impl_type_pat_inner {
	($_fn:expr, $_args:ident $_pos:expr; [] []) => {};

	($fn:expr, $args:ident $pos:expr; [] [$opt:ident $(; $oopt:ident)*]) => {
		let $opt = $args.get($pos).map(|x| (*x).clone());
		impl_type_pat_inner!($fn, $args $pos + 1; [] [$($oopt)*])
	};

	($fn:expr, $args:ident $pos:expr; [$req:ident $($oreq:ident)*] $opts:tt) => {
		let $req = $args.get($pos).expect(concat!("Missing expected arg `", stringify!($req), "` for `", $fn, "`")).clone();
		impl_type_pat_inner!($fn, $args $pos + 1; [$($oreq)*] $opts)
	};
}

macro_rules! impl_type_pat {
	($_fn:expr, $_self:ident () $body:block) => { $body };
	($_fn:expr, $_self:ident (_) $body:block) => { Some(BoundFn::bind_void(|_, _| $body)) };

	($fn:expr, $self:ident $fnargs:tt $body:block) => {
		impl_type_pat!($fn, $self $fnargs $body _);
	};

	($fn:expr, $self:ident $fnargs:tt $body:block $env:pat) => {
		impl_type_pat!($fn, $self $fnargs $body $env, _args);
	};


	($_fn:expr, $self:ident ($this:pat) $body:block $env:pat, $args:pat) => {
		Some($self.upgrade().bind_to(|$this, $args, $env| $body))
	};

	($fn:expr, $self:ident ($this:ident $(, $req:ident)* $(; $opt:ident)*) $body:block $env:pat, $args:ident) => {
		Some($self.upgrade().bind_to(|$this, $args, $env| {
			impl_type_pat_inner!($fn, $args 0; [$($req)*] [$($opt)*]);
			$body
		}))
	};
}

macro_rules! impl_type {
	(defaults fn $fn_name:ident<$T:ident>($obj:ident, $attr:ident) where {$($params:tt)*};
		$(static ref $static:ident: $static_ty:ty = $static_body:expr;)*
		$(fn $fn:tt $args:tt $($vars:pat,)* $body:block)*
	) => {
		pub fn $fn_name<$T: 'static>($obj: &::obj::Object<$T>, $attr: &str) -> Option<::obj::types::BoundFn>
			where $($params)* {
			lazy_static! {
				$(
					static ref $static: $static_ty = $static_body;
				)*
			}

			use obj::{*, types::*};
			match $attr {
				$(
					$fn => impl_type_pat!(concat!(stringify!($type), ".", $fn), $obj $args $body $($vars),*)
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


		impl ::obj::types::IntoObject for $type {
			type Type = Self;
			fn into_object(self) -> ::obj::SharedObject<Self> {
				::obj::Object::new(self).into()
			}
		}

		impl $crate::obj::types::Type for $crate::obj::Object<$type> {
			fn get_default_attr($self: &Self, $attr: &str) -> Option<$crate::obj::types::BoundFn> {
				use obj::{*, types::*};
				match $attr {
					$(
						$fn => impl_type_pat!(concat!(stringify!($type), ".", $fn), $self $args $body $($vars),*)
					),*
				}
			}

			fn display_fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
				::std::fmt::Display::fmt(&self.data, f)
			}
		}
	}
}

mod bool;
mod text;
mod any;
mod boundfn;
mod null;
mod num;
mod shared;
mod map;
mod list;
mod var;
mod block;
pub mod opers;


pub use self::boundfn::BoundFn;
pub use self::num::{Number, Sign, Integer};
pub use self::map::Map;
pub use self::list::List;
pub use self::null::Null;
pub use self::text::Text;
pub use self::var::{Var, Missing};
pub use self::block::{Block, BlockExec};

use std::fmt::{self, Debug, Formatter};

pub trait Type {
	fn get_default_attr(&self, attr: &str) -> Option<BoundFn>;
	fn display_fmt(&self, f: &mut Formatter) -> fmt::Result;
}

use std::hash::Hash;
use env::Environment;
use obj::{AnyShared, SharedObject, Object};

pub trait IntoObject : Sized {
	type Type : Send + Sync + 'static + ?Sized;
	fn into_object(self) -> SharedObject<Self::Type>;
}

impl<T: Debug + Eq + Hash + Send + Sync + 'static> IntoObject for SharedObject<T> where Object<T>: Type {
	type Type = T;
	fn into_object(self) -> SharedObject<Self::Type> {
		self
	}
}