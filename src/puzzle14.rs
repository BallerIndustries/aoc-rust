use std::collections::{HashMap};
use std::os::raw::c_double;

pub fn simulate(text: String, iterations: i32) -> i64 {
    let pieces: Vec<&str> = text.split("\n\n").collect();
    let mut rules: HashMap<(char, char), char> = HashMap::new();
    let mut compound: String = pieces[0].into();

    for rule in pieces[1].lines() {
        let tmp: Vec<&str> = rule.split(" -> ").collect();
        let from_chars: Vec<char> = tmp[0].chars().collect();
        let to_chars: Vec<char> = tmp[1].chars().collect();
        rules.insert((from_chars[0], from_chars[1]), to_chars[0]);
    }

    let mut pair_frequency: HashMap<(char, char), i64> = HashMap::new();
    let chars: Vec<char> = compound.chars().collect();

    // Determine the frequency of pairs
    for i in 0..compound.len() - 1 {
        let a = chars[i];
        let b = chars[i+1];
        let counter = pair_frequency.entry((a, b)).or_insert(0);
        *counter += 1;
    }

    print_pair_frequency(&pair_frequency);

    // Increment the compounds
    for _ in 0..iterations {
        let mut new_pair_frequency: HashMap<(char, char), i64> = HashMap::new();

        for (pair, count) in pair_frequency {
            let char_to_insert = *rules.get(&pair).unwrap();
            let left_pair = (pair.0, char_to_insert);
            let left_counter = new_pair_frequency.entry(left_pair).or_insert(0);
            *left_counter += count;

            let right_pair = (char_to_insert, pair.1);
            let right_counter = new_pair_frequency.entry(right_pair).or_insert(0);
            *right_counter += count;
        }

        pair_frequency = new_pair_frequency;
        print_pair_frequency(&pair_frequency);
    }

    return count_delta_v2(&pair_frequency);
}


pub fn part_a(text: String) -> i64 {
    return simulate(text, 10);
}

pub fn part_b(text: String) -> i64 {
    return simulate(text, 40);
}

fn print_pair_frequency(freq: &HashMap<(char, char), i64>) {
    for ((a, b), count) in freq {
        print!("{}{} = {} ", *a, *b, *count)
    }

    print!("\n");
}

fn count_delta_v2(pair_frequency: &HashMap<(char, char), i64>) -> i64 {
    let mut char_to_count: HashMap<char, i64> = HashMap::new();

    for ((a, b), count) in pair_frequency {
        let a_counter = char_to_count.entry(*a).or_insert(0);
        *a_counter += count;

        let b_counter = char_to_count.entry(*b).or_insert(0);
        *b_counter += count;
    }

    let max = *char_to_count.values().max().unwrap();
    let min = *char_to_count.values().min().unwrap();
    let delta = max as f64 - min as f64;
    return (delta / 2f64).ceil() as i64;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::*;
    const FILENAME: &str = "./resources/2021/14.txt";
    const EXAMPLE: &str = "NNCB

CH -> B
HH -> N
CB -> H
NH -> C
HB -> C
HC -> B
HN -> C
NN -> C
BH -> H
NC -> B
NB -> B
BN -> B
BB -> N
BC -> B
CC -> N
CN -> C";

    #[test]
    fn puzzle_part_a() {
        let text = read_all_text(FILENAME);
        assert_eq!(part_a(text), 3095)
    }

    #[test]
    fn puzzle_part_b() {
        let text = read_all_text(FILENAME);
        assert_eq!(part_b(text), 0)
    }

    #[test]
    fn example_part_a() {
        assert_eq!(part_a(EXAMPLE.into()), 1588);
    }

    #[test]
    fn example_part_b() {
        assert_eq!(part_b(EXAMPLE.into()), 2188189693529);
    }
}