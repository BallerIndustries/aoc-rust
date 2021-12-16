#[derive(Debug)]
pub struct Node {
    pub version: u64,
    pub value: String,
    pub children: Vec<Node>
}

fn to_binary(c: char) -> &'static str {
    match c {
        '0' => "0000",
        '1' => "0001",
        '2' => "0010",
        '3' => "0011",
        '4' => "0100",
        '5' => "0101",
        '6' => "0110",
        '7' => "0111",
        '8' => "1000",
        '9' => "1001",
        'A' => "1010",
        'B' => "1011",
        'C' => "1100",
        'D' => "1101",
        'E' => "1110",
        'F' => "1111",
        _ => "",
    }
}

fn to_int(text: &str) -> u64 {
    return u64::from_str_radix(text, 2).unwrap();
}

fn read_chars(binary: &str, index: usize, count: usize) -> (&str, usize) {
    return (&binary[index..index+count], index + count);
}

fn read_int(binary: &str, index: usize, count: usize) -> (u64, usize) {
    let result = read_chars(binary, index, count);
    return (to_int(result.0), result.1)
}

pub fn part_a(text: String) -> u64 {
    let mut binary: String = "".into();

    for char in text.chars() {
        binary.push_str(to_binary(char))
    }

    let (_, head) = parse_packet(&binary, 0);
    return sum_versions(&head);
}

fn sum_versions(node: &Node) -> u64 {
    let children_versions = node.children.iter()
        .map(|it| sum_versions(&it))
        .collect::<Vec<u64>>();

    let mut total: u64 = 0;

    for v in children_versions {
        total += v;
    }

    return total + node.version;
}

fn parse_packet(binary: &String, index: usize) -> (usize, Node) {
    let (version, index) = read_int(&binary, index, 3);
    let (type_yo, mut index) = read_int(&binary, index, 3);

    if type_yo == 4 {
        // Literal Value
        let mut result = read_chars(&binary, index, 5);
        let mut temp = result.0;
        index = result.1;
        let mut number_buffer: String = temp[1..5].into();

        while temp.chars().collect::<Vec<char>>()[0] == '1' {
            result = read_chars(&binary, index, 5);
            temp = result.0;
            index = result.1;
            number_buffer.push_str(&temp[1..5]);
        }

        let value = to_int(&number_buffer);
        return (index, Node { version, value: value.to_string(), children: Vec::new() })
    }
    else {
        let mut result = read_chars(&binary, index, 1);
        let len_type_id = result.0;
        index = result.1;

        let node_name: String = match type_yo {
            0 => { "sum".into() }
            1 => { "product".into() }
            2 => { "minimum".into() }
            3 => { "maximum".into() }
            5 => { "greater_than".into() }
            6 => { "less_than".into() }
            7 => { "equal_to".into() }
            _ => { panic!("oh no!") }
        };

        if len_type_id == "0" {
            // Fifteen bit number
            result = read_chars(&binary, index, 15);

            let length = to_int(result.0) as usize;
            index = result.1;
            let end_index = index + length;
            let mut children: Vec<Node> = vec![];

            while index < end_index {
                let horse_result = parse_packet(binary, index);
                index = horse_result.0;
                children.push(horse_result.1);
            }

            return (index, Node { version, value: node_name, children: children })
        }
        else {
            // Eleven bit number
            result = read_chars(&binary, index, 11);

            let count = to_int(result.0) as usize;
            index = result.1;
            let mut children: Vec<Node> = vec![];

            for _ in 0..count {
                let horse_result = parse_packet(binary, index);
                index = horse_result.0;
                children.push(horse_result.1);
            }

            return (index, Node { version, value: node_name, children: children })
        }
    }
}

pub fn part_b(text: String) -> u64 {
    let mut binary: String = "".into();

    for char in text.chars() {
        binary.push_str(to_binary(char))
    }

    let (_, head) = parse_packet(&binary, 0);
    return resolve_node(&head);
}

pub fn resolve_node(node: &Node) -> u64 {
    let horse: &str = &node.value;

    return match horse {
        "sum" => {
            node.children.iter()
                .map(|it| resolve_node(it)).sum()
        }
        "product" => {
            node.children.iter()
                .map(|it| resolve_node(it)).product()
        }
        "minimum" => {
            node.children.iter()
                .map(|it| resolve_node(it)).min().unwrap()
        }
        "maximum" => {
            node.children.iter()
                .map(|it| resolve_node(it)).max().unwrap()
        }
        "greater_than" => {
            let pieces: Vec<u64> = node.children.iter()
                .map(|it| resolve_node(it)).collect();

            if pieces[0] > pieces[1] { 1 } else { 0 }
        }
        "less_than" => {
            let pieces: Vec<u64> = node.children.iter()
                .map(|it| resolve_node(it)).collect();

            if pieces[0] < pieces[1] { 1 } else { 0 }
        }
        "equal_to" => {
            let pieces: Vec<u64> = node.children.iter()
                .map(|it| resolve_node(it)).collect();

            if pieces[0] == pieces[1] { 1 } else { 0 }
        }
        _ => {
            node.value.parse::<u64>().unwrap()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::*;
    const FILENAME: &str = "./resources/2021/16.txt";

    #[test]
    fn puzzle_part_a() {
        let text = read_all_text(FILENAME);
        assert_eq!(part_a(text), 879)
    }

    #[test]
    fn puzzle_part_b() {
        let text = read_all_text(FILENAME);
        assert_eq!(part_b(text), 539051801941)
    }

    #[test]
    fn example_part_a_1() {
        assert_eq!(part_a("D2FE28".into()), 6);
    }

    #[test]
    fn example_part_a_2() {
        assert_eq!(part_a("38006F45291200".into()), 9);
    }

    #[test]
    fn example_part_a_3() {
        assert_eq!(part_a("EE00D40C823060".into()), 14);
    }

    #[test]
    fn example_part_a_4() {
        assert_eq!(part_a("8A004A801A8002F478".into()), 16);
    }

    #[test]
    fn example_part_b() {
        assert_eq!(part_b("C200B40A82".into()), 3);
    }

    #[test]
    fn example_part_b_2() {
        assert_eq!(part_b("04005AC33890".into()), 54);
    }

    #[test]
    fn example_part_b_3() {
        assert_eq!(part_b("9C0141080250320F1802104A08".into()), 1);
    }
}