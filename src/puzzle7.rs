use std::collections::{HashMap};

pub fn part_a(text: &str) -> u16 {
    let mut wire_to_signal: HashMap<&str, u16> = HashMap::new();

    while !wire_to_signal.contains_key("a") {
        for line in text.lines() {
            let segments: Vec<&str> = line.split_whitespace().collect();

            if segments[0] == "NOT" {
                if let Some(value) = get_value(&wire_to_signal, segments[1]) {
                    wire_to_signal.insert(segments[3], !value);
                }
            }
            else if segments[1] == "->" {
                if let Some(value) = get_value(&wire_to_signal, segments[0]) {
                    wire_to_signal.insert(segments[2], value);
                }
            }
            else {
                if let Some(op_a) = get_value(&wire_to_signal, segments[0]) {
                    if let Some(op_b) = get_value(&wire_to_signal, segments[2]) {
                        if segments[1] == "AND" {
                            wire_to_signal.insert(segments[4], op_a & op_b);
                        }
                        else if segments[1] == "OR" {
                            wire_to_signal.insert(segments[4], op_a | op_b);
                        }
                        else if segments[1] == "LSHIFT" {
                            wire_to_signal.insert(segments[4], op_a << op_b);
                        }
                        else if segments[1] == "RSHIFT" {
                            wire_to_signal.insert(segments[4], op_a >> op_b);
                        }
                    }
                }
            }
        }
    }

    return *wire_to_signal.get("a").unwrap()
}

pub fn get_value(wire_to_signal: &HashMap<&str, u16>, value: &str) -> Option<u16> {
    let is_numeric = value.chars().all(|c| c.is_numeric());

    if is_numeric {
        let parsed = value.parse::<u16>().unwrap();
        return Some(parsed)
    }

    if !wire_to_signal.contains_key(value) {
        return None
    }

    return Some(*wire_to_signal.get(value).unwrap());
}

pub fn get_value_with_overrides(overrides: &HashMap<&str, u16>, wire_to_signal: &HashMap<&str, u16>, value: &str) -> Option<u16> {
    let is_numeric = value.chars().all(|c| c.is_numeric());

    if is_numeric {
        let parsed = value.parse::<u16>().unwrap();
        return Some(parsed)
    }

    if overrides.contains_key(value) {
        return Some(*overrides.get(value).unwrap())
    }


    if !wire_to_signal.contains_key(value) {
        return None
    }

    return Some(*wire_to_signal.get(value).unwrap());
}

pub fn part_b(text: String) -> u16 {
    let mut wire_to_signal: HashMap<&str, u16> = HashMap::new();
    let a_signal = part_a(&text);

    let mut overrides: HashMap<&str, u16> = HashMap::new();
    overrides.insert("b", a_signal);

    while !wire_to_signal.contains_key("a") {
        for line in text.lines() {
            let segments: Vec<&str> = line.split_whitespace().collect();

            if segments[0] == "NOT" {
                if let Some(value) = get_value_with_overrides(&overrides, &wire_to_signal, segments[1]) {
                    wire_to_signal.insert(segments[3], !value);
                }
            }
            else if segments[1] == "->" {
                if let Some(value) = get_value_with_overrides(&overrides, &wire_to_signal, segments[0]) {
                    wire_to_signal.insert(segments[2], value);
                }
            }
            else {
                if let Some(op_a) = get_value_with_overrides(&overrides, &wire_to_signal, segments[0]) {
                    if let Some(op_b) = get_value_with_overrides(&overrides, &wire_to_signal, segments[2]) {
                        if segments[1] == "AND" {
                            wire_to_signal.insert(segments[4], op_a & op_b);
                        }
                        else if segments[1] == "OR" {
                            wire_to_signal.insert(segments[4], op_a | op_b);
                        }
                        else if segments[1] == "LSHIFT" {
                            wire_to_signal.insert(segments[4], op_a << op_b);
                        }
                        else if segments[1] == "RSHIFT" {
                            wire_to_signal.insert(segments[4], op_a >> op_b);
                        }
                    }
                }
            }
        }
    }

    return *wire_to_signal.get("a").unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::*;
    const FILENAME: &str = "./resources/2015/7.txt";

    #[test]
    fn puzzle_part_a() {
        let text = read_all_text(FILENAME);
        assert_eq!(part_a(&text), 956)
    }

    #[test]
    fn puzzle_part_b() {
        let text = read_all_text(FILENAME);
        assert_eq!(part_b(text), 40149)
    }
}