// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
  pub val: i32,
  pub next: Option<Box<ListNode>>,
}

impl ListNode {
  #[inline]
  pub fn new(val: i32) -> Self {
    ListNode { next: None, val }
  }
}

impl std::fmt::Display for ListNode {
  /// # ListNode display formatter
  ///
  /// ```
  /// use leetcode::linked;
  ///
  /// let linked = linked!(1);
  /// assert_eq!(linked.to_string(), "ListNode(1)");
  ///
  /// let linked = linked!(1,2,3,4);
  /// assert_eq!(linked.to_string(), "ListNode(1 => 2 => 3 => 4)");
  /// ```
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    vec![1];
    write!(f, "ListNode({:?}", self.val)?;
    let mut next = &self.next;
    while let Some(node) = next {
      write!(f, " => {:?}", node.val)?;
      next = &node.next;
    }
    write!(f, ")")
  }
}

#[cfg(test)]
pub trait ToVec {
  fn to_vec(self) -> Vec<i32>;
}

#[cfg(test)]
pub trait IntoListNode {
  fn into_list_node(self) -> Option<Box<ListNode>>;
}

#[cfg(test)]
impl ToVec for Option<Box<ListNode>> {
  fn to_vec(self) -> Vec<i32> {
    let mut vec = Vec::new();
    let mut curr = self;
    while let Some(node) = curr {
      vec.push(node.val);
      curr = node.next;
    }
    vec
  }
}

#[cfg(test)]
impl IntoListNode for Vec<i32> {
  fn into_list_node(self) -> Option<Box<ListNode>> {
    let mut head = None;
    for val in self.into_iter().rev() {
      let curr = ListNode { val, next: head };
      head = Some(Box::new(curr));
    }
    head
  }
}

/// # linked!
///
/// This macro helps to create a [ListNode] easily.
///
/// ```
/// use leetcode::linked;
///
/// // Default case
/// let list = linked!();
/// assert_eq!(list.val, 0);
/// assert_eq!(list.next, None);
///
/// // One argument case
/// let list = linked!(1);
/// assert_eq!(list.val, 1);
/// assert_eq!(list.next, None);
///
/// // Many arguments case
/// let list = linked!(1,2,3);
/// assert_eq!(list.val, 1);
/// let next = list.next.unwrap();
/// assert_eq!(next.val, 2);
/// let next = next.next.unwrap();
/// assert_eq!(next.val, 3);
/// assert_eq!(next.next, None);
/// ```
#[macro_export]
macro_rules! linked {

  () => {
    $crate::helpers::ListNode::new(0)
  };
  ($x:expr $(,)?) => {
    $crate::helpers::ListNode{val:$x,next:None}
  };
  ($x:expr,$($y:expr),+) => {
    $crate::helpers::ListNode{val:$x,next:Some(Box::new(linked!($($y),+)))}
  };
}
