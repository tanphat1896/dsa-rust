use std::{cell::RefCell, rc::Rc};

use crate::datastructure::tree::TreeNode;

use super::Solution;

impl Solution {
    pub fn invert_tree(node: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        if node.is_none() {
            return None;
        }

        let node = node.clone().unwrap();
        let left = &node.borrow().left;
        let right = &node.borrow().right;

        let node = TreeNode {
            val: node.borrow().val,
            left: Self::invert_tree(right.clone()),
            right: Self::invert_tree(left.clone()),
        };

        Some(Rc::new(RefCell::new(node)))
    }
}

#[cfg(test)]
mod invert_bin_tree_test {
    use crate::{algo::tree::Solution, datastructure::tree::TreeNode};

    #[test]
    fn test1() {
        let root = TreeNode::make_tree(vec![4, 2, 7, 1, 3, 6, 9]);
        let expected = TreeNode::make_tree(vec![4, 7, 2, 9, 6, 3, 1]);
        assert_eq!(expected, Solution::invert_tree(root))
    }

    #[test]
    fn test2() {
        let root = TreeNode::make_tree(vec![2, 1, 3]);
        let expected = TreeNode::make_tree(vec![2, 3, 1]);
        assert_eq!(expected, Solution::invert_tree(root))
    }

    #[test]
    fn test3() {
        let root = TreeNode::make_tree(vec![]);
        let expected = TreeNode::make_tree(vec![]);
        assert_eq!(expected, Solution::invert_tree(root))
    }
}
