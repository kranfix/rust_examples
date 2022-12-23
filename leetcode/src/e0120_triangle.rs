pub fn minimum_total(triangle: Vec<Vec<i32>>) -> i32 {
  let mut triangle = triangle;
  for i in (0..triangle.len() - 1).rev() {
    let last = triangle.pop().unwrap();
    let curr = &mut triangle[i];
    for j in 0..i + 1 {
      curr[j] += last[j].min(last[j + 1]);
    }
  }
  triangle[0][0]
}

#[cfg(test)]
mod test {
  use crate::e0120_triangle::minimum_total;

  #[test]
  fn example_1() {
    let triangle = vec![vec![2], vec![3, 4], vec![6, 5, 7], vec![4, 1, 8, 3]];
    assert_eq!(minimum_total(triangle), 11);
  }

  #[test]
  fn example_2() {
    let triangle = vec![vec![-10]];
    assert_eq!(minimum_total(triangle), -10);
  }
}
