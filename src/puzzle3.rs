use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Point {
    pub x: i32,
    pub y: i32
}

pub fn part_a(text: String) -> i32 {
    let mut visited: HashMap<Point, i32> = HashMap::new();
    let mut position = Point {x: 0, y: 0};
    let mut index: usize = 0;
    visited.insert(position, 1);

    while index < text.len() {
        if let Some(new_position) = handle_char(&text, index, position) {
            position = new_position;
            visited.insert(position, 1);
        }

        index += 1
    }

    return visited.keys().count() as i32
}

pub fn part_b(text: String) -> i32 {
    let mut visited: HashMap<Point, i32> = HashMap::new();
    let mut position_a = Point {x: 0, y: 0};
    let mut position_b = Point {x: 0, y: 0};
    let mut index: usize = 0;
    visited.insert(position_a, 1);

    while index < text.len() {
        if let Some(new_position) = handle_char(&text, index, position_a) {
            position_a = new_position;
            visited.insert(position_a, 1);
        }

        index += 1;

        if let Some(new_position) = handle_char(&text, index, position_b) {
            position_b = new_position;
            visited.insert(position_b, 1);
        }

        index += 1;
    }

    return visited.keys().count() as i32;
}

fn handle_char(
    text: &String,
    index: usize,
    position: Point
) -> Option<Point> {
    if index >= text.len() {
        return None
    }

    let char: char = text.chars().nth(index).unwrap();

    let new_position = match char {
        '^' => Point { x: position.x, y: position.y - 1},
        'v' => Point { x: position.x, y: position.y + 1},
        '>' => Point { x: position.x + 1, y: position.y},
        '<' => Point { x: position.x - 1, y: position.y},
        _ => panic!("This isn't supposed to happen!")
    };

    return Option::Some(new_position)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::*;
    const FILENAME: &str = "./resources/2015/3.txt";

    #[test]
    fn puzzle_part_a() {
        let text = read_all_text(FILENAME);
        assert_eq!(part_a(text), 2592)
    }

    #[test]
    fn puzzle_part_b() {
        let text = read_all_text(FILENAME);
        assert_eq!(part_b(text), 2360)
    }
}