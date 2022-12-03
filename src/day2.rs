use crate::utils;

pub fn day(test: bool) {
    println!("{}", ex1(test));
    println!("{}", ex2(test));
}

pub fn ex1(test: bool) -> i32 {
    let lines = utils::read_input(2, test).expect("Should have been able to read the file");
    let mut total: i32 = 0;
    for line in lines {
        let mut bytes: Vec<i16> = line.bytes().map(i16::from).take(3).collect();
        bytes[2] -= 23; // Transform 'X', 'Y', 'Z' into 'A', 'B', 'C'
        let choice_score = bytes[2] - 64; // 'A' is 65
        let diff = bytes[2] - bytes[0]; // diff ∈ [-1;2]
        let win_score = ((diff + 1).rem_euclid(3)) * 3; // shift by 1, then positive modulo to have 0: loss, 1: draw, 2: win
        total += (choice_score + win_score) as i32;
    }
    total
}

pub fn ex2(test: bool) -> i32 {
    let lines = utils::read_input(2, test).expect("Should have been able to read the file");
    let mut total: i32 = 0;
    for line in lines {
        let bytes: Vec<i16> = line.bytes().map(i16::from).take(3).collect();
        let win_score = (bytes[2] - 88) * 3; // 'X' is 88
        let shift = bytes[2] - 89; // [-1;1]
        let choice_score = ((bytes[0] - 65) + shift).rem_euclid(3) + 1; // [0;2] + shift, then modulo to have cycle, then +1 for score
        total += (choice_score + win_score) as i32;
    }
    total
}
