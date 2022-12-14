struct Solution;

impl Solution {
  pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
    if nums1.is_empty() {
      return find_median_in_sorted_arrays_by_index(&nums2);
    }

    if nums2.is_empty() {
      return find_median_in_sorted_arrays_by_index(&nums1);
    }

    find_mean(nums1, nums2)
  }
}

fn find_mean(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
  let total = nums1.len() + nums2.len();

  let mut index1: usize = 0;
  let mut index2: usize = 0;

  let length = (total >> 1) + 1;

  let mut val1 = if nums1[0] > nums2[0] {
    nums2[0]
  } else {
    nums1[0]
  };
  let mut val2 = val1;

  for _ in 0..length {
    val1 = val2;

    if index1 >= nums1.len() {
      val2 = nums2[index2];
      index2 += 1;
      continue;
    }

    if index2 >= nums2.len() {
      val2 = nums1[index1];
      index1 += 1;
      continue;
    }

    let current1 = nums1[index1];
    let current2 = nums2[index2];
    if current1 > current2 {
      val2 = current2;
      index2 += 1;
    } else {
      val2 = current1;
      index1 += 1;
    }
  }

  if total % 2 == 0 {
    ((val1 + val2) as f64) / 2.0
  } else {
    val2 as f64
  }
}

fn find_median_in_sorted_arrays_by_index(nums: &[i32]) -> f64 {
  let len = nums.len();
  let mid = len >> 1; // = sum / 2
  if len % 2 == 1 {
    nums[mid] as f64
  } else {
    ((nums[mid] + nums[mid - 1]) as f64) / 2.0
  }
}

#[cfg(test)]
mod test {
  use super::Solution;

  #[test]
  fn example_1() {
    let nums1 = vec![1, 3];
    let nums2 = vec![2];
    assert_eq!(
      Solution::find_median_sorted_arrays(nums1.clone(), nums2.clone()),
      2.0
    );
    assert_eq!(Solution::find_median_sorted_arrays(nums2, nums1), 2.0);
  }

  #[test]
  fn example_2() {
    let nums1 = vec![1, 2];
    let nums2 = vec![3, 4];
    assert_eq!(
      Solution::find_median_sorted_arrays(nums1.clone(), nums2.clone()),
      2.5
    );
    assert_eq!(Solution::find_median_sorted_arrays(nums2, nums1), 2.5);
  }

  #[test]
  fn one_is_empty_other_has_odd() {
    let nums1 = vec![1, 2, 3];
    let nums2 = vec![];
    assert_eq!(
      Solution::find_median_sorted_arrays(nums1.clone(), nums2.clone()),
      2.0
    );
    assert_eq!(Solution::find_median_sorted_arrays(nums2, nums1), 2.0);
  }

  #[test]
  fn one_is_empty_other_has_even() {
    let nums1 = vec![1, 2, 3, 4];
    let nums2 = vec![];
    assert_eq!(
      Solution::find_median_sorted_arrays(nums1.clone(), nums2.clone()),
      2.5
    );
    assert_eq!(Solution::find_median_sorted_arrays(nums2, nums1), 2.5);
  }
}
