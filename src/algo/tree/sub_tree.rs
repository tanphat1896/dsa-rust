use std::{cell::RefCell, rc::Rc};

use super::Solution;
use crate::datastructure::tree::TreeNode;

type RefNode = Rc<RefCell<TreeNode>>;

impl Solution {
    pub fn is_subtree(root: Option<RefNode>, sub_root: Option<RefNode>) -> bool {
        match (root, sub_root) {
            (None, None) => true,
            (Some(root), Some(sub_root)) => {
                if root.borrow().val == sub_root.borrow().val {
                    if Self::same_tree(&root.borrow().left, &sub_root.borrow().left)
                        && Self::same_tree(&root.borrow().right, &sub_root.borrow().right)
                    {
                        return true;
                    }
                }

                Self::is_subtree(root.borrow().left.clone(), Some(sub_root.clone()))
                    || Self::is_subtree(root.borrow().right.clone(), Some(sub_root.clone()))
            }
            _ => false,
        }
    }

    fn same_tree(p: &Option<RefNode>, q: &Option<RefNode>) -> bool {
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
mod test_subtree {
    use std::{cell::RefCell, rc::Rc};

    use crate::{algo::tree::Solution, datastructure::tree::TreeNode};

    #[test]
    fn test_is_subtree() {
        let root = TreeNode::new(1, TreeNode::nochild(1).into(), None);
        let sroot = TreeNode::new(1, None, None);

        assert_eq!(
            true,
            Solution::is_subtree(
                Rc::new(RefCell::new(root)).into(),
                Rc::new(RefCell::new(sroot)).into()
            )
        );
    }

    #[test]
    fn test_is_subtree_2() {
        let root = TreeNode::make_tree(vec![3, 4, 5, 1, 2]);
        let sroot = TreeNode::make_tree(vec![4, 1, 2]);

        assert_eq!(true, Solution::is_subtree(root, sroot));
    }

    #[test]
    fn test_is_subtree_3() {
        let root = TreeNode::make_tree(vec![3, 4, 5, 1, 2, -1, -1, -1, -1, 0]);
        let sroot = TreeNode::make_tree(vec![4, 1, 2]);

        assert_eq!(false, Solution::is_subtree(root, sroot));
    }
}
