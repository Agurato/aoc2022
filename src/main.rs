mod day1;
mod utils;

use std::env;

fn main() {
    let arguments: Vec<String> = env::args().collect();
    let day_str = arguments.get(1).unwrap();
    let day_number: u8 = day_str.parse().expect("line should contain a number");

    let is_test = arguments.get(2) == Some(&"test".to_string());
    match day_number {
        1 => day1::day(is_test),
        _ => panic!("kaput"),
    }
}
