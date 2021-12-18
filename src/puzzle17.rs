
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Point {
    pub x: i32,
    pub y: i32
}

impl Point {
    fn set(&mut self, x: i32, y: i32) {
        self.x = x;
        self.y = y;
    }
}

pub fn part_a(_text: String) -> i32 {
    let ranges = _text.split(": ").collect::<Vec<&str>>()[1].split(", ").collect::<Vec<&str>>();

    let x_range: Vec<i32> = ranges[0].replace("x=", "").split("..").map(|x| x.parse::<i32>().unwrap()).collect();
    let y_range: Vec<i32> = ranges[1].replace("y=", "").split("..").map(|x| x.parse::<i32>().unwrap()).collect();



    // let mut current = Point { x: 0, y: 0 };
    //
    // while current.y < y_end {
    //
    // }

    let mut y_max = 0;

    for x_delta in -200..200 {
        for y_delta in -200..200 {
            let (hit_target, max_height) = will_hit_target(x_delta, y_delta, &x_range, &y_range);

            if hit_target && max_height > y_max {
                y_max = max_height;
            }
        }
    }

    return y_max;
}

fn will_hit_target(
    _x_delta: i32,
    _y_delta: i32,
    x_range: &Vec<i32>,
    y_range: &Vec<i32>
) -> (bool, i32) {
    let x_start = x_range[0];
    let x_end = x_range[1];
    let y_start = y_range[0];
    let y_end = y_range[1];
    let mut y_max = -1;

    let mut x_delta = _x_delta;
    let mut y_delta = _y_delta;
    let mut position = Point { x: 0, y: 0 };

    //while y_delta < 0 && position.y < y_end {
    loop {
        let new_x = position.x + x_delta;
        let new_y = position.y + y_delta;
        position.set(new_x, new_y);

        if new_y > y_max {
            y_max = new_y;
        }

        if new_x > x_end || new_y < y_start {
            break;
        }

        // Check if we are in the trench
        if new_x >= x_start && new_x <= x_end && new_y >= y_start && new_y <= y_end {
            println!("Hit the target using initial velocity of ({},{}) y_max = {}", _x_delta, _y_delta, y_max);
            return (true, y_max);
        }

        if x_delta > 0 {
            x_delta -= 1
        }
        else if x_delta < 0{
            x_delta += 1
        }

        y_delta -= 1;
    }

    return (false, y_max);
}

pub fn part_b(_text: String) -> i32 {
    panic!("Not implemented")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::*;
    const FILENAME: &str = "./resources/2021/17.txt";

    #[test]
    fn puzzle_part_a() {
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
        assert_eq!(part_a("target area: x=20..30, y=-10..-5".into()), 45);
    }

    #[test]
    fn example_part_b() {
        assert_eq!(part_b("".into()), 0);
    }
}