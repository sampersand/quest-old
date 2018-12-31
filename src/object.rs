pub(crate) mod typed;
mod object;
pub use self::typed::TypedObject;
pub use self::object::Object;

pub trait IntoObject {
	fn into_object(self) -> Object;
}