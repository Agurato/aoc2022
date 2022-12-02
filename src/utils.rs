use std::fs;

pub fn read_input(day_number: u8, test: bool) -> Result<Vec<String>, std::io::Error> {
    let mut filepath = format!("input/day{day_number}");
    if test {
        filepath = format!("{filepath}_test");
    }
    filepath = format!("{filepath}.txt");
    let content = fs::read_to_string(filepath)?;
    Ok(content.lines().map(String::from).collect())
}
