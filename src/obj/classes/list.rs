use std::ops::Deref;
use obj::{Classes, QObject, Exception};
use std::fmt::{self, Debug, Display, Formatter};
use obj::classes::{QNum, QNull, utils::IndexPos};
use env::Environment;
use std::hash::{Hash, Hasher};

#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
pub struct QList(Vec<QObject>);

impl QList {
	#[inline]
	pub fn new(v: Vec<QObject>) -> QList {
		QList(v)
	}
}

impl From<Vec<QObject>> for QList {
	#[inline]
	fn from(list: Vec<QObject>) -> QList {
		QList::new(list)
	}
}

impl From<QList> for Vec<QObject> {
	#[inline]
	fn from(qlist: QList) -> Vec<QObject> {
		qlist.0
	}
}

impl From<Vec<QObject>> for QObject {
	#[inline]
	fn from(list: Vec<QObject>) -> QObject {
		QList::from(list).into()
	}
}

impl AsRef<[QObject]> for QList {
	#[inline]
	fn as_ref(&self) -> &[QObject] {
		self.0.as_ref()
	}
}

impl Deref for QList {
	type Target = [QObject];
	#[inline]
	fn deref(&self) -> &[QObject] {
		self.0.deref()
	}
}

impl Display for QList {
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		write!(f, "[")?;
		if !self.0.is_empty() {
			write!(f, "{:#}", self.0[0])?;
			for val in &self.0[1..] {
				write!(f, ", {:#}", val)?;
			}
		}
		write!(f, "]")
	}
}

fn into_vec(pos: &QObject, env: &Environment) -> Vec<QObject> {
	pos.as_list(env).expect("`@list` is required to merge with `QList`").into()
}

default_attrs! { for QList, with variant List;
	use QObj;

	fn "get_attr" (_this, attr) with env vars obj {
		if attr.is_num() {
			obj.call_attr("get", &[attr], env)
		} else {
			call_super!(QObj("get_attr") for obj, vars, env)
		}
	}

	fn "@list" (this) {
		Ok(this.clone().into())
	}

	fn "@bool" (this) with env this {
		Ok(this.0.is_empty().into())
	}

	fn "empty!" (mut this) with _env _var obj{
		this.0.clear();
		Ok(obj.clone())
	}

	fn "empty?" (this) {
		Ok(this.0.is_empty().into())
	}

	fn "len" (this) {
		Ok(QNum::new(this.0.len() as _).into())
	}

	fn "push" (mut this, pos) {
		this.0.push(pos.clone());
		Ok(QNull.into())
	}

	fn "pop" (mut this, pos) {
		Ok(if let Some(ele) = this.0.pop() {
			ele
		} else {
			info!("Attempted to pop from an empty list ({:?}); returning null", this);
			().into()
		})
	}

	fn "has" (this, var) {
		Ok(this.0.contains(var).into())
	}

	fn "get" (this, pos) with env {
		Ok(match IndexPos::from_qobject(this.0.len(), pos, env) {
			IndexPos::InBounds(pos) => this.0[pos].clone(),
			IndexPos::OutOfBounds(_) | IndexPos::Underflow(_) => ().into(),
			IndexPos::NotAnInt(pos) => panic!("Can't index with non-integer num `{}`", pos)
		})
	}

	fn "set" (mut this, pos, val) with env {
		let len = this.0.len();

		let pos = match IndexPos::from_qobject(len, pos, env) {
			IndexPos::InBounds(pos) => pos,
			IndexPos::OutOfBounds(pos) => {
				this.0.reserve(len - pos);
				for _ in len..=pos {
					this.0.push(QNull.into());
				}
				pos
			},
			IndexPos::Underflow(pos) => panic!("Underflow! {} is out of bounds (len={})", pos, len),
			IndexPos::NotAnInt(pos) => panic!("Can't index with non-integer num `{}`", pos)
		};
		this.0[pos] = val.clone();
		Ok(val.clone())
	}

	fn "del" (mut this, pos) with env {
		Ok(match IndexPos::from_qobject(this.0.len(), pos, env) {
			IndexPos::InBounds(pos) => this.0.remove(pos),
			IndexPos::OutOfBounds(_) | IndexPos::Underflow(_) => QNull.into(),
			IndexPos::NotAnInt(pos) => panic!("Can't index with non-integer num `{}`", pos)
		})
	}

	// operators 

	fn "|" (this, other) with env {
		let mut v = this.0.clone();
		let other = into_vec(other, env);
		for ele in other.iter() {
			if !v.contains(ele) {
				v.push(ele.clone());
			}
		}
		Ok(v.into())
	}

	fn "&" (this, other) with env {
		let mut v = vec![];
		let ref this = this.0;
		for ele in into_vec(other, env).iter() {
			if this.contains(ele) {
				v.push(ele.clone());
			}
		}
		Ok(v.into())
	}
	fn "^^" (this, other) with env {
		let mut v = vec![];
		let ref this = this.0;
		let other = into_vec(other, env);
		for ele in other.iter() {
			if !this.contains(ele) {
				v.push(ele.clone());
			}
		}
		for ele in this.iter() {
			if !other.contains(ele) {
				v.push(ele.clone());
			}
		}
		Ok(v.into())
	}
}