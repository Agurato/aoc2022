mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod utils;

use std::env;

fn main() {
    let arguments: Vec<String> = env::args().collect();
    let day_str = arguments.get(1).unwrap();
    let day_number: u8 = day_str.parse().expect("line should contain a number");

    let is_test = arguments.get(2) == Some(&"test".to_string());
    match day_number {
        1 => day1::day(is_test),
        2 => day2::day(is_test),
        3 => day3::day(is_test),
        4 => day4::day(is_test),
        5 => day5::day(is_test),
        6 => day6::day(is_test),
        _ => panic!("kaput"),
    }
}
