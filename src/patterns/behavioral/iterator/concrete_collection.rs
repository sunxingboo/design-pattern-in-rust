use std::rc::Rc;
use super::concrete_iterator::{DepthFirstIterator, BreadthFirstIterator};
use super::collection::Collection;

// 二叉树。本示例中只简单演示遍历，因此使用Option<Box<T>>来定义节点。
pub struct Tree {
	val: i32,
	// left: Option<Rc<RefCell<Tree>>>,
	// right: Option<Rc<RefCell<Tree>>>,
	left: Option<Box<Tree>>,
	right: Option<Box<Tree>>,
}

impl Tree {
	pub fn new(val: i32) -> Self {
		Tree{
			val,
			left: None,
			right: None,
		}
	}

	pub fn set_left(&mut self, node: Tree) {
		self.left = Option::from(Box::new(node));
	}

	pub fn set_right(&mut self, node: Tree) {
		self.right = Option::from(Box::new(node));
	}
}

impl Collection for Tree {
	type Item = i32;
	type DepthFirstIterator = DepthFirstIterator;
	type BreadthFirstIterator = BreadthFirstIterator;

	fn depth_first_iterator(self) -> Self::DepthFirstIterator {
		DepthFirstIterator::new(Rc::new(self))
	}

	fn breadth_first_iterator(self) -> Self::BreadthFirstIterator {
		BreadthFirstIterator::new(Rc::new(self))
	}
}