use std::collections::{HashMap, LinkedList};
use crate::puzzle14::address::Nil;

#[derive(Clone)]
enum address {
    address(Box<Node>),
    Nil,
}

#[derive(Clone)]
struct Node {
    value: char,
    next: address,
}

impl Node {
    fn append(&mut self, elem: char) {
        match self.next {
            address::address(ref mut next_address) => {
                next_address.append(elem);
            }
            address::Nil => {
                let node = Node {
                    value: elem,
                    next: address::Nil,
                };
                self.next = address::address(Box::new(node))
            }
        }
    }

    fn insert_after(&mut self, elem: char) {
        match self.next {
            address::address(ref mut next_address) => {

                let node = Node {
                    value: elem,
                    next: self.next.clone(),
                };
                self.next = address::address(Box::new(node))


            }
            address::Nil => {
                let node = Node {
                    value: elem,
                    next: address::Nil,
                };
                self.next = address::address(Box::new(node))
            }
        }
    }
}

pub fn part_a(text: String) -> i64 {
    let pieces: Vec<&str> = text.split("\n\n").collect();
    let mut rules: HashMap<(char, char), char> = HashMap::new();
    let mut char_to_count: HashMap<char, i64> = HashMap::new();


    for rule in pieces[1].lines() {
        let tmp: Vec<&str> = rule.split(" -> ").collect();
        let from_chars: Vec<char> = tmp[0].chars().collect();
        let to_chars: Vec<char> = tmp[1].chars().collect();
        rules.insert((from_chars[0], from_chars[1]), to_chars[0]);

        char_to_count.insert(from_chars[0], 0);
        char_to_count.insert(from_chars[1], 0);
        char_to_count.insert(to_chars[0], 0);
    }

    let mut counter = 0;
    let mut compound: String = pieces[0].into();

    while counter < 10 {
        compound = run_step(&rules, &compound);
        counter += 1;

        let delta = count_delta(&mut char_to_count, &mut compound);
        println!("delta = {}", delta);
    }

    return count_delta(&mut char_to_count, &mut compound);
}

fn count_delta(char_to_count: &mut HashMap<char, i64>, compound: &mut String) -> i64 {
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

fn run_step_v2(rules: &HashMap<(char, char), char>, head: &mut Node) -> String {
    let mut current = head;

    while matches!(current.next, Nil) == false {
        let a = current.value;

        let b = match current.next {
            address::address(ref newAddress) => {
                newAddress.value
            }
            address::Nil => {
                panic!("aaa")
            }
        };

        let middler = rules.get(&(a, b)).unwrap();
        current.insert_after(*middler);
        println!("ok")
    }

    panic!("aaa");
    return "".into()


    // let chars: Vec<char> = compound.chars().collect();
    // let mut buffer: String = "".into();
    // buffer.push(chars[0]);
    //
    // for index in 0..chars.len()-1 {
    //     let tuple = (chars[index], chars[index+1]);
    //     let in_between_char = rules.get(&tuple).unwrap();
    //
    //     buffer.push(*in_between_char);
    //     buffer.push(tuple.1);
    // }
    //
    // return buffer;
}

pub fn part_b(text: String) -> i64 {
    let pieces: Vec<&str> = text.split("\n\n").collect();
    let mut rules: HashMap<(char, char), char> = HashMap::new();
    let mut char_to_count: HashMap<char, i64> = HashMap::new();

    for rule in pieces[1].lines() {
        let tmp: Vec<&str> = rule.split(" -> ").collect();
        let from_chars: Vec<char> = tmp[0].chars().collect();
        let to_chars: Vec<char> = tmp[1].chars().collect();
        rules.insert((from_chars[0], from_chars[1]), to_chars[0]);

        char_to_count.insert(from_chars[0], 0);
        char_to_count.insert(from_chars[1], 0);
        char_to_count.insert(to_chars[0], 0);
    }

    let mut counter = 0;
    let mut compound: String = pieces[1].into();
    // let mut compound_head = create_list(compound);
    //
    // println!("aaa");
    //
    // while counter < 40 {
    //     compound = run_step_v2(&rules, &mut compound_head);
    //     counter += 1;
    //     // println!("counter = {}", counter);
    //     counter += 1
    // }


    panic!("aaa");
    // for char in compound.chars() {
    //     let counter = char_to_count.entry(char).or_insert(0);
    //     *counter += 1;
    // }
    //
    // let max = char_to_count.values().max().unwrap();
    // let min = char_to_count.values().min().unwrap();
    //
    // return max - min;
}

// fn create_list(text: String) -> &'static mut Node {
//     let chars: Vec<char> = text.chars().collect();
//     let mut head = Node { value: chars[0], next: address::Nil };
//
//     for index in 1..chars.len() {
//         head.append(chars[index])
//     }
//
//     return &mut head;
// }

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::*;
    const FILENAME: &str = "./resources/2021/14.txt";

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
        assert_eq!(part_a("NNCB

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
CN -> C".into()), 1588);
    }

    #[test]
    fn example_part_b() {
        assert_eq!(part_b("NNCB

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
CN -> C".into()), 2188189693529);
    }
}