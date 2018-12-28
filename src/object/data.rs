use std::any::TypeId;

#[derive(Debug)]
pub struct Data {
	typeid: TypeId,
	data: *const (),
	_drop: fn(*mut (), TypeId)
}

impl Data {
	pub fn new<T: 'static>(data: T) -> Data {
		Data {
			typeid: TypeId::of::<T>(),
			data: Box::into_raw(Box::new(data)) as *const (),
			_drop: |ptr, id| unsafe {
				assert_eq!(id, TypeId::of::<T>(), "Drop used on wrong function");
				Box::from_raw(ptr as *mut T);
			}
		}
	}

	pub fn is<T: 'static>(&self) -> bool {
		self.typeid == TypeId::of::<T>()
	}

	pub fn try_as_ref<T: 'static>(&self) -> Option<&T> {
		if TypeId::of::<T>() == self.typeid {
			Some(unsafe {
				&*(self.data as *const T)
			})
		} else {
			None
		}
	}
}

impl Drop for Data {
	fn drop(&mut self) {
		(self._drop)(self.data as *mut (), self.typeid)
	}
}