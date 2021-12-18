#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Node {
    pub level: u32,
    pub index: Option<usize>,
    pub left_node: Option<usize>,
    pub left_value: Option<u32>,
    pub right_node: Option<usize>,
    pub right_value: Option<u32>,
}

impl Node {
    fn set_index(&mut self, value: usize) {
        self.index = Some(value);
    }
}

// pub struct Node<T> {
//     parent: Option<NodeId>,
//     previous_sibling: Option<NodeId>,
//     next_sibling: Option<NodeId>,
//     first_child: Option<NodeId>,
//     last_child: Option<NodeId>,
//
//     /// The actual data which will be stored within the tree
//     pub data: T,
// }

pub fn parse_node(
    line: &Vec<char>,
    start_index: usize,
    level: u32,
    nodes: &mut Vec<Node>
) -> (Node, usize) {
    let mut index = start_index;
    let mut hit_comma = false;
    let mut left_value: Option<u32> = None;
    let mut right_value: Option<u32> = None;
    let mut left_node: Option<usize> = None;
    let mut right_node: Option<usize> = None;

    while index < line.len() {
        let char = line[index];

        if char == '[' {
            let (node, new_index) = parse_node(line, index+1, level+1, nodes);
            nodes.push(node);

            if hit_comma {
                right_node = Some(nodes.len() - 1);
            }
            else {
                left_node = Some(nodes.len() - 1);
            }

            index = new_index;
        }
        else if char == ',' {
            hit_comma = true
        }
        else if char == ']' {
            let node = Node {
                index: None,
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
                left_value = Some(value);
            }
        }

        index += 1;
    }

    panic!("This should be unreachable");
}

pub fn parse(line: &str) -> Vec<Node> {
    let chars = line.chars().collect::<Vec<char>>();
    let mut nodes: Vec<Node> = vec![];
    let head = parse_node(&chars, 1, 0, &mut nodes).0;
    nodes.push(head);

    for index in 0..nodes.len() {
        nodes[index].set_index(index);
        println!("nodes[index] = {:?}", nodes[index]);
    }

    return nodes
}

pub fn find_heavily_nested(index: usize, parent_index: Option<usize>, nodes: &Vec<Node>) -> (Option<usize>, Option<usize>) {
    let number = &nodes[index];

    if number.level == 4 {
        return (number.index, parent_index);
    }

    if let Some(left_index) = number.left_node {
        return find_heavily_nested(left_index, Some(index), nodes);
    }

    if let Some(right_index) = number.right_node {
        return find_heavily_nested(right_index, Some(index), nodes);
    }

    return (None, None);
}

pub fn find_big_number(index: usize, parent_index: Option<usize>, nodes: &Vec<Node>) -> (Option<usize>, Option<usize>) {
    let number = &nodes[index];

    if let Some(value) = number.left_value {
        if value >= 10 {
            return (Some(index), parent_index)
        }
    }

    if let Some(value) = number.right_value {
        if value >= 10 {
            return (Some(index), parent_index)
        }
    }

    if let Some(left_node) = number.left_node {
        let result = find_big_number(left_node, Some(index), nodes);

        if result.0.is_some() {
            return result;
        }
    }

    if let Some(right_node) = number.right_node {
        let result = find_big_number(right_node, Some(index), nodes);

        if result.0.is_some() {
            return result;
        }
    }

    return (None, None)
}


pub fn part_a(text: String) -> i32 {
    let sailfish_numbers: Vec<Vec<Node>> = text.lines().map(|l| parse(l)).collect();

    for nodes in sailfish_numbers {
        let head_index = nodes.len() - 1;
        // let head: &Node = nodes.last().unwrap();
        // println!("head = {:?}", head);

        //let head_index = head.index.unwrap();
        let (mut nested_index, mut parent_index) = find_heavily_nested(head_index, None, &nodes);
        println!("nested_index = {:?} parent_index = {:?}", nested_index, parent_index);

        let (mut nested_index, mut parent_index) = find_big_number(head_index, None, &nodes);
        println!("nested_index = {:?} parent_index = {:?}", nested_index, parent_index);


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
            index: Some(0),
            level: 0,
            left_value: Some(1),
            left_node: None,
            right_value: Some(2),
            right_node: None,
        };

        assert_eq!(*parse("[1,2]").last().unwrap(), expected);
    }

    #[test]
    fn parse_test_2() {
        let expected = Node {
            index: Some(1),
            level: 0,
            left_value: None,
            left_node: Some(0),
            right_value: Some(3),
            right_node: None,
        };

        assert_eq!(*parse("[[1,2],3]").last().unwrap(), expected);
    }

    #[test]
    fn parse_test_3() {
        let expected = Node {
            index: Some(1),
            level: 0,
            left_value: Some(9),
            left_node: None,
            right_value: None,
            right_node: Some(0),
        };

        assert_eq!(*parse("[9,[8,7]]").last().unwrap(), expected);
    }
}