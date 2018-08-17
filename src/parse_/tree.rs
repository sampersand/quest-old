use obj::AnyObject;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Tree {
	nodes: Vec<AnyObject>
}


impl From<Vec<AnyObject>> for Tree {
	fn from(nodes: Vec<AnyObject>) -> Tree {
		Tree { nodes }
	}
}