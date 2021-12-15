use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Point {
    pub x: usize,
    pub y: usize
}

fn get_paths_v2(grid: &Vec<Vec<u32>>, p: Point) -> Vec<Point> {
    let width = grid[0].len();
    let height = grid.len();
    let mut paths: Vec<Point> = vec![];

    if p.x > 0 {
        paths.push(Point { x: p.x - 1, y: p.y })
    }
    if p.x < width - 1 {
        paths.push(Point { x: p.x + 1, y: p.y })
    }
    if p.y > 0 {
        paths.push(Point { x: p.x, y: p.y - 1 })
    }
    if p.y < height - 1 {
        paths.push(Point { x: p.x, y: p.y + 1 })
    }

    return paths;
}

fn get_paths(grid: &Vec<Vec<u32>>, p: Point, visited: &HashSet<Point>) -> Vec<Point> {
    let width = grid[0].len();
    let height = grid.len();
    let mut paths: Vec<Point> = vec![];

    if p.x > 0 {
        paths.push(Point { x: p.x - 1, y: p.y })
    }
    if p.x < width - 1 {
        paths.push(Point { x: p.x + 1, y: p.y })
    }
    if p.y > 0 {
        paths.push(Point { x: p.x, y: p.y - 1 })
    }
    if p.y < height - 1 {
        paths.push(Point { x: p.x, y: p.y + 1 })
    }

    return paths.into_iter()
        .filter(|p| !visited.contains(p))
        .collect()
}

fn h(point: Point, grid: &Vec<Vec<u32>>) -> u32 {
    return grid[point.y][point.x];
}

fn calculate_risk_score(grid: Vec<Vec<u32>>, came_from: &HashMap<Point, Point>, current: Point) -> u32 {
    let mut total = 0u32;
    let mut _current = current;

    while came_from.contains_key(&_current) {
        let score = grid[_current.y][_current.x];
        println!("_current = {:?} score = {}", _current, score);
        total += score;
        _current = came_from[&_current];
    }

    return total;
}

pub fn part_a(text: String) -> u32 {
    let grid = text.lines().map(|line| {
        line.chars().map(|c| c.to_digit(10).unwrap() ).collect::<Vec<u32>>()
    }).collect::<Vec<Vec<u32>>>();

    let target = Point { x: grid[0].len() - 1, y: grid.len() - 1 };
    let start = Point { x: 0, y: 0 };
    let mut open_set = vec![start];
    let mut came_from: HashMap<Point, Point> = HashMap::new();
    let mut g_score: HashMap<Point, u32> = HashMap::new();
    g_score.insert(start, 0);

    let mut f_score: HashMap<Point, u32> = HashMap::new();
    f_score.insert(start, h(start, &grid));

    while !open_set.is_empty() {
        let current = open_set.remove(0);

        if current == target {
            return calculate_risk_score(grid, &came_from, current);
        }

        let neighbors = get_paths_v2(&grid, current);
        let BIG_VALUE: u32 = 4294967295;

        for neighbor in neighbors {
            let tentative_g_score = g_score[&current] + grid[neighbor.y][neighbor.x];

            if tentative_g_score < *g_score.get(&neighbor).unwrap_or(&BIG_VALUE) {
                came_from.insert(neighbor, current);
                f_score.insert(neighbor, tentative_g_score);
                g_score.insert(neighbor, tentative_g_score + h(neighbor, &grid));

                if !open_set.contains(&neighbor) {
                    open_set.push(neighbor)
                }
            }
        }
    }

    panic!("Open set is empty but goal was never reached");
}

pub fn part_b(text: String) -> i32 {
    panic!("Not implemented")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::*;
    const FILENAME: &str = "./resources/2021/15.txt";

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
        assert_eq!(part_a("1163751742
1381373672
2136511328
3694931569
7463417111
1319128137
1359912421
3125421639
1293138521
2311944581
".into()), 40);
    }

    #[test]
    fn example_part_b() {
        assert_eq!(part_b("".into()), 0);
    }
}