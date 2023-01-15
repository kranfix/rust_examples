#[derive(Debug, Clone, PartialEq)]
pub struct Queen(u8, u8); // (i, j)

impl Queen {
  fn has_conflict(&self, other: &Queen) -> bool {
    if self.0 == other.0 || self.1 == other.1 {
      return true;
    }

    if self.0.abs_diff(other.0) == self.1.abs_diff(other.1) {
      return true;
    }

    false
  }

  fn has_conflict_with_any(&self, queens: &[Queen]) -> bool {
    queens.iter().any(|q| self.has_conflict(q))
  }
}

impl PartialEq<(u8, u8)> for Queen {
  fn eq(&self, other: &(u8, u8)) -> bool {
    self.0 == other.0 && self.1 == other.1
  }
}

struct Table {
  n: u8,
  queens: Vec<Queen>,
}

impl Table {
  fn new(n: u8) -> Table {
    Table { n, queens: vec![] }
  }

  fn can_be_added(&self, queen: &Queen) -> bool {
    if (self.queens.len() as u8) < self.n && queen.0 < self.n && queen.1 < self.n {
      return !self.queens.iter().any(|c| c.has_conflict(queen));
    }
    false
  }

  fn propose_next_queen(&self, since: u8) -> Option<Queen> {
    let j = self.queens.len() as u8;
    if j >= self.n {
      return None;
    }
    for i in since..self.n {
      let q = Queen(i, j);
      if !q.has_conflict_with_any(&self.queens) {
        return Some(q);
      }
    }
    None
  }

  fn is_full(&self) -> bool {
    self.n == (self.queens.len() as u8)
  }

  fn move_next(&mut self) -> bool {
    while let Some(l) = self.queens.pop() {
      if let Some(q) = self.propose_next_queen(l.0 + 1) {
        self.queens.push(q);
        return false;
      }
    }
    true
  }
}

pub struct NQueens {
  inner: Table,
  is_done: bool,
}

impl NQueens {
  fn new(n: u8) -> NQueens {
    NQueens {
      inner: Table::new(n),
      is_done: false,
    }
  }
}

impl Iterator for NQueens {
  type Item = Vec<Queen>;

  fn next(&mut self) -> Option<Self::Item> {
    while !self.is_done {
      while let Some(q) = self.inner.propose_next_queen(0) {
        self.inner.queens.push(q);
      }
      if self.inner.is_full() {
        let val = self.inner.queens.clone();
        self.is_done = self.inner.move_next();
        return Some(val);
      } else {
        self.is_done = self.inner.move_next();
      }
    }
    None
  }
}

#[cfg(test)]
mod test {
  use super::NQueens;

  fn helper<const N: usize, const M: usize>(solutions: [[(u8, u8); N]; M]) {
    let results = NQueens::new(N as u8).collect::<Vec<Vec<_>>>();
    assert_eq!(results.len(), solutions.len());
    for (res, sol) in results.iter().zip(solutions.iter()) {
      assert_eq!(res, sol);
    }
  }

  #[test]
  fn example_1() {
    helper([[(0, 0)]]);
  }

  #[test]
  fn example_2() {
    helper::<2, 0>([]);
  }

  #[test]
  fn example_3() {
    helper::<3, 0>([]);
  }

  #[test]
  fn example_4() {
    helper([
      [(1, 0), (3, 1), (0, 2), (2, 3)],
      [(2, 0), (0, 1), (3, 2), (1, 3)],
    ]);
  }

  #[test]
  fn example_5() {
    helper([
      [(0, 0), (2, 1), (4, 2), (1, 3), (3, 4)],
      [(0, 0), (3, 1), (1, 2), (4, 3), (2, 4)],
      [(1, 0), (3, 1), (0, 2), (2, 3), (4, 4)],
      [(1, 0), (4, 1), (2, 2), (0, 3), (3, 4)],
      [(2, 0), (0, 1), (3, 2), (1, 3), (4, 4)],
      [(2, 0), (4, 1), (1, 2), (3, 3), (0, 4)],
      [(3, 0), (0, 1), (2, 2), (4, 3), (1, 4)],
      [(3, 0), (1, 1), (4, 2), (2, 3), (0, 4)],
      [(4, 0), (1, 1), (3, 2), (0, 3), (2, 4)],
      [(4, 0), (2, 1), (0, 2), (3, 3), (1, 4)],
    ]);
  }
}

#[cfg(test)]
mod queen_test {
  use super::Queen;

  #[test]
  fn has_conflict_test() {
    let q = Queen(2, 3);

    assert!(!q.has_conflict(&Queen(0, 0)));
    assert!(!q.has_conflict(&Queen(0, 2)));
    assert!(!q.has_conflict(&Queen(0, 4)));
    assert!(!q.has_conflict(&Queen(4, 2)));

    assert!(q.has_conflict(&Queen(2, 3)));

    assert!(q.has_conflict(&Queen(0, 1)));
    assert!(q.has_conflict(&Queen(1, 2)));
    assert!(q.has_conflict(&Queen(3, 4)));
    assert!(q.has_conflict(&Queen(4, 5)));

    assert!(q.has_conflict(&Queen(0, 5)));
    assert!(q.has_conflict(&Queen(1, 4)));
    assert!(q.has_conflict(&Queen(3, 2)));
    assert!(q.has_conflict(&Queen(4, 1)));
    assert!(q.has_conflict(&Queen(5, 0)));

    assert!(q.has_conflict(&Queen(2, 0)));
    assert!(q.has_conflict(&Queen(2, 1)));
    assert!(q.has_conflict(&Queen(2, 2)));
    assert!(q.has_conflict(&Queen(2, 4)));
    assert!(q.has_conflict(&Queen(2, 5)));

    assert!(q.has_conflict(&Queen(0, 3)));
    assert!(q.has_conflict(&Queen(1, 3)));
    assert!(q.has_conflict(&Queen(3, 3)));
    assert!(q.has_conflict(&Queen(4, 3)));
    assert!(q.has_conflict(&Queen(5, 3)));
  }
}
