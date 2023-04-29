use std::{
  cell::{Ref, RefCell},
  rc::Rc,
};

#[derive(PartialEq, Eq)]
pub struct DoubleLinkedNode<T>(Rc<RefCell<Inner<T>>>);

impl<T> Clone for DoubleLinkedNode<T> {
  fn clone(&self) -> Self {
    Self(self.0.clone())
  }
}

impl<T> DoubleLinkedNode<T> {
  #[inline]
  pub fn new(val: T) -> Self {
    Self(Rc::new(RefCell::new(Inner::new(val))))
  }

  pub fn val(&self) -> Ref<T> {
    Ref::map(self.0.borrow(), |inner| &inner.val)
  }

  pub fn next(&self) -> Option<Self> {
    self.0.borrow().next.clone()
  }

  pub fn prev(&self) -> Option<Self> {
    self.0.borrow().prev.clone()
  }

  pub fn set_next(&self, next: DoubleLinkedNode<T>) {
    {
      next.0.borrow_mut().prev = Some(self.clone());
    }
    self.0.borrow_mut().next = Some(next);
  }
}

impl<T: std::fmt::Debug> std::fmt::Debug for DoubleLinkedNode<T> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.debug_struct("DoubleLinkedNode")
      .field("val", &self.val())
      .field("next", &self.0.borrow().next)
      .finish()
  }
}

#[derive(PartialEq, Eq, Clone)]
struct Inner<T> {
  val: T,
  prev: Option<DoubleLinkedNode<T>>,
  next: Option<DoubleLinkedNode<T>>,
}

impl<T> Inner<T> {
  #[inline]
  pub fn new(val: T) -> Self {
    Inner {
      next: None,
      val,
      prev: None,
    }
  }
}

#[macro_export]
macro_rules! double_linked {
  () => {
    let node = $crate::helpers::DoubleLinkedNode::new(0);
    (node.clone(), node)
  };
  ($x:expr $(,)?) => {{
    let node = $crate::helpers::DoubleLinkedNode::new($x);
    (node.clone(), node)
  }};
  ($x:expr,$($y:expr),+) => {{
    let first = $crate::helpers::DoubleLinkedNode::new($x);
    let (second, last) = double_linked!($($y),+);
    first.set_next(second);
    (first, last)
  }};
  (@first $x:expr $(,)?) => {{
    $crate::helpers::DoubleLinkedNode::new($x)
  }};
  (@first $x:expr,$($y:expr),+) => {{
    let first = $crate::helpers::DoubleLinkedNode::new($x);
    let second = double_linked!(@first $($y),+);
    first.set_next(second);
    first
  }};
  (@last $($y:expr),+) => {{
    let (_, last) = double_linked!($($y),+);
    last
  }};
}

impl<T: std::fmt::Debug> std::fmt::Display for DoubleLinkedNode<T> {
  /// # DoubleLinkedNode display formatter
  ///
  /// ```
  /// use leetcode::double_linked;
  ///
  /// let (first, last) = double_linked!(1);
  /// assert_eq!(first.to_string(), "DoubleLinkedNode(1)");
  /// assert_eq!(last.to_string(), "DoubleLinkedNode(1)");
  ///
  /// let (first, last) = double_linked!(1,2,3,4);
  /// assert_eq!(first.to_string(), "DoubleLinkedNode(1 => 2 => 3 => 4)");
  /// assert_eq!(last.to_string(), "DoubleLinkedNode(4)");
  ///
  /// let first = double_linked!(@first 1);
  /// assert_eq!(first.to_string(), "DoubleLinkedNode(1)");
  ///
  /// let first = double_linked!(@first 1,2,3,4);
  /// assert_eq!(first.to_string(), "DoubleLinkedNode(1 => 2 => 3 => 4)");
  ///
  /// let last = double_linked!(@last 1);
  /// assert_eq!(last.to_string(), "DoubleLinkedNode(1)");
  ///
  /// let last = double_linked!(@last 1,2,3,4);
  /// assert_eq!(last.to_string(), "DoubleLinkedNode(4)");
  /// ```
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    vec![1];
    write!(f, "DoubleLinkedNode({:?}", self.val())?;
    let mut next = self.next();
    while let Some(node) = next {
      write!(f, " => {:?}", node.val())?;
      next = node.next();
    }
    write!(f, ")")
  }
}
