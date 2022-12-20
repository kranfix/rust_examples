struct Solution;

impl Solution {
  pub fn first_missing_positive(nums: Vec<i32>) -> i32 {
    let mut nums = nums;

    fn clear_back(nums: &mut Vec<i32>, i: usize) {
      loop {
        match nums.last() {
          Some(&l) if l <= i as i32 || l > nums.len() as i32 => nums.pop(),
          _ => return,
        };
      }
    }

    clear_back(&mut nums, 0);

    let mut i = 0;
    while i < nums.len() {
      if nums[i] as usize == i + 1 {
        i += 1;
      } else if nums[i] <= i as i32 || nums[i] > nums.len() as i32 {
        nums.swap_remove(i);
        clear_back(&mut nums, i);
      } else {
        let j = (nums[i] - 1) as usize;
        if nums[j] != nums[i] {
          nums.swap(i, j);
        } else {
          nums.swap_remove(i);
          clear_back(&mut nums, i);
        }
      }
    }

    i as i32 + 1
  }
}

#[cfg(test)]
mod test {
  use super::Solution;

  #[test]
  fn example_1() {
    let nums = vec![1, 2, 0];
    assert_eq!(Solution::first_missing_positive(nums), 3);
  }

  #[test]
  fn example_2() {
    let nums = vec![3, 4, -1, 1];
    assert_eq!(Solution::first_missing_positive(nums), 2);
  }

  #[test]
  fn example_3() {
    let nums = vec![7, 8, 9, 11, 12];
    assert_eq!(Solution::first_missing_positive(nums), 1);
  }

  #[test]
  fn double_two() {
    let nums = vec![2, 2];
    assert_eq!(Solution::first_missing_positive(nums), 1);
  }
}
