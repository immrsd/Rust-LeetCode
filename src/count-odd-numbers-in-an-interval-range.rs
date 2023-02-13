/*
Given two non-negative integers low and high. 
Return the count of odd numbers between low and high (inclusive).
*/

impl Solution {

  pub fn count_odds(low: i32, high: i32) -> i32 {
      let first_is_odd = low % 2 == 1;
      if first_is_odd {
          let next_odds = (high - low) / 2;
          1 + next_odds
      } else {
          let next_odds = (1 + high - low) / 2;
          next_odds
      }
  }
}
