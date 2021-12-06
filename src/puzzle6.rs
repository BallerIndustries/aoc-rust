use std::collections::HashMap;

pub fn part_a(text: String) -> i32 {
    return simulate(text, 80);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct DayAndTimer {
    pub day_number: i32,
    pub timer: i32
}

fn simulate(text: String, final_day: i32) -> i32 {
    // This memo stores how many fish will be produced by a fish with a timer starting at 1.
    // Day 0 - [1]    -> 1
    // Day 1 - [0, 8] -> 2
    // Day 2 - [6, 7] -> 2
    // Day 3 - [5, 6] -> 2
    // Day 4 - [4, 5] -> 2
    // Day 5 - [3, 4] -> 2
    // Day 6 - [2, 3] -> 2
    // Day 7 - [1, 2] -> 2
    // Day 8 - [0, 1, 8] -> 3
    // Day 9 - [6, 0, 7, 8] -> 4
    // Day 10 - [6, 0, 7, 8] -> 4
    let mut memo: HashMap<DayAndTimer, i32> = HashMap::new();

    for timer in 1..=5 {
        add_to_memo(&mut memo, timer, final_day)
    }

    let timers: Vec<i32> = text.split(",").map(|x| x.parse::<i32>().unwrap()).collect();

    return timers.iter().map(|timer| {
        memo.get(&DayAndTimer { timer: *timer, day_number: final_day }).unwrap()
    }).sum();

    fn add_to_memo(memo: &mut HashMap<DayAndTimer, i32>, initial_timer: i32, final_day: i32) {
        memo.insert(DayAndTimer{ day_number: 0, timer: initial_timer}, 1);
        let mut day_num = 0;
        let mut fish_timers = vec![initial_timer];

        while day_num <= final_day {
            day_num += 1;
            let mut six_count = 0;

            // Decrement Counters
            for timer in fish_timers.iter_mut() {
                *timer -= 1;

                if *timer == -1 {
                    *timer = 6;
                    six_count += 1;
                }
            }

            // Spawn new fish
            for _ in 0..six_count {
                fish_timers.push(8);
            }

            let total_fish = fish_timers.iter().count() as i32;
            memo.insert(DayAndTimer{ day_number: day_num, timer: initial_timer}, total_fish);
        }
    }
}

pub fn part_b(text: String) -> i64 {
    let numbers: Vec<i64> = text.split(",").map(|x| x.parse::<i64>().unwrap()).collect();
    let mut timer_to_count: Vec<i64> = (0..=8).map(|x| 0).collect();

    for number in numbers {
        timer_to_count[number as usize] += 1
    }

    let mut day_num = 1;

    while day_num <= 256 {
        let mut counter: i32 = 8;
        let mut copy: Vec<i64> = (0..=8).map(|x| 0).collect();

        // shift timer_to_count
        while counter >= 0 {
            if counter > 0 {
                copy[(counter-1) as usize] = timer_to_count[counter as usize]
            }
            else {
                copy[6] = copy[6] + timer_to_count[0];
                copy[8] = copy[8] + timer_to_count[0];
            }

            counter -= 1;
        }

        timer_to_count = copy;
        println!("Day {} {:?}", day_num, timer_to_count);
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
        assert_eq!(part_a(text), 0)
    }

    #[test]
    fn puzzle_part_b() {
        let text = read_all_text(FILENAME);
        assert_eq!(part_b(text), 0)
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