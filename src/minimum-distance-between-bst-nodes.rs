/*
https://leetcode.com/problems/minimum-distance-between-bst-nodes/

Given the root of a Binary Search Tree (BST), return the minimum difference 
between the values of any two different nodes in the tree.
*/

use std::cmp;
use std::rc::Rc;
use std::cell::RefCell;

type NodePointer = Rc<RefCell<TreeNode>>;

impl Solution {
    pub fn min_diff_in_bst(root: Option<NodePointer>) -> i32 {
        let mut values: Vec<i32> = Vec::new();
        populate_values(root.clone(), &mut values);
        let mut min_diff = values[1] - values[0]; // Length >= 2 is guaranteed
        for i in 2..values.len() {
            let diff = values[i] - values[i - 1];
            min_diff = cmp::min(min_diff, diff);
        }
        min_diff
    }
}

fn populate_values(node: Option<NodePointer>, values: &mut Vec<i32>) {
    if let Some(node) = node {
        let (val, left, right) = match RefCell::borrow(&node) {
            n => (n.val, n.left.clone(), n.right.clone())
        };
        populate_values(left, values);
        values.push(val);
        populate_values(right, values);
    }
}
