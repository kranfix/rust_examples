use crate::helpers::list_node_box::ListNode;

struct Solution;

impl Solution {
  pub fn add_two_numbers(
    l1: Option<Box<ListNode>>,
    l2: Option<Box<ListNode>>,
  ) -> Option<Box<ListNode>> {
    add(l1, l2, 0)
  }
}

fn add(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>, carry: i32) -> Option<Box<ListNode>> {
  let (mut sum, n1, n2) = match (l1, l2) {
    (None, None) => {
      if carry > 0 {
        return Some(Box::new(ListNode::new(carry)));
      } else {
        return None;
      }
    }
    (Some(node1), None) => (node1.val + carry, node1.next, None),
    (None, Some(node2)) => (node2.val + carry, node2.next, None),
    (Some(node1), Some(node2)) => (node1.val + node2.val + carry, node1.next, node2.next),
  };
  let mut carry = 0;
  if sum > 9 {
    carry = 1;
    sum -= 10;
  }

  let mut sum_node = ListNode::new(sum);

  sum_node.next = add(n1, n2, carry);

  Some(Box::new(sum_node))
}

#[cfg(test)]
mod test {
  use crate::helpers::list_node_box::{IntoListNode, ToVec};

  use super::Solution;

  #[test]
  fn example_1() {
    let l1 = vec![2, 4, 3].into_list_node();
    let l2 = vec![5, 6, 4].into_list_node();

    let sum = Solution::add_two_numbers(l1, l2);
    assert_eq!(sum.to_vec(), [7, 0, 8]);
  }

  #[test]
  fn example_2() {
    let l1 = vec![0].into_list_node();
    let l2 = vec![0].into_list_node();

    let sum = Solution::add_two_numbers(l1, l2);
    assert_eq!(sum.to_vec(), [0]);
  }

  #[test]
  fn example_3() {
    let l1 = vec![9, 9, 9, 9, 9, 9, 9].into_list_node();
    let l2 = vec![9, 9, 9, 9].into_list_node();

    let sum = Solution::add_two_numbers(l1, l2);
    assert_eq!(sum.to_vec(), [8, 9, 9, 9, 0, 0, 0, 1]);
  }
}
