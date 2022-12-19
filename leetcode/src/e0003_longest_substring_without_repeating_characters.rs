use std::collections::HashMap;

struct Solution;

impl Solution {
  pub fn length_of_longest_substring(s: String) -> i32 {
    let mut chars: HashMap<char, usize> = HashMap::new();
    let mut g = 0..0; // global (start, end)
    let mut l = 0..0; // local (start, end)
    for (i, c) in s.char_indices() {
      l = match chars.get(&c) {
        Some(index) if index + 1 > l.start => (index + 1)..(i + 1),
        _ => l.start..(i + 1),
      };
      if l.len() > g.len() {
        g = l.clone();
      }
      chars.insert(c, i);
    }
    g.len() as i32
  }
}

#[cfg(test)]
mod test {
  use super::Solution;

  #[test]
  fn example_1() {
    let s = "abcabcbb".to_string();
    assert_eq!(Solution::length_of_longest_substring(s), 3);
  }

  #[test]
  fn example_2() {
    let s = "bbbbb".to_string();
    assert_eq!(Solution::length_of_longest_substring(s), 1);
  }

  #[test]
  fn example_3() {
    let s = "pwwkew".to_string();
    assert_eq!(Solution::length_of_longest_substring(s), 3);
  }

  #[test]
  fn empty_string() {
    let s = "".to_string();
    assert_eq!(Solution::length_of_longest_substring(s), 0);
  }
}
