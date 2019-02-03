macro_rules! getarg {
	($args:ident[$pos:expr]: $type:ty) => {
		getarg!($args[$pos]).and_then(|obj| obj.downcast_or_err::<$type>())
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
		use $crate::map::Map;
		$(object_map!(@UNTYPED $prefix; map $name $func);)*
		map
	})};

	(TYPED $type:ty, $map:expr; $($name:tt => $func:expr),*) => {$crate::shared::Shared::new({
		let mut map = $map;
		use $crate::map::Map;
		$(object_map!(@TYPED $type; map $name $func);)*
		map
	})};

	(@UNTYPED $prefix:literal; $map:ident $name:literal $func:expr) => {
		assert!(
			$map.set(
				$crate::object::Object::new_variable($name),
				$crate::object::Object::new_named_untyped_rustfn(concat!($prefix, "::", $name), $func)
			).is_none()
		);
	};

	(@TYPED $type:ty; $map:ident $name:literal $func:expr) => {
		// this will fail if $name is a number, but i cant check for it, so whatever
		assert!(
			$map.set(
				$crate::object::Object::new_variable($name),
				$crate::object::Object::new_named_rustfn::<_, $type>(
					concat!(stringify!($type), "::", $name),
					$func
				)
			).is_none(),
			concat!("Found a duplicate entry for '", $name, "'.")
		);
	};

	(@TYPED $type:ty; $_map:ident $invalid:tt $_func:expr) => {
		compile_error!(concat!("Invalid type name encountered: `" stringify!($type), "::", stringify!($invalid), "`"))
	};
}

macro_rules! impl_type {
	(for $type:ty, $map:expr; $($impl:tt)*) => {
		lazy_static::lazy_static! {
			pub static ref OBJECT_MAP: $crate::shared::Shared<dyn $crate::map::Map> = 
				object_map!(TYPED $type, $map; $($impl)*);
		}

		impl $crate::object::types::Type for $type {
			fn get_type_map() -> $crate::shared::Shared<dyn $crate::map::Map> {
				OBJECT_MAP.clone()
			}
		}
	};

	(for $type:ty, $map:expr; $($name:tt => $func:expr,)*) => {
		impl_type!(for $type, $map; $($name => $func),*);
	};

	(for $type:ty; $($name:tt => $func:expr),*) => {
		impl_type!(for $type,
			$crate::map::ParentMap::<std::collections::HashMap<_, _>>::new_default($crate::object::types::basic::BASIC_MAP.clone());
			$($name => $func),*
		);
	};

	(for $type:ty; $($name:tt => $func:expr,)*) => {
		impl_type!(for $type; $($name => $func),*);
	};

}