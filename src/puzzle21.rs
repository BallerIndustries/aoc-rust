use std::cmp::max;
use std::collections::HashMap;

struct Player {
    index: u32,
    points: u32
}

impl Player {
    fn advance(&mut self, steps: u32) {
        self.index = (self.index + steps) % 10;
        self.points += self.index + 1;
    }
}

struct MockDie {
    value: u32,
    roll_count: u32
}

struct State {
    p1_turn: bool,
    p1_score: u8,
    p2_score: u8,
    p1_position: u8,
    p2_position: u8,
    frequency: u64
}

impl MockDie {
    fn increment(&mut self) {
        self.value += 1;

        if self.value > 100 {
            self.value = 1;
        }
    }

    fn roll_three(&mut self) -> u32 {
        let r1 = self.value;
        self.increment();
        let r2 = self.value;
        self.increment();
        let r3 = self.value;
        self.increment();

        self.roll_count += 3;
        return r1 + r2 + r3;
    }
}

pub fn part_a(text: String) -> u32 {
    let tmp: Vec<u32> = text.lines().map(|l| l.split(": ").collect::<Vec<&str>>()[1].parse::<u32>().unwrap()).collect();
    let p1 = tmp[0];
    let p2 = tmp[1];


    let mut die = MockDie { value: 1, roll_count: 0 };
    let mut player_one_turn = false;
    let player_one = &mut Player { index: p1-1, points: 0 };
    let player_two = &mut Player { index: p2-1, points: 0 };

    while player_one.points < 1000 && player_two.points < 1000 {
        player_one_turn = !player_one_turn;

        if player_one_turn {
            let roll_sum = die.roll_three();
            player_one.advance(roll_sum);
            //println!("Player 1 rolls {} and moves to space {} for a total score of {}", roll_sum, &player_one.index + 1, &player_one.points)
        }
        else {
            let roll_sum = die.roll_three();
            player_two.advance(roll_sum);
            //println!("Player 2 rolls {} and moves to space {} for a total score of {}", roll_sum, &player_two.index + 1, &player_two.points)
        }
    }

    if player_one_turn {
        return die.roll_count * player_two.points;
    } else {
        return die.roll_count * player_one.points;
    }
}

pub fn part_b(text: String) -> u64 {
    let roll_frequency = create_roll_frequency();
    let mut frontier = parse_frontier(text);
    let mut p1_wins = 0u64;
    let mut p2_wins = 0u64;

    while !frontier.is_empty() {
        let state = frontier.pop().unwrap();

        if state.p1_score >= 21 {
            p1_wins += state.frequency;
            continue
        }

        if state.p2_score >= 21 {
            p2_wins += state.frequency;
            continue
        }

        for (roll, frequency) in roll_frequency.iter() {
            if state.p1_turn {
                let p1_position = (state.p1_position + roll) % 10;
                let p1_score = state.p1_score + (p1_position + 1);
                let frequency = state.frequency * frequency;

                frontier.push(
                    State {
                        p1_turn: !state.p1_turn,
                        p1_score: p1_score,
                        p2_score: state.p2_score,
                        p1_position: p1_position,
                        p2_position: state.p2_position,
                        frequency: frequency,
                    }
                )
            }
            else {
                let p2_position = (state.p2_position + roll) % 10;
                let p2_score = state.p2_score + (p2_position + 1);
                let frequency = state.frequency * frequency;

                frontier.push(
                    State {
                        p1_turn: !state.p1_turn,
                        p1_score: state.p1_score,
                        p2_score: p2_score,
                        p1_position: state.p1_position,
                        p2_position: p2_position,
                        frequency: frequency,
                    }
                )
            }
        }
    }

    println!("p1_wins = {} p2_wins = {}", p1_wins, p2_wins);
    return max(p1_wins, p2_wins)
}

fn parse_frontier(text: String) -> Vec<State> {
    let tmp: Vec<u8> = text.lines().map(|l| l.split(": ").collect::<Vec<&str>>()[1].parse::<u8>().unwrap()).collect();

    vec![
        State {
            p1_turn: true,
            p1_score: 0,
            p2_score: 0,
            p1_position: tmp[0] - 1,
            p2_position: tmp[1] - 1,
            frequency: 1,
        }
    ]
}

fn create_roll_frequency() -> HashMap<u8, u64> {
    let mut roll_frequency: HashMap<u8, u64> = HashMap::new();

    for d1 in 1..=3 {
        for d2 in 1..=3 {
            for d3 in 1..=3 {
                let counter = roll_frequency
                    .entry(d1 + d2 + d3)
                    .or_insert(0);

                *counter += 1;
            }
        }
    }
    roll_frequency
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::*;
    const FILENAME: &str = "./resources/2021/21.txt";

    #[test]
    fn puzzle_part_a() {
        let text = read_all_text(FILENAME);
        assert_eq!(part_a(text), 571032)
    }

    #[test]
    fn puzzle_part_b() {
        let text = read_all_text(FILENAME);
        assert_eq!(part_b(text), 49975322685009)
    }

    #[test]
    fn example_part_a() {
        assert_eq!(part_a("Player 1 starting position: 4
Player 2 starting position: 8".into()), 739785);
    }

    #[test]
    fn example_part_b() {
        assert_eq!(part_b("Player 1 starting position: 4
Player 2 starting position: 8".into()), 444356092776315);
    }
}