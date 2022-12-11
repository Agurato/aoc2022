use crate::utils;

pub fn day(test: bool) {
    let lines = utils::read_input(10, test).expect("Should have been able to read the file");
    println!("{}", ex1(&lines));
    println!("{}", ex2(&lines));
}

#[derive(Debug)]
struct Instruction {
    op: Op,
    value: i32,
}

#[derive(Debug)]
enum Op {
    Noop,
    Addx,
}

pub fn ex1(lines: &Vec<String>) -> i32 {
    let mut x: i32 = 1;
    let mut signal_strength: i32 = 0;
    let mut instructions: Vec<Instruction> = Vec::new();
    for line in lines {
        // let line_split: Vec<&str> = line.split(" ").collect();
        // let op = line_split[0];
        // match op {
        //     "noop" => cycle += 1,
        //     "addx" => {
        //         let value = line_split[1];
        //         for _ in 0..2 {
        //             cycle += 1
        //         }
        //     }
        //     _ => continue,
        // }
        let line_split: Vec<&str> = line.split(" ").collect();
        let op = line_split[0];
        match op {
            "noop" => instructions.push(Instruction {
                op: Op::Noop,
                value: 0,
            }),
            "addx" => {
                instructions.push(Instruction {
                    op: Op::Noop,
                    value: 0,
                });
                instructions.push(Instruction {
                    op: Op::Addx,
                    value: line_split[1].parse().unwrap(),
                });
            }
            _ => continue,
        }
    }
    let mut line_count = 1;
    for (cycle, instruction) in instructions.iter().enumerate() {
        if cycle % 40 == 20 {
            signal_strength += cycle as i32 * x;
            dbg!(cycle);
            dbg!(x);
            dbg!(cycle as i32 * x);
            dbg!(line_count);
        }
        match instruction.op {
            Op::Noop => {
                line_count += 1;
            }
            Op::Addx => x += instruction.value,
        };
    }
    signal_strength
}

pub fn ex2(lines: &Vec<String>) -> i32 {
    for line in lines {}
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day() {
        let lines = utils::read_input(10, false).expect("Should have been able to read the file");
        assert_eq!(ex1(&lines), 0);
        assert_eq!(ex2(&lines), 0);
    }
}
