use crate::Shared;

#[derive(Debug)]
pub struct Environment {

}

impl Environment {
	pub fn current() -> Shared<Environment> {
		Shared::new(Environment{})
	}
}