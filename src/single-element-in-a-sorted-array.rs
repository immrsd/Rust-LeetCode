/*
https://leetcode.com/problems/single-element-in-a-sorted-array/description/

You are given a sorted array consisting of only integers where every element appears exactly twice, 
except for one element which appears exactly once.

Return the single element that appears only once.

Your solution must run in O(log n) time and O(1) space.
*/

impl Solution {

  pub fn single_non_duplicate(nums: Vec<i32>) -> i32 {
      match nums.len() {
          0 => return -1,
          1 => return nums[0],
          _ => (),
      };

      fn find_non_dup_recurse(nums: Vec<i32>, left: usize, right: usize) -> Option<i32> {
          if left > right {
              return None;
          }
          let mid = left + (right - left) / 2;
          if let Some(pair_left) = find_pair_left(&nums, mid) {
              let (next_left, next_right) = resolve_next_pointers(pair_left, left, right);
              find_non_dup_recurse(nums, next_left, next_right)
          } else {
              Some(nums[mid])
          }
      }

      fn find_pair_left(nums: &Vec<i32>, index: usize) -> Option<usize> {
          let target = nums[index];
          match index {
              0 => {
                  if nums[1] == target { Some(0) } else { None }
              },
              i if i == nums.len() - 1 => {
                  if nums[i - 1] == target { Some(i - 1) } else { None }
              },
              i => {
                  if target == nums[i - 1] {
                      Some(i - 1)
                  } else if target == nums[i + 1] {
                      Some(i)
                  } else {
                      None
                  }
              },
          }
      }

      fn resolve_next_pointers(pair_left: usize, left: usize, right: usize) -> (usize, usize) {
          if pair_left % 2 == 0 {
              (pair_left + 2, right)
          } else {
              (left, pair_left - 1)
          }
      }
      
      let end_index = nums.len();
      find_non_dup_recurse(nums, 0, end_index).unwrap_or(-1)
  }
}
