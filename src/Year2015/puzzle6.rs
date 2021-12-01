use std::cmp::{max};

fn parse_coordinates(text: &str) -> (usize, usize) {
    let temp: Vec<&str> = text.split(",").collect();
    return (temp[0].parse().unwrap(), temp[1].parse().unwrap())
}

pub fn part_a(text: String) -> i32 {
    let mut lights = [[false; 1000]; 1000];

    for line in text.lines() {
        let temp: Vec<&str> = line.split(" ").collect();

        if temp[1] == "on" {
            let start = parse_coordinates(temp[2]);
            let end = parse_coordinates(temp[4]);

            for x in start.0..=end.0 {
                for y in start.1..=end.1 {
                    lights[x][y] = true
                }
            }
        }
        else if temp[1] == "off" {
            let start = parse_coordinates(temp[2]);
            let end = parse_coordinates(temp[4]);

            for x in start.0..=end.0 {
                for y in start.1..=end.1 {
                    lights[x][y] = false
                }
            }
        }
        else if temp[0] == "toggle" {
            let start = parse_coordinates(temp[1]);
            let end = parse_coordinates(temp[3]);

            for x in start.0..=end.0 {
                for y in start.1..=end.1 {
                    lights[x][y] = !lights[x][y]
                }
            }
        }
        else {
            panic!("What the hell!")
        }
    }

    return lights.iter().flatten().filter(|is_on| {
        **is_on == true
    }).count() as i32;
}

pub fn part_b(text: String) -> i32 {
    let mut lights = vec![vec![0i32; 1000]; 1000];

    for line in text.lines() {
        let temp: Vec<&str> = line.split(" ").collect();

        if temp[1] == "on" {
            let start = parse_coordinates(temp[2]);
            let end = parse_coordinates(temp[4]);

            for x in start.0..=end.0 {
                for y in start.1..=end.1 {
                    lights[x][y] += 1
                }
            }
        }
        else if temp[1] == "off" {
            let start = parse_coordinates(temp[2]);
            let end = parse_coordinates(temp[4]);

            for x in start.0..=end.0 {
                for y in start.1..=end.1 {
                    lights[x][y] = max(lights[x][y] - 1, 0)
                }
            }
        }
        else if temp[0] == "toggle" {
            let start = parse_coordinates(temp[1]);
            let end = parse_coordinates(temp[3]);

            for x in start.0..=end.0 {
                for y in start.1..=end.1 {
                    lights[x][y] += 2
                }
            }
        }
        else {
            panic!("What the hell!")
        }
    }

    return lights.iter().flatten().sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::*;
    const FILENAME: &str = "./resources/2015/6.txt";

    #[test]
    fn puzzle_part_a() {
        let text = read_all_text(FILENAME);
        assert_eq!(part_a(text), 569999)
    }

    #[test]
    fn puzzle_part_b() {
        let text = read_all_text(FILENAME);
        assert_eq!(part_b(text), 17836115)
    }

    #[test]
    fn example_part_b() {
        assert_eq!(part_b("turn on 0,0 through 0,0".into()), 1);
        assert_eq!(part_b("toggle 0,0 through 999,999".into()), 2000000);
    }
}