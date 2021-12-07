pub fn part_a(text: String) -> i64 {
    return simulate(text, 80);
}

pub fn part_b(text: String) -> i64 {
    return simulate(text, 256);
}

fn simulate(text: String, target_day_number: i32) -> i64 {
    let numbers: Vec<usize> = text.split(",")
        .map(|x| x.parse::<usize>().unwrap())
        .collect();
    let mut fish: Vec<i64> = vec![0i64; 9];

    for number in numbers {
        fish[number] += 1
    }

    let mut day_num = 1;

    while day_num <= target_day_number {
        fish.rotate_left(1);
        fish[6] += fish[8];
        day_num += 1
    }

    return fish.iter().sum();
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::*;
    const FILENAME: &str = "./resources/2021/6.txt";

    #[test]
    fn puzzle_part_a() {
        let text = read_all_text(FILENAME);
        assert_eq!(part_a(text), 391888)
    }

    #[test]
    fn puzzle_part_b() {
        let text = read_all_text(FILENAME);
        assert_eq!(part_b(text), 1754597645339)
    }

    #[test]
    fn example_part_a() {
        assert_eq!(simulate("3,4,3,1,2".into(), 2), 6);
        assert_eq!(simulate("3,4,3,1,2".into(), 3), 7);
        assert_eq!(simulate("3,4,3,1,2".into(), 18), 26);
    }

    #[test]
    fn example_part_b() {
        assert_eq!(part_b("3,4,3,1,2".into()), 26984457539);
    }
}