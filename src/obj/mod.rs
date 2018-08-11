macro_rules! __default_attrs_helper {
	(nice_attr; "{}") => { "{{}}" };
	(nice_attr; $other:tt) => { $other };
	(assign_args; $_:expr, $vars:ident) => { };
	(assign_args; $num:expr, $vars:ident; $var:ident = $var_opt:expr $(;$other:ident = $other_opt:expr)*) => {
		let $var = $vars.get($num).map(|x| (**x).clone()).unwrap_or_else(|| $var_opt);
		__default_attrs_helper!(assign_args; $num + 1, $vars $(;$other = $other_opt)*);
	};
	(assign_args; $num:expr, $vars:ident $var:ident $($args:ident)* $(;$other:ident = $other_opt:expr)*) => {
		let $var = $vars[$num];
		__default_attrs_helper!(assign_args; $num + 1, $vars $($args)* $(;$other => $other_opt)*);
	};

	(fn $attr:tt $var:ident $args:tt [] $body:block $for:ty) => { __default_attrs_helper!(fn $attr $var $args [with] $body $for); };
	(fn $attr:tt $var:ident $args:tt [with] $body:block $for:ty) => { __default_attrs_helper!(fn $attr $var $args [with _env] $body $for); };
	(fn $attr:tt $var:ident $args:tt [with $env:ident] $body:block $for:ty) => { __default_attrs_helper!(fn $attr $var $args [with $env vars] $body $for); };
	(fn $attr:tt $var:ident $args:tt [with $env:ident $vars:ident] $body:block $for:ty) => { __default_attrs_helper!(fn $attr $var $args [with $env $vars obj] $body $for); };
	(fn $attr:tt $var:ident () [with $env:ident $vars:ident $obj:ident] $body:block $for:ty) => { __default_attrs_helper!(fn $attr $var (_args) [with $env $vars $obj] $body $for); };
	(fn $attr:tt QObj ($arg:ident $(,$args:ident)* $(; $opt:ident = $opt_val:expr)*) [with $env:ident $vars:ident $obj:ident] $body:block $for:ty) => {
		|$obj, $vars, $env| {
			const MIN_LEN: usize = argcount!($($args)*);
			assert_ge!($vars.len(), MIN_LEN, concat!("Invalid args length ({} < {}) for rustfn `", stringify!($for), ".", $attr, "`"), $vars.len(), MIN_LEN);

			let $arg: &$for = $obj.deref();
			__default_attrs_helper!(assign_args; 0, $vars $($args)* $(;$opt = $opt_val)*);
			$body
		}
	};

	(; fn $attr:tt $var:ident $cls:expr; $arg:ident ($($args:ident)* $(; $opt:ident = $opt_val:expr)*) $env:ident $vars:ident $obj:ident $body:block $for:ty) => {{
		use obj::object::Classes;
		const MIN_LEN: usize = argcount!($($args)*);
		assert_ge!($vars.len(), MIN_LEN, concat!("Invalid args length ({} < {}) for rustfn `", stringify!($for), ".", __default_attrs_helper!(nice_attr; $attr), "`"), $vars.len(), MIN_LEN);
		let cls = $cls;
		let $arg = match cls {
			Classes::$var(thing) => thing,
			other => panic!(concat!("Expected a `", stringify!($var), "`, found a `{}`"), other)
		};
		__default_attrs_helper!(assign_args; 0, $vars $($args)* $(;$opt = $opt_val)*);
		$body
	}};

	(fn $attr:tt $var:ident (mut $arg:ident $(,$args:ident)* $(; $opt:ident = $opt_val:expr)*) [with $env:ident $vars:ident $obj:ident] $body:block $for:ty) => {
		|$obj, $vars, $env| {
			use std::ops::DerefMut;
			let mut cls_ref = $obj.class_mut();
			let x = __default_attrs_helper!(; fn $attr $var cls_ref.deref_mut(); $arg ($($args)* $(; $opt = $opt_val)*) $env $vars $obj $body $for);
			drop(cls_ref);
			x
		}
	};

	(fn $attr:tt $var:ident ($arg:ident $(,$args:ident)* $(; $opt:ident = $opt_val:expr)*) [with $env:ident $vars:ident $obj:ident] $body:block $for:ty) => {
		|$obj, $vars, $env| {
			use std::ops::DerefMut;
			let cls_ref = $obj.class();
			let x = __default_attrs_helper!(; fn $attr $var cls_ref.deref(); $arg ($($args)* $(; $opt = $opt_val)*) $env $vars $obj $body $for);
			drop(cls_ref);
			x
		}
	};
}

macro_rules! default_attrs {
	(for $for:ty; $(use $what:ident;)* $(fn $attr:tt $args:tt $($with_stuff:ident)* $body:block)*) => {
		impl ::obj::attrs::HasDefaultAttrs for $for {
			fn default_attrs() -> &'static ::obj::attrs::DefaultAttrs { &DEFAULT_ATTRS }
		}

		lazy_static!{
			static ref DEFAULT_ATTRS: ::obj::attrs::DefaultAttrs = {
				use std::borrow::Borrow;
				use obj::attrs::{DefaultAttrs, HasDefaultAttrs};
				use obj::{object::QObj, classes::boundfn::RustFn};
				let mut m = DefaultAttrs::new();

				$( m.extend($what::default_attrs().iter()); )*
				$(
					m.insert(
						$attr.into(),
						RustFn(
							concat!(stringify!($for), ".`", $attr, "`"),
							__default_attrs_helper!(fn $attr QObj $args [$($with_stuff)*] $body $for)
						)
					);
				)*
				m
			};
		}
	};
	(for $for:ty, with variant $var:ident; $(use $what:ident;)* $(fn $attr:tt $args:tt $($with_stuff:ident)* $body:block)*) => {
		impl ::obj::attrs::HasDefaultAttrs for $for {
			fn default_attrs() -> &'static ::obj::attrs::DefaultAttrs { &DEFAULT_ATTRS }
		}

		lazy_static!{
			static ref DEFAULT_ATTRS: ::obj::attrs::DefaultAttrs = {
				use std::borrow::Borrow;
				use obj::attrs::{DefaultAttrs, HasDefaultAttrs};
				use obj::{object::QObj, classes::boundfn::RustFn};
				let mut m = DefaultAttrs::new();

				$( m.extend($what::default_attrs().iter()); )*
				$(
					m.insert(
						$attr.into(),
						RustFn(
							concat!(stringify!($for), ".`", $attr, "`"),
							__default_attrs_helper!(fn $attr $var $args [$($with_stuff)*] $body $for)
						)
					);
				)*
				m
			};
		}
	};
}


mod object;
mod attrs;
pub mod classes;

pub use self::object::{Id, IdType, QObject, Classes};
