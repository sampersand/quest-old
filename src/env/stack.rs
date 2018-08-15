use obj::AnyObject;

#[derive(Debug, Clone, Default)]
pub struct Stack {
	objs: Vec<AnyObject>,
	opers: Vec<AnyObject>
}

impl Stack {
	pub fn handle(&mut self, obj: AnyObject) {
		unimplemented!("TODO: handle ({:?})", obj)
	}

	pub fn finish(self) -> AnyObject {
		unimplemented!("TODO: finish {:?}", self)
	}
	pub fn is_empty(&self) -> bool {
		self.objs.is_empty() && self.opers.is_empty()
	}
}
