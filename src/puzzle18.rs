#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Node {
    pub level: u32,
    pub index: Option<usize>,
    pub parent_index: Option<usize>,
    pub left_node: Option<usize>,
    pub left_value: Option<u32>,
    pub right_node: Option<usize>,
    pub right_value: Option<u32>,
}

impl Node {
    fn split_left(&mut self, index: usize) {
        self.left_value = None;
        self.left_node = Some(index);
    }

    fn split_right(&mut self, index: usize) {
        self.right_value = None;
        self.right_node = Some(index);
    }

    fn set_index(&mut self, value: usize) {
        self.index = Some(value);
    }

    fn set_parent_index(&mut self, value: usize) {
        self.parent_index = Some(value);
    }

    fn increment_left(&mut self, value: u32) {
        self.left_value = Some(self.left_value.unwrap() + value);
    }

    fn increment_right(&mut self, value: u32) {
        self.right_value = Some(self.right_value.unwrap() + value);
    }

    fn zero_out_child(&mut self, child_index: usize) {
        if self.left_node.is_some() && self.left_node.unwrap() == child_index {
            self.left_node = None;
            self.left_value = Some(0);
        }
        else if self.right_node.is_some() && self.right_node.unwrap() == child_index {
            self.right_node = None;
            self.right_value = Some(0);
        }
        else {
            panic!("Oh no!");
        }
    }
}

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
                parent_index: None,
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

pub fn reduce_and_render(line: &str) -> String {
    let mut nodes = &mut parse(line);
    let head_index = nodes.len() - 1;
    reduce(&mut nodes);

    let mut buffer = &mut "".into();
    debug(head_index, nodes, buffer);

    return buffer.clone();
}

pub fn reduce(_nodes: &mut Vec<Node>) {
    let mut nodes = _nodes;
    let head_index = nodes.len() - 1;
    let mut count = 0;

    loop {
        count += 1;
        let result = find_heavily_nested(head_index, &nodes);
        println!("result = {:?}", result);

        if let Some(heavily_nested) = result {
            explode_v2(head_index, heavily_nested, &mut nodes);
            continue;
        }

        let result = find_big_number(head_index, &nodes);
        println!("result = {:?}", result);

        if let Some(big_number_index) = result {
            split(big_number_index, &mut nodes);
            continue
        }

        println!("count = {}", count);
        break;
    }
}

pub fn debug(index: usize, nodes: &Vec<Node>, buffer: &mut String) {
    let head = &nodes[index];

    buffer.push('[');

    if head.left_value.is_some() {
        buffer.push_str(&head.left_value.unwrap().to_string())
    }
    else {
        debug(head.left_node.unwrap(), nodes, buffer)
    }

    buffer.push(',');

    if head.right_value.is_some() {
        buffer.push_str(&head.right_value.unwrap().to_string())
    }
    else {
        debug(head.right_node.unwrap(), nodes, buffer)
    }

    buffer.push(']');
}

pub fn parse(line: &str) -> Vec<Node> {
    let chars = line.chars().collect::<Vec<char>>();
    let mut nodes: Vec<Node> = vec![];
    let head = parse_node(&chars, 1, 0, &mut nodes).0;
    nodes.push(head);

    // Set index
    for index in 0..nodes.len() {
        nodes[index].set_index(index);
    }

    // Set parent index
    for index in 0..nodes.len() {
        if let Some(left_index) = nodes[index].left_node {
            nodes[left_index].set_parent_index(index);
        }
        if let Some(right_index) = nodes[index].right_node {
            nodes[right_index].set_parent_index(index);
        }
    }

    return nodes
}

pub fn find_heavily_nested(index: usize, nodes: &Vec<Node>) -> Option<usize> {
    let number = &nodes[index];
    println!("find_heavily_nested() index = {}", index);

    if number.level == 4 {
        return number.index;
    }

    if let Some(left_index) = number.left_node {
        let result =  find_heavily_nested(left_index, nodes);

        if result.is_some() {
            return result
        }
    }

    if let Some(right_index) = number.right_node {
        let result =  find_heavily_nested(right_index, nodes);

        if result.is_some() {
            return result
        }
    }

    return None;
}

pub fn find_big_number(index: usize, nodes: &Vec<Node>) -> Option<usize> {
    let number = &nodes[index];

    if let Some(value) = number.left_value {
        if value >= 10 {
            return Some(index)
        }
    }

    if let Some(value) = number.right_value {
        if value >= 10 {
            return Some(index)
        }
    }

    if let Some(left_node) = number.left_node {
        let result = find_big_number(left_node, nodes);

        if result.is_some() {
            return result;
        }
    }

    if let Some(right_node) = number.right_node {
        let result = find_big_number(right_node, nodes);

        if result.is_some() {
            return result;
        }
    }

    return None
}

pub fn split(index: usize, nodes: &mut Vec<Node>) {
    let length = &mut nodes.len();
    let mut node = &mut nodes[index];

    if let Some(value) = node.left_value {
        if value >= 10 {
            let left_value = value / 2;
            let right_value = left_value + (value % 2);

            let child = Node {
                level: node.level + 1,
                index: Some(*length),
                parent_index: node.index,
                left_node: None,
                left_value: Some(left_value),
                right_node: None,
                right_value: Some(right_value),
            };

            node.split_left(*length);
            nodes.push(child);
        }
    }
    else if let Some(value) = node.right_value {
        if value >= 10 {
            let left_value = value / 2;
            let right_value = left_value + (value % 2);

            let child = Node {
                level: node.level + 1,
                index: Some(*length),
                parent_index: node.index,
                left_node: None,
                left_value: Some(left_value),
                right_node: None,
                right_value: Some(right_value),
            };

            node.split_right(*length);
            nodes.push(child);
        }
    }
    else {
        panic!("FUck!");
    }
}

pub fn explode_v2(
    start_index: usize,
    target_index: usize,
    nodes: &mut Vec<Node>
) {
    let mut stack: Vec<usize> = Vec::new();
    let mut node_index: Option<usize> = Some(start_index);
    let mut hit_target = false;

    let mut before_index: Option<usize> = None;
    let mut before_is_left = false;
    let mut after_index: Option<usize> = None;
    let mut after_is_left = false;

    while !stack.is_empty() || node_index.is_some() {
        if node_index.is_some() {
            stack.push(node_index.unwrap());
            node_index = nodes[node_index.unwrap()].left_node;
        }
        else {
            node_index = Some(stack.pop().unwrap());
            let unwrapped_node_index = node_index.unwrap();

            if unwrapped_node_index == target_index {
                hit_target = true;
            }
            else {
                let node = &nodes[unwrapped_node_index];

                if let Some(left_value) = node.left_value {
                    if !hit_target {
                        before_index = node_index;
                        before_is_left = true;
                    }
                    else {
                        after_index = node_index;
                        after_is_left = true;
                        break;
                    }

                    //println!("{}", left_value);
                }
                if let Some(right_value) = node.right_value {
                    if !hit_target {
                        before_index = node_index;
                        before_is_left = false;
                    }
                    else {
                        after_index = node_index;
                        after_is_left = false;
                        break;
                    }

                    //println!("{}", right_value);
                }
            }



            node_index = nodes[unwrapped_node_index].right_node;
        }
    }

    //println!("Hi Angus");

    let left_value = nodes[target_index].left_value.unwrap();
    let right_value = nodes[target_index].right_value.unwrap();

    if before_index.is_some() {
        if before_is_left {
            nodes[before_index.unwrap()].increment_left(left_value)
        }
        else {
            nodes[before_index.unwrap()].increment_right(left_value)
        }
    }

    if after_index.is_some() {
        if after_is_left {
            nodes[after_index.unwrap()].increment_left(right_value)
        }
        else {
            nodes[after_index.unwrap()].increment_right(right_value)
        }
    }

    // Then, the entire exploding pair is replaced with the regular number 0
    let parent_index = nodes[target_index].parent_index.unwrap().clone();
    nodes[parent_index].zero_out_child(target_index);
}

// pub fn explode(index: usize, nodes: &mut Vec<Node>) {
//     let start_index = index;
//     let left_value = nodes[index].left_value.unwrap();
//     let right_value = nodes[index].right_value.unwrap();
//
//     // Explode to the left
//     let mut current_index: Option<usize> = nodes[index].parent_index;
//
//     loop {
//         let i = current_index.unwrap();
//
//         if nodes[i].left_value.is_some() {
//             nodes[i].increment_left(left_value);
//             break;
//         }
//
//         current_index = nodes[current_index.unwrap()].parent_index;
//
//         if current_index.is_none() {
//             break
//         }
//     }
//
//     // Explode to the right
//     let mut current_index: Option<usize> = nodes[index].parent_index;
//
//     loop {
//         let i = current_index.unwrap();
//
//         if nodes[i].right_value.is_some() {
//             nodes[i].increment_right(right_value);
//             break;
//         }
//
//         current_index = nodes[current_index.unwrap()].parent_index;
//
//         if current_index.is_none() {
//             break
//         }
//     }
//
//     // Then, the entire exploding pair is replaced with the regular number 0
//     let parent_index = nodes[start_index].parent_index.unwrap().clone();
//     nodes[parent_index].zero_out_child(start_index);
// }

pub fn part_a(text: String) -> i32 {
    let mut sailfish_numbers: Vec<Vec<Node>> = text.lines().map(|l| parse(l)).collect();

    for index in 0..sailfish_numbers.len() {
        let nodes = reduce(&mut sailfish_numbers[index]);
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
    fn explosion_reducing_examples() {
        assert_eq!(reduce_and_render("[[[[[9,8],1],2],3],4]".into()), "[[[[0,9],2],3],4]");
        assert_eq!(reduce_and_render("[7,[6,[5,[4,[3,2]]]]]".into()), "[7,[6,[5,[7,0]]]]");
        assert_eq!(reduce_and_render("[[6,[5,[4,[3,2]]]],1]".into()), "[[6,[5,[7,0]]],3]");
        assert_eq!(reduce_and_render("[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]".into()), "[[3,[2,[8,0]]],[9,[5,[7,0]]]]");
    }

    #[test]
    fn example_part_a_3() {
        assert_eq!(part_a("[7,[6,[5,[4,[3,2]]]]]".into()), 3488);
    }

    #[test]
    fn example_part_a_4() {
        assert_eq!(part_a("[[6,[5,[4,[3,2]]]],1]".into()), 3488);
    }

    #[test]
    fn example_part_a_5() {
        assert_eq!(part_a("[[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]]".into()), 3488);
    }

    #[test]
    fn example_part_b() {
        assert_eq!(part_b("".into()), 0);
    }

    #[test]
    fn parse_test_1() {
        let expected = Node {
            parent_index: None,
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
            parent_index: None,
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
            parent_index: None,
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