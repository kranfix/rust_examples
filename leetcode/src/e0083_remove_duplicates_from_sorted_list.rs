use crate::helpers::list_node_box::ListNode;

struct Solution;

impl Solution {
  pub fn delete_duplicates(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut created_head = head?;
    let mut current = created_head.next.take();
    let mut tail = &mut created_head;

    while let Some(mut node) = current {
      current = node.next.take();
      if node.val != tail.val {
        tail.next = Some(node);
        tail = tail.next.as_mut().unwrap();
      }
    }
    Some(created_head)
  }
}

#[cfg(test)]
mod test {
  use crate::helpers::list_node_box::{IntoListNode, ToVec};

  use super::Solution;

  #[test]
  fn example_1() {
    let head = vec![1, 1, 2].into_list_node();
    let output = Solution::delete_duplicates(head).to_vec();
    assert_eq!(output, [1, 2]);
  }

  #[test]
  fn example_2() {
    let head = vec![1, 1, 2, 3, 3].into_list_node();
    let output = Solution::delete_duplicates(head).to_vec();
    assert_eq!(output, [1, 2, 3]);
  }
}
