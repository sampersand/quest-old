use super::{TypedObject, Type, Types};
use crate::{Shared, Object};
use crate::collections::{Mapping, ParentalMap};
use std::fmt::{self, Display, Formatter};
use lazy_static::lazy_static;

// to make it easier on my eyes
macro_rules! basic_map {
	($($args:tt)*) => {
		lazy_static! { 
			pub static ref BASIC_MAP: Object = function_map!(
				prefix="Baisc",
				downcast_fn=__error,
				$($args)*
			);
		}
	}
}

basic_map! {
	fn "@bool" (_) {
		true.into_object()
	}
}

















