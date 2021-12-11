use std::collections::HashSet;

pub fn part_a(text: String) -> i32 {
    let mut energy_levels: Vec<Vec<u32>> = Vec::new();

    for line in text.lines() {
        let mut row: Vec<u32> = Vec::new();

        for char in line.chars() {
            row.push(char.to_digit(10).unwrap());
        }

        energy_levels.push(row);
    }

    let mut flash_count = 0;

    for _step in 0..100 {
        let result: (i32, Vec<Vec<u32>>) = run_step(energy_levels);
        flash_count += result.0;
        energy_levels = result.1;
    }

    return flash_count;
}

fn get_neighbours(x: usize, y: usize, width: usize, height: usize) -> Vec<Point> {
    let mut neighbors: Vec<Point> = Vec::new();

    for x_delta in -1isize ..= 1isize {
        for y_delta in -1isize ..= 1isize {
           let can_x = (x as isize) + x_delta;
           let can_y = (y as isize) + y_delta;

            if can_x >= 0 && can_x < width as isize && can_y >= 0 && can_y < height as isize {
                neighbors.push(Point {x: can_x as usize, y: can_y as usize })
            }
        }
    }

    return neighbors;
}

pub fn run_step(energy_levels: Vec<Vec<u32>>) -> (i32, Vec<Vec<u32>>) {
    let mut cloned = energy_levels.clone();
    let height = cloned.len();
    let width = cloned[0].len();
    let mut pending_flashers: Vec<Point> = Vec::new();
    let mut already_flashed: HashSet<Point> = HashSet::new();

    for y in 0..height {
        for x in 0..width {
            cloned[y][x] += 1;

            if cloned[y][x] > 9 {
                pending_flashers.push(Point {x: x, y: y});
            }
        }
    }

    while pending_flashers.len() > 0 {
        let point = pending_flashers.pop().unwrap();
        already_flashed.insert(point);
        let neighbors = get_neighbours(point.x, point.y, width, height);

        for neighbor in neighbors {
            cloned[neighbor.y][neighbor.x] += 1;

            if cloned[neighbor.y][neighbor.x] > 9 && !pending_flashers.contains(&neighbor) && !already_flashed.contains(&neighbor) {
                pending_flashers.push(neighbor);
            }
        }
    }

    let mut flash_count = 0;

    for p in already_flashed {
        cloned[p.y][p.x] = 0;
        flash_count += 1;
    }

    return (flash_count, cloned);
}

pub fn part_b(text: String) -> i32 {
    let mut energy_levels: Vec<Vec<u32>> = Vec::new();

    for line in text.lines() {
        let mut row: Vec<u32> = Vec::new();

        for char in line.chars() {
            row.push(char.to_digit(10).unwrap());
        }

        energy_levels.push(row);
    }

    let mut step_num = 0;
    let width = energy_levels[0].len();
    let height = energy_levels.len();
    let octopus_count: i32 = (width * height) as i32;

    loop {
        step_num += 1;
        let result: (i32, Vec<Vec<u32>>) = run_step(energy_levels);
        energy_levels = result.1;

        if result.0 == octopus_count {
            break
        }
    }

    return step_num;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Point {
    pub x: usize,
    pub y: usize
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::*;
    const FILENAME: &str = "./resources/2021/11.txt";

    #[test]
    fn puzzle_part_a() {
        let text = read_all_text(FILENAME);
        assert_eq!(part_a(text), 1747)
    }

    #[test]
    fn puzzle_part_b() {
        let text = read_all_text(FILENAME);
        assert_eq!(part_b(text), 505)
    }

    #[test]
    fn example_part_a() {
        assert_eq!(part_a("5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526".into()), 1656);
    }

    #[test]
    fn example_part_b() {
        assert_eq!(part_b("5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526".into()), 195);
    }
}