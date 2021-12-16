use std::collections::{HashMap};

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

fn h(point: Point, target: Point) -> u32 {
    return ((target.x - point.x) + (target.y - point.y)) as u32;
}

fn calculate_risk_score(grid: &Vec<Vec<u32>>, came_from: &HashMap<Point, Point>, current: Point) -> u32 {
    let mut total = 0u32;
    let mut _current = current;

    while came_from.contains_key(&_current) {
        let score = grid[_current.y][_current.x];
        //println!("_current = {:?} score = {}", _current, score);
        total += score;
        _current = came_from[&_current];
    }

    return total;
}

pub fn part_a(text: String) -> u32 {
    let grid = text.lines().map(|line| {
        line.chars().map(|c| c.to_digit(10).unwrap() ).collect::<Vec<u32>>()
    }).collect::<Vec<Vec<u32>>>();

    return a_star(&grid);
}

fn a_star(grid: &Vec<Vec<u32>>) -> u32 {
    let target = Point { x: grid[0].len() - 1, y: grid.len() - 1 };
    let start = Point { x: 0, y: 0 };
    let mut open_set = vec![start];
    let mut came_from: HashMap<Point, Point> = HashMap::new();
    let mut g_score: HashMap<Point, u32> = HashMap::new();
    g_score.insert(start, 0);

    let mut f_score: HashMap<Point, u32> = HashMap::new();
    f_score.insert(start, 0);

    while !open_set.is_empty() {
        let current = open_set.remove(0);

        if current == target {
            return calculate_risk_score(grid, &came_from, current);
        }

        let neighbors = get_paths_v2(&grid, current);
        let big_value: u32 = 4294967295;

        for neighbor in neighbors {
            let tentative_g_score = g_score[&current] + grid[neighbor.y][neighbor.x];

            if tentative_g_score < *g_score.get(&neighbor).unwrap_or(&big_value) {
                came_from.insert(neighbor, current);
                g_score.insert(neighbor, tentative_g_score);
                f_score.insert(neighbor, tentative_g_score + h(neighbor, target));

                if !open_set.contains(&neighbor) {
                    open_set.push(neighbor)
                }
            }
        }

        open_set.sort_by(|p1, p2| {
            let x: u32 = f_score[&p1];
            let x1: u32 = f_score[&p2];
            return x.cmp(&x1)
        })
    }

    panic!("Open set is empty but goal was never reached");
}

pub fn part_b(text: String) -> u32 {
    let lines: Vec<&str> = text.lines().collect();
    // let mut grid = text.lines().map(|line| {
    //     line.chars().map(|c| c.to_digit(10).unwrap() ).collect::<Vec<u32>>()
    // }).collect::<Vec<Vec<u32>>>();

    let width = lines[0].len();
    let height = lines.len();

    let new_height = height * 5;
    let new_width = width * 5;

    let mut grid = vec![vec![0u32; new_width]; new_height];

    for y in 0..height {
        for x in 0..width {
            let line: Vec<char> = lines[y].chars().collect();
            grid[y][x] = line[x].to_digit(10).unwrap();
        }
    }

    for y in 0..new_height {
        for x in 0..new_width {
            let value = grid[y][x];

            if value == 0 {
                //let increment = (y / height) + (x / width);
                let y_increment = y / height;
                let x_increment = x / width;
                let total_increment = (y_increment + x_increment) as u32;

                let original_risk_level = grid[y - y_increment * height][x - x_increment * width];
                grid[y][x] = original_risk_level + total_increment;

                if grid[y][x] > 9 {
                    grid[y][x] = grid[y][x] % 9;
                }
            }
        }
    }


    return a_star(&grid);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::*;
    const FILENAME: &str = "./resources/2021/15.txt";

    #[test]
    fn puzzle_part_a() {
        let text = read_all_text(FILENAME);
        assert_eq!(part_a(text), 398)
    }

    #[test]
    fn puzzle_part_b() {
        let text = read_all_text(FILENAME);
        assert_eq!(part_b(text), 2817)
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
    fn example_part_a_2() {
        assert_eq!(part_a("1911191111
1119111991
9999999111
9999911199
9999119999
9999199999
9111199999
9199999111
9111911191
9991119991".into()), 40);
    }

    #[test]
    fn example_part_b() {
        assert_eq!(part_b("1163751742
1381373672
2136511328
3694931569
7463417111
1319128137
1359912421
3125421639
1293138521
2311944581
".into()), 315);
    }
}