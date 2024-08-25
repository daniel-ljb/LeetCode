// Definition for a binary tree node.
// #[derive(Debug, PartialEq, Eq)]
// pub struct TreeNode {
//   pub val: i32,
//   pub left: Option<Rc<RefCell<TreeNode>>>,
//   pub right: Option<Rc<RefCell<TreeNode>>>,
// }
// 
// impl TreeNode {
//   #[inline]
//   pub fn new(val: i32) -> Self {
//     TreeNode {
//       val,
//       left: None,
//       right: None
//     }
//   }
// }
use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn postorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut result: Vec<i32> = Vec::new();
        let mut stack: Vec<Option<Rc<RefCell<TreeNode>>>> = Vec::new();
        stack.push(root);
        while stack.len() > 0 {
            if let Some(n) = stack.pop().unwrap() {
                let node = n.borrow();
                result.push(node.val);
                stack.push(node.left.clone());
                stack.push(node.right.clone());
            }
        }
        result.reverse();
        result
    }
}