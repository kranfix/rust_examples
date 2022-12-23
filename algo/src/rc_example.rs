use std::{
  borrow::Borrow,
  cell::RefCell,
  rc::{Rc, Weak},
};

struct RcNode {
  val: i32,
  next: Option<Rc<RefCell<RcNode>>>,
  //next: RefCell<Weak<RcNode>>,
}

impl Drop for RcNode {
  fn drop(&mut self) {
    println!("RcNode({}): dropped", self.val);
  }
}

struct WeakNode {
  val: i32,
  next: RefCell<Weak<WeakNode>>,
}

impl Drop for WeakNode {
  fn drop(&mut self) {
    println!("WeakNode({}): dropped", self.val);
  }
}

fn circular_rc() {
  let a = Rc::new(RefCell::new(RcNode { val: 1, next: None }));
  let b = Rc::new(RefCell::new(RcNode { val: 2, next: None }));
  let c = Rc::new(RefCell::new(RcNode { val: 3, next: None }));

  (*c).borrow_mut().next = Some(a.clone()); // ((1,1),(0,1),(0,1))
  (*b).borrow_mut().next = Some(c); // ((1,1),(0,1),(0,1))
  (*a).borrow_mut().next = Some(b); // ((1,1),(0,1),(0,1))

  println!("before drop(a)");
  drop(a); // ((1,1),(0,1),(0,1))
  println!("after drop(a)");
}

fn circular_weak() {
  let a = Rc::new(WeakNode {
    val: 1,
    next: RefCell::new(Weak::new()),
  });
  let b = Rc::new(WeakNode {
    val: 2,
    next: RefCell::new(Weak::new()),
  });
  let c = Rc::new(WeakNode {
    val: 3,
    next: RefCell::new(Weak::new()),
  });

  *c.next.borrow_mut() = Rc::downgrade(&a);
  *b.next.borrow_mut() = Rc::downgrade(&c);
  *a.next.borrow_mut() = Rc::downgrade(&b);

  println!("before drop(a)");
  drop(a); // ((1,1),(0,1),(0,1))
  println!("after drop(a)");

  let c_a = match (*c.next.borrow()).upgrade() {
    None => {
      println!("Debo manejar el error");
      return;
    }
    Some(a_ref) => (*a_ref).borrow().val,
  };

  println!("c.next.val = {}", c_a);
}
