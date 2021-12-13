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

pub fn part_a(text: String) -> i32 {
    let temp = text.split("\n\n").collect::<Vec<&str>>();
    let point_text: &str = temp[0];
    let mut points: Vec<Point> = point_text.lines().map(|line| {
        let jur: Vec<i32> = line.split(",").map(|it| it.parse::<i32>().unwrap()).collect();
        return Point { x: jur[0], y: jur[1] }
    }).collect();

    let command_text = temp[1];
    let commands: Vec<&str> = command_text.lines().collect();

    let octopus: &str = commands[0];
    let pieces: Vec<&str> = octopus.split("=").collect();
    let axis = pieces[0].split_whitespace().collect::<Vec<&str>>()[2];
    let value = pieces[1].parse::<i32>().unwrap();

    if axis == "x" {
        for point in points.iter_mut() {
            point.fold_x(value)
        }
    }
    else if axis == "y" {
        for point in points.iter_mut() {
            point.fold_y(value);
        }
    }
    else {
        panic!("oh no");
    }

    let mut horses: HashSet<Point> = HashSet::new();

    for point in points {
        horses.insert(point);
    }

    return horses.len() as i32;
}

pub fn part_b(text: String) -> i32 {
    let temp = text.split("\n\n").collect::<Vec<&str>>();
    let point_text: &str = temp[0];
    let mut points: Vec<Point> = point_text.lines().map(|line| {
        let jur: Vec<i32> = line.split(",").map(|it| it.parse::<i32>().unwrap()).collect();
        return Point { x: jur[0], y: jur[1] }
    }).collect();

    let command_text = temp[1];
    let commands: Vec<&str> = command_text.lines().collect();

    for octopus in commands {
        //let octopus: &str = commands[0];
        let pieces: Vec<&str> = octopus.split("=").collect();
        let axis = pieces[0].split_whitespace().collect::<Vec<&str>>()[2];
        let value = pieces[1].parse::<i32>().unwrap();

        if axis == "x" {
            for point in points.iter_mut() {
                point.fold_x(value)
            }
        }
        else if axis == "y" {
            for point in points.iter_mut() {
                point.fold_y(value);
            }
        }
        else {
            panic!("oh no");
        }
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::*;
    const FILENAME: &str = "./resources/2021/13.txt";

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
        assert_eq!(part_a("6,10
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
fold along x=5".into()), 17);
    }

    #[test]
    fn example_part_b() {
        assert_eq!(part_b("6,10
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
fold along x=5".into()), 16);
    }
}