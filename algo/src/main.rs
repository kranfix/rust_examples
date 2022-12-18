use std::{sync::mpsc::channel, thread};

fn main() {
    print_square_sorted(vec![]);
    print_square_sorted(vec![1, 2, 4, 8]);
    print_square_sorted(vec![-8, -4, -2, -1]);
    print_square_sorted(vec![-8, -4, -2, -1, 1, 2, 4, 8]);
    print_square_sorted(vec![-8, -4, -2, -1, 0, 0, 1, 2, 4, 8]);
    print_square_sorted(vec![-1, 0]);
    print_square_sorted(vec![0, 1]);
    print_square_sorted(vec![0]);
    print_square_sorted(vec![1]);
    print_square_sorted(vec![-1]);
    print_square_sorted(vec![-1, 1]);

    print_natural_numbers(150);
}

fn print_square_sorted(list: Vec<i64>) {
    let fist_pos_idx = list
        .iter()
        .enumerate()
        .find_map(|(i, val)| if *val > 0 { Some(i) } else { None })
        .unwrap_or(list.len());

    let (neg, pos) = (
        list[0..fist_pos_idx].iter().rev(),
        list[fist_pos_idx..].iter(),
    );

    let mut left_iter = neg.map(|v| v * v);
    let mut right_iter = pos.map(|v| v * v);

    let mut left = left_iter.next();
    let mut right = right_iter.next();

    print!("[");
    while left.is_some() || right.is_some() {
        let l = left.unwrap_or(i64::MAX);
        let r = right.unwrap_or(i64::MAX);

        if l < r {
            print!("{l}, ");
            left = left_iter.next();
        } else {
            print!("{r}, ");
            right = right_iter.next();
        }
    }
    println!("]");
}

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
