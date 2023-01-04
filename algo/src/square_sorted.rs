use std::{fmt::Display, ops::Mul};

pub fn print_square_sorted(list: Vec<i64>) {
  let mut f = |n: i64| print!("{n},");
  print!("[");
  square_sorted(list, i64::MAX, &mut f);
  println!("]");
}

fn square_sorted<T, F>(list: Vec<T>, max: T, f: &mut F)
where
  T: Ord + Display + Default + Copy + Mul<T, Output = T>,
  F: FnMut(T),
{
  let fist_pos_idx = list
    .iter()
    .enumerate()
    .find_map(|(i, val)| if *val > T::default() { Some(i) } else { None })
    .unwrap_or(list.len());

  let mut left_iter = list[0..fist_pos_idx].iter().rev().map(|v| *v * *v);
  let mut right_iter = list[fist_pos_idx..].iter().map(|v| *v * *v);

  let mut left = left_iter.next();
  let mut right = right_iter.next();
  while left.is_some() || right.is_some() {
    let l = left.unwrap_or(max);
    let r = right.unwrap_or(max);

    if l < r {
      f(l);
      left = left_iter.next();
    } else {
      f(r);
      right = right_iter.next();
    }
  }
}

#[cfg(test)]
mod test {
  use super::square_sorted;

  fn sort(list: Vec<i64>) -> Vec<i64> {
    let mut sorted = Vec::new();
    let mut f = |n: i64| sorted.push(n);
    square_sorted(list, i64::MAX, &mut f);
    sorted
  }

  #[test]
  fn example() {
    assert_eq!(sort(vec![]), []);
    assert_eq!(sort(vec![1, 2, 4, 8]), [1, 4, 16, 64]);
    assert_eq!(sort(vec![-8, -4, -2, -1]), [1, 4, 16, 64]);
    assert_eq!(
      sort(vec![-8, -4, -2, -1, 1, 2, 4, 8]),
      [1, 1, 4, 4, 16, 16, 64, 64]
    );
    assert_eq!(
      sort(vec![-8, -4, -2, -1, 0, 0, 1, 2, 4, 8]),
      [0, 0, 1, 1, 4, 4, 16, 16, 64, 64]
    );
    assert_eq!(sort(vec![-1, 0]), [0, 1]);
    assert_eq!(sort(vec![0, 1]), [0, 1]);
    assert_eq!(sort(vec![0]), [0]);
    assert_eq!(sort(vec![1]), [1]);
    assert_eq!(sort(vec![-1]), [1]);
    assert_eq!(sort(vec![-1, 1]), [1, 1]);
  }
}
