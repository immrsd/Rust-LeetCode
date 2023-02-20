/*
https://leetcode.com/problems/binary-tree-zigzag-level-order-traversal/

Given the root of a binary tree, return the zigzag level order traversal of its nodes' values. (i.e., from left to right, then right to left for the next level and alternate between).
*/

use std::rc::Rc;
use std::cell::RefCell;

type TreeRef = Rc<RefCell<TreeNode>>;

impl Solution {
    pub fn zigzag_level_order(root: Option<TreeRef>) -> Vec<Vec<i32>> {
        let root = match root {
            Some(r) => r,
            None => return vec![],
        };

        fn collect_values_recurse(
            batch: Vec<TreeRef>, 
            should_reverse: bool, 
            mut result: Vec<Vec<i32>>
        ) -> Vec<Vec<i32>> {
            let len = batch.len();
            if len == 0 { 
                return result;
            }
            let mut values: Vec<i32> = Vec::with_capacity(len);
            let mut next_batch: Vec<TreeRef> = Vec::with_capacity(len * 2);
            for node in batch.into_iter() {
                match node.borrow() {
                    node => {
                        values.push(node.val);
                        node.left.clone().map(|n| next_batch.push(n));
                        node.right.clone().map(|n| next_batch.push(n));
                    }
                };
            }
            if should_reverse {
                values.reverse()
            }
            result.push(values);
            collect_values_recurse(
                next_batch, 
                !should_reverse, 
                result
            )
        }
        
        collect_values_recurse(vec![root], false, vec![])
    }
}
