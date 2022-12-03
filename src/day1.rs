use crate::utils;

pub fn day(test: bool) {
    ex1(test);
    ex2(test);
}

pub fn ex1(test: bool) {
    let lines = utils::read_input(1, test).expect("Should have been able to read the file");

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
    println!("{max_calory}")
}

pub fn ex2(test: bool) {
    let lines = utils::read_input(1, test).expect("Should have been able to read the file");

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

    let top: u32 = calories[..3].into_iter().sum();

    println!("{top}")
}
