pub fn part_a(text: String) -> i32 {
    return text.split("\n").filter(|line| {
        let has_three_vowels = line.chars().filter(|c| {
            *c == 'a' || *c == 'e' || *c == 'i' || *c == 'o' || *c == 'u'
        }).count() >= 3;

        let has_repeating_letter: bool = has_repeating_letter(line);

        let does_not_have_jurs = !line.contains("ab") &&
            !line.contains("cd") &&
            !line.contains("pq") &&
            !line.contains("xy");

        if has_repeating_letter && has_three_vowels && does_not_have_jurs {
            println!("success")
        }

        println!("has_three_vowels = {} has_repeating_letter = {} does_not_have_jurs = {}", has_three_vowels, has_repeating_letter, does_not_have_jurs);
        has_three_vowels && has_repeating_letter && does_not_have_jurs
    }).count() as i32;
}

fn has_repeating_letter(line: &&str) -> bool {
    let mut index: usize = 1;

    while index < line.len() {
        let prev = line.chars().nth(index-1).unwrap();
        let current = line.chars().nth(index).unwrap();

        if prev == current {
            return true
        }

        index += 1
    }

    return false
}

pub fn part_b(text: String) -> i32 {
    return text.split("\n").filter(|line| {
        let has_repeating_pair = has_repeating_pair(line);
        let has_surrounded_letter = has_surrounded_letter(line);

        println!("has_repeating_pair = {} has_surrounded_letter = {}", has_repeating_pair, has_surrounded_letter);
        has_repeating_pair && has_surrounded_letter
    }).count() as i32;
}

fn has_repeating_pair(line: &&str) -> bool {
    let mut index: usize = 1;

    while index < line.len() {
        let pair: &str = &line[index-1..index+1];

        if line.matches(pair).count() >= 2 {
            println!("pair = {} pair.len() = {}", pair, pair.len());
            return true
        }

        index += 1
    }

    return false
}

fn has_surrounded_letter(line: &&str) -> bool {
    let mut index: usize = 0;

    while index < line.len() - 2 {
        let current = line.chars().nth(index).unwrap();
        let next = line.chars().nth(index+2).unwrap();

        if current == next {
            return true
        }

        index += 1
    }

    return false
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::*;
    const FILENAME: &str = "./resources/2015/5.txt";

    #[test]
    fn puzzle_part_a() {
        let text = read_all_text(FILENAME);
        assert_eq!(part_a(text), 238)
    }

    #[test]
    fn puzzle_part_b() {
        let text = read_all_text(FILENAME);
        assert_eq!(part_b(text), 69)
    }

    #[test]
    fn example_part_a() {
        assert_eq!(part_a("ugknbfddgicrmopn".into()), 1);
        assert_eq!(part_a("aaa".into()), 1);
        assert_eq!(part_a("jchzalrnumimnmhp".into()), 0);
        assert_eq!(part_a("haegwjzuvuyypxyu".into()), 0);
        assert_eq!(part_a("dvszwmarrgswjxmb".into()), 0);
    }

    #[test]
    fn example_part_b() {
        assert_eq!(part_b("qjhvhtzxzqqjkmpb".into()), 1);
        assert_eq!(part_b("xxyxx".into()), 1);
        assert_eq!(part_b("uurcxstgmygtbstg".into()), 0);
        assert_eq!(part_b("ieodomkazucvgmuy".into()), 0);
    }
}

