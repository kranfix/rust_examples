struct BoxNode {
  val: i32,
  next: Option<Box<BoxNode>>,
}

impl Drop for BoxNode {
  fn drop(&mut self) {
    println!("BoxNode({}): dropped", self.val);
  }
}

fn drop_example() {
  let node = BoxNode { val: 3, next: None };
}

fn circular_box() {
  let mut a = Box::new(BoxNode { val: 1, next: None }); // x
  let mut b = Box::new(BoxNode { val: 2, next: None }); // y
  let mut c = Box::new(BoxNode { val: 3, next: None }); // z

  b.next = Some(c);
  //a.next = Some(b);
  //a.next.as_mut().unwrap().next.as_mut().unwrap().next = Some(a);
}
