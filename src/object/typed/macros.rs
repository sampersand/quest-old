macro_rules! impl_typed_object {
	($ty:ident, $new:ident, $downcast:ident) => { impl_typed_object!(@; $ty, $new, $downcast); };
	($ty:ident, _, $downcast:ident) => { impl_typed_object!(@; $ty,, $downcast); };
	(@; $ty:ident, $($new:ident)?, $downcast:ident) => {
		impl From<$ty> for $crate::object::typed::Types {
			fn from(val: $ty) -> Self {
				$crate::object::typed::Types::$ty(val)
			}
		}

		impl From<$ty> for $crate::object::TypedObject {
			fn from(obj: $ty) -> Self {
				$crate::object::TypedObject::new(obj)
			}
		}


		impl $crate::object::TypedObject {
			$(
				pub fn $new<T: Into<$ty>>(val: T) -> Self {
					$crate::object::TypedObject::new(val.into())
				}
			)?

			pub fn $downcast(&self) -> Option<&$ty> {
				if let $crate::object::typed::Types::$ty(ref val) = self.data {
					Some(val)
				} else {
					None
				}
			}
		}

		impl $crate::Object {
			/// note: this clones the object
			pub fn $downcast(&self) -> Option<$ty> {
				self.map().read()
				    .downcast_ref::<$crate::object::TypedObject>()
				    .and_then($crate::object::TypedObject::$downcast)
				    .cloned()
			}
		}

		impl $crate::object::IntoObject for $ty {
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
		let $req: &$crate::Object = *$args.get($pos).ok_or_else(|| $crate::Error::MissingArgument {
			func: $name,
			pos: $pos
		})?;
		_assign_args!($args $name, $pos + 1, [$($oreq)*] $opt)
	};

	($args:ident $name:expr, $pos:expr, [] [$opt:ident=$val:expr, $($other:tt)*]) => {
		let $opt: &$crate::Object = *$args.get($pos).unwrap_or_else(|| $val);
		_assign_args!($args $name, $pos + 1, [] [$($other)*])
	}
}
// !($name, 0, $self [$($req)*] [$($opt $val),*]);

macro_rules! _create_rustfn {
	((_ $(,$req:ident)* $(;$opt:ident=$val:expr)*) $body:block $downcast:ident $name:expr) => (|args| {
		_assign_args!(args $name, 0, [$($req)*] [$($opt $val,)*]);
		Ok($body)
	});

	((@noread; $($req:ident),* $(;$opt:ident=$val:expr)*) $body:block $downcast:ident $name:expr) => (|args| {
		_assign_args!(args $name, 0, [$($req)*] [$($opt $val,)*]);
		Ok($body)
	});

	(($self:ident $(,$req:ident)* $(;$opt:ident=$val:expr)*) $body:block $downcast:ident $name:expr) => (|args| {
		_assign_args!(args $name, 0, [$self $($req)*] [$($opt $val,)*]);
		let $self = $self.$downcast()
			.ok_or_else(|| $crate::Error::BadArgument {
				func: $name,
				msg: concat!($name, " called with bad `self` argument"),
				position: 0,
				arg: $self.clone()
			})?;
		Ok($body)
	});

	($bad_args:tt $body:block $downcast:ident $name:expr) => {
		compile_error!(concat!("Bad args for `", $name, "`: ", stringify!($bad_args)))
	}
}
macro_rules! impl_type {
	(for $ty:ty, downcast_fn = $downcast:ident; $(fn $name:tt $args:tt $body:block)* ) => {
		impl $crate::object::typed::Type for $ty {
			fn create_mapping() -> $crate::Shared<dyn $crate::Mapping> {
				use $crate::{Shared, Object, object::IntoObject};
				use $crate::object::typed::*;
				lazy_static::lazy_static! {
					static ref PARENT: $crate::Object = $crate::Object::new({
						let mut map = $crate::collections::Map::default();
						$({
							map.set(
								_name_to_object!($name),
								{
									macro_rules! internal_name {
										() => (concat!(stringify!($ty), "::", stringify!($name) ));
									}

									TypedObject::new_rustfn(
										internal_name!(),
										_create_rustfn!($args $body $downcast internal_name!())
									).objectify()
								}
							);
						})*
						map
					});
				}
				$crate::Shared::new(
					$crate::collections::ParentalMap::new_default(|| PARENT.clone())
				)
			}
		}
	}
}