struct Solution;

impl Solution {
  pub fn count_good_triplets(arr: Vec<i32>, a: i32, b: i32, c: i32) -> i32 {
    let mut count = 0;
    let mut f = |_: &[i32], _, _, _| count += 1;
    good_triplets(&arr, a, b, c, &mut f);
    count
  }
}

fn good_triplets<F>(arr: &[i32], a: i32, b: i32, c: i32, f: &mut F)
where
  F: FnMut(&[i32], usize, usize, usize),
{
  for i in 0..arr.len() - 2 {
    let x = arr[i];
    for j in (i + 1)..arr.len() - 1 {
      let y = arr[j];
      if (x - y).abs() <= a {
        for (k, &z) in arr.iter().enumerate().skip(j + 1) {
          if (y - z).abs() <= b && (z - x).abs() <= c {
            f(arr, i, j, k);
          }
        }
      }
    }
  }
}

#[cfg(test)]
mod test {
  use crate::e1534_count_good_triplets::Solution;

  use super::good_triplets;

  fn collect_good_triplets(arr: Vec<i32>, a: i32, b: i32, c: i32) -> Vec<(i32, i32, i32)> {
    let mut triplets = Vec::new();
    let mut f = |arr: &[i32], i, j, k| triplets.push((arr[i], arr[j], arr[k]));
    good_triplets(&arr, a, b, c, &mut f);
    triplets
  }

  #[test]
  fn example_1() {
    let arr = vec![3, 0, 1, 1, 9, 7];
    let expected = [(3, 0, 1), (3, 0, 1), (3, 1, 1), (0, 1, 1)];
    assert_eq!(collect_good_triplets(arr.clone(), 7, 2, 3), expected);
    assert_eq!(
      Solution::count_good_triplets(arr, 7, 2, 3),
      expected.len() as i32
    );
  }

  #[test]
  fn example_2() {
    let arr = vec![1, 1, 2, 2, 3];
    let expected = [];
    assert_eq!(collect_good_triplets(arr.clone(), 0, 0, 1), expected);
    assert_eq!(
      Solution::count_good_triplets(arr, 0, 0, 1),
      expected.len() as i32
    );
  }
}
