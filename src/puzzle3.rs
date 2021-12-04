pub fn part_a(text: String) -> i32 {
    let lines: Vec<&str> = text.lines().collect();
    let length = lines.get(0).unwrap().len();
    let mut gamma_rate_str = "".to_owned();
    let mut epsilon_rate_str = "".to_owned();

    for index in 0..length {
        let mut zero_count = 0;
        let mut one_count = 0;

        for line in text.lines() {
            if line.chars().nth(index).unwrap() == '0' {
                zero_count += 1
            }
            else if line.chars().nth(index).unwrap() == '1' {
                one_count += 1
            }
        }

        if zero_count > one_count {
            gamma_rate_str.push_str("0");
            epsilon_rate_str.push_str("1");
        }
        else {
            gamma_rate_str.push_str("1");
            epsilon_rate_str.push_str("0");
        }
    }

    let gamma_rate = isize::from_str_radix(&gamma_rate_str, 2).unwrap();
    let epsilon_rate = isize::from_str_radix(&epsilon_rate_str, 2).unwrap();

    return (gamma_rate * epsilon_rate) as i32;
}

pub fn part_b(text: String) -> i32 {
    let lines: Vec<&str> = text.lines().collect();
    let length = lines.get(0).unwrap().len();

    let mut oxygen_candidates: Vec<&str> = Vec::new();
    let mut carbon_candidates: Vec<&str> = Vec::new();

    for line in lines {
        oxygen_candidates.push(line);
        carbon_candidates.push(line);
    }

    if oxygen_candidates.len() == 1 {
        println!("oh  no")
    }

    fn is_one(candidates: &Vec<&str>) -> bool {
        return candidates.len() == 1
    }

    fn count_zeroes(index: usize, candidates: &Vec<&str>) -> i32 {
        let mut count = 0;

        for line in candidates {
            if line.chars().nth(index).unwrap() == '0' {
                count += 1
            }
        }

        return count
    }

    fn filter_candidates(
        index: usize,
        char_to_keep: char,
        candidates: Vec<&str>
    ) -> Vec<&str> {
        let mut copy: Vec<&str> = Vec::new();

        for line in candidates {
            if line.chars().nth(index).unwrap() == char_to_keep {
                copy.push(line)
            }
        }

        return copy
    }


    for index in 0..length {
        if is_one(&oxygen_candidates) {
            break
        }

        let zero_count = count_zeroes(index, &oxygen_candidates);
        let one_count = (oxygen_candidates.len() as i32) - zero_count;

        if one_count >= zero_count {
            oxygen_candidates = filter_candidates(index, '1', oxygen_candidates);
        }
        else {
            oxygen_candidates = filter_candidates(index, '0', oxygen_candidates);
        }
    }

    for index in 0..length {
        if is_one(&carbon_candidates) {
            break
        }

        let zero_count = count_zeroes(index, &carbon_candidates);
        let one_count = (carbon_candidates.len() as i32) - zero_count;

        if zero_count <= one_count {
            carbon_candidates = filter_candidates(index, '0', carbon_candidates);
        }
        else {
            carbon_candidates = filter_candidates(index, '1', carbon_candidates);
        }
    }

    let gamma_rate = isize::from_str_radix(&carbon_candidates.get(0).unwrap(), 2).unwrap();
    let epsilon_rate = isize::from_str_radix(&oxygen_candidates.get(0).unwrap(), 2).unwrap();

    return (gamma_rate * epsilon_rate) as i32;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::*;
    const FILENAME: &str = "./resources/2021/3.txt";

    #[test]
    fn puzzle_part_a() {
        let text = read_all_text(FILENAME);
        assert_eq!(part_a(text), 3309596)
    }

    #[test]
    fn puzzle_part_b() {
        let text = read_all_text(FILENAME);
        assert_eq!(part_b(text), 2981085)
    }
}