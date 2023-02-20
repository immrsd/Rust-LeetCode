/*
https://leetcode.com/problems/add-to-array-form-of-integer/

The array-form of an integer num is an array representing its digits in left to right order.

For example, for num = 1321, the array form is [1,3,2,1].
Given num, the array-form of an integer, and an integer k, return the array-form of the integer num + k.
*/

impl Solution {
  pub fn add_to_array_form(num: Vec<i32>, k: i32) -> Vec<i32> {
      if k == 0 { return num; }

      // Prepare iterators for a and b digits
      let mut a_iter = num.into_iter().rev();
      let mut remaining_b = k;
      let mut b_iter = std::iter::from_fn(move || {
          if remaining_b == 0 {
              None
          } else {
              let b = remaining_b % 10;
              remaining_b /= 10;
              Some(b)
          }
      });

      // Prepare result digits iterator that consumes a_iter and b_iter
      let mut carry = false;
      let mut result_iter = std::iter::from_fn(move || {
          match (a_iter.next(), b_iter.next()) {
              (None, None) => {
                  if carry {
                      carry = false;
                      Some(1)
                  } else {
                      None
                  }
              },
              (a, b) => {
                  let a = a.unwrap_or(0);
                  let b = b.unwrap_or(0);
                  let a_plus_b = a + b + if carry { 1 } else { 0 };
                  if a_plus_b >= 10 {
                      carry = true;
                      Some(a_plus_b - 10)
                  } else {
                      carry = false;
                      Some(a_plus_b)
                  }
              },
          }
      });

      // Collect result digits and reverse it
      let mut result: Vec<i32> = result_iter.collect();
      result.reverse();
      result
  }
}
