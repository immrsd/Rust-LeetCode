/*
https://leetcode.com/problems/add-binary/description/

Given two binary strings a and b, return their sum as a binary string.
*/

enum Binary {
  Zero,
  One
}

impl Solution {
  pub fn add_binary(a: String, b: String) -> String {
      if a.is_empty() { return b; }
      if b.is_empty() { return a; }

      let mut a_chars = a.chars().rev();
      let mut b_chars = b.chars().rev();
      let mut pairs_iter = std::iter::from_fn(move || {
          match (a_chars.next(), b_chars.next()) {
              (None, None) => None,
              (a, b) => Some((to_value(a), to_value(b))),
          }
      });

      let mut one_in_mind = false;
      let mut binary_sum_iter = std::iter::from_fn(|| {
          match pairs_iter.next() {
              Some((a, b)) => {
                  let sum = if one_in_mind { a + b + 1 } else { a + b };
                  one_in_mind = sum > 1;
                  if sum % 2 == 0 {
                      Some(Binary::Zero)
                  } else {
                      Some(Binary::One)
                  }
              },
              None => {
                  if one_in_mind {
                      one_in_mind = false;
                      Some(Binary::One)
                  } else {
                      None
                  }
              },
          }
      });

      binary_sum_iter
          .map(|x| match x { Binary::Zero => '0', Binary::One => '1' })
          .collect::<Vec<char>>()
          .into_iter()
          .rev()
          .collect::<String>()
  }
}

fn to_value(c: Option<char>) -> u8 {
  match c {
      None => 0,
      Some('0') => 0,
      Some(_) => 1,
  }
}
