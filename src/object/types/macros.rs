macro_rules! data_err {
	(read in $ty:ty, $name:expr) => {const_concat!("read err in ", stringify!($ty), "::", $name) };
	(write in $ty:ty, $name:expr) => {const_concat!("write err in ", stringify!($ty), "::", $name) }
}

#[cfg(test)]
macro_rules! assert_param_missing {
	($expr:expr) => (assert_param_missing!($expr, 0));
	($expr:expr, $pos:expr) => (
		match $expr {
			Err(crate::err::Error::MissingArgument { pos: 0, .. }) => {},
			other => panic!("invalid response returend from `assert_param_missing({:?},{:?})`: {:?}", $expr, $pos, other)
		}
	);
}

#[cfg(test)]
macro_rules! assert_obj_duplicated {
	($obj1:expr, $obj2:expr) => ({
		assert_eq!(*$obj1.unwrap_data(), *$obj2.unwrap_data());
		unsafe { 
			assert_ne!($obj1.data_ptr(), $obj2.data_ptr());
		}
		assert!(!$obj1._map_only_for_testing().ptr_eq($obj2._map_only_for_testing()));
	})
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

macro_rules! define_blank {
	(struct $struct:ident;) => { define_blank!(struct $struct, BLANK_MAP;); };
	(struct $struct:ident, $map:ident; $($impl_type_block:tt)*) => {
		struct $struct;
		impl ::std::hash::Hash for $struct {
			fn hash<H: ::std::hash::Hasher>(&self, _: &mut H) {
				unreachable!(concat!("Attempted to hash a", stringify!($struct)));
			}
		}

		impl Eq for $struct {}
		impl PartialEq for $struct {
			fn eq(&self, _: &$struct) -> bool {
				unreachable!(concat!("Attempted to compare a", stringify!($struct)));
			}
		}

		impl ::std::fmt::Debug for $struct {
			fn fmt(&self, _: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
				unreachable!(concat!("Attempted to debug format a", stringify!($struct)));

			}
		}

		impl $struct {
			fn new_any() -> $crate::object::AnyObject {
				$crate::object::Object::new($struct).as_any()
			}
		}

		impl_type!{ for $struct, map $map; $($impl_type_block)* }
	}
}



macro_rules! object_map {
	(UNTYPED $prefix:literal, $map:expr; $($name:expr => $func:expr,)*) => (
		object_map!(UNTYPED $prefix, $map; $($name => $func),*)
	);

	(UNTYPED $prefix:literal, $map:expr; $($name:expr => $func:expr),*) => {$crate::shared::Shared::new({
		let mut map = $map;
		use $crate::map::Map as __MapOnlyForAccessToFuncs;
		$(object_map!(@UNTYPED $prefix; map $name, $func);)*
		map
	})};

	(TYPED $type:ty, $map:expr; $($name:expr => $func:expr),*) => {$crate::shared::Shared::new({
		let mut map = $map;
		use $crate::map::Map as __MapOnlyForAccessToFuncs;
		$(object_map!(@TYPED $type; map $name, $func);)*
		map
	})};

	(@UNTYPED $prefix:literal; $map:ident $name:expr, $func:expr) => {
		assert!(
			$map.set(
				$crate::object::Object::new_variable($name),
				$crate::object::Object::new_named_untyped_rustfn(const_concat!($prefix, "::", $name), $func)
			).is_none()
		);
	};

	(@TYPED $type:ty; $map:ident $name:expr, $func:expr) => {
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

	(@TYPED $type:ty; $_map:ident $invalid:expr, $_func:expr) => {
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

	(for $type:ty, map $mapname:ident $map:expr; $($name:expr => $func:expr,)*) => {
		impl_type!(for $type, map $mapname: $map; $($name => $func),*);
	};

	(for $type:ty, map $mapname:ident; $($name:expr => $func:expr),*) => {
		impl_type!(for $type,
			map $mapname: $crate::map::ParentMap::<std::collections::HashMap<_, _>>::new_default($crate::object::types::basic::BASIC_MAP.clone());
			$($name => $func),*
		);
	};


	(for $type:ty; $($name:expr => $func:expr),*) => {
		impl_type!(for $type, map OBJECT_MAP; $($name => $func),*);
	};

	(for $type:ty; $($name:expr => $func:expr,)*) => {
		impl_type!(for $type; $($name => $func),*);
	};

}