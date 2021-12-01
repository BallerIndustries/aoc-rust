pub fn part_a(text: String) -> i32 {
    let mut values: Vec<i32> = Vec::new();

    for line in text.lines() {
        values.push(line.parse::<i32>().unwrap());
    }

    let mut count = 0;

    for index in 1..values.len() {
        let prev = values[index-1];
        let current = values[index];

        if (current > prev) {
            count += 1
        }
    }

    return count;
}

pub fn part_b(text: String) -> i32 {
    let mut values: Vec<i32> = Vec::new();

    for line in text.lines() {
        values.push(line.parse::<i32>().unwrap());
    }

    let mut jurValues: Vec<i32> = Vec::new();

    for index in 0..(values.len() - 2) {
        jurValues.push(values[index] + values[index+1] + values[index+2])
    }

    let mut count = 0;

    for index in 1..jurValues.len() {
        let prev = jurValues[index-1];
        let current = jurValues[index];

        if (current > prev) {
            count += 1
        }
    }

    return count;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::*;
    const FILENAME: &str = "./resources/2021/1.txt";

    #[test]
    fn puzzle_part_a() {
        let text = read_all_text(FILENAME);
        assert_eq!(part_a(text), 0)
    }

    #[test]
    fn puzzle_part_b() {
        let text = read_all_text(FILENAME);
        assert_eq!(part_b(text), 0)
    }

    #[test]
    fn example_part_a() {
        assert_eq!(part_a("".into()), 0);
    }
}