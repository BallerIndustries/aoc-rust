pub fn part_a(text: String) -> i32 {
    let mut positions: Vec<i32> = text.split(",").map(|x| x.parse::<i32>().unwrap()).collect();
    positions.sort();

    let min = positions[0];
    let max = positions[positions.len() - 1];

    let mut costs: Vec<i32> = (min..=max).map(|target| {
        let cost: i32 = positions.iter().map(|current| {
            (target - current).abs()
        }).sum();

        return cost;
    }).collect();

    costs.sort();
    return costs[0];
}

pub fn part_b(text: String) -> i32 {
    let mut positions: Vec<i32> = text.split(",").map(|x| x.parse::<i32>().unwrap()).collect();
    positions.sort();

    let x_min = positions[0];
    let x_max = positions[positions.len() - 1];

    let mut costs: Vec<i32> = (x_min..=x_max).map(|target| {
        positions.iter().map(|current| {
            return measure_fuel(target, *current);
        }).sum()
    }).collect();

    costs.sort();
    return costs[0];
}

fn measure_fuel(target: i32, current: i32) -> i32 {
    let delta = (target - current).abs();
    let start = 0;
    let end = delta+1;
    return (delta * (start + end)) / 2;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::*;
    const FILENAME: &str = "./resources/2021/7.txt";

    #[test]
    fn puzzle_part_a() {
        let text = read_all_text(FILENAME);
        assert_eq!(part_a(text), 328318)
    }

    #[test]
    fn puzzle_part_b() {
        let text = read_all_text(FILENAME);
        assert_eq!(part_b(text), 89791146)
    }

    #[test]
    fn example_part_b() {
        assert_eq!(part_b("16,1,2,0,4,2,7,1,2,14".into()), 168);
        assert_eq!(measure_fuel(2, 5), 6);
        assert_eq!(measure_fuel(16, 5), 66);
        assert_eq!(measure_fuel(2, 5), 6);
        assert_eq!(measure_fuel(7, 7), 0);
    }
}