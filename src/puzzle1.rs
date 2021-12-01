use std::ops::Range;

pub fn part_a(text: String) -> i32 {
    let lines: Vec<&str> = text.lines().collect();

    return (1..lines.len()).filter(|index| {
        let prev = lines[*index-1].parse::<i32>().unwrap();
        let current = lines[*index].parse::<i32>().unwrap();
        current > prev
    }).count() as i32;
}

pub fn part_b(text: String) -> i32 {
    let values: Vec<i32> = text.lines().map(|x| {
        x.parse::<i32>().unwrap()
    }).collect();

    return (1..values.len()-2).filter(|index| {
        let prev: i32 = (*index-1..*index+2).map(|i| values[i]).sum();
        let current: i32 = (*index..*index+3).map(|i| values[i]).sum();
        current > prev
    }).count() as i32;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::*;
    const FILENAME: &str = "./resources/2021/1.txt";

    #[test]
    fn puzzle_part_a() {
        let text = read_all_text(FILENAME);
        assert_eq!(part_a(text), 1154)
    }

    #[test]
    fn puzzle_part_b() {
        let text = read_all_text(FILENAME);
        assert_eq!(part_b(text), 1127)
    }
}