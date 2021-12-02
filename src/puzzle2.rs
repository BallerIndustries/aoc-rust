pub fn part_a(text: String) -> i32 {
    let mut x = 0;
    let mut y = 0;

    for line in text.lines() {
        let tmp: Vec<&str> = line.split_whitespace().collect();
        let value: i32 = tmp[1].parse::<i32>().unwrap();

        if tmp[0] == "forward" {
            x += value;
        }
        else if tmp[0] == "up" {
            y -= value;
        }
        else if tmp[0] == "down" {
            y += value
        }

        // println!("{},{}", x, y)
    }

    return x * y;
}

pub fn part_b(text: String) -> i32 {
    let mut x = 0;
    let mut y = 0;
    let mut aim = 0;

    for line in text.lines() {
        let tmp: Vec<&str> = line.split_whitespace().collect();
        let value: i32 = tmp[1].parse::<i32>().unwrap();

        if tmp[0] == "forward" {
            x += value;
            y += aim * value;
        }
        else if tmp[0] == "up" {
            aim -= value
        }
        else if tmp[0] == "down" {
            aim += value
        }

        // println!("{}", line);
        // println!("{},{} aim = {}\n", x, y, aim);
    }

    return x * y;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::*;
    const FILENAME: &str = "./resources/2021/2.txt";

    #[test]
    fn puzzle_part_a() {
        let text = read_all_text(FILENAME);
        assert_eq!(part_a(text), 2322630)
    }

    #[test]
    fn puzzle_part_b() {
        let text = read_all_text(FILENAME);
        assert_eq!(part_b(text), 2105273490)
    }

    #[test]
    fn example_part_a() {
        let x = "forward 5
            down 5
            forward 8
            up 3
            down 8
            forward 2";
        assert_eq!(part_a(x.into()), 150);
    }

    #[test]
    fn example_part_b() {
        let x = "forward 5
            down 5
            forward 8
            up 3
            down 8
            forward 2";
        assert_eq!(part_b(x.into()), 900);
    }
}