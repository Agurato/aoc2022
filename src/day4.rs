use crate::utils;

pub fn day(test: bool) {
    let lines = utils::read_input(4, test).expect("Should have been able to read the file");
    println!("{}", ex1(&lines));
    println!("{}", ex2(&lines));
}

pub fn ex1(lines: &Vec<String>) -> i32 {
    for line in lines {}
    0
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
        let lines = utils::read_input(4, false).expect("Should have been able to read the file");
        assert_eq!(ex1(&lines), 0);
        assert_eq!(ex2(&lines), 0);
    }
}
