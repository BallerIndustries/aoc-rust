use std::collections::{HashMap, HashSet};

pub fn part_a(text: String) -> i32 {
    let mut paths: HashMap<&str, Vec<&str>> = HashMap::new();

    for line in text.lines() {
        let tmp: Vec<&str> = line.split("-").collect();
        let mut forward: &mut Vec<&str> = paths.entry(tmp[0]).or_insert(Vec::new());
        forward.push(tmp[1]);

        let mut backward: &mut Vec<&str> = paths.entry(tmp[1]).or_insert(Vec::new());
        backward.push(tmp[0]);
    }

    let mut current = "start";
    let mut visited: Vec<&str> = Vec::new();
    visited.push(current);

    // Create a state struct with current and visited
    let mut states: Vec<(&str, Vec<&str>)> = vec![(current, visited)];
    let mut path_count = 0;

    while states.len() > 0 {
        let (current, visited) = states.pop().unwrap();

        if current == "end" {
            path_count += 1;
            continue
        }

        //println!("current = {}", current);

        // TODO: Make this idiomatic
        let _path_options = paths.get(current);

        if _path_options.is_none() {
            continue
        }

        let path_options = _path_options.unwrap();

        for path in path_options {
            if visited.contains(path) {
                //println!("Already visited {} not going to visit it again", path);
                continue
            }

            // Should be a viable path, pop it on the stack
            let mut visited_clone = visited.clone();

            if is_lower_case(path) {
                visited_clone.push(path);
            }

            states.push((path, visited_clone))
        }
    }

    return path_count;
}

fn is_lower_case(text: &str) -> bool {
    for char in text.chars() {
        if char.is_uppercase() {
            return false
        }
    }

    return true
}

pub fn part_b(text: String) -> i32 {
    panic!("Not implemented")
}

// #[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
// struct State<'a> {
//     pub current: &'a str,
//     pub visited: HashSet<&'a str>
// }

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::*;
    const FILENAME: &str = "./resources/2021/12.txt";

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
        assert_eq!(part_a("start-A
start-b
A-c
A-b
b-d
A-end
b-end".into()), 10);
    }

    #[test]
    fn example_part_b() {
        assert_eq!(part_b("".into()), 0);
    }
}