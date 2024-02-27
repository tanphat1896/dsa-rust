use std::cell::RefCell;
use std::rc::Rc;

use crate::datastructure::tree::TreeNode;
struct Solution;

type OptNode = Option<Rc<RefCell<TreeNode>>>;

impl Solution {
    pub fn is_balanced(root: OptNode) -> bool {
        Solution::h(&root) != -1
    }

    fn h(node: &OptNode) -> i32 {
        let node = match node {
            None => return 0,
            Some(v) => v,
        };

        let l = Self::h(&node.borrow().left);
        let r = Self::h(&node.borrow().right);

        if i32::abs(l - r) > 1 || l < 0 || r < 0 {
            return -1;
        }

        1 + i32::max(l, r)
    }
}

#[cfg(test)]
mod test {
    use std::{cell::RefCell, rc::Rc};

    use crate::datastructure::tree::TreeNode;

    use super::Solution;

    #[test]
    fn test_balanced() {
        let root = TreeNode::new(1, Some(TreeNode::new(0, None, None)), None);
        assert_eq!(
            true,
            Solution::is_balanced(Some(Rc::new(RefCell::new(root))))
        )
    }

    #[test]
    fn test_unbalanced() {
        let root = TreeNode::new(
            1,
            Some(TreeNode::new(0, None, Some(TreeNode::new(2, None, None)))),
            None,
        );
        assert_eq!(
            false,
            Solution::is_balanced(Some(Rc::new(RefCell::new(root))))
        )
    }
}
