use std::cell::RefCell;
use std::rc::Rc;

use crate::datastructure::tree::TreeNode;

use super::Solution;

impl Solution {
    pub fn level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        let mut result: Vec<Vec<i32>> = Vec::new();

        if let Some(node) = root {
            Self::level_ord(&mut result, &Some(node), 0);
        }

        result
    }

    fn level_ord(result: &mut Vec<Vec<i32>>, node: &Option<Rc<RefCell<TreeNode>>>, level: usize) {
        if node.is_none() {
            return;
        }

        let node = node.clone().unwrap();

        if result.len() == level {
            result.push(Vec::new())
        }

        result[level].push(node.borrow().val);
        Self::level_ord(result, &node.borrow().left, level + 1);
        Self::level_ord(result, &node.borrow().right, level + 1);
    }
}

#[cfg(test)]
mod test {
    use std::vec;

    use crate::datastructure::tree::TreeNode;

    use super::Solution;

    #[test]
    fn test1() {
        let expected = vec![vec![3], vec![9, 20], vec![15, 7]];
        let root = TreeNode::make_tree(vec![3, 9, 20, -1, -1, 15, 7]);
        assert_eq!(expected, Solution::level_order(root))
    }

    #[test]
    fn test2() {
        let expected = vec![vec![1]];
        let root = TreeNode::make_tree(vec![1]);
        assert_eq!(expected, Solution::level_order(root))
    }

    #[test]
    fn test3() {
        let expected: Vec<Vec<i32>> = vec![];
        let root = TreeNode::make_tree(vec![]);
        assert_eq!(expected, Solution::level_order(root))
    }
}
