use std::fs;

pub fn part_a(text: String) -> i64 {
    let mut counter: i64 = 0;

    for (_, c) in text.chars().enumerate() {
        if c == '(' {
            counter += 1
        }
        else if c == ')' {
            counter -= 1
        }
    }

    return counter
}

pub fn read_all_text(filename: &str) -> String {
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");
    return contents
}


#[cfg(test)]
mod tests {
    use crate::puzzle1::*;

    #[test]
    fn puzzle_part_a() {
        let text = read_all_text("./resources/2015/1.txt");
        assert_eq!(part_a(text), 138)
    }

    #[test]
    fn example_part_a() {
        let text = String::from("(())");
        assert_eq!(part_a(text), 0)
    }
}