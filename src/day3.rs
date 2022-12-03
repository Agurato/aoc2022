use crate::utils;

pub fn day(test: bool) {
    println!("{}", ex1(test));
    println!("{}", ex2(test));
}

pub fn ex1(test: bool) -> i32 {
    let lines = utils::read_input(3, test).expect("Should have been able to read the file");
    let mut priority: i32 = 0;
    for line in lines {
        let (comp1, comp2) = line.split_at(line.len() / 2);
        let comp2_bytes = comp2.as_bytes();
        let item = comp1
            .as_bytes()
            .iter()
            .find(|x| comp2_bytes.contains(x))
            .unwrap()
            .clone();
        if 'a' as u8 <= item && item <= 'z' as u8 {
            priority += item as i32 - 96 // 'a' is 97
        } else {
            priority += item as i32 - 38 // 65-27
        }
    }
    priority
}

pub fn ex2(test: bool) -> i32 {
    let lines = utils::read_input(3, test).expect("Should have been able to read the file");
    let mut total: i32 = 0;
    for line_index in (0..lines.len()).step_by(3) {
        let line_2 = lines[line_index + 1].as_bytes();
        let line_3 = lines[line_index + 2].as_bytes();
        let item = lines[line_index]
            .as_bytes()
            .iter()
            .find(|x| line_2.contains(x) && line_3.contains(x))
            .unwrap()
            .clone();
        if 'a' as u8 <= item && item <= 'z' as u8 {
            total += item as i32 - 96 // 'a' is 97
        } else {
            total += item as i32 - 38 // 65-27
        }
    }
    total
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day() {
        assert_eq!(ex1(false), 7990);
        assert_eq!(ex2(false), 2602);
    }
}
