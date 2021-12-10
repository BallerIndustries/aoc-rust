pub fn part_a(text: String) -> i64 {
    let mut high_score = 0i64;

    for line in text.lines() {
        let char_op = get_illegal_char(line);

        if char_op.is_none() {
            continue
        }

        let char = char_op.unwrap();
        //println!("line = {} has an unexpected = {}", line, char);

        if char == ']' {
            high_score += 57
        }
        else if char == '>' {
            high_score += 25137
        }
        else if char == ')' {
            high_score += 3
        }
        else if char == '}' {
            high_score += 1197
        }
        else {
            panic!("what the hell man!");
        }
    }

    return high_score;
}

fn get_illegal_char(line: &str) -> Option<char> {
    let mut stack: Vec<char> = Vec::new();

    for char in line.chars() {
        if char == '(' || char == '[' || char == '<' || char == '{' {
            stack.push(char)
        }
        else {
            let open_bracket = stack.pop().unwrap();

            if open_bracket == '(' && char == ')' ||
                open_bracket == '[' && char == ']' ||
                open_bracket == '<' && char == '>' ||
                open_bracket == '{' && char == '}' {
                continue
            }
            else {
                return Some(char)
            }
        }
    }

    return None;
}

pub fn part_b(text: String) -> i64 {
    let mut high_scores: Vec<i64> = Vec::new();
    let completeable: Vec<&str> = text.lines().filter(|l| get_illegal_char(*l).is_none()).collect();

    for line in completeable {
        let remainder: String = get_remainder(line);
        //println!("remainder for line {} is {}", line, remainder);

        let score = score_remainder(&remainder);
        high_scores.push(score);
    }

    high_scores.sort();
    return high_scores[high_scores.len()/2];
}

fn score_remainder(remainder: &str) -> i64 {
    let mut score = 0i64;

    for char in remainder.chars() {
        score *= 5;

        if char == ']' {
            score += 2;
        }
        else if char == ')' {
            score += 1;
        }
        else if char == '>' {
            score += 4;
        }
        else if char == '}' {
            score += 3;
        }
        else {
            panic!("what th ehlel man");
        }
    }

    return score;
}


fn get_remainder(line: &str) -> String {
    let mut stack: Vec<char> = Vec::new();

    for char in line.chars() {
        if char == '(' || char == '[' || char == '<' || char == '{' {
            stack.push(char)
        }
        else {
            let open_bracket = stack.pop().unwrap();

            if open_bracket == '(' && char == ')' ||
                open_bracket == '[' && char == ']' ||
                open_bracket == '<' && char == '>' ||
                open_bracket == '{' && char == '}' {
                continue
            }
            else {
                break;
            }
        }
    }

    return stack.iter().map(|c| {
        if *c == '(' {
            return ')'
        }
        if *c == '[' {
            return ']'
        }
        if *c == '<' {
            return '>'
        }
        if *c == '{' {
           return '}'
        }
        else {
            panic!("no!")
        }
    }).rev().collect();
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::*;
    const FILENAME: &str = "./resources/2021/10.txt";

    #[test]
    fn puzzle_part_a() {
        let text = read_all_text(FILENAME);
        assert_eq!(part_a(text), 442131)
    }

    #[test]
    fn puzzle_part_b() {
        let text = read_all_text(FILENAME);
        assert_eq!(part_b(text), 3646451424)
    }

    #[test]
    fn example_part_a() {
        assert_eq!(part_a("[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]
".into()), 26397);
    }

    #[test]
    fn example_part_b() {
        assert_eq!(part_b("[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]".into()), 288957);
    }
}