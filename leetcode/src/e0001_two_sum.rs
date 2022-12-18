use std::collections::HashMap;

struct Solution;

impl Solution {
  pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut map = HashMap::new();

    let n = nums.len();
    for (i, num) in nums.iter().enumerate() {
      let complement = target - num;
      match map.get(&complement) {
        Some(&complement_index) => return vec![complement_index as i32, i as i32],
        None => map.insert(num, i),
      };
    }
    vec![n as i32, n as i32]
  }
}

#[cfg(test)]
mod test {
  use super::Solution;

  #[test]
  fn example_1() {
    let nums = vec![2, 7, 11, 15];
    let result = Solution::two_sum(nums, 9);
    assert_eq!(result, vec![0, 1]);
  }

  #[test]
  fn example_2() {
    let nums = vec![3, 2, 4];
    let result = Solution::two_sum(nums, 6);
    assert_eq!(result, vec![1, 2]);
  }

  #[test]
  fn example_3() {
    let nums = vec![3, 3];
    let result = Solution::two_sum(nums, 6);
    assert_eq!(result, vec![0, 1]);
  }
}
