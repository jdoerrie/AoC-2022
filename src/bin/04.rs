struct Range {
    min: u32,
    max: u32,
}

fn to_range(input: &str) -> Range {
    let (min, max) = input.split_once('-').unwrap();
    Range {
        min: min.parse().unwrap(),
        max: max.parse().unwrap(),
    }
}

fn encloses(lhs: &Range, rhs: &Range) -> bool {
    lhs.min <= rhs.min && rhs.max <= lhs.max
}

fn disjoint(lhs: &Range, rhs: &Range) -> bool {
    lhs.max < rhs.min || rhs.max < lhs.min
}

pub fn part_one(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .filter_map(|l| l.split_once(','))
            .map(|(l, r)| (to_range(l), to_range(r)))
            .filter(|(x, y)| encloses(x, y) || encloses(y, x))
            .count() as u32,
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .filter_map(|l| l.split_once(','))
            .map(|(l, r)| (to_range(l), to_range(r)))
            .filter(|(x, y)| !disjoint(x, y))
            .count() as u32,
    )
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 4);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 4);
        assert_eq!(part_one(&input), Some(2));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 4);
        assert_eq!(part_two(&input), Some(4));
    }
}
