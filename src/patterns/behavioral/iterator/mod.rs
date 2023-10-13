mod iterator;
mod concrete_iterator;
mod collection;
mod concrete_collection;

#[cfg(test)]
mod tests {
	use crate::patterns::behavioral::iterator::collection::Collection;
	use crate::patterns::behavioral::iterator::iterator::Iterator;
	use super::concrete_collection::Tree;

	#[test]
	fn base() {
		let c1 = Tree::new(1);
		let c2 = Tree::new(1);

		c1.depth_first_iterator().next();
		c2.breadth_first_iterator().next();
	}
}