use std::collections::VecDeque;
use string_builder::Builder;

use crate::utils;

#[derive(Debug)]
struct Move {
    size: u32,
    from: usize,
    to: usize,
}

pub fn day(test: bool) {
    let lines = utils::read_input(5, test).expect("Should have been able to read the file");
    println!("{}", ex1(&lines));
    println!("{}", ex2(&lines));
}

pub fn ex1(lines: &Vec<String>) -> String {
    let (mut stacks, moves) = build_stacks_and_moves(lines);
    for m in moves {
        for _ in 0..m.size {
            match stacks[m.from - 1].pop_front() {
                Some(c) => stacks[m.to - 1].push_front(c),
                _ => (),
            };
        }
    }

    let mut res: Vec<char> = Vec::new();
    for s in stacks {
        match s.front() {
            Some(c) => res.push(c.clone()),
            _ => (),
        };
    }
    res.iter().collect()
}

pub fn ex2(lines: &Vec<String>) -> i32 {
    for line in lines {}
    0
}

fn build_stacks_and_moves(lines: &Vec<String>) -> (Vec<VecDeque<char>>, Vec<Move>) {
    let mut stacks_desc: Vec<&String> = Vec::new();
    let mut move_desc: Vec<&String> = Vec::new();
    let mut stack_number: usize = 0;
    for line in lines {
        if line.contains("[") {
            stacks_desc.push(line)
        }
        if line.starts_with(" 1") {
            stack_number = usize::from(line.as_bytes()[line.len() - 2] - 48) // '0' is 48
        }
        if line.starts_with("move") {
            move_desc.push(line);
        }
    }
    let mut stacks: Vec<VecDeque<char>> = vec![VecDeque::new(); stack_number];
    for sd in stacks_desc {
        let mut i = 0;
        for c in sd
            .as_bytes()
            .iter()
            .map(|x| x.clone() as char)
            .skip(1)
            .step_by(4)
        {
            if c != ' ' {
                stacks[i].push_back(c)
            }
            i += 1;
        }
    }
    let mut moves: Vec<Move> = Vec::new();
    for md in move_desc {
        let md_split: Vec<&str> = md.split(" ").collect();
        moves.push(Move {
            size: md_split[1].parse().unwrap(),
            from: md_split[3].parse().unwrap(),
            to: md_split[5].parse().unwrap(),
        });
    }

    (stacks, moves)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day() {
        let lines = utils::read_input(5, false).expect("Should have been able to read the file");
        assert_eq!(ex1(&lines), "VCTFTJQCG");
        assert_eq!(ex2(&lines), 0);
    }
}
