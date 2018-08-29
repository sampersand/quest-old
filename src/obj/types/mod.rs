macro_rules! impl_type_pat_inner {
	($_fn:expr, $_args:ident $_pos:expr; [] []) => {};


	($fn:expr, $args:ident $pos:expr; [] [shared $opt:ident $opt_val:expr $(,$($oopt:ident)* $oopt_val:expr)*]) => {
		let $opt = *$args.get($pos).map(|x| (**x).clone()).unwrap_or_else(|| $opt_val);
		impl_type_pat_inner!($fn, $args $pos + 1; [] [$($oopt $oopt_val),*])
	};

	($fn:expr, $args:ident $pos:expr; [] [mut $opt:ident $opt_val:expr $(,$($oopt:ident)* $oopt_val:expr)*]) => {
		let opt_shared = $args.get($pos).map(|x| (**x).clone()).unwrap_or_else(|| $opt_val);
		let ref mut $opt = *opt_shared.write();
		impl_type_pat_inner!($fn, $args $pos + 1; [] [$($oopt $oopt_val),*])
	};

	($fn:expr, $args:ident $pos:expr; [] [$opt:ident $opt_val:expr $(,$($oopt:ident)* $oopt_val:expr)*]) => {
		let opt_shared = $args.get($pos).map(|x| (**x).clone()).unwrap_or_else(|| $opt_val);
		let ref $opt = *opt_shared.read();
		impl_type_pat_inner!($fn, $args $pos + 1; [] [$($oopt $oopt_val),*])
	};

	($_fn:expr, $_args:ident $_pos:expr; [] [$bad:ident $opt:ident $($other:tt)*]) => {
		compiler_error!("argument must either be `", $opt, "`, `mut ", $opt, "`, or `shared ", $opt, "` not `", $bad, " ", $opt, "`")
	};


	($fn:expr, $args:ident $pos:expr; [shared $req:ident $(,$($oreq:ident)*)*] $opts:tt) => {
		let $req = *$args.get($pos).expect(concat!("Missing expected arg `", stringify!($req), "` for `", $fn, "`"));
		impl_type_pat_inner!($fn, $args $pos + 1; [$($($oreq)*),*] $opts)
	};

	($fn:expr, $args:ident $pos:expr; [mut $req:ident $(,$($oreq:ident)*)*] $opts:tt) => {
		let req_shared = $args.get($pos).expect(concat!("Missing expected arg `", stringify!($req), "` for `", $fn, "`"));
		let ref mut $req = *req_shared.write();
		impl_type_pat_inner!($fn, $args $pos + 1; [$($($oreq)*),*] $opts)
	};

	($fn:expr, $args:ident $pos:expr; [$req:ident $(,$($oreq:ident)*)*] $opts:tt) => {
		let req_shared = $args.get($pos).expect(concat!("Missing expected arg `", stringify!($req), "` for `", $fn, "`"));
		let ref $req = *req_shared.read();
		impl_type_pat_inner!($fn, $args $pos + 1; [$($($oreq)*),*] $opts)
	};

	($_fn:expr, $_args:ident $_pos:expr; [$bad:ident $req:ident $(,$($_oreq:ident)*)*] $_opts:tt) => {
		compiler_error!("argument must either be `", $req, "`, `mut ", $req, "`, or `shared ", $req, "` not `", $bad, " ", $req, "`")
	};

}

macro_rules! impl_type_pat {
	($_fn:expr, $_self:ident (_) $body:block) => { $body };

	($fn:expr, $self:ident $fn_args:tt $body:block) => {
		impl_type_pat!($fn, $self $fn_args $body _);
	};

	($fn:expr, $self:ident $fn_args:tt $body:block $env:pat) => {
		impl_type_pat!($fn, $self $fn_args $body $env, _args);
	};


	($_fn:expr, $self:ident (shared $this:ident) $body:block $env:pat, $args:pat) => {
		Some($self.upgrade().bind_to_shared(|$this, $args, $env| $body))
	};

	($_fn:expr, $self:ident (mut $this:ident) $body:block $env:pat, $args:pat) => {
		Some($self.upgrade().bind_to_mut(|$this, $args, $env| $body))
	};

	($_fn:expr, $self:ident ($this:ident) $body:block $env:pat, $args:pat) => {
		Some($self.upgrade().bind_to(|$this, $args, $env| $body))
	};


	($fn:expr, $self:ident (shared $this:ident $(,$($req:ident)*)* $(;$($opt:ident)* = $val:expr)*) $body:block $env:pat, $args:ident) => {
		Some($self.upgrade().bind_to_shared(|$this, $args, $env| {
			impl_type_pat_inner!($fn, $args 0; [$($($req)*),*] [$($($opt)* $val),*]);
			$body
		}))
	};

	($fn:expr, $self:ident (mut $this:ident $(,$($req:ident)*)* $(;$($opt:ident)* = $val:expr)*) $body:block $env:pat, $args:ident) => {
		Some($self.upgrade().bind_to_mut(|$this, $args, $env| {
			impl_type_pat_inner!($fn, $args 0; [$($($req)*),*] [$($($opt)* $val),*]);
			$body
		}))
	};

	($fn:expr, $self:ident ($this:ident $(,$($req:ident)*)* $(;$($opt:ident)* = $val:expr)*) $body:block $env:pat, $args:ident) => {
		Some($self.upgrade().bind_to(|$this, $args, $env| {
			impl_type_pat_inner!($fn, $args 0; [$($($req)*),*] [$($($opt)* $val),*]);
			$body
		}))
	}
}

macro_rules! impl_type {

	(for $type:ty, with $self:ident $attr:ident; 
		$(fn $fn:tt $args:tt $($vars:pat,)* $body:block)*
	) => {
		impl $crate::obj::types::Type for $crate::obj::Object<$type> {
			fn get_default_attr($self: &Self, $attr: &str) -> Option<$crate::obj::types::BoundFn> {
				use obj::{*, types::*};
				match $attr {
					$(
						$fn => impl_type_pat!(concat!(stringify!($type), ".", $fn), $self $args $body $($vars),*)
					),*
				}
			}

			fn debug_fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
				::std::fmt::Debug::fmt(&self.data(), f)
			}

			fn display_fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
				::std::fmt::Display::fmt(&self.data(), f)
			}
		}
	}
}

impl_type! {
	for !, with self attr;
	fn _ (_) { unreachable!(); }
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

use std::fmt::{self, Debug, Formatter};

pub trait Type {
	fn get_default_attr(&self, attr: &str) -> Option<BoundFn>;
	fn debug_fmt(&self, f: &mut Formatter) -> fmt::Result;
	fn display_fmt(&self, f: &mut Formatter) -> fmt::Result;
}

use std::hash::Hash;
use obj::{AnyShared, SharedObject, Object};

pub trait IntoObject : Sized where Object<Self::Type>: Type {
	type Type : Sized + Eq + Hash + Debug + 'static;
	fn into_object(self) -> SharedObject<Self::Type>;
	fn into_anyobject(self) -> AnyShared{
		self.into_object() as AnyShared
	}
}

impl<T: Debug + Eq + Hash + 'static> IntoObject for SharedObject<T> where Object<T>: Type {
	type Type = T;
	fn into_object(self) -> SharedObject<Self::Type> {
		self
	}
}

impl<T: Sized + Debug + Eq + Hash + 'static> IntoObject for T where Object<T>: Type {
	type Type = Self;
	fn into_object(self) -> SharedObject<Self::Type> {
		Object::new(self).into()
	}
}


pub use self::boundfn::BoundFn;
pub use self::num::{Number, Sign};
pub use self::map::Map;
pub use self::list::List;
pub use self::null::Null;
pub use self::text::Text;