use std::collections::{HashMap, HashSet};

pub fn part_a(text: String) -> i32 {
    let mut counter: i32 = 0;

    for line in text.lines() {
        let tmp: Vec<&str> = line.split(" | ").collect();
        let outputs = tmp[1].split_whitespace();

        // 1, 4, 7, 8
        // Digit 1 has 2 segments
        // Digit 4 has 4 segments
        // Digit 7 has 3 segments
        // Digit 8 has 7 segments
        let good_boys = outputs.filter(|x| x.len() == 2 || x.len() == 4 || x.len() == 3 || x.len() == 7)
            .count();
        counter += good_boys as i32;
    }

    return counter;
}

pub fn part_b(text: String) -> i64 {
    // AMBIGUOUS
    // Digit 0 has 6 segments
    // Digit 9 has 6 segments
    // Digit 6 has 6 segments

    // AMBIGUOUS
    // Digit 2 has 5 segments
    // Digit 3 has 5 segments
    // Digit 5 has 5 segments

    // UNIQUE
    // Digit 1 has 2 segments
    // Digit 4 has 4 segments
    // Digit 7 has 3 segments
    // Digit 8 has 7 segments

    // Thinking: If you get four of the same digits with 5/6 segments it is impossible to determine the digit
    // Thinking: If you get a digit with (2,4,3,7) segments, you can uniquely identify them
    let mut normal_digit_to_segments: HashMap<i32, Vec<char>> = HashMap::new();
    normal_digit_to_segments.insert(0, vec!['a', 'b', 'c', 'e', 'f', 'g']);
    normal_digit_to_segments.insert(1, vec!['c', 'f']);
    normal_digit_to_segments.insert(2, vec!['a', 'c', 'd', 'e', 'f', 'g']);
    normal_digit_to_segments.insert(3, vec!['a', 'c', 'd', 'f', 'g']);
    normal_digit_to_segments.insert(4, vec!['b', 'c', 'd', 'f']);
    normal_digit_to_segments.insert(5, vec!['a', 'b', 'd', 'f', 'g']);
    normal_digit_to_segments.insert(6, vec!['a', 'b', 'd', 'e', 'f', 'g']);
    normal_digit_to_segments.insert(7, vec!['a', 'c', 'f']);
    normal_digit_to_segments.insert(8, vec!['a', 'b', 'c', 'd', 'e', 'f', 'g']);
    normal_digit_to_segments.insert(9, vec!['a', 'b', 'c', 'd', 'f', 'g']);

    let mut total = 0i64;

    for line in text.lines() {
        let tmp: Vec<&str> = line.split(" | ").collect();
        let outputs: Vec<&str> = tmp[1].split_whitespace().collect();
        let digits: Vec<&str> = tmp[0].split_whitespace().collect();
        let mut known_digits: HashMap<i32, Vec<char>> = HashMap::new();

        for digit in digits {
            if digit.len() == 2 {
                known_digits.insert(1, digit.chars().collect());
            }
            else if digit.len() == 4 {
                known_digits.insert(4, digit.chars().collect());
            }
            else if digit.len() == 3 {
                known_digits.insert(7, digit.chars().collect());
            }
            else if digit.len() == 7 {
                known_digits.insert(8, digit.chars().collect());
            }
        }

        // Figure out digit 6
        let digit_6 = figure_out_digit_6(&tmp, &mut known_digits);
        known_digits.insert(6, digit_6);

        // Now we know 1, 4, 7, 8, 6
        let digit_0 = figure_out_digit_0(&tmp, &mut known_digits);
        known_digits.insert(0, digit_0);

        // Now we know 1, 4, 7, 8, 6, 0
        let digit_9 = figure_out_digit_9(&tmp, &mut known_digits);
        known_digits.insert(9, digit_9);

        // Now we know 1, 4, 7, 8, 6, 0, 9
        let digit_3 = figure_out_digit_3(&tmp, &mut known_digits);
        known_digits.insert(3, digit_3);

        // Now we know 1, 4, 7, 8, 6, 0, 9, 3
        let digit_2 = figure_out_digit_2(&tmp, &mut known_digits);
        known_digits.insert(2, digit_2);

        let digit_5 = figure_out_digit_5(&tmp, &mut known_digits);
        known_digits.insert(5, digit_5);

        verify_known_digits(&known_digits);




        let mut crazy_horse: HashMap<String, i32> = HashMap::new();

        for (key, characters) in known_digits {
            // let mut buffer = String::from("");
            //
            // for char in characters {
            //     buffer.push(char);
            // }

            let mut cheese: Vec<char> = characters.iter().map(|x| *x).collect();
            cheese.sort();
            let string = cheese.into_iter().collect::<String>();
            crazy_horse.insert(string, key);
        }


        let telephone: Vec<String> = outputs.iter().map(|output| {
            let mut output_chars: Vec<char> = output.chars().collect();
            output_chars.sort();
            let string = output_chars.into_iter().collect::<String>();
            return crazy_horse.get(&string).unwrap().to_string()
        }).collect();

        let final_result = telephone.into_iter().collect::<String>().parse::<i64>().unwrap();
        total += final_result;


        // // We can determine A by finding the char in '7' that is not in '1'
        // for key_a in 0..=9 {
        //     for key_b  in 0..=9 {
        //         if key_a == key_b {
        //             continue
        //         }
        //
        //         let _key_a_segments = known_digits.get(&key_a);
        //         let _key_b_segments = known_digits.get(&key_b);
        //
        //         if let (Some(key_a_segment), Some(key_b_segment)) = (_key_a_segments, _key_b_segments) {
        //             let difference = diff(key_a_segment, key_b_segment);
        //
        //             if difference.len() == 1 {
        //                 let normal_difference = diff(normal_digit_to_segments.get(&key_a).unwrap(), normal_digit_to_segments.get(&key_b).unwrap());
        //                 segment_mapping.insert(difference[0], normal_difference[0]);
        //                 println!("kaboom");
        //             }
        //         }
        //     }
        // }
    }

    return total;
    // println!("WIP");
    // panic!("Not implemented")
}

fn verify_known_digits(known_digits: &HashMap<i32, Vec<char>>) {
    if known_digits.len() != 10 {
        panic!("fuck you!");
    }

    let mut set: HashSet<Vec<char>> = HashSet::new();

    //for value in known_digits.values() {
    for (_, value) in known_digits.into_iter() {
        //println!("{} = {:?}", key, value);
        set.insert(value.clone());
    }

    if set.len() != 10 {
        panic!("Fuck you!");
    }
}

fn figure_out_digit_6(tmp: &Vec<&str>, known_digits: &mut HashMap<i32, Vec<char>>) -> Vec<char> {
    let six_segmenters: Vec<&str> = tmp[0].split_whitespace().filter(|x| x.len() == 6).map(|x| x).collect();
    let one_segments = known_digits.get(&1).unwrap();

    let octopus: &str = six_segmenters.iter().filter(|x| {
        let list_a = one_segments;
        let list_b = &x.chars().collect();
        let result = in_a_missing_from_b(list_a, list_b);
        return result.len() == 1 && not_known(x.chars().collect(), known_digits);
    }).collect::<Vec<&&str>>()[0];

    return octopus.chars().collect()
}

fn figure_out_digit_0(tmp: &Vec<&str>, known_digits: &mut HashMap<i32, Vec<char>>) -> Vec<char> {
    let six_segmenters: Vec<&str> = tmp[0].split_whitespace().filter(|x| x.len() == 6).map(|x| x).collect();
    let four_segments = known_digits.get(&4).unwrap();

    let octopus: &str = six_segmenters.iter().filter(|x| {
        let list_b = four_segments;
        let list_a = &x.chars().collect();
        let result = in_a_missing_from_b(list_a, list_b);
        return result.len() == 3 && not_known(x.chars().collect(), known_digits);
    }).collect::<Vec<&&str>>()[0];

    return octopus.chars().collect()
}

fn figure_out_digit_9(tmp: &Vec<&str>, known_digits: &mut HashMap<i32, Vec<char>>) -> Vec<char> {
    let six_segmenters: Vec<&str> = tmp[0].split_whitespace().filter(|x| x.len() == 6).map(|x| x).collect();
    let four_segments = known_digits.get(&4).unwrap();

    let octopus: &str = six_segmenters.iter().filter(|x| {
        let list_a = &x.chars().collect();
        let list_b = four_segments;
        let result = in_a_missing_from_b(list_a, list_b);
        return result.len() == 2 && not_known(x.chars().collect(), known_digits);
    }).collect::<Vec<&&str>>()[0];

    return octopus.chars().collect()
}

fn figure_out_digit_3(tmp: &Vec<&str>, known_digits: &mut HashMap<i32, Vec<char>>) -> Vec<char> {
    let five_segmenters: Vec<&str> = tmp[0].split_whitespace().filter(|x| x.len() == 5).map(|x| x).collect();
    let one_segments = known_digits.get(&1).unwrap();

    let octopus: &str = five_segmenters.iter().filter(|x| {
        let list_a = one_segments;
        let list_b = &x.chars().collect();
        let result = in_a_missing_from_b(list_a, list_b);
        return result.len() == 0 && not_known(x.chars().collect(), known_digits);
    }).collect::<Vec<&&str>>()[0];

    return octopus.chars().collect()
}

fn figure_out_digit_2(tmp: &Vec<&str>, known_digits: &mut HashMap<i32, Vec<char>>) -> Vec<char> {
    let five_segmenters: Vec<&str> = tmp[0].split_whitespace().filter(|x| x.len() == 5).map(|x| x).collect();
    let four_segments = known_digits.get(&4).unwrap();

    let octopus: &str = five_segmenters.iter().filter(|x| {
        let list_a = four_segments;
        let list_b = &x.chars().collect();
        let result = in_a_missing_from_b(list_a, list_b);
        return result.len() == 2 && not_known(x.chars().collect(), known_digits);
    }).collect::<Vec<&&str>>()[0];

    return octopus.chars().collect()
}

fn figure_out_digit_5(tmp: &Vec<&str>, known_digits: &mut HashMap<i32, Vec<char>>) -> Vec<char> {
    let five_segmenters: Vec<&str> = tmp[0].split_whitespace().filter(|x| x.len() == 5).map(|x| x).collect();
    let four_segments = known_digits.get(&4).unwrap();

    let octopus: &str = five_segmenters.iter().filter(|x| {
        let list_a = four_segments;
        let list_b = &x.chars().collect();
        let result = in_a_missing_from_b(list_a, list_b);
        return result.len() == 1 && not_known(x.chars().collect(), known_digits);
    }).collect::<Vec<&&str>>()[0];

    return octopus.chars().collect()
}

fn not_known(candidate: Vec<char>, known_digits: &HashMap<i32, Vec<char>>) -> bool {
    for value in known_digits.values().into_iter() {
        if candidate == *value {
            //println!("{:?}", value);
            return false
        }
    }

    return true


    // let vec: Vec<Vec<char>> = known_digits.values().clone().map(|x| *x).collect();
    // return vec.contains(&candidate);
    //return *vec.contains(candidate);
}

fn in_a_missing_from_b(list_a: &Vec<char>, list_b: &Vec<char>) -> Vec<char> {
    let mut difference: Vec<char> = Vec::new();

    for item in list_a {
        if !list_b.contains(&item) && !difference.contains(&item) {
            difference.push(*item)
        }
    }

    return difference
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::*;
    const FILENAME: &str = "./resources/2021/8.txt";

    #[test]
    fn puzzle_part_a() {
        let text = read_all_text(FILENAME);
        assert_eq!(part_a(text), 294)
    }

    #[test]
    fn puzzle_part_b() {
        let text = read_all_text(FILENAME);
        assert_eq!(part_b(text), 973292)
    }

    #[test]
    fn example_part_b_1() {
        assert_eq!(part_b("acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf".into()), 5353);
    }

    #[test]
    fn example_part_b_2() {
        assert_eq!(part_b("be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce".into()), 61229);
    }
}