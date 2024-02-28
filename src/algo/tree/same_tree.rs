#![allow(dead_code)]

use std::cell::RefCell;
use std::rc::Rc;

use crate::datastructure::tree::TreeNode;

use super::Solution;

impl Solution {
    pub fn is_same_tree(
        p: Option<Rc<RefCell<TreeNode>>>,
        q: Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        match (p, q) {
            (None, None) => true,
            (Some(p), Some(q)) => {
                p.borrow().val == q.borrow().val
                    && (Self::is_same_tree(p.borrow().left.clone(), q.borrow().left.clone())
                        && Self::is_same_tree(p.borrow().right.clone(), q.borrow().right.clone()))
            }
            _ => false,
        }
    }
}

#[cfg(test)]
mod same_tree_tests {
    use std::{cell::RefCell, rc::Rc};

    use crate::{algo::tree::Solution, datastructure::tree::TreeNode};

    #[test]
    fn test_same_tree() {
        let p = TreeNode::new(1, TreeNode::nochild(2).into(), TreeNode::nochild(3).into());
        let q = TreeNode::new(1, TreeNode::nochild(2).into(), TreeNode::nochild(3).into());

        assert_eq!(
            true,
            Solution::is_same_tree(
                Rc::new(RefCell::new(p)).into(),
                Rc::new(RefCell::new(q)).into()
            )
        )
    }

    #[test]
    fn test_diff_tree_1() {
        let p = TreeNode::new(1, TreeNode::nochild(2).into(), None);
        let q = TreeNode::new(1, None, TreeNode::nochild(2).into());

        assert_eq!(
            false,
            Solution::is_same_tree(
                Rc::new(RefCell::new(p)).into(),
                Rc::new(RefCell::new(q)).into()
            )
        )
    }

    #[test]
    fn test_diff_tree_2() {
        let p = TreeNode::new(1, TreeNode::nochild(1).into(), TreeNode::nochild(2).into());
        let q = TreeNode::new(1, TreeNode::nochild(2).into(), TreeNode::nochild(1).into());

        assert_eq!(
            false,
            Solution::is_same_tree(
                Rc::new(RefCell::new(p)).into(),
                Rc::new(RefCell::new(q)).into()
            )
        )
    }

    #[test]
    fn test_diff_tree_3() {
        let p = TreeNode::new(1, TreeNode::nochild(1).into(), TreeNode::nochild(2).into());

        assert_eq!(
            false,
            Solution::is_same_tree(Rc::new(RefCell::new(p)).into(), None)
        )
    }
}
