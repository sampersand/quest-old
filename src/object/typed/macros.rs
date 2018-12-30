macro_rules! impl_quest_conversion {
	($as_fn:ident -> $inner:ty, $func:literal $downcast_fn:ident) => {
		impl $crate::Object {
			/// note: this clones the object
			pub fn $as_fn(&self) -> ::std::result::Result<$inner, $crate::Error> {
				use $crate::object::IntoObject;
				self.call(&$func.into_object(), &[])?
					.$downcast_fn()
					.ok_or_else(|| $crate::Error::ConversionFailure {
						func: $func,
						obj: self.clone()
					})

			}
		}
	}
}
macro_rules! impl_typed_conversion {
	($obj:ty, $inner:ty) => {
		impl $crate::object::IntoObject for $inner {
			fn into_object(self) -> $crate::Object {
				<$obj>::from(self).into_object()
			}
		}

		impl From<$inner> for $obj {
			fn from(inner: $inner) -> Self {
				<$obj>::new(inner)
			}
		}

		impl AsRef<$inner> for $obj {
			fn as_ref(&self) -> &$inner {
				&self.0
			}
		}

		impl From<$obj> for $inner {
			fn from(obj: $obj) -> $inner {
				obj.into_inner()
			}
		}

	}
}
macro_rules! impl_typed_object {
	($obj:ident, $new:ident, $downcast:ident, $is:ident) => { impl_typed_object!(@; $obj, $new, $downcast, $is); };
	($obj:ident, _, $downcast:ident, $is:ident) => { impl_typed_object!(@; $obj,, $downcast, $is); };
	(@; $obj:ident, $($new:ident)?, $downcast:ident, $is:ident) => {
		impl From<$obj> for $crate::object::typed::Types {
			fn from(val: $obj) -> Self {
				$crate::object::typed::Types::$obj(val)
			}
		}

		impl From<$obj> for $crate::object::TypedObject {
			fn from(obj: $obj) -> Self {
				$crate::object::TypedObject::new(obj)
			}
		}


		impl $crate::object::TypedObject {
			$(
				pub fn $new<T: Into<$obj>>(val: T) -> Self {
					$crate::object::TypedObject::new(val.into())
				}
			)?

			pub fn $downcast(&self) -> Option<&$obj> {
				if let $crate::object::typed::Types::$obj(ref val) = self.data {
					Some(val)
				} else {
					None
				}
			}

			pub fn $is(&self) -> bool {
				self.$downcast().is_some()
			}
		}

		impl $crate::Object {
			/// note: this clones the object
			pub fn $downcast(&self) -> Option<$obj> {
				self.map().read()
				    .downcast_ref::<$crate::object::TypedObject>()
				    .and_then($crate::object::TypedObject::$downcast)
				    .cloned()
			}
			
			pub fn $is(&self) -> bool {
				self.$downcast().is_some()
			}
		}

		impl $crate::object::IntoObject for $obj {
			fn into_object(self) -> $crate::Object {
				$crate::object::TypedObject::from(self).objectify()
			}
		}
	}
}


macro_rules! _name_to_object {
	($name:literal) => {
		TypedObject::new_var(
			$name // NOTE: this will crash if `$name` is a number
		).objectify()
	};
	((var $($rest:tt)+)) => { TypedObject::new_var( stringify!($($rest)*) ) };
	((num $num:expr)) => { TypedObject::new_num( $num ) };
	($other:tt) => {
		compiler_error!(concat!("Invalid name specified: '", stringify!($other), "'"));
	};
}

macro_rules! _assign_args {
	($_args:ident $_name:expr, $_pos:expr, [] []) => {};

	($args:ident $name:expr, $pos:expr, [$req:ident $($oreq:ident)*] $opt:tt) => {
		let $req: &$crate::Object = *$args.get($pos).ok_or_else(||
			$crate::Error::MissingArgument { func: $name, pos: $pos })?;
		_assign_args!($args $name, $pos + 1, [$($oreq)*] $opt)
	};

	($args:ident $name:expr, $pos:expr, [] [$opt:ident=$val:expr, $($other:tt)*]) => {
		let $opt: &$crate::Object = *$args.get($pos).unwrap_or_else(|| $val);
		_assign_args!($args $name, $pos + 1, [] [$($other)*])
	}
}
// !($name, 0, $self [$($req)*] [$($opt $val),*]);

macro_rules! _create_rustfn {
	(, $($others:tt)*) => { _create_rustfn!(args, $($others)*); };

	($args_ident:ident, (_ $(,$req:ident)* $(;$opt:ident=$val:expr)*) $body:block $downcast:ident $name:expr) => (
		|$args_ident| {
			_assign_args!($args_ident $name, 0, [$($req)*] [$($opt $val,)*]);
			Ok($body)
		}
	);

	($args_ident:ident, (@ $($req:ident),* $(;$opt:ident=$val:expr)*) $body:block $downcast:ident $name:expr) => (
		|$args_ident| {
			_assign_args!($args_ident $name, 0, [$($req)*] [$($opt $val,)*]);
			Ok($body)
		}
	);

	($args_ident:ident, ($self:ident $(,$req:ident)* $(;$opt:ident=$val:expr)*) $body:block $downcast:ident $name:expr) => (
		|$args_ident| {
			_assign_args!($args_ident $name, 0, [$self $($req)*] [$($opt $val,)*]);
			let $self = $self.$downcast()
				.ok_or_else(|| $crate::Error::BadArgument {
					func: $name,
					msg: concat!($name, " called with bad `self` argument"),
					position: 0,
					obj: $self.clone()
				})?;
			Ok($body)
		}
	);

	($args_ident:ident, $bad_args:tt $body:block $downcast:ident $name:expr) => {
		compile_error!(concat!("Bad args for `", $name, "`: ", stringify!($bad_args)))
	}
}
macro_rules! impl_type {
	(for $obj:ty, downcast_fn = $downcast:ident; $(fn $name:tt $args:tt $($args_ident:ident)? $body:block)* ) => {
		impl_type!{
			for $obj,
			downcast_fn = $downcast,
			parent=$crate::object::typed::basic::BASIC_MAP;
			$(fn $name $args $($args_ident)?$body)*
		}
	};
	(for $obj:ty, downcast_fn = $downcast:ident, parent=$parent:expr; $(fn $name:tt $args:tt $($args_ident:ident)? $body:block)* ) => {
		impl $crate::object::typed::Type for $obj {
			fn create_mapping() -> $crate::Shared<dyn $crate::Mapping> {
				use lazy_static::lazy_static;
				use crate::{Shared, Object, collections::ParentalMap};
				lazy_static! {
					static ref PARENT: Object = 
						Object::new(ParentalMap::new_mapped(
							|| $parent.clone(),
							function_map!(
								prefix = stringify!($obj),
								downcast_fn = $downcast,
								$(fn $name $args $($args_ident)? $body)*
							)
						));
				}
				Shared::new(
					ParentalMap::new_default(|| PARENT.clone())
				)
			}
		}
	}
}

macro_rules! function_map {
	(prefix=$prefix:expr, downcast_fn = $downcast:ident,
	 $(fn $name:tt $args:tt $($args_ident:ident)? $body:block)* ) => {
		$crate::Object::new({
			let mut map = $crate::collections::Map::default();
			use $crate::err::{Error::*, Result};
			use $crate::{Shared, Object, object::IntoObject};
			use $crate::object::typed::*;

			$(map.set(_name_to_object!($name), {
				macro_rules! function {
					() => (concat!($prefix, "::", $name));
				}
				TypedObject::new_rustfn(
					function!(),
					_create_rustfn!($($args_ident)?, $args $body $downcast function!())
				).objectify()
			}); )*
			map
		});
	}
}





