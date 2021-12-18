#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Node {
    pub level: u32,
    pub left_node: Box<Option<Node>>,
    pub left_value: Option<u32>,
    pub right_node: Box<Option<Node>>,
    pub right_value: Option<u32>,
}

pub fn parse_node(
    line: &Vec<char>,
    start_index: usize,
    level: u32,
) -> (Node, usize) {
    let mut index = start_index;
    let mut hit_comma = false;
    let mut left_value: Option<u32> = None;
    let mut right_value: Option<u32> = None;
    let mut left_node: Box<Option<Node>> = Box::new(None);
    let mut right_node: Box<Option<Node>> = Box::new(None);

    while index < line.len() {
        let char = line[index];

        if char == '[' {
            let (node, new_index) = parse_node(line, index+1, level+1);

            if hit_comma {
                right_node = Box::new(Some(node));
            }
            else {
                left_node = Box::new(Some(node));
            }

            index = new_index;
        }
        else if char == ',' {
            hit_comma = true
        }
        else if char == ']' {
            let node = Node {
                level,
                left_value,
                right_value,
                left_node,
                right_node
            };

            return (node, index)
        }
        else if char.is_digit(10) {
            let value = char.to_digit(10).unwrap();

            if hit_comma {
                right_value = Some(value);
            }
            else {
                left_value =  Some(value);
            }
        }

        index += 1;
    }

    panic!("This should be unreachable");
}

pub fn parse(line: &str) -> Node {
    let chars = line.chars().collect::<Vec<char>>();
    return parse_node(&chars, 1, 0).0;
}

pub fn find_heavily_nested(number: Node, parent: Option<Node>) -> (Option<(Node)>, Option<Node>) {
    if number.level == 4 {
        return (Some(&number), parent);
    }
    if number.left_node.is_some() {
        let node = &number.left_node.unwrap();
        let _parent = Some(number);
        return find_heavily_nested(node, _parent)
    }
    if number.right_node.is_some() {
        return find_heavily_nested(&number.right_node.unwrap(), Some(number))
    }

    return (None, None);
}

pub fn find_big_number(number: Node) -> Option<Node> {
    if let Some(value) = number.left_value {
        if value >= 10 {
            return Some(number)
        }
    }

    if let Some(value) = number.right_value {
        if value >= 10 {
            return Some(number)
        }
    }

    if let Some(left_node) = *number.left_node {
        let result = find_big_number(left_node);

        if result.is_some() {
            return result
        }
    }

    if let Some(right_node) = *number.right_node {
        let result = find_big_number(right_node);

        if result.is_some() {
            return result
        }
    }

    return None
}


pub fn part_a(text: String) -> i32 {
    let sailfish_numbers = text.lines().map(|l| parse(l)).collect::<Vec<Node>>();

    for number in sailfish_numbers {
        // let big_number = find_big_number(number);
        //
        // if big_number.is_some() {
        //     println!("found a big_number")
        // }


        let (heavily_nested, parent) = find_heavily_nested(number, None);

        if let Some(value) = heavily_nested {
            println!("found a heavily nested {:?}", value)


        }
    }




    println!("");
    panic!("Not implemented")


    // To explode a pair, the pair's left value is added to the first regular number to the left of the exploding pair
    // (if any), and the pair's right value is added to the first regular number to the right of the exploding pair
    // (if any). Exploding pairs will always consist of two regular numbers. Then, the entire exploding pair
    // is replaced with the regular number 0.


    // To split a regular number, replace it with a pair; the left element of the pair should be the regular number
    // divided by two and rounded down, while the right element of the pair should be the regular number divided by
    // two and rounded up. For example, 10 becomes [5,5], 11 becomes [5,6], 12 becomes [6,6], and so on.


    // To check whether it's the right answer, the snailfish teacher only checks the magnitude of the final sum.
    // The magnitude of a pair is 3 times the magnitude of its left element plus 2 times the magnitude of its right element.
    // The magnitude of a regular number is just that number.

}

pub fn part_b(_text: String) -> i32 {
    panic!("Not implemented")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::*;
    const FILENAME: &str = "./resources/2021/18.txt";

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
        assert_eq!(part_a("[[[0,[4,5]],[0,0]],[[[4,5],[2,6]],[9,5]]]
[7,[[[3,7],[4,3]],[[6,3],[8,8]]]]
[[2,[[0,8],[3,4]]],[[[6,7],1],[7,[1,6]]]]
[[[[2,4],7],[6,[0,5]]],[[[6,8],[2,8]],[[2,1],[4,5]]]]
[7,[5,[[3,8],[1,4]]]]
[[2,[2,2]],[8,[8,1]]]
[2,9]
[1,[[[9,3],9],[[9,0],[0,7]]]]
[[[5,[7,4]],7],1]
[[[[4,2],2],6],[8,7]]".into()), 3488);
    }

    #[test]
    fn example_part_a_2() {
        assert_eq!(part_a("[[[[[9,8],1],2],3],4]".into()), 3488);
    }

    #[test]
    fn example_part_a_3() {
        assert_eq!(part_a("[7,[6,[5,[4,[3,2]]]]]".into()), 3488);
    }





    #[test]
    fn example_part_b() {
        assert_eq!(part_b("".into()), 0);
    }

    #[test]
    fn parse_test_1() {
        let expected = Node {
            level: 0,
            left_value: Some(1),
            left_node: Box::new(None),
            right_value: Some(2),
            right_node: Box::new(None),
        };

        assert_eq!(parse("[1,2]"), expected);
    }

    #[test]
    fn parse_test_2() {
        let sub = Node {
            level: 1,
            left_value: Some(1),
            left_node: Box::new(None),
            right_value: Some(2),
            right_node: Box::new(None),
        };

        let expected = Node {
            level: 0,
            left_value: None,
            left_node: Box::new(Some(sub)),
            right_value: Some(3),
            right_node: Box::new(None),
        };

        assert_eq!(parse("[[1,2],3]"), expected);
    }

    #[test]
    fn parse_test_3() {
        let sub = Node {
            level: 1,
            left_value: Some(8),
            left_node: Box::new(None),
            right_value: Some(7),
            right_node: Box::new(None),
        };

        let expected = Node {
            level: 0,
            left_value: Some(9),
            left_node: Box::new(None),
            right_value: None,
            right_node: Box::new(Some(sub)),
        };

        assert_eq!(parse("[9,[8,7]]"), expected);
    }
}