use std::collections::{HashSet, VecDeque};

use crate::utils;

pub fn day(test: bool) {
    let lines = utils::read_input(6, test).expect("Should have been able to read the file");
    println!("{}", ex1(&lines));
    println!("{}", ex2(&lines));
}

pub fn ex1(lines: &Vec<String>) -> i32 {
    let mut last3: VecDeque<char> = VecDeque::new();
    let mut packet = lines[0].chars();
    for _ in 0..3 {
        last3.push_front(packet.next().unwrap())
    }
    let mut count = 4;
    loop {
        match packet.next() {
            Some(c) => {
                if !last3.contains(&c) && !has_doubles(&last3) {
                    break;
                }
                last3.pop_back();
                last3.push_front(c);
                count += 1;
            }
            None => break,
        }
    }
    count
}

pub fn ex2(lines: &Vec<String>) -> i32 {
    let mut last13: VecDeque<char> = VecDeque::new();
    let mut packet = lines[0].chars();
    for _ in 0..13 {
        last13.push_front(packet.next().unwrap())
    }
    let mut count = 14;
    loop {
        match packet.next() {
            Some(c) => {
                if !last13.contains(&c) && !has_doubles(&last13) {
                    break;
                }
                last13.pop_back();
                last13.push_front(c);
                count += 1;
            }
            None => break,
        }
    }
    count
}

fn has_doubles(vector: &VecDeque<char>) -> bool {
    let set: HashSet<_> = vector.iter().collect();
    set.len() != vector.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day() {
        let lines = utils::read_input(6, false).expect("Should have been able to read the file");
        assert_eq!(ex1(&lines), 1544);
        assert_eq!(ex2(&lines), 2145);
    }
}
