use super::iterator::Iterator;

// 二叉树数据集。
pub trait Collection {
	type Item;
	type DepthFirstIterator: Iterator<Item = Self::Item>;
	type BreadthFirstIterator: Iterator<Item = Self::Item>;

	fn depth_first_iterator(self) -> Self::DepthFirstIterator;
	fn breadth_first_iterator(self) -> Self::BreadthFirstIterator;
}