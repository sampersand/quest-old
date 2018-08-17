use env::{Environment, Binding, parse::{Parsable, Token}};
use obj::{AnyObject, SharedObject};

use std::fmt::{self, Display, Formatter};	

pub type QBinding = SharedObject<Binding>;

define_attrs! { for QBinding;
	use QObject<Binding>;

	fn "@bool" () {
		Ok(QBool::from(false))
	}
}