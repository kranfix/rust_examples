struct Solution;

impl Solution {
  pub fn max_sub_array(nums: Vec<i32>) -> i32 {
    let (mut l, mut g): (i32, i32) = (nums[0], nums[0]);
    for &num in nums.iter().skip(1) {
      l = if l > 0 { num + l } else { num };
      g = g.max(l);
    }
    g
  }
}

fn recursive_max_sub_array(nums: &mut Vec<i32>) -> (i32, i32) {
  if nums.len() == 1 {
    return (nums[0], nums[0]);
  }
  match nums.pop() {
    Some(z) => {
      let (mut l, mut g) = recursive_max_sub_array(nums);
      l = if l > 0 { l + z } else { z };
      g = g.max(l);
      (l, g)
    }
    None => unreachable!(),
  }
}

#[cfg(test)]
mod test {
  use super::Solution;

  #[test]
  fn example_1() {
    let nums = vec![-2, 1, -3, 4, -1, 2, 1, -5, 4];
    assert_eq!(Solution::max_sub_array(nums), 6);
  }

  #[test]
  fn example_2() {
    let nums = vec![1];
    assert_eq!(Solution::max_sub_array(nums), 1);
  }

  #[test]
  fn example_3() {
    let nums = vec![5, 4, -1, 7, 8];
    assert_eq!(Solution::max_sub_array(nums), 23);
  }

  #[test]
  fn one_negative_element() {
    let nums = vec![-1];
    assert_eq!(Solution::max_sub_array(nums), -1);
  }
}
