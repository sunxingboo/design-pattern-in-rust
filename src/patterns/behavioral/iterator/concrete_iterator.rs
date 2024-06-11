use std::rc::Rc;
use super::concrete_collection::Tree;
use super::iterator::Iterator;

// 深度优先迭代器
#[allow(unused)]
pub struct DepthFirstIterator {
	collection: Rc<Tree>,
}

impl DepthFirstIterator {
	pub fn new(collection: Rc<Tree>) -> Self {
		DepthFirstIterator {
			collection
		}
	}
}

impl Iterator for DepthFirstIterator {
	type Item = i32;

	fn next(&self) -> Option<Self::Item> {
		println!("deep first iterator");
		None
	}
}

// 广度优先迭代器
#[allow(unused)]
pub struct BreadthFirstIterator {
	collection: Rc<Tree>,
}

impl BreadthFirstIterator {
	pub fn new(collection: Rc<Tree>) -> Self {
		BreadthFirstIterator {
			collection
		}
	}
}

impl Iterator for BreadthFirstIterator {
	type Item = i32;

	fn next(&self) -> Option<Self::Item> {
		println!("width first iterator");
		None
	}
}