
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

pub fn part_b(text: String) -> i64 {
    let mut floor_number: i64 = 0;
    let mut index = 1;

    for (_, c) in text.chars().enumerate() {
        if c == '(' {
            floor_number += 1
        }
        else if c == ')' {
            floor_number -= 1
        }

        if floor_number == -1 {
            return index
        }

        index += 1
    }

    panic!("Failed to reach floor -1")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::*;

    #[test]
    fn puzzle_part_a() {
        let text = read_all_text("./resources/2015/1.txt");
        assert_eq!(part_a(text), 138)
    }

    #[test]
    fn puzzle_part_b() {
        let text = read_all_text("./resources/2015/1.txt");
        assert_eq!(part_b(text), 1771)
    }

    #[test]
    fn example_part_a() {
        let text = "(())".into();
        assert_eq!(part_a(text), 0)
    }
}