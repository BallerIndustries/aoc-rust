pub fn part_a(text: String) -> String {
    panic!("Not implemented")
}

pub fn part_b(text: String) -> String {
    panic!("Not implemented")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::*;
    const FILENAME: &str = "./resources/2015/REPLACE_THIS_WITH_THE_RIGHT_NUMBER.txt";

    #[test]
    fn puzzle_part_a() {
        let text = read_all_text(FILENAME);
        assert_eq!(part_a(text), "horse")
    }

    #[test]
    fn puzzle_part_b() {
        let text = read_all_text(FILENAME);
        assert_eq!(part_b(text), "horse")
    }

    #[test]
    fn example_part_a() {
        assert_eq!(part_a("1x1x10".into()), "dog");
        assert_eq!(part_a("2x3x4".into()), "dog");
    }
}