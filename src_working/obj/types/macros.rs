macro_rules! impl_type_pat_inner {
	($_args:ident $_pos:expr; [] []) => {};

	($args:ident $pos:expr; [] [$opt:ident $(; $oopt:ident)*]) => {
		let $opt = $args.get($pos).map(|x| (*x).clone());
		impl_type_pat_inner!($args $pos + 1; [] [$($oopt)*])
	};

	($args:ident $pos:expr; [$req:ident $($oreq:ident)*] $opts:tt) => {
		let $req = $args.get($pos).expect(concat!("Missing expected arg `", stringify!($req), "`")).clone();
		impl_type_pat_inner!($args $pos + 1; [$($oreq)*] $opts)
	};
}

macro_rules! impl_type_pat {
	// (fn $_self:ident () $body:block $_env:pat, $_args:pat) => { $body };
	// (fn $_self:ident (_) $body:block $_env:pat, $_args:pat) => { Some(BoundFnOld::bind_void(|_, _| $body)) };

	// (fn self:ident $args:tt $body:block) => {
	// 	impl_type_pat!(fn $self $args $body _env);
	// };

 	// fn(this, {...})
	(fn $self:ident ($this:pat, $body:block)) => {
		impl_type_pat!(fn $self($this, _, _, () $body));
	};

 	// fn(this, () {...})
	(fn $self:ident ($this:pat, () $body:expr)) => {
		impl_type_pat!(fn $self($this, _, _, () $body));
	};

	// fn(this, (...) {...})
	(fn $self:ident ($this:pat, $params:tt $body:expr)) => {
		impl_type_pat!(fn $self($this, _, args, $params $body));
	};

	// fn(this, env:pat, {...})
	(fn $self:ident ($this:pat, $env:pat, $body:expr)) => {
		impl_type_pat!(fn $self($this, $env, _, () $body));
	};

	// fn(this, env:pat, () {...})
	(fn $self:ident ($this:pat, $env:pat, () $body:expr)) => {
		impl_type_pat!(fn $self($this, $env, _, $params $body));
	};

	// fn(this, env:pat, (...) {...})
	(fn $self:ident ($this:pat, $env:pat, $params:tt $body:expr)) => {
		impl_type_pat!(fn $self($this, $env, args, $params $body));
	};

	// fn(this, env:pat, args:pat, {...})
	(fn $self:ident ($this:pat, $env:pat, $args:pat, $body:expr)) => {
		Some(Ok($self.upgrade().bind_to__(move |$this, $args, $env| $body).into_anyshared()))
	};
	
	// fn(this, env:pat, args:ident, (...) {...})
	(fn $self:ident ($this:pat, $env:pat, $args:ident, ($($req:ident),* $(; $opt:ident)*) $body:expr)) => {
		Some(Ok($self.upgrade().bind_to__(move|$this, $args, $env| {
			impl_type_pat_inner!($args 0; [$($req)*] [$($opt)*]);
			$body
		}).into_anyshared()))
	};

	// fn(this, env:pat, args:pat, () {...})
	(fn $self:ident ($this:pat, $env:pat, $args:tt, () $body:expr)) => {
		Some(Ok($self.upgrade().bind_to__(move |$this, $args, $env| $body).into_anyshared()))
	};

	(fn $_self:ident ($_this:pat, $_env:pat, $args:pat, $params:tt $body:expr)) => {
		compile_error!(concat!("Args needs to be present for parametered functions (got `", stringify!($args), "`, with params: `", stringify!($params), "`"));
	};

	(fn $_self:ident $bad:tt) => {
		compile_error!(concat!("Invalid `fn` statement `", stringify!($bad), "`"));
	};

	(eval $self:ident $body:block) => {
		$body
	};

	(eval $self:ident ($this:pat, $body:block)) => {
		{
			let $this = $self.data;
			$body
		}
	};
	(eval $_self:ident $_bad:tt) => {
		compile_error!("Invalid")
	};

	(None $_self:ident ()) => { None };
	(None $_self:ident $_badargs:tt) => {
		compile_error!("Invalid `None` statement")
	};

	($other:ident $_self:ident $_badother:tt) => {
		compile_error!(concat!("Unrecognized type `", stringify!($other), "`"))
	}
}

macro_rules! match_defaults {
	(with $self:ident $attr:ident $ele:ident $env:ident;
		$(for<$ty:ty> $parse_pre:block {
			$($fn:pat => $fn_ty:ident $fn_args:tt),*
		})*
		_ => $else_ty:ident $else_args:tt
	) => {
		{
			use obj::types::*;
			Option::None::<AnyResult>
			$(
				.or_else(|| {
					let attr = $attr.read();
					if let Some($ele) = attr.downcast_ref::<$ty>() {
						match $parse_pre {
							$($fn => impl_type_pat!($fn_ty $self $fn_args)),*
						}
					} else {
						None
					}
				})
			)*
			.or_else(|| impl_type_pat!($else_ty $self $else_args))
		}
	}
}

macro_rules! impl_type {
	(type $type:ty, with $self:ident $attr:ident $ele:ident $env:ident;
		$(for<$ty:ty> $parse_pre:block {
			$($fn:pat => $fn_ty:ident $fn_args:tt),*
		})*
		_ => $else_ty:ident $else_args:tt
	) => {

		impl $crate::obj::types::HasDefaults for $crate::obj::Object<$type> {
			fn get_default($self: &Self, $attr: $crate::obj::AnyShared, $env: &$crate::env::Environment) -> Option<$crate::obj::AnyResult> {
				match_defaults!(with $self $attr $ele $env;
					$(for<$ty> $parse_pre {
						$($fn => $fn_ty $fn_args),*
					})*
					_ => $else_ty $else_args
				)
			}
		}
	}
}

// 	(for $type:ty, with $self:ident $attr:ident; 
// 		$(static ref $static:ident: $static_ty:ty = $static_body:expr;)*
// 		$(fn $fn:tt $args:tt $($vars:pat,)* $body:block)*
// 	) => {
// 		lazy_static! {
// 			$(
// 				static ref $static: $static_ty = $static_body;
// 			)*
// 		}


// 		impl ::obj::types::IntoObject for $type {
// 			type Type = Self;
// 			fn into_object(self) -> ::obj::SharedObject<Self> {
// 				::obj::Object::new(self).into()
// 			}
// 		}

// 		impl $crate::obj::types::Type for $crate::obj::Object<$type> {
// 			fn get_default($self: &Self, $attr: $crate::obj::AnyShared) -> Option<$crate::obj::types::BoundFnOld> {
// 				use obj::{*, types::*};
// 				let attr = {
// 						$attr.read().downcast_ref::<Var>().map(|x| x.data)
// 							.or_else(|| $attr.read().downcast_ref::<Missing>().map(|x| x.data.into()))
// 							.map(|x| x.try_as_str().expect("bad data str"))
// 				};

// 				match attr {
// 					$(
// 						Some($fn) => impl_type_pat!(concat!(stringify!($type), ".", $fn), $self $args $body $($vars),*)
// 					,)*
// 					None => unimplemented!()
// 				}
// 			}

// 			fn display_fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
// 				::std::fmt::Display::fmt(&self.data, f)
// 			}
// 		}
// 	}
// }
