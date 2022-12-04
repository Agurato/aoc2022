use crate::utils;

pub fn day(test: bool) {
    let lines = utils::read_input(1, test).expect("Should have been able to read the file");
    println!("{}", ex1(&lines));
    println!("{}", ex2(&lines));
}

pub fn ex1(lines: &Vec<String>) -> u32 {
    let mut max_calory: u32 = 0;
    let mut current_calory: u32 = 0;
    for line in lines {
        if line.len() == 0 {
            if max_calory < current_calory {
                max_calory = current_calory
            }
            current_calory = 0;
            continue;
        }
        let calory: u32 = line.parse().expect("line should contain a number");
        current_calory += calory
    }
    if max_calory < current_calory {
        max_calory = current_calory
    }
    max_calory
}

pub fn ex2(lines: &Vec<String>) -> u32 {
    let mut calories: Vec<u32> = Vec::new();
    let mut calory_count: u32 = 0;
    for line in lines {
        if line.len() == 0 {
            calories.push(calory_count);
            calory_count = 0;
            continue;
        }
        let calory: u32 = line.parse().expect("line should contain a number");
        calory_count += calory
    }
    calories.push(calory_count);

    calories.sort_by(|a, b| b.cmp(a));
    // println!("{calories:#?}");

    calories[..3].into_iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day() {
        let lines = utils::read_input(1, false).expect("Should have been able to read the file");
        assert_eq!(ex1(&lines), 69501);
        assert_eq!(ex2(&lines), 202346);
    }
}
