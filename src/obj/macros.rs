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

	(main $fn:tt $ty:ty; $env:ident $args:ident $obj:ident [$($req:ident)*] [$($opt:ident $opt_val:expr),*] $block:block) => {
		|obj, $args, $env| -> Result<AnyObject> {
			let $obj: $ty = AnyObject::downcast(obj).expect(concat!("invalid argument passed to `", stringify!($ty), "`.", $fn));

			const MIN_ARG_LEN: usize = argcount!($($req)*);
			assert!($args.len() >= MIN_ARG_LEN, "A minimum of {} args are required, but only {} were found", MIN_ARG_LEN, $args.len());

			__define_attr_fn!(assign_args $args 0; [$($req)*] [$($opt $opt_val,)*]);

			// match $block { // so type inferences can be made
			// 	Ok(obj) => Some($crate::env::Token::Obj(obj as AnyObject)),
			// 	Err(err) => Err(err)
			// }
			unimplemented!()
		}
	};

	($fn:tt $ty:ty, $fn_args:tt $body:block) => (__define_attr_fn!($fn $ty, $fn_args with env args $body));
	($fn:tt $ty:ty, $fn_args:tt with $body:block) => (__define_attr_fn!($fn $ty, $fn_args with env args $body));
	($fn:tt $ty:ty, $fn_args:tt with $env:ident $body:block) => (__define_attr_fn!($fn $ty, $fn_args with $env args $body));
	($fn:tt $ty:ty, $fn_args:tt with $env:ident $args:ident $body:block) => (__define_attr_fn!($fn $ty, $fn_args with $env $args obj $body));

	($fn:tt $ty:ty, () with $env:ident $args:ident $obj:ident $body:block) => {
		__define_attr_fn!(main $fn $ty; $env $args $obj [] [] $body)
	};


	($fn:tt $ty:ty, (_ $(,$required:ident)* $(; $optional:ident = $val:expr)*) with $env:ident $args:ident $obj:ident $body:block) => {
		__define_attr_fn!(main $fn $ty; $env $args $obj [$($required)*] [$($optional $val),*] $body);
	};

	($fn:tt $ty:ty, ($this:ident $(,$required:ident)* $(; $optional:ident = $val:expr)*) with $env:ident $args:ident $obj:ident $body:block) => {
		__define_attr_fn!(main $fn $ty; $env $args $obj [$($required)*] [$($optional $val),*] { let $this = &*$obj.read(); $body } )
	};


	($fn:tt $ty:ty, (mut $this:ident $(,$required:ident)* $(; $optional:ident = $val:expr)*) with $env:ident $args:ident $obj:ident $body:block) => {
		__define_attr_fn!(main $fn $ty; $env $args $obj [$($required)*] [$($optional $val),*] { let mut $this = &mut *$obj.write(); $body } )
	};

	($fn:tt $ty:ty, ($this:ident $(,$required:ident)* $(; $optional:ident = $val:expr)*) with $env:ident $args:ident $obj:ident $body:block) => {
		__define_attr_fn!(main $fn $ty; $env $args $obj [$($required)*] [$($optional $val),*] { let $this = &*$obj.read(); $body } )
	};
}

macro_rules! define_attrs {
	(for $ty:ty; $(use $include:ty;)* $(fn $fn:tt $fn_args:tt $($params:ident)* $body:block)*) => {
		define_attrs!(for $ty, with Literal; $(use $include;)* $(fn $fn $fn_args $($params)* $body)*);
	};

	(for $ty:ty, with $precedence:ident; $(use $include:ty;)* $(fn $fn:tt $fn_args:tt $($params:ident)* $body:block)*) => {
		impl $crate::obj::classes::Class for $ty {
			const GET_DEFAULTS: $crate::obj::classes::GetDefault = |obj, id| DEFAULT_ATTRS.get(id).map(|func| func.bind_to(obj.clone()) as _);
			const HAS_DEFAULTS: $crate::obj::classes::HasDefault = |id| DEFAULT_ATTRS.contains_key(id) ;
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
					h.insert(Id::from($fn), boundfn::BindableFn(__define_attr_fn!($fn $ty, $fn_args $($params)* $body)));
				)*
				h
			};
		}
	}
}

