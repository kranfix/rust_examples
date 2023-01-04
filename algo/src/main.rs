#![allow(dead_code)]

pub mod box_example;
pub mod hanoi;
pub mod hanoi_state_machine;
pub mod natural_numbers;
pub mod rc_example;
pub mod square_sorted;

use natural_numbers::print_natural_numbers;

fn main() {
  print_natural_numbers(150);
}
