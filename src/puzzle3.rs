pub fn part_a(text: String) -> i32 {
    let lines: Vec<&str> = text.lines().collect();
    let length = lines.get(0).unwrap().len();
    let mut gamma_rate_str = "".to_owned();
    let mut epsilon_rate_str = "".to_owned();

    for index in 0..length {
        let mut zeroCount = 0;
        let mut oneCount = 0;

        for line in text.lines() {
            if line.chars().nth(index).unwrap() == '0' {
                zeroCount += 1
            }
            else if line.chars().nth(index).unwrap() == '1' {
                oneCount += 1
            }
        }

        if zeroCount > oneCount {
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
    let mut lines: Vec<&str> = text.lines().collect();
    let length = lines.get(0).unwrap().len();

    // let mut oxygenCandidates: Vec<&str> = text.lines().collect();
    // let mut carbonCandidates: Vec<&str> = text.lines().collect();
    let mut oxygenCandidates: Vec<&str> = Vec::new();
    let mut carbonCandidates: Vec<&str> = Vec::new();

    for line in lines {
        oxygenCandidates.push(line);
        carbonCandidates.push(line);
    }

    if oxygenCandidates.len() == 1 {
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
        if is_one(&oxygenCandidates) {
            break
        }

        let mut zero_count = count_zeroes(index, &oxygenCandidates);
        let mut one_count = (oxygenCandidates.len() as i32) - zero_count;

        if one_count >= zero_count {
            oxygenCandidates = filter_candidates(index, '1', oxygenCandidates);
        }
        else {
            oxygenCandidates = filter_candidates(index, '0', oxygenCandidates);
        }
    }

    for index in 0..length {
        if is_one(&carbonCandidates) {
            break
        }

        let mut zero_count = count_zeroes(index, &carbonCandidates);
        let mut one_count = (carbonCandidates.len() as i32) - zero_count;

        if zero_count <= one_count {
            carbonCandidates = filter_candidates(index, '0', carbonCandidates);
        }
        else {
            carbonCandidates = filter_candidates(index, '1', carbonCandidates);
        }
    }

    let gamma_rate = isize::from_str_radix(&carbonCandidates.get(0).unwrap(), 2).unwrap();
    let epsilon_rate = isize::from_str_radix(&oxygenCandidates.get(0).unwrap(), 2).unwrap();

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