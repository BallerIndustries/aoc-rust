pub fn part_a(text: String) -> i32 {
    let lines: Vec<&str> = text.lines().collect();
    let length = lines[0].len();
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

    let gamma_rate = i32::from_str_radix(&gamma_rate_str, 2).unwrap();
    let epsilon_rate = i32::from_str_radix(&epsilon_rate_str, 2).unwrap();

    return gamma_rate * epsilon_rate;
}

pub fn part_b(text: String) -> i32 {
    let epsilon_rate = filter_down(text.lines().collect(), |zero_count: i32, one_count: i32| {
        if one_count >= zero_count { '1' } else { '0' }
    });

    let gamma_rate = filter_down(text.lines().collect(), |zero_count: i32, one_count: i32| {
        if zero_count <= one_count { '0' } else { '1' }
    });

    return gamma_rate * epsilon_rate
}

fn filter_down(mut candidates: Vec<&str>, pick_char: fn(i32, i32) -> char) -> i32 {
    let length = candidates.len();

    for index in 0..length {
        if candidates.len() == 1 {
            break
        }

        let zero_count = count_zeroes(index, &candidates);
        let one_count = (candidates.len() as i32) - zero_count;
        let char = pick_char(zero_count, one_count);
        candidates = filter_candidates(index, char, candidates);
    }

    return i32::from_str_radix(candidates[0], 2).unwrap();
}

fn count_zeroes(index: usize, candidates: &Vec<&str>) -> i32 {
    return candidates.iter().filter(|line|
        line.chars().nth(index).unwrap() == '0'
    ).count() as i32
}

fn filter_candidates(
    index: usize,
    char_to_keep: char,
    candidates: Vec<&str>
) -> Vec<&str> {
    return candidates.iter().filter(|line| {
        line.chars().nth(index).unwrap() == char_to_keep
    }).map(|x| *x).collect()
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