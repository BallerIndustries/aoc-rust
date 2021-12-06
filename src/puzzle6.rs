pub fn part_a(text: String) -> i64 {
    return simulate(text, 80);
}

pub fn part_b(text: String) -> i64 {
    return simulate(text, 256);
}

fn simulate(text: String, target_day_number: i32) -> i64 {
    let numbers: Vec<i64> = text.split(",").map(|x| x.parse::<i64>().unwrap()).collect();
    let mut timer_to_count: Vec<i64> = (0..=8).map(|_| 0).collect();

    for number in numbers {
        timer_to_count[number as usize] += 1
    }

    let mut day_num = 1;

    while day_num <= target_day_number {
        let mut counter: i32 = 8;
        let mut copy: Vec<i64> = (0..=8).map(|_| 0).collect();

        // shift timer_to_count
        while counter >= 0 {
            if counter > 0 {
                copy[(counter - 1) as usize] = timer_to_count[counter as usize]
            } else {
                copy[6] = copy[6] + timer_to_count[0];
                copy[8] = copy[8] + timer_to_count[0];
            }

            counter -= 1;
        }

        timer_to_count = copy;
        day_num += 1
    }

    return timer_to_count.iter().sum();
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