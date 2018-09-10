use env::Environment;
use obj::Error;

__impl_type! {
	for Environment, with self attr;

	fn "." (this, attr) env, {
		{
			let me = this.read();
			if let Some(attr) = me.data.get(&attr).or_else(|| me.attrs.get(&attr, env).ok()) {
				return Ok(attr)
			}
		}
		Err(Error::MissingAttr { obj: this, attr })
	}

	fn ".?" (this, attr) {
		Ok(this.read().data.has(&attr).into_object())
	}

	fn ".=" (this, attr, val) env, {
		this.write().data.set(attr, val.clone());
		Ok(val)
	}

	fn ".~" (this, attr) {
		this.write().data.del(&attr);
		Ok(this)
	}

	fn _ () {
		any::__get_default(self, attr)
	}
}