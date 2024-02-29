use std::{cell::RefCell, rc::Rc};

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32, left: Option<TreeNode>, right: Option<TreeNode>) -> Self {
        TreeNode {
            val,
            left: left.map(|v| Rc::new(RefCell::new(v))),
            right: right.map(|v| Rc::new(RefCell::new(v))),
        }
    }
    
    pub fn nochild(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }

    pub fn make_tree(vec: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        if vec.len() == 0 {
            None
        } else {
            Self::make_node(&vec, 0)
        }
    }

    fn make_node(vec: &Vec<i32>, idx: usize) -> Option<Rc<RefCell<TreeNode>>> {
        if idx >= vec.len() || vec[idx] == -1 {
            return None;
        }

        let node = TreeNode {
            val: vec[idx],
            left: Self::make_node(&vec, idx * 2 + 1),
            right: Self::make_node(&vec, idx * 2 + 2),
        };

        Rc::new(RefCell::new(node)).into()
    }
}

#[cfg(test)]
mod test_tree_node {
    use std::{cell::RefCell, rc::Rc};

    use super::TreeNode;

    #[test]
    fn test_make_tree() {
        let expected = TreeNode::new(
            1,
            TreeNode::new(2, TreeNode::nochild(3).into(), None).into(),
            TreeNode::new(4, None, None).into(),
        );

        assert_eq!(
            Some(Rc::new(RefCell::new(expected))),
            TreeNode::make_tree(vec![1, 2, 4, 3])
        )
    }
}
