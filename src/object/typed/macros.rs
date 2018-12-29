macro_rules! impl_typed_object {
	($ty:ident, $new:ident, $downcast:ident) => { impl_typed_object!(@; $ty, $new, $downcast); };
	($ty:ident, _, $downcast:ident) => { impl_typed_object!(@; $ty,, $downcast); };
	(@; $ty:ident, $($new:ident)?, $downcast:ident) => {
		impl From<$ty> for Types {
			fn from(id: $ty) -> Types {
				Types::$ty(id)
			}
		}


		impl TypedObject {
			$(
				pub fn $new<T: Into<$ty>>(val: T) -> Self {
					TypedObject::new(val.into())
				}
			)?

			pub fn $downcast(&self) -> Option<&$ty> {
				if let Types::$ty(ref val) = self.data {
					Some(val)
				} else {
					None
				}
			}

		}

		impl Shared<Object> {
			/// note: this clones the object
			pub fn $downcast(&self) -> Option<$ty> {
				self.read().map.read()
				    .downcast_ref::<TypedObject>()
				    .and_then(TypedObject::$downcast)
				    .cloned()
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

macro_rules! _create_rustfn {
	($name:ident $args:tt $body:block) => (|args| {
		unimplemented!();
	})
}
macro_rules! impl_type {
	(for $ty:ty; $(fn $name:tt $args:tt $body:block)* ) => {
		impl $crate::object::typed::Type for $ty {
			fn create_mapping() -> $crate::Shared<dyn $crate::Mapping> {
				lazy_static::lazy_static! {
					static ref PARENT: $crate::Shared<$crate::Object> = $crate::Shared::new($crate::Object::new({
						let mut map = $crate::collections::Map::default();
						$({
							map.set(
								_name_to_object!($name),
								{
									const INTERNAL_NAME: &'static str = concat!(
										stringify!($ty), "::", stringify!($name)
									);

									TypedObject::new_rustfn(
										INTERNAL_NAME,
										_create_rustfn!(INTERNAL_NAME $args $body)
									).objectify()
								}
							);
						})*
						map
					}));
				}
				$crate::Shared::new(
					$crate::collections::ParentalMap::new_default(PARENT.clone())
				)
			}
		}
	}
}