use std::{sync::mpsc::channel, thread};

fn print_natural_numbers(n: u64) {
  let (sender1, receiver1) = channel();
  let (sender2, receiver2) = channel();

  sender2.send(()).unwrap();

  thread::scope(|s| {
    let t1 = s.spawn(move || {
      for i in (1..=n).step_by(2) {
        receiver2.recv().unwrap();
        println!("T0: {i}");
        match sender1.send(()) {
          Ok(_) => {}
          Err(_) => break,
        };
      }
    });

    let t2 = s.spawn(move || {
      for i in (2..=n).step_by(2) {
        receiver1.recv().unwrap();
        println!("T1: {i}");
        match sender2.send(()) {
          Ok(_) => {}
          Err(_) => break,
        };
      }
    });

    t1.join().unwrap();
    t2.join().unwrap();
  });
}
