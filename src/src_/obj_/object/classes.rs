use std::sync::Arc;
use obj_::{QObject__, classes_, object::QObj, attrs::HasDefaultAttrs};
use std::fmt::{self, Debug, Display, Formatter};

#[derive(Clone, PartialEq, Eq, Hash)]
pub enum Classes {
	Var(classes_::QVar),
	BoundFn(classes_::QBoundFn),
	Null(classes_::QNull),

	Bool(classes_::QBool),
	Text(classes_::QText),
	Num(classes_::QNum),
	Oper(classes_::QOper),

	Block(classes_::QBlock),
	List(classes_::QList),
	Map(classes_::QMap),
}

impl Debug for Classes {
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		match self {
			Classes::Var(val) => Debug::fmt(val, f),
			Classes::BoundFn(val) => Debug::fmt(val, f),
			Classes::Null(val) => Debug::fmt(val, f),
		
			Classes::Bool(val) => Debug::fmt(val, f),
			Classes::Text(val) => Debug::fmt(val, f),
			Classes::Num(val) => Debug::fmt(val, f),
			Classes::Oper(val) => Debug::fmt(val, f),
		
			Classes::Block(val) => Debug::fmt(val, f),
			Classes::List(val) => Debug::fmt(val, f),
			Classes::Map(val) => Debug::fmt(val, f),
		}
	}
}

impl Display for Classes {
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		match self {
			Classes::Var(val) => Display::fmt(val, f),
			Classes::BoundFn(val) => Display::fmt(val, f),
			Classes::Null(val) => Display::fmt(val, f),
		
			Classes::Bool(val) => Display::fmt(val, f),
			Classes::Text(val) => Display::fmt(val, f),
			Classes::Num(val) => Display::fmt(val, f),
			Classes::Oper(val) => Display::fmt(val, f),
		
			Classes::Block(val) => Display::fmt(val, f),
			Classes::List(val) => Display::fmt(val, f),
			Classes::Map(val) => Display::fmt(val, f),
		}
	}
}

impl<T: HasDefaultAttrs + Into<Classes>> From<T> for QObject__ {
	fn from(obj: T) -> QObject__ {
		QObject__::from(QObj::new(obj))
	}
}

macro_rules! enum_from {
	($($var:ident $src:ident)+) => {
		$(
			impl From<classes_::$src> for Classes {
				#[inline]
				fn from(src: classes_::$src) -> Classes {
					Classes::$var(src)
				}
			}
		)+
	}
}

impl<O: Into<classes_::QOper>> From<O> for Classes {
	fn from(oper: O) -> Classes {
		Classes::Oper(oper.into())
	}
}

enum_from! {
	Var QVar
	BoundFn QBoundFn
	Null QNull
	Bool QBool
	Text QText
	Num QNum
	Block QBlock
	List QList
	Map QMap
}