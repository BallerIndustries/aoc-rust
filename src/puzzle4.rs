use std::collections::HashSet;

pub fn part_a(text: String) -> i32 {
    let (numbers, mut boards) = parse_text(text);

    for number in numbers {
        for board in boards.iter_mut() {
            zero_out_match(number, board);

            if has_bingo(board) {
                let unmarked_sum: i32 = board.iter().flatten().sum();
                return unmarked_sum * number;
            }
        }
    }

    panic!("Ruh oh")
}

pub fn part_b(text: String) -> i32 {
    let (numbers, mut boards) = parse_text(text);
    let board_len = boards.len();
    let mut board_indexes: HashSet<usize> = HashSet::new();

    for number in numbers {
        for (board_index, board) in boards.iter_mut().enumerate() {
            if board_indexes.contains(&board_index) {
                continue
            }

            zero_out_match(number, board);

            if has_bingo(board) {
                board_indexes.insert(board_index);
            }

            if board_indexes.len() == board_len {
                let unmarked_sum: i32 = board.iter().flatten().sum();
                return unmarked_sum * number;
            }
        }
    }

    panic!("Ruh oh")
}

fn zero_out_match(number: i32, board: &mut Vec<Vec<i32>>) {
    for row in board.iter_mut() {
        for value in row.iter_mut() {
            if *value == number {
                *value = 0
            }
        }
    }
}

fn parse_text(text: String) -> (Vec<i32>, Vec<Vec<Vec<i32>>>) {
    let lines: Vec<&str> = text.lines().collect();
    let numbers: Vec<i32> = lines.get(0).unwrap().split(",").map(|x| x.parse::<i32>().unwrap()).collect();

    let pieces: Vec<&str> = text.split("\n\n").collect();
    let mut boards: Vec<Vec<Vec<i32>>> = Vec::new();

    for index in 1..pieces.len() {
        let board_text = pieces[index];
        let board_lines: Vec<&str> = board_text.lines().collect();

        let board: Vec<Vec<i32>> = board_lines.iter().map(|l| {
            l.split_whitespace().map(|t| t.parse::<i32>().unwrap()).collect()
        }).collect();

        boards.push(board);
    }
    (numbers, boards)
}

fn has_bingo(board: &Vec<Vec<i32>>) -> bool {
    for row in board {
        let row_bingo = row.iter().filter(|x| **x != 0).count() == 0;

        if row_bingo {
            return true
        }
    }

    for y in 0..5 {
        let column: Vec<i32> = board.iter().map(|row| row[y]).collect();
        let column_bingo = column.iter().filter(|x| **x != 0).count() == 0;

        if column_bingo {
            return true
        }
    }

    return false
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::*;
    const FILENAME: &str = "./resources/2021/4.txt";

    #[test]
    fn puzzle_part_a() {
        let text = read_all_text(FILENAME);
        assert_eq!(part_a(text), 33348)
    }

    #[test]
    fn puzzle_part_b() {
        let text = read_all_text(FILENAME);
        assert_eq!(part_b(text), 8112)
    }
}