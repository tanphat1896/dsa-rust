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
}
