macro_rules! data_err {
	(read in $ty:ty, $name:ident) => {const_concat!("read err in ", stringify!($ty), "::", $name) };
	(write in $ty:ty, $name:ident) => {const_concat!("write err in ", stringify!($ty), "::", $name) }
}

#[cfg(test)]
macro_rules! assert_param_missing {
	($expr:expr) => (assert_param_missing!($expr, 0));
	($expr:expr, $pos:expr) => (
		match $expr {
			Err(Error::MissingArgument { pos: 0, .. }) => {},
			other => panic!("invalid response returend from `assert_param_missing({:?},{:?})`: {:?}", $expr, $pos, other)
		}
	);
}
macro_rules! getarg {
	($args:ident[$pos:expr]: $type:ty) => {
		getarg!($args[$pos]).and_then(|obj| obj.downcast_or_err::<$type>())
	};
	($args:ident[$pos:expr] @ $conv_func:ident) => {
		getarg!($args[$pos]).and_then(|obj| obj.$conv_func())
	};

	($args:ident[$pos:expr]) => {
		$args.get($pos).map(|x| *x).ok_or_else(|| $crate::err::Error::MissingArgument {
			pos: $pos,
			args: $args.iter().map(|x| (*x).clone()).collect()
		})
	}
}

macro_rules! object_map {
	(UNTYPED $prefix:literal, $map:expr; $($name:tt => $func:expr,)*) => (
		object_map!(UNTYPED $prefix, $map; $($name => $func),*)
	);

	(UNTYPED $prefix:literal, $map:expr; $($name:tt => $func:expr),*) => {$crate::shared::Shared::new({
		let mut map = $map;
		use $crate::map::Map as __MapOnlyForAccessToFuncs;
		$(object_map!(@UNTYPED $prefix; map $name $func);)*
		map
	})};

	(TYPED $type:ty, $map:expr; $($name:tt => $func:expr),*) => {$crate::shared::Shared::new({
		let mut map = $map;
		use $crate::map::Map as __MapOnlyForAccessToFuncs;
		$(object_map!(@TYPED $type; map $name $func);)*
		map
	})};

	(@UNTYPED $prefix:literal; $map:ident $name:tt $func:expr) => {
		assert!(
			$map.set(
				$crate::object::Object::new_variable($name),
				$crate::object::Object::new_named_untyped_rustfn(const_concat!($prefix, "::", $name), $func)
			).is_none()
		);
	};

	(@TYPED $type:ty; $map:ident $name:tt $func:expr) => {
		// this will fail if $name is a number, but i cant check for it, so whatever
		assert!(
			$map.set(
				$crate::object::Object::new_variable($name),
				$crate::object::Object::new_named_rustfn::<_, $type>(
					const_concat!(stringify!($type), "::", $name),
					$func
				)
			).is_none(),
			const_concat!("Found a duplicate entry for '", $name, "'.")
		);
	};

	(@TYPED $type:ty; $_map:ident $invalid:tt $_func:expr) => {
		compile_error!(const_concat!("Invalid type name encountered: `", stringify!($type), "::", stringify!($invalid), "`"))
	};
}

macro_rules! impl_type {
	(for $type:ty, map $mapname:ident: $map:expr; $($impl:tt)*) => {
		lazy_static::lazy_static! {
			pub static ref $mapname: $crate::shared::Shared<dyn $crate::map::Map> = 
				object_map!(TYPED $type, $map; $($impl)*);
		}

		impl $crate::object::types::Type for $type {
			fn get_type_map() -> $crate::shared::Shared<dyn $crate::map::Map> {
				$mapname.clone()
			}
		}
	};

	(for $type:ty, map $mapname:ident $map:expr; $($name:tt => $func:expr,)*) => {
		impl_type!(for $type, map $mapname: $map; $($name => $func),*);
	};

	(for $type:ty, map $mapname:ident; $($name:tt => $func:expr),*) => {
		impl_type!(for $type,
			map $mapname: $crate::map::ParentMap::<std::collections::HashMap<_, _>>::new_default($crate::object::types::basic::BASIC_MAP.clone());
			$($name => $func),*
		);
	};


	(for $type:ty; $($name:tt => $func:expr),*) => {
		impl_type!(for $type, map OBJECT_MAP; $($name => $func),*);
	};

	(for $type:ty; $($name:tt => $func:expr,)*) => {
		impl_type!(for $type; $($name => $func),*);
	};

}