use std::iter::Iterator;

#[derive(Clone)]
struct Hanoi<T = u32> {
  num_disks: u32,
  from: T,
  to: T,
  spare: T,
}

impl<T> Hanoi<T> {
  fn new(num_disks: u32, from: T, to: T, spare: T) -> Hanoi<T> {
    Hanoi {
      num_disks,
      from,
      to,
      spare,
    }
  }

  fn iter(&self) -> HanoiIter<T>
  where
    T: Clone,
  {
    let stack = vec![self.clone()];
    HanoiIter { stack }
  }
}

struct HanoiIter<T> {
  stack: Vec<Hanoi<T>>,
}

impl<T: Clone> Iterator for HanoiIter<T> {
  type Item = (T, T);

  fn next(&mut self) -> Option<(T, T)> {
    while let Some(h) = self.stack.pop() {
      if h.num_disks == 1 {
        return Some((h.from, h.to));
      }
      self.stack.push(Hanoi::new(
        h.num_disks - 1,
        h.spare.clone(),
        h.to.clone(),
        h.from.clone(),
      ));
      self
        .stack
        .push(Hanoi::new(1, h.from.clone(), h.to.clone(), h.spare.clone()));
      self
        .stack
        .push(Hanoi::new(h.num_disks - 1, h.from, h.spare, h.to));
    }
    None
  }
}

#[cfg(test)]
mod test {
  use super::Hanoi;

  #[test]
  fn example_n_1() {
    let h = Hanoi::new(1, 1, 2, 3);
    let steps = h.iter().collect::<Vec<_>>();
    assert_eq!(steps, [(1, 2)]);
  }

  #[test]
  fn example_n_2() {
    let h = Hanoi::new(2, 1, 2, 3);
    let steps = h.iter().collect::<Vec<_>>();
    assert_eq!(steps, [(1, 3), (1, 2), (3, 2)]);
  }

  #[test]
  fn example_n_3() {
    let h = Hanoi::new(3, "A", "C", "B");
    let steps = h.iter().collect::<Vec<_>>();
    let expected = [
      ("A", "C"),
      ("A", "B"),
      ("C", "B"),
      ("A", "C"),
      ("B", "A"),
      ("B", "C"),
      ("A", "C"),
    ];
    assert_eq!(steps, expected);
  }
}
