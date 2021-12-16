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

pub fn part_a(text: String) -> i32 {
    let mut binary: String = "".into();

    for char in text.chars() {
        binary.push_str(to_binary(char))
    }

    let mut index: usize = 0;
    parse_packet(&binary, index);

    println!();
    panic!("Not implemented")
}

fn parse_packet(binary: &str, mut index: usize) -> usize {
    let (version, mut index) = read_int(&binary, index, 3);
    let (t, mut index) = read_int(&binary, index, 3);

    println!("version = {}", version);

    if t == 4 {
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
        println!("Parsed literal value with version = {} and value = {}", version, value);
    } else {
        let mut result = read_chars(&binary, index, 1);
        let len_type_id = result.0;
        index = result.1;

        if len_type_id == "0" {
            // Fifteen bit number
            result = read_chars(&binary, index, 15);

            let length = to_int(result.0) as usize;
            index = result.1;
            let end_index = index + length;

            while index < end_index {
                index = parse_packet(binary, index)
            }

        } else {
            // Eleven bit number
            result = read_chars(&binary, index, 11);

            let count = to_int(result.0) as usize;
            index = result.1;

            for _ in 0..count {
                index = parse_packet(binary, index)
            }
        }
    }

    return index
}

pub fn part_b(text: String) -> i32 {
    panic!("Not implemented")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::*;
    const FILENAME: &str = "./resources/2021/16.txt";

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
    fn example_part_a_1() {
        assert_eq!(part_a("D2FE28".into()), 0);
    }

    #[test]
    fn example_part_a_2() {
        assert_eq!(part_a("38006F45291200".into()), 0);
    }

    #[test]
    fn example_part_a_3() {
        assert_eq!(part_a("EE00D40C823060".into()), 0);
    }

    #[test]
    fn example_part_a_4() {
        assert_eq!(part_a("8A004A801A8002F478".into()), 0);
    }







    #[test]
    fn example_part_b() {
        assert_eq!(part_b("".into()), 0);
    }
}