
pub fn part_a(text: String) -> i32 {
    let mut counter = 0;

    loop {
        let digest = md5::compute(format!("{}{}", text, counter));
        let digest_text = format!("{:x}", digest);

        // if starts with five zeros, that's the answer
        if has_leading_zeroes(digest_text, 5) {
            return counter
        }

        counter += 1
    }
}

fn has_leading_zeroes(text: String, count: usize) -> bool {
    let first_five: &str = &text[0..count];

    return first_five.chars().all(|c| {
        c == '0'
    })
}

pub fn part_b(text: String) -> i32 {
    let mut counter = 0;

    loop {
        let digest = md5::compute(format!("{}{}", text, counter));
        let digest_text = format!("{:x}", digest);

        // if starts with five zeros, that's the answer
        if has_leading_zeroes(digest_text, 6) {
            return counter
        }

        counter += 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::*;
    const FILENAME: &str = "./resources/2015/4.txt";

    #[test]
    fn puzzle_part_a() {
        let text = read_all_text(FILENAME);
        assert_eq!(part_a(text), 282749)
    }

    #[test]
    fn puzzle_part_b() {
        let text = read_all_text(FILENAME);
        assert_eq!(part_b(text), 9962624)
    }
}