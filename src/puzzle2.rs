pub fn part_a(text: String) -> i64 {
    let mut total: i64 = 0;
    let lines = text.split("\n");

    for line in lines {
        let mut tmp = line.split("x");
        let l = tmp.next().unwrap().parse::<i64>().unwrap();
        let w = tmp.next().unwrap().parse::<i64>().unwrap();
        let h = tmp.next().unwrap().parse::<i64>().unwrap();

        let side_a = l*w;
        let side_b = w*h;
        let side_c = h*l;

        total += (2*side_a) + (2*side_b) + (2*side_c);
        println!("{}", total);

        if side_a <= side_b && side_a <= side_c {
            total += side_a
        }
        else if side_b <= side_a && side_b <= side_c {
            total += side_b
        }
        else {
            total += side_c
        }

        println!("{}", total);
    }

    return total;
}

pub fn part_b(text: String) -> i64 {
    let mut total: i64 = 0;
    let lines = text.split("\n");

    for line in lines {
        let mut tmp = line.split("x");
        let l = tmp.next().unwrap().parse::<i64>().unwrap();
        let w = tmp.next().unwrap().parse::<i64>().unwrap();
        let h = tmp.next().unwrap().parse::<i64>().unwrap();

        let perim_a = (l+w)*2;
        let perim_b = (w+h)*2;
        let perim_c = (h+l)*2;

        total += l*w*h;

        if perim_a <= perim_b && perim_a <= perim_c {
            total += perim_a
        }
        else if perim_b <= perim_a && perim_b <= perim_c {
            total += perim_b
        }
        else {
            total += perim_c
        }
    }

    return total;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::*;

    #[test]
    fn puzzle_part_a() {
        let text = read_all_text("./resources/2015/2.txt");
        assert_eq!(part_a(text), 1606483)
    }

    #[test]
    fn puzzle_part_b() {
        let text = read_all_text("./resources/2015/2.txt");
        assert_eq!(part_b(text), 3842356)
    }

    #[test]
    fn example_part_a() {
        assert_eq!(part_a("1x1x10".into()), 43);
        assert_eq!(part_a("2x3x4".into()), 58);
    }
}