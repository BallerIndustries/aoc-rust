use std::cmp::{max, min};
use std::collections::HashMap;

fn increment_point(points: &mut HashMap<Point, i32>, point: Point) {
    let counter = points.entry(point).or_insert(0);
    *counter += 1;
}

pub fn part_a(text: String) -> i32 {
    let lines: Vec<Vec<i32>> = text.lines().map(|line| {
        let tmp: Vec<i32> = line.split(" -> ").flat_map(|horse| {
           horse.split(",").map(|x| x.parse::<i32>().unwrap())
        }).collect();

        tmp
    }).collect();

    let mut visited: HashMap<Point, i32> = HashMap::new();

    for line in lines {
        let x1 = line[0];
        let y1 = line[1];
        let x2 = line[2];
        let y2 = line[3];

        if x1 == x2 {
            let min_y = min(y1, y2);
            let max_y = max(y1, y2);

            // Vertical
            for y in min_y..=max_y {
                let point = Point { x: x1, y: y };
                increment_point(&mut visited, point);
            }
        }
        else if y1 == y2 {
            let min_x = min(x1, x2);
            let max_x = max(x1, x2);

            // Horizontal
            for x in min_x..=max_x {
                let point = Point { x: x, y: y1 };
                increment_point(&mut visited, point);
            }
        }
    }

    return visited.iter().filter(|entry| *entry.1 >= 2).count() as i32;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Point {
    pub x: i32,
    pub y: i32
}

pub fn part_b(text: String) -> i32 {
    let lines: Vec<Vec<i32>> = text.lines().map(|line| {
        let tmp: Vec<i32> = line.split(" -> ").flat_map(|horse| {
            horse.split(",").map(|x| x.parse::<i32>().unwrap())
        }).collect();

        tmp
    }).collect();

    let mut visited: HashMap<Point, i32> = HashMap::new();

    for line in lines {
        let x1 = line[0];
        let y1 = line[1];
        let x2 = line[2];
        let y2 = line[3];

        if x1 == x2 {
            let min_y = min(y1, y2);
            let max_y = max(y1, y2);

            // Vertical
            for y in min_y..=max_y {
                let point = Point { x: x1, y: y };
                increment_point(&mut visited, point);
            }
        }
        else if y1 == y2 {
            let min_x = min(x1, x2);
            let max_x = max(x1, x2);

            // Horizontal
            for x in min_x..=max_x {
                let point = Point { x: x, y: y1 };
                increment_point(&mut visited, point);
            }
        }
        else {
            // Diagonal
            let min_x = min(x1, x2);
            let max_x = max(x1, x2);
            let min_y = min(y1, y2);
            let max_y = max(y1, y2);

            let mut x = x1;
            let mut y = y1;

            loop {
                if x > max_x || x < min_x || y > max_y || y < min_y {
                    break
                }

                let point = Point { x: x, y: y };
                increment_point(&mut visited, point);

                if x1 < x2 {
                    x += 1
                }
                else {
                    x -= 1
                }

                if y1 < y2 {
                    y += 1
                }
                else {
                    y -= 1
                }
            }
        }
    }

    return visited.iter().filter(|entry| *entry.1 >= 2).count() as i32;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::*;
    const FILENAME: &str = "./resources/2021/5.txt";

    #[test]
    fn puzzle_part_a() {
        let text = read_all_text(FILENAME);
        assert_eq!(part_a(text), 6564)
    }

    #[test]
    fn puzzle_part_b() {
        let text = read_all_text(FILENAME);
        assert_eq!(part_b(text), 19172)
    }

    #[test]
    fn example_part_a() {
        assert_eq!(part_a("0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2
".into()), 5);
    }

    #[test]
    fn example_part_b() {
        assert_eq!(part_b("0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2
".into()), 12);
    }
}