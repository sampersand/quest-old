use parse::{Parsable, Stream};
use obj::{Object, AnyShared, AnyResult, SharedObject, types::{IntoObject, HasDefaults}};
use obj::types::{Var, Number, any, BoundFn};
use env::Environment;


// impl HasDefaults for Object<bool> {
// 	fn get_default_var(&self, attr: &str, env: &Environment) -> Option<AnyResult> {
// 		match attr {
// 			"@bool" => Some(Ok(
// 				self.bind_to(|this, _, _| Ok(this.read().duplicate()))
// 					.into_anyshared()
// 				)),
// 			"foo" => Some(Ok(self.bind_to(|_, _, _| Ok(12.into_anyshared())).into_anyshared())),
// 			_ => any::get_default(self, ::obj::Id::from_nonstatic_str(attr).into_anyshared(), env)
// 		}
// 	}
// }

__impl_type! {
	for bool, with self attr;

	fn "@bool" (this) {
		Ok(this.read().duplicate())
	}

	fn "@num" (this) {
		Ok(Number::from(this.read().data as Integer).into_object())
	}

	fn _ () {
		any::__get_default(self, attr)
	}
}

// impl_type! {
// 	type bool, with self attr ele env;
// 	for<Var> { ele.data.id_str() } {
// 		"@bool" => fn (this, {
// 			Ok(this.read().duplicate())
// 		}),

// 		"@num" => fn (this, {
// 			Ok(Number::from(this.read().data as Integer).into_object())
// 		}),

// 		_ => None ()
// 	}

// 	_ => eval {
// 		any::get_default(self, attr.clone(), env)
// 	}
// }

// impl HasDefaults for Object<bool> {
// 	fn get_default_var(&self, attr: &str, env: &Environment) -> Option<AnyResult> {
// 		match attr {
// 			"@bool" => Some(Ok(
// 				self.bind_to(|this, _, _| Ok(this.read().duplicate()))
// 					.into_anyshared()
// 				)),
// 			"foo" => Some(Ok(self.bind_to(|_, _, _| Ok(12.into_anyshared())).into_anyshared())),
// 			_ => any::get_default(self, ::obj::Id::from_nonstatic_str(attr).into_anyshared(), env)
// 		}
// 	}
// }

// impl IntoObject for bool {
// 	type Type = bool;
// 	fn into_object(self) -> SharedObject<bool> {
// 		Object::new(self)
// 	}
// }

// 		impl $crate::obj::types::IntoObject for $type {
// 			type Type = Self;
// 			fn into_object(self) -> $crate::obj::SharedObject<Self> {
// 				$crate::obj::Object::new(self).into()
// 			}
// 		}

// 		impl $crate::obj::types::HasDefaults for $crate::obj::Object<$type> {
// 			fn get_default($self: &Self, $attr: $crate::obj::AnyShared, $env: &$crate::env::Environment) -> Option<$crate::obj::AnyShared> {
// 				match_defaults!(with $self $attr $ele $env;
// 					$(for<$ty> $parse_pre {
// 						$($fn => $fn_ty $fn_args),*
// 					})*
// 					_ => $else_ty $else_args
// 				)
// 			}
// 		}

// 		impl $crate::obj::types::Type for $crate::obj::Object<$type> {
// 			fn display_fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
// 				::std::fmt::Display::fmt(&self.data, f)
// 			}
// 		}
// 	}