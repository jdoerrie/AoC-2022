use itertools::Itertools;

struct Monkey {
    id: usize,
    items: Vec<i64>,
    op: Box<dyn Fn(i64) -> i64>,
    divisor: i64,
    next: [usize; 2],
}

impl Monkey {
    fn new() -> Self {
        Monkey {
            id: 0,
            items: Vec::new(),
            op: Box::new(|x| x),
            divisor: 1,
            next: [0; 2],
        }
    }
}

fn parse_id(input: &str) -> usize {
    input
        .strip_prefix("Monkey ")
        .unwrap()
        .strip_suffix(':')
        .unwrap()
        .parse()
        .unwrap()
}

fn parse_items(input: &str) -> Vec<i64> {
    input
        .strip_prefix("Starting items: ")
        .unwrap()
        .split(", ")
        .map(str::parse)
        .map(Result::unwrap)
        .collect()
}

fn parse_op(input: &str) -> Box<dyn Fn(i64) -> i64> {
    let tokens = input
        .strip_prefix("Operation: new = ")
        .unwrap()
        .split_ascii_whitespace()
        .collect_vec();

    let lhs = tokens[0].parse();
    let rhs = tokens[2].parse();

    let bin_op = match tokens[1] {
        "+" => |a: &i64, b: &i64| *a + *b,
        "*" => |a: &i64, b: &i64| *a * *b,
        _ => unreachable!(),
    };

    Box::new(move |x| bin_op(lhs.as_ref().unwrap_or(&x), rhs.as_ref().unwrap_or(&x)))
}

fn parse_test(input: &str) -> i64 {
    input
        .strip_prefix("Test: divisible by ")
        .unwrap()
        .parse()
        .unwrap()
}

fn parse_next_true(input: &str) -> usize {
    input
        .strip_prefix("If true: throw to monkey ")
        .unwrap()
        .parse()
        .unwrap()
}

fn parse_next_false(input: &str) -> usize {
    input
        .strip_prefix("If false: throw to monkey ")
        .unwrap()
        .parse()
        .unwrap()
}

fn parse_monkey(input: &str) -> Monkey {
    let mut monkey = Monkey::new();
    for line in input.lines() {
        let trimmed = line.trim();
        match trimmed {
            m if trimmed.starts_with("Monkey") => monkey.id = parse_id(m),
            items if trimmed.starts_with("Starting") => monkey.items = parse_items(items),
            op if trimmed.starts_with("Operation") => monkey.op = parse_op(op),
            test if trimmed.starts_with("Test") => monkey.divisor = parse_test(test),
            next if trimmed.starts_with("If false") => monkey.next[0] = parse_next_false(next),
            next if trimmed.starts_with("If true") => monkey.next[1] = parse_next_true(next),
            "" => continue,
            _ => unreachable!(),
        }
    }

    monkey
}

fn solve(input: &str, n_iters: usize, div: i64) -> usize {
    let mut monkeys = input.split("\n\n").map(parse_monkey).collect_vec();
    let modulus: i64 = monkeys.iter().map(|m| m.divisor).product();
    let mut monkey_counts = vec![0; monkeys.len()];
    for _ in 0..n_iters {
        for i in 0..monkeys.len() {
            let monkey = &mut monkeys[i];

            assert!(i == monkey.id);
            let mut next_items = [Vec::new(), Vec::new()];

            for item in std::mem::take(&mut monkey.items) {
                let new_item = ((monkey.op)(item) / div) % modulus;
                next_items[(new_item % monkey.divisor == 0) as usize].push(new_item);
                monkey_counts[i] += 1;
            }

            for (next, mut items) in std::iter::zip(monkey.next, next_items) {
                monkeys[next].items.append(&mut items);
            }
        }
    }

    monkey_counts.iter().sorted().rev().take(2).product()
}

pub fn part_one(input: &str) -> Option<usize> {
    Some(solve(input, 20, 3))
}

pub fn part_two(input: &str) -> Option<usize> {
    Some(solve(input, 10000, 1))
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 11);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 11);
        assert_eq!(part_one(&input), Some(10605));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 11);
        assert_eq!(part_two(&input), Some(2713310158));
    }
}
