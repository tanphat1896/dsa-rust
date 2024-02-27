struct Solution;

use std::cell::RefCell;
use std::rc::Rc;

use crate::datastructure::tree::TreeNode;

impl Solution {
    pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        match root {
            None => 0,
            Some(node) => {
                let TreeNode { left, right, .. } = &*node.borrow();
                i32::max(
                    Solution::max_depth(left.clone()),
                    Solution::max_depth(right.clone()),
                ) + 1
            }
        }
    }
}

#[cfg(test)]
mod test {
    use std::{cell::RefCell, rc::Rc};

    use crate::datastructure::tree::TreeNode;

    use super::Solution;

    #[test]
    fn test() {
        let root = TreeNode::new(
            1,
            Some(TreeNode::new(2, None, None)),
            Some(TreeNode::new(3, Some(TreeNode::new(4, None, None)), None)),
        );

        assert_eq!(3, Solution::max_depth(Some(Rc::new(RefCell::new(root)))));
    }
}
