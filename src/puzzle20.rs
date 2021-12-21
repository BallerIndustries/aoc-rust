use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Point {
    pub x: i32,
    pub y: i32
}

pub fn part_a(text: String) -> i32 {
    let tmp: Vec<&str> = text.split("\n\n").collect();
    let algo: Vec<char> = tmp[0].chars().collect();
    let mut image_grid: HashMap<Point, char> = parse_grid(tmp[1]);

    for _ in 0..25 {
        image_grid = enhance(&image_grid, &algo, '0');
        //debug(&image_grid);
        image_grid = enhance(&image_grid, &algo, '1');
        //debug(&image_grid);
    }



    return image_grid.values().filter(|it| **it == '#').count() as i32;
}

fn debug(grid: &HashMap<Point, char>) {
    let min_x = grid.keys().map(|it| it.x).min().unwrap();
    let max_x = grid.keys().map(|it| it.x).max().unwrap();
    let min_y = grid.keys().map(|it| it.y).min().unwrap();
    let max_y = grid.keys().map(|it| it.y).max().unwrap();

    for y in min_y..=max_y {
        for x in min_x..=max_x {
            let point = Point{x, y};

            if grid.contains_key(&point) {
                print!("{}", grid.get(&point).unwrap());
            }
            else {
                print!(".");
            }
        }
        print!("\n");
    }
}

fn enhance(grid: &HashMap<Point, char>, algo: &Vec<char>, default_char: char) -> HashMap<Point, char> {
    let mut output_grid: HashMap<Point, char> = HashMap::new();
    let min_x = grid.keys().map(|it| it.x).min().unwrap();
    let max_x = grid.keys().map(|it| it.x).max().unwrap();
    let min_y = grid.keys().map(|it| it.y).min().unwrap();
    let max_y = grid.keys().map(|it| it.y).max().unwrap();

    for y in min_y-1..=max_y+1 {
        for x in min_x-1..=max_x+1 {
            let point = Point { x, y };
            enhance_point(grid, algo, &mut output_grid, &point, default_char);
        }
    }

    return output_grid
}

fn enhance_point(
    grid: &HashMap<Point, char>,
    algo: &Vec<char>,
    output_grid: &mut HashMap<Point, char>,
    point: &Point,
    default_char: char
) {
    let mut chars: String = "".into();

    for y in point.y - 1..=point.y + 1 {
        for x in point.x - 1..=point.x + 1 {
            if let Some(value) = grid.get(&Point{x, y}) {
                if *value == '#' {
                    chars.push('1')
                }
                else {
                    chars.push('0')
                }
            }
            else {
                chars.push(default_char)
            }
        }
    }

    let index = usize::from_str_radix(&chars, 2).unwrap();
    let super_char = algo[index];
    output_grid.insert(*point, super_char);
}

fn parse_grid(text: &str) -> HashMap<Point, char> {
    let lines: Vec<&str> = text.lines().collect();
    let mut grid: HashMap<Point, char> = HashMap::new();

    for y in 0..lines.len() {
        let line: Vec<char> = lines[y].chars().collect();

        for x in 0..line.len() {
            let char = line[x];

            if char == '#' {
                grid.insert(Point {x: x as i32, y: y as i32}, char);
            }
        }
    }

    return grid
}

pub fn part_b(text: String) -> i32 {
    panic!("Not implemented")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::*;
    const FILENAME: &str = "./resources/2021/20.txt";

    #[test]
    fn puzzle_part_a() {
        // Not 8249
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
        assert_eq!(part_a("..#.#..#####.#.#.#.###.##.....###.##.#..###.####..#####..#....#..#..##..###..######.###...####..#..#####..##..#.#####...##.#.#..#.##..#.#......#.###.######.###.####...#.##.##..#..#..#####.....#.#....###..#.##......#.....#..#..#..##..#...##.######.####.####.#.#...#.......#..#.#.#...####.##.#......#..#...##.#.##..#...##.#.##..###.#......#.#.......#.#.#.####.###.##...#.....####.#..#..#.##.#....##..#.####....##...##..#...#......#.#.......#.......##..####..#...#.#.#...##..#.#..###..#####........#..####......#..#

#..#.
#....
##..#
..#..
..###".into()), 35);
    }

    #[test]
    fn example_part_b() {
        assert_eq!(part_b("".into()), 0);
    }
}