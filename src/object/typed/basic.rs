use super::{TypedObject, Type, Types};
use crate::{Shared, Object};
use crate::collections::{Mapping, ParentalMap};
use std::fmt::{self, Display, Formatter};
use lazy_static::lazy_static;

// to make it easier on my eyes
macro_rules! basic_map {
	($($args:tt)*) => {
		lazy_static! { 
			pub static ref BASIC_MAP: Object = Object::new(ParentalMap::new_mapped(
				|| super::pristine::PRISTINE_MAP.clone(),
				function_map!(prefix="Baisc", downcast_fn=__error, $($args)*)
			));
		}
	}
}

basic_map! {
	fn "@bool" (_) {
		true.into_object()
	}

	fn "==" (@lhs, rhs) {
		lhs.call_attr("===", &[rhs])?
	}

	fn "!=" (@lhs, rhs) {
		lhs.call_attr("==", &[rhs])?
		   .call_attr("!", &[])?
	}

	fn "===" (@lhs, rhs) {
		(lhs.id() == rhs.id()).into_object()
	}

	fn "!==" (@lhs, rhs) {
		lhs.call_attr("===", &[rhs])?
		   .call_attr("!", &[])?
	}


	fn "!" (@this) {
		(!this.as_bool()?.as_ref()).into_object()
	}

	fn "and" (@lhs, rhs) {
		if *lhs.as_bool()?.as_ref() {
			rhs.clone()
		} else {
			lhs.clone()
		}
	}

	fn "or" (@lhs, rhs) {
		if *lhs.as_bool()?.as_ref() {
			lhs.clone()
		} else {
			rhs.clone()
		}
	}
}


















