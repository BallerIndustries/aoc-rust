use std::collections::{HashMap};

pub fn part_a(text: String) -> i64 {
    let pieces: Vec<&str> = text.split("\n\n").collect();
    let mut rules: HashMap<(char, char), char> = HashMap::new();

    for rule in pieces[1].lines() {
        let tmp: Vec<&str> = rule.split(" -> ").collect();
        let from_chars: Vec<char> = tmp[0].chars().collect();
        let to_chars: Vec<char> = tmp[1].chars().collect();
        rules.insert((from_chars[0], from_chars[1]), to_chars[0]);
    }

    let mut counter = 0;
    let mut compound: String = pieces[0].into();

    while counter < 10 {
        compound = run_step(&rules, &compound);
        counter += 1;
    }

    return count_delta(&compound);
}

fn count_delta(compound: &str) -> i64 {
    let mut char_to_count: HashMap<char, i64> = HashMap::new();

    for char in compound.chars() {
        let counter = char_to_count.entry(char).or_insert(0);
        *counter += 1;
    }

    let max = char_to_count.values().max().unwrap();
    let min = char_to_count.values().min().unwrap();
    return max - min;
}

fn run_step(rules: &HashMap<(char, char), char>, compound: &str) -> String {
    let chars: Vec<char> = compound.chars().collect();
    let mut buffer: String = "".into();
    buffer.push(chars[0]);

    for index in 0..chars.len()-1 {
        let tuple = (chars[index], chars[index+1]);
        let in_between_char = rules.get(&tuple).unwrap();

        buffer.push(*in_between_char);
        buffer.push(tuple.1);
    }

    return buffer;
}

pub fn part_b(text: String) -> i64 {
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
    for _ in 0..10 {
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
        //println!("pair_frequency = {:?}", pair_frequency);
    }



    println!("hi");

    // let mut counter = 0;
    // println!("{}\n", compound);
    //
    // while counter < 10 {
    //     compound = run_step(&rules, &compound);
    //     println!("{}\n", compound);
    //     counter += 1;
    // }

    return count_delta_v2(&pair_frequency);
}

fn print_pair_frequency(freq: &HashMap<(char, char), i64>) {
    for entry in freq {
        print!("{}{} = {} ", entry.0.1, entry.0.1, entry.1)
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
    return max - min;
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