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
