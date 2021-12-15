use std::collections::HashSet;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Point {
    pub x: usize,
    pub y: usize
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

pub fn part_a(text: String) -> u32 {
    let grid = text.lines().map(|line| {
        line.chars().map(|c| c.to_digit(10).unwrap() ).collect::<Vec<u32>>()
    }).collect::<Vec<Vec<u32>>>();

    let target = Point { x: grid[0].len() - 1, y: grid.len() - 1 };
    let mut states: Vec<(Point, HashSet<Point>)> = vec![(Point { x: 0, y: 0 }, HashSet::new())];
    let mut min_risk_total = 4294967295;

    while !states.is_empty() {
        let result = states.pop().unwrap();
        let current = result.0;
        let mut visited = result.1;
        visited.insert(current);

        let m_distance: u32 = ((target.x - current.x) + (target.y - current.y)) as u32;
        let risk_total: u32 = visited.iter().map(|p| grid[p.y][p.x]).sum::<u32>() - grid[0][0];

        if risk_total + m_distance > min_risk_total {
            continue
        }

        if current == target {
            if risk_total < min_risk_total {
                min_risk_total = risk_total
            }

            continue
        }

        // We haven't hit the target
        let mut paths: Vec<Point> = get_paths(&grid, current, &visited);
        // paths.sort_by(|a, b| {
        //     let a_value = grid[a.y][a.x];
        //     let b_value = grid[b.y][b.x];
        //
        //     return a_value.cmp(&b_value)
        // });

        for path in paths {
            states.push((path, visited.clone()))
        }
    }

    // Go for a DFS with pruning?
    return min_risk_total
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