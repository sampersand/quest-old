macro_rules! __define_attr_fn {
	(assign_args $args:ident $num:expr; [] []) => { };
	(assign_args $args:ident $num:expr; [] [$optional:ident opt:expr, $($opt:ident $opt_val:expr),*]) => {
		let $optional = $args.get($num).unwrap_or_else(|| $opt);
		__define_attr_fn!(assign_args args $num + 1; ; [$($opt $opt_val)*] );
	};

	(assign_args $args:ident $num:expr; [$required:ident $($required_other:ident)*] $optional:tt) => {
		let $required = $args[$num];
		__define_attr_fn!(assign_args args $num + 1; $($required_other)*; $optional);
	};

	(main $ty:ty; $env:ident $args:ident $obj:ident [$($req:ident)*] [$($opt:ident $opt_val:expr),*] $block:block) => {
		|obj: &AnyObject, $args: &[&AnyObject], $env: &Environment| -> Result<AnyObject> {
			// let $obj = cast!
			const MIN_ARG_LEN: usize = argcount!($($req)*);
			assert!($args.len() >= MIN_ARG_LEN, "A minimum of {} args are required, but only {} were found", MIN_ARG_LEN, $args.len());

			__define_attr_fn!(assign_args $args 0; [$($req)*] [$($opt $opt_val,)*]);
			match $block { // so type inferences can be made
				Ok(obj) => Ok(obj as AnyObject),
				Err(err) => Err(err)
			}
		}
	};

	($ty:ty, $fn_args:tt $body:block) => (__define_attr_fn!($ty, $fn_args with env args $body));
	($ty:ty, $fn_args:tt with $body:block) => (__define_attr_fn!($ty, $fn_args with env args $body));
	($ty:ty, $fn_args:tt with $env:ident $body:block) => (__define_attr_fn!($ty, $fn_args with $env args $body));
	($ty:ty, $fn_args:tt with $env:ident $args:ident $body:block) => (__define_attr_fn!($ty, $fn_args with $env $args obj $body));
	($ty:ty, () with $env:ident $args:ident $obj:ident $body:block) => {
		|$obj: &AnyObject, $args: &[&AnyObject], $env: &Environment| -> Result<AnyObject> {
			let $obj: $ty = unimplemented!();
			match $body {
				Ok(obj) => Ok(obj as AnyObject),
				Err(err) => Err(err)
			}
		}
	};
	($ty:ty, (mut $this:ident $(,$required:ident)* $(; $optional:ident = $val:expr)*) with $env:ident $args:ident $obj:ident $body:block) => {
		|$obj: &AnyObject, $args: &[&AnyObject], $env: &Environment| -> Result<AnyObject> {
			const MIN_ARG_LEN: usize = argcount!($($required)*);
			assert!($args.len() >= MIN_ARG_LEN, "A minimum of {} args are required, but only {} were found", MIN_ARG_LEN, $args.len());
			let $obj: $ty = unimplemented!();
			let mut $this = &mut *$obj.write();
			__define_attr_fn!(assign_args $args 0; $($required)*; [$($optional = $val,)*]);
			match $body {
				Ok(obj) => Ok(obj as AnyObject),
				Err(err) => Err(err)
			}
		}
	};
	($ty:ty, ($this:ident $(,$required:ident)* $(; $optional:ident = $val:expr)*) with $env:ident $args:ident $obj:ident $body:block) => {
		__define_attr_fn!(main $ty; $env $args $obj [$($required)*] [$($optional $val),*] { let $this = &*$obj.read(); $body } )
		// |obj: &AnyObject, $args: &[&AnyObject], $env: &Environment| -> Result<AnyObject> {
		// 	// __define_attr_fn!(downcast_obj
		// 	let $obj: $ty = unimplemented!();
		// 	const MIN_ARG_LEN: usize = argcount!($($required)*);
		// 	assert!($args.len() >= MIN_ARG_LEN, "A minimum of {} args are required, but only {} were found", MIN_ARG_LEN, $args.len());
		// 	let $this = &*$obj.read();
		// 	__define_attr_fn!(assign_args $args 0; $($required)*; [$($optional = $val,)*]);
		// 	match $body {
		// 		Ok(obj) => Ok(obj as AnyObject),
		// 		Err(err) => Err(err)
		// 	}
		// }
	};
}

macro_rules! define_attrs {
	(for $ty:ty; $(use $include:ty;)* $(fn $fn:tt $fn_args:tt $($params:ident)* $body:block)*) => {
		impl $crate::obj::classes::QuestClass for $ty {
			const GET_DEFAULTS: fn(&$crate::obj::AnyObject, &$crate::obj::Id) -> Option<$crate::obj::AnyObject> = |obj, id| {
				if let Some(func) = DEFAULT_ATTRS.get(id) {
					Some(func.bind_to(obj.clone()) as _)
				} else {
					None
				}
			};

			const HAS_DEFAULTS: fn(&$crate::obj::AnyObject, &$crate::obj::Id) -> bool = |obj, id| {
				unimplemented!()
			};
		}

		lazy_static! {
			static ref DEFAULT_ATTRS: ::std::collections::HashMap<$crate::obj::Id, $crate::obj::classes::boundfn::BindableFn> = {
				use $crate::obj::{Id, Result, AnyObject, QObject, classes::*};
				use $crate::env::Environment;
				use $crate::shared::Shared;
				let mut h = ::std::collections::HashMap::<Id, boundfn::BindableFn>::new();
				// $(
				// 	h.extend(<$include>::get_default_attrs().iter());
				// )*
				$(
					h.insert(Id::from($fn), boundfn::BindableFn(__define_attr_fn!($ty, $fn_args $($params)* $body)));
				)*
				h
			};
		}
	}
}






