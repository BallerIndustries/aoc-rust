use std::collections::HashSet;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Point {
    pub x: i32,
    pub y: i32
}

impl Point {
    fn fold_x(&mut self, value: i32) {
        if self.x > value {
            let distance = self.x - value;
            self.x = value - distance;
        }
    }

    fn fold_y(&mut self, value: i32) {
        if self.y > value {
            let distance = self.y - value;
            self.y = value - distance;
        }
    }
}

fn parse_points(temp: &Vec<&str>) -> Vec<Point> {
    temp[0].lines().map(|line| {
        let jur: Vec<i32> = line.split(",").map(|it| it.parse::<i32>().unwrap()).collect();
        return Point { x: jur[0], y: jur[1] }
    }).collect()
}

pub fn part_a(text: String) -> i32 {
    let temp = text.split("\n\n").collect::<Vec<&str>>();
    let mut points = parse_points(&temp);

    let commands: Vec<&str> = temp[1].lines().collect();
    process_command(&mut points, commands[0]);

    let mut distinct_points: HashSet<Point> = HashSet::new();

    for point in points {
        distinct_points.insert(point);
    }

    return distinct_points.len() as i32;
}

pub fn part_b(text: String) -> i32 {
    let temp = text.split("\n\n").collect::<Vec<&str>>();
    let mut points = parse_points(&temp);
    let commands: Vec<&str> = temp[1].lines().collect();

    for command in commands {
        process_command(&mut points, command)
    }

    let max_x = points.clone().iter().map(|p| p.x).max().unwrap();
    let max_y = points.clone().iter().map(|p| p.y).max().unwrap();

    let mut distinct: HashSet<Point> = HashSet::new();

    for point in points {
        distinct.insert(point);
    }

    for y in 0..=max_y {
        for x in 0..=max_x {
            let current = Point { x, y };

            if distinct.contains(&current) {
                print!("#");
            }
            else {
                print!(".");
            }
        }

        print!("\n");
    }

    return distinct.len() as i32;
}

fn process_command(points: &mut Vec<Point>, command: &str) {
    let pieces: Vec<&str> = command.split("=").collect();
    let axis = pieces[0].split_whitespace().collect::<Vec<&str>>()[2];
    let value = pieces[1].parse::<i32>().unwrap();

    if axis == "x" {
        for point in points.iter_mut() {
            point.fold_x(value)
        }
    } else if axis == "y" {
        for point in points.iter_mut() {
            point.fold_y(value);
        }
    } else {
        panic!("oh no");
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::*;
    const FILENAME: &str = "./resources/2021/13.txt";
    const EXAMPLE_A: &str = "6,10
0,14
9,10
0,3
10,4
4,11
6,0
6,12
4,1
0,13
10,12
3,4
3,0
8,4
1,10
2,14
8,10
9,0

fold along y=7
fold along x=5";

    #[test]
    fn puzzle_part_a() {
        let text = read_all_text(FILENAME);
        assert_eq!(part_a(text), 610)
    }

    #[test]
    fn puzzle_part_b() {
        let text = read_all_text(FILENAME);
        assert_eq!(part_b(text), 95)
    }

    #[test]
    fn example_part_a() {
        assert_eq!(part_a(EXAMPLE_A.into()), 17);
    }

    #[test]
    fn example_part_b() {
        assert_eq!(part_b(EXAMPLE_A.into()), 16);
    }
}