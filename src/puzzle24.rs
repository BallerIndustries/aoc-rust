pub fn transpile(text: String) -> i32 {
    let mut counter = 0;

    for line in text.lines() {
        let tmp = line.split_whitespace().collect::<Vec<&str>>();

        match tmp[0] {
            "inp" => {
                println!("{} = input[{}];", tmp[1], counter);
                counter += 1;
            }
            "add" => { println!("{} = {} + {};", tmp[1], tmp[1], tmp[2]) }
            "mul" => { println!("{} = {} * {};", tmp[1], tmp[1], tmp[2]) }
            "div" => { println!("{} = {} / {};", tmp[1], tmp[1], tmp[2]) }
            "mod" => { println!("{} = {} % {};", tmp[1], tmp[1], tmp[2]) }
            "eql" => { println!("{} = eql({},{});", tmp[1], tmp[1], tmp[2]) }
            _ => { panic!() }
        }
    }

    panic!();
}

fn puzzle(input: Vec<i64>) -> i64 {
    let mut w = 0i64;
    let mut x = 0i64;
    let mut y = 0i64;
    let mut z = 0i64;

    w = input[0];
    x = x * 0;
    x = x + z;
    x = x % 26;
    z = z / 1;
    x = x + 12;
    x = eql(x, w);
    x = eql(x, 0);
    y = y * 0;
    y = y + 25;
    y = y * x;
    y = y + 1;
    z = z * y;
    y = y * 0;
    y = y + w;
    y = y + 6;
    y = y * x;
    z = z + y;




    w = input[1];
    x = x * 0;
    x = x + z;
    x = x % 26;
    z = z / 1;
    x = x + 10;
    x = eql(x, w);
    x = eql(x, 0);
    y = y * 0;
    y = y + 25;
    y = y * x;
    y = y + 1;
    z = z * y;
    y = y * 0;
    y = y + w;
    y = y + 2;
    y = y * x;
    z = z + y;
    w = input[2];
    x = x * 0;
    x = x + z;
    x = x % 26;
    z = z / 1;
    x = x + 10;
    x = eql(x, w);
    x = eql(x, 0);
    y = y * 0;
    y = y + 25;
    y = y * x;
    y = y + 1;
    z = z * y;
    y = y * 0;
    y = y + w;
    y = y + 13;
    y = y * x;
    z = z + y;
    w = input[3];
    x = x * 0;
    x = x + z;
    x = x % 26;
    z = z / 26;
    x = x + -6;
    x = eql(x, w);
    x = eql(x, 0);
    y = y * 0;
    y = y + 25;
    y = y * x;
    y = y + 1;
    z = z * y;
    y = y * 0;
    y = y + w;
    y = y + 8;
    y = y * x;
    z = z + y;
    w = input[4];
    x = x * 0;
    x = x + z;
    x = x % 26;
    z = z / 1;
    x = x + 11;
    x = eql(x, w);
    x = eql(x, 0);
    y = y * 0;
    y = y + 25;
    y = y * x;
    y = y + 1;
    z = z * y;
    y = y * 0;
    y = y + w;
    y = y + 13;
    y = y * x;
    z = z + y;
    w = input[5];
    x = x * 0;
    x = x + z;
    x = x % 26;
    z = z / 26;
    x = x + -12;
    x = eql(x, w);
    x = eql(x, 0);
    y = y * 0;
    y = y + 25;
    y = y * x;
    y = y + 1;
    z = z * y;
    y = y * 0;
    y = y + w;
    y = y + 8;
    y = y * x;
    z = z + y;
    w = input[6];
    x = x * 0;
    x = x + z;
    x = x % 26;
    z = z / 1;
    x = x + 11;
    x = eql(x, w);
    x = eql(x, 0);
    y = y * 0;
    y = y + 25;
    y = y * x;
    y = y + 1;
    z = z * y;
    y = y * 0;
    y = y + w;
    y = y + 3;
    y = y * x;
    z = z + y;
    w = input[7];
    x = x * 0;
    x = x + z;
    x = x % 26;
    z = z / 1;
    x = x + 12;
    x = eql(x, w);
    x = eql(x, 0);
    y = y * 0;
    y = y + 25;
    y = y * x;
    y = y + 1;
    z = z * y;
    y = y * 0;
    y = y + w;
    y = y + 11;
    y = y * x;
    z = z + y;
    w = input[8];
    x = x * 0;
    x = x + z;
    x = x % 26;
    z = z / 1;
    x = x + 12;
    x = eql(x, w);
    x = eql(x, 0);
    y = y * 0;
    y = y + 25;
    y = y * x;
    y = y + 1;
    z = z * y;
    y = y * 0;
    y = y + w;
    y = y + 10;
    y = y * x;
    z = z + y;
    w = input[9];
    x = x * 0;
    x = x + z;
    x = x % 26;
    z = z / 26;
    x = x + -2;
    x = eql(x, w);
    x = eql(x, 0);
    y = y * 0;
    y = y + 25;
    y = y * x;
    y = y + 1;
    z = z * y;
    y = y * 0;
    y = y + w;
    y = y + 8;
    y = y * x;
    z = z + y;
    w = input[10];
    x = x * 0;
    x = x + z;
    x = x % 26;
    z = z / 26;
    x = x + -5;
    x = eql(x, w);
    x = eql(x, 0);
    y = y * 0;
    y = y + 25;
    y = y * x;
    y = y + 1;
    z = z * y;
    y = y * 0;
    y = y + w;
    y = y + 14;
    y = y * x;
    z = z + y;
    w = input[11];
    x = x * 0;
    x = x + z;
    x = x % 26;
    z = z / 26;
    x = x + -4;
    x = eql(x, w);
    x = eql(x, 0);
    y = y * 0;
    y = y + 25;
    y = y * x;
    y = y + 1;
    z = z * y;
    y = y * 0;
    y = y + w;
    y = y + 6;
    y = y * x;
    z = z + y;
    w = input[12];
    x = x * 0;
    x = x + z;
    x = x % 26;
    z = z / 26;
    x = x + -4;
    x = eql(x, w);
    x = eql(x, 0);
    y = y * 0;
    y = y + 25;
    y = y * x;
    y = y + 1;
    z = z * y;
    y = y * 0;
    y = y + w;
    y = y + 8;
    y = y * x;
    z = z + y;
    w = input[13];
    x = x * 0;
    x = x + z;
    x = x % 26;
    z = z / 26;
    x = x + -12;
    x = eql(x, w);
    x = eql(x, 0);
    y = y * 0;
    y = y + 25;
    y = y * x;
    y = y + 1;
    z = z * y;
    y = y * 0;
    y = y + w;
    y = y + 2;
    y = y * x;
    z = z + y;

    return z;
}

fn eql(a: i64, b: i64) -> i64 {
    return if a == b { 1 } else { 0 };
}

pub fn part_b(_text: String) -> i32 {
    panic!("Not implemented")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::*;

    const FILENAME: &str = "./resources/2021/24.txt";

    #[test]
    fn go_the_computer() {
        for d1 in (1i64..=9i64).rev() {
            for d2 in (1i64..=9i64).rev() {
                for d3 in (1i64..=9i64).rev() {
                    for d4 in (1i64..=9i64).rev() {
                        for d5 in (1i64..=9i64).rev() {
                            for d6 in (1i64..=9i64).rev() {
                                for d7 in (1i64..=9i64).rev() {
                                    for d8 in (1i64..=9i64).rev() {
                                        for d9 in (1i64..=9i64).rev() {
                                            for d10 in (1i64..=9i64).rev() {
                                                for d11 in (1i64..=9i64).rev() {
                                                    for d12 in (1i64..=9i64).rev() {
                                                        for d13 in (1i64..=9i64).rev() {
                                                            for d14 in (1i64..=9i64).rev() {
                                                                let result = puzzle(vec![d1, d2, d3, d4, d5, d6, d7, d8, d9, d10, d11, d12, d13, d14]);

                                                                if result == 0 {
                                                                    println!("{}{}{}{}{}{}{}{}{}{}{}{}{}{}", d1, d2, d3, d4, d5, d6, d7, d8, d9, d10, d11, d12, d13, d14);
                                                                    panic!();
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }


    #[test]
    fn puzzle_part_a() {
        let text = read_all_text(FILENAME);
        assert_eq!(transpile(text), 0)
    }

    #[test]
    fn puzzle_part_b() {
        let text = read_all_text(FILENAME);
        assert_eq!(part_b(text), 0)
    }

    #[test]
    fn example_part_a() {
        assert_eq!(transpile("".into()), 0);
    }

    #[test]
    fn example_part_b() {
        assert_eq!(part_b("".into()), 0);
    }
}