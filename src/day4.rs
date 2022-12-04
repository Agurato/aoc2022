use crate::utils;

pub fn day(test: bool) {
    let lines = utils::read_input(4, test).expect("Should have been able to read the file");
    println!("{}", ex1(&lines));
    println!("{}", ex2(&lines));
}

pub fn ex1(lines: &Vec<String>) -> i32 {
    let mut count = 0;
    for line in lines {
        let elves: Vec<&str> = line.split(",").collect();
        let elf1: Vec<&str> = elves[0].split("-").collect();
        let elf2: Vec<&str> = elves[1].split("-").collect();
        let elf1_start = elf1[0].parse::<i32>().unwrap();
        let elf1_end = elf1[1].parse::<i32>().unwrap();
        let elf2_start = elf2[0].parse::<i32>().unwrap();
        let elf2_end = elf2[1].parse::<i32>().unwrap();

        if (elf1_start <= elf2_start && elf1_end >= elf2_end)
            || (elf2_start <= elf1_start && elf2_end >= elf1_end)
        {
            count += 1;
        }
    }
    count
}

pub fn ex2(lines: &Vec<String>) -> i32 {
    let mut count = 0;
    for line in lines {
        let elves: Vec<&str> = line.split(",").collect();
        let elf1: Vec<&str> = elves[0].split("-").collect();
        let elf2: Vec<&str> = elves[1].split("-").collect();
        let elf1_start = elf1[0].parse::<i32>().unwrap();
        let elf1_end = elf1[1].parse::<i32>().unwrap();
        let elf2_start = elf2[0].parse::<i32>().unwrap();
        let elf2_end = elf2[1].parse::<i32>().unwrap();

        if (elf1_start <= elf2_start && elf1_end >= elf2_start)
            || (elf2_start <= elf1_start && elf2_end >= elf1_start)
        {
            count += 1;
        }
    }
    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day() {
        let lines = utils::read_input(4, false).expect("Should have been able to read the file");
        assert_eq!(ex1(&lines), 464);
        assert_eq!(ex2(&lines), 770);
    }
}
