// use std::cmp::{max, min};
// use std::collections::HashMap;
//
// fn parse_ranges(text: &str) -> Vec<(i32, i32)> {
//     let axis: Vec<(i32, i32)> = text.split(",").map(|a| {
//         let tmp = a.split("=").collect::<Vec<&str>>();
//         let range: Vec<i32> = tmp[1].split("..").map(|it| it.parse::<i32>().unwrap()).collect();
//         (range[0], range[1])
//     }).collect();
//
//     return axis;
// }
//
// #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
// struct Point {
//     pub x: i32,
//     pub y: i32,
//     pub z: i32
// }
//
// struct Cube {
//     pub enabled: bool,
//     pub x1: i32,
//     pub x2: i32,
//     pub y1: i32,
//     pub y2: i32,
//     pub z1: i32,
//     pub z2: i32,
// }
//
// impl Cube {
//     fn contains(self, other: Cube) -> bool {
//         return other.x1 >= self.x1 && other.x2 <= self.x2
//             && other.y1 >= self.y1 && other.y2 <= self.y2
//             && other.z1 >= self.z1 && other.z2 <= self.z2
//     }
//
//     // fn intersects(self, other: Cube) -> Cube {
//     //
//     // }
// }
//
// pub fn part_a(text: String) -> i32 {
//     let mut grid: HashMap<Point, bool> = HashMap::new();
//
//     for line in text.lines() {
//         let tmp = line.split_whitespace().collect::<Vec<&str>>();
//         let ranges = parse_ranges(tmp[1]);
//         let enabled = tmp[0] == "on";
//
//         if !within_window(&ranges) {
//             continue
//         }
//
//         let normalized: Vec<(i32, i32)> = normalise(&ranges);
//
//         let x_range = normalized[0];
//         let y_range = normalized[1];
//         let z_range = normalized[2];
//
//         for x in x_range.0..=x_range.1 {
//             for y in y_range.0..=y_range.1 {
//                 for z in z_range.0..=z_range.1 {
//                     grid.insert(Point{ x, y, z}, enabled);
//                 }
//             }
//         }
//     }
//
//     return grid.values().filter(|i| **i == true).count() as i32;
// }
//
// fn normalise(ranges: &Vec<(i32, i32)>) -> Vec<(i32, i32)> {
//     ranges.iter().map(|it| {
//         return (max(-50, it.0), min(50, it.1));
//     }).collect()
// }
//
// fn within_window(ranges: &Vec<(i32, i32)>) -> bool {
//     for (from, to) in ranges {
//         if !(*from >= -50 && *from <= 50) {
//             return false
//         }
//
//         if !(*to >= -50 && *to <= 50) {
//             return false
//         }
//     }
//
//     return true;
// }
//
// pub fn part_b(text: String) -> u64 {
//     let cubes: Vec<Cube> = text.lines().map(|line| {
//         let tmp = line.split_whitespace().collect::<Vec<&str>>();
//         let ranges = parse_ranges(tmp[1]);
//         let enabled = tmp[0] == "on";
//         Cube {
//             enabled,
//             x1: ranges[0].0,
//             x2: ranges[0].1,
//             y1: ranges[1].0,
//             y2: ranges[1].1,
//             z1: ranges[2].0,
//             z2: ranges[2].1,
//         }
//     }).collect();
//
//     println!("");
//
//
//
//
//     panic!("Not implemented")
// }
//
// #[cfg(test)]
// mod tests {
//     use super::*;
//     use crate::util::*;
//     const FILENAME: &str = "./resources/2021/22.txt";
//
//     #[test]
//     fn puzzle_part_a() {
//         let text = read_all_text(FILENAME);
//         assert_eq!(part_a(text), 658691)
//     }
//
//     #[test]
//     fn puzzle_part_b() {
//         let text = read_all_text(FILENAME);
//         assert_eq!(part_b(text), 0)
//     }
//
//     #[test]
//     fn example_part_a() {
//         assert_eq!(part_a("on x=-20..26,y=-36..17,z=-47..7
// on x=-20..33,y=-21..23,z=-26..28
// on x=-22..28,y=-29..23,z=-38..16
// on x=-46..7,y=-6..46,z=-50..-1
// on x=-49..1,y=-3..46,z=-24..28
// on x=2..47,y=-22..22,z=-23..27
// on x=-27..23,y=-28..26,z=-21..29
// on x=-39..5,y=-6..47,z=-3..44
// on x=-30..21,y=-8..43,z=-13..34
// on x=-22..26,y=-27..20,z=-29..19
// off x=-48..-32,y=26..41,z=-47..-37
// on x=-12..35,y=6..50,z=-50..-2
// off x=-48..-32,y=-32..-16,z=-15..-5
// on x=-18..26,y=-33..15,z=-7..46
// off x=-40..-22,y=-38..-28,z=23..41
// on x=-16..35,y=-41..10,z=-47..6
// off x=-32..-23,y=11..30,z=-14..3
// on x=-49..-5,y=-3..45,z=-29..18
// off x=18..30,y=-20..-8,z=-3..13
// on x=-41..9,y=-7..43,z=-33..15
// on x=-54112..-39298,y=-85059..-49293,z=-27449..7877
// on x=967..23432,y=45373..81175,z=27513..53682".into()), 590784);
//     }
//
//     #[test]
//     fn example_part_b() {
//         assert_eq!(part_b("on x=-20..26,y=-36..17,z=-47..7
// on x=-20..33,y=-21..23,z=-26..28
// on x=-22..28,y=-29..23,z=-38..16
// on x=-46..7,y=-6..46,z=-50..-1
// on x=-49..1,y=-3..46,z=-24..28
// on x=2..47,y=-22..22,z=-23..27
// on x=-27..23,y=-28..26,z=-21..29
// on x=-39..5,y=-6..47,z=-3..44
// on x=-30..21,y=-8..43,z=-13..34
// on x=-22..26,y=-27..20,z=-29..19
// off x=-48..-32,y=26..41,z=-47..-37
// on x=-12..35,y=6..50,z=-50..-2
// off x=-48..-32,y=-32..-16,z=-15..-5
// on x=-18..26,y=-33..15,z=-7..46
// off x=-40..-22,y=-38..-28,z=23..41
// on x=-16..35,y=-41..10,z=-47..6
// off x=-32..-23,y=11..30,z=-14..3
// on x=-49..-5,y=-3..45,z=-29..18
// off x=18..30,y=-20..-8,z=-3..13
// on x=-41..9,y=-7..43,z=-33..15
// on x=-54112..-39298,y=-85059..-49293,z=-27449..7877
// on x=967..23432,y=45373..81175,z=27513..53682".into()), 2758514936282235);
//     }
// }