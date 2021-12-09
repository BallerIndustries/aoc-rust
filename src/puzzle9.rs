use std::collections::HashSet;

pub fn part_a(text: String) -> u32 {
    let mut grid: Vec<Vec<u32>> = Vec::new();

    for line in text.lines() {
        let mut current: Vec<u32> = Vec::new();

        for char in line.chars() {
            let digit: u32 = char.to_digit(10).unwrap();
            current.push(digit);
        }

        grid.push(current);
    }

    let height = grid.len();
    let width = grid[0].len();
    let mut sum: u32 = 0;

    for y in 0..height {
        for x in 0..width {
            let mut left = 10u32;
            let mut right = 10u32;
            let mut up = 10u32;
            let mut down = 10u32;

            if x > 0 {
                left = grid[y][x-1]
            }
            if x < width - 1 {
                right = grid[y][x+1]
            }
            if y > 0 {
                up = grid[y-1][x]
            }
            if y < height - 1 {
                down = grid[y+1][x]
            }

            let me = grid[y][x];

            if me < left && me < right && me < up && me < down {
                sum += me + 1
            }
        }
    }

    return sum;
}

pub fn part_b(text: String) -> u32 {
    let mut grid: Vec<Vec<u32>> = Vec::new();

    for line in text.lines() {
        let mut current: Vec<u32> = Vec::new();

        for char in line.chars() {
            let digit: u32 = char.to_digit(10).unwrap();
            current.push(digit);
        }

        grid.push(current);
    }

    let height = grid.len();
    let width = grid[0].len();
    let mut low_points: Vec<Point> = Vec::new();

    for y in 0..height {
        for x in 0..width {
            let mut left = 10u32;
            let mut right = 10u32;
            let mut up = 10u32;
            let mut down = 10u32;

            if x > 0 {
                left = grid[y][x-1]
            }
            if x < width - 1 {
                right = grid[y][x+1]
            }
            if y > 0 {
                up = grid[y-1][x]
            }
            if y < height - 1 {
                down = grid[y+1][x]
            }

            let me = grid[y][x];

            if me < left && me < right && me < up && me < down {
                low_points.push(Point { x, y })
            }
        }
    }

    let mut basin_sizes: Vec<u32> = low_points.iter()
        .map(|p| measure_low_point(&grid, p))
        .collect();

    basin_sizes.sort();
    let l = basin_sizes.len();
    return basin_sizes[l-1] * basin_sizes[l-2] * basin_sizes[l-3];
}

fn try_add(grid: &Vec<Vec<u32>>, point: &Point, to_visit: &mut Vec<Point>, visited: &HashSet<Point>) {
    let value = grid[point.y][point.x];

    if value == 9 {
        return
    }
    if visited.contains(point) {
        return
    }

    to_visit.push(*point);
}

fn measure_low_point(grid: &Vec<Vec<u32>>, point: &Point) -> u32 {
    let height = grid.len();
    let width = grid[0].len();

    let mut to_visit: Vec<Point> = Vec::new();
    let mut visited: HashSet<Point> = HashSet::new();
    to_visit.push(*point);

    while to_visit.len() > 0 {
        let current = to_visit.remove(0);
        visited.insert(current);

        if current.x > 0 {
            let left =  Point { x: current.x - 1, y: current.y };
            try_add(grid, &left, &mut to_visit, &visited);
        }
        if current.x < width - 1 {
            let right =  Point { x: current.x + 1, y: current.y };
            try_add(grid, &right, &mut to_visit, &visited);
        }
        if current.y > 0 {
            let up =  Point { x: current.x, y: current.y - 1 };
            try_add(grid, &up, &mut to_visit, &visited);
        }
        if current.y < height - 1 {
            let down =  Point { x: current.x, y: current.y + 1 };
            try_add(grid, &down, &mut to_visit, &visited);
        }
    }

    return visited.len() as u32;
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
    const FILENAME: &str = "./resources/2021/9.txt";

    #[test]
    fn puzzle_part_a() {
        let text = read_all_text(FILENAME);
        assert_eq!(part_a(text), 518)
    }

    #[test]
    fn puzzle_part_b() {
        let text = read_all_text(FILENAME);
        assert_eq!(part_b(text), 949905)
    }
}