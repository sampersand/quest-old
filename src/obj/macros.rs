macro_rules! __define_attr_fn {
	(assign_args $args:ident $num:expr; ; []) => { };
	(assign_args $args:ident $num:expr; ; [$optional:ident = $opt:expr, $($rest:tt)*]) => {
		let $optional = $args.get($num).unwrap_or_else(|| $opt);
		__define_attr_fn!(assign_args args $num + 1; ; [$($rest)*] );
	};

	(assign_args $args:ident $num:expr; $required:ident $($required_other:ident)*; $optional:tt) => {
		let $required = $args[$num];
		__define_attr_fn!(assign_args args $num + 1; $($required_other)*; $optional);
	};

	($ty:ty, $fn_args:tt $body:block) => (__define_attr_fn!($ty, $fn_args with env args $body));
	($ty:ty, $fn_args:tt with $body:block) => (__define_attr_fn!($ty, $fn_args with env args $body));
	($ty:ty, $fn_args:tt with $env:ident $body:block) => (__define_attr_fn!($ty, $fn_args with $env args $body));
	($ty:ty, $fn_args:tt with $env:ident $args:ident $body:block) => (__define_attr_fn!($ty, $fn_args with $env $args obj $body));
	($ty:ty, () with $env:ident $args:ident $obj:ident $body:block) => {
		|$obj: &Shared<QObject<$ty>>, $args: &[&SharedObject], $env: &Environment| -> Result<SharedObject> {
			let res: Result<_> = $body;
			Ok(Shared::from(res?))
		}
	};
	($ty:ty, (mut $this:ident $(,$required:ident)* $(; $optional:ident = $val:expr)*) with $env:ident $args:ident $obj:ident $body:block) => {
		|$obj: &Shared<QObject<$ty>>, $args: &[&SharedObject], $env: &Environment| -> Result<SharedObject> {
			const MIN_ARG_LEN: usize = argcount!($($required)*);
			assert!($args.len() >= MIN_ARG_LEN, "A minimum of {} args are required, but only {} were found", MIN_ARG_LEN, $args.len());
			let mut $this = &mut *$obj.write();
			__define_attr_fn!(assign_args $args 0; $($required)*; [$($optional = $val,)*]);
			let res: Result<_> = $body;
			Ok(Shared::from(res?))
		}
	};

	($ty:ty, ($this:ident $(,$required:ident)* $(; $optional:ident = $val:expr)*) with $env:ident $args:ident $obj:ident $body:block) => {
		|$obj: &Shared<QObject<$ty>>, $args: &[&SharedObject], $env: &Environment| -> Result<SharedObject> {
			const MIN_ARG_LEN: usize = argcount!($($required)*);
			assert!($args.len() >= MIN_ARG_LEN, "A minimum of {} args are required, but only {} were found", MIN_ARG_LEN, $args.len());
			let $this = &*$obj.read();
			__define_attr_fn!(assign_args $args 0; $($required)*; [$($optional = $val,)*]);
			let res: Result<_> = $body;
			Ok(Shared::from(res?))
		}
	};
}

macro_rules! define_attrs {
	(static ref $name:ident for $ty:ty; $(use $include:ty;)* $(fn $fn:tt $fn_args:tt $($params:ident)* $body:block)*) => {
		lazy_static! {
			static ref $name: ::obj::classes::DefaultAttrs<$ty> = {
				use obj::{Id, SharedObject, classes::*};
				use env::Environment;
				use shared::Shared;
				let mut h = DefaultAttrs::<$ty>::new();
				$(
					h.extend(<$include>::default_attrs().iter());
				)*
				$(
					h.insert(Id::from($fn), __define_attr_fn!($ty, $fn_args $($params)* $body));
				)*
				h
			};
		}
	}
}
