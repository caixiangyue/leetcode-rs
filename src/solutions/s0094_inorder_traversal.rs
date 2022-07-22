// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

pub struct Solution {}

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        fn preorder_dfs(node: &Option<Rc<RefCell<TreeNode>>>, ans: &mut Vec<i32>) {
            if node.is_none() {
                return;
            }

            preorder_dfs(&node.as_ref().unwrap().borrow().left, ans);
            ans.push(node.as_ref().unwrap().borrow().val);
            preorder_dfs(&node.as_ref().unwrap().borrow().right, ans);
        }
        let mut ans = vec![];
        preorder_dfs(&root, &mut ans);
        ans
    }
}

#[test]
fn test_1() {
    let n = Some(Rc::new(RefCell::new(TreeNode::new(1))));
    assert_eq!(vec![1], Solution::inorder_traversal(n));
}
