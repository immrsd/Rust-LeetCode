/*
https://leetcode.com/problems/invert-binary-tree/

Given the root of a binary tree, invert the tree, and return its root.
*/

use std::rc::Rc;
use std::cell::RefCell;

type TreeRef = Rc<RefCell<TreeNode>>;

impl Solution {
    pub fn invert_tree(root: Option<TreeRef>) -> Option<TreeRef> {
        root.map(|root| {
            match root.borrow_mut() {
                mut node => {
                    let left = node.left.clone();
                    let right = node.right.clone();
                    node.left = Self::invert_tree(right);
                    node.right = Self::invert_tree(left);
                }
            };
            root
        })
    }
}
