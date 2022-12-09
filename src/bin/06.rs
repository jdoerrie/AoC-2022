use std::collections::HashSet;

fn solve(input: &str, window_size: usize) -> usize {
    input
        .as_bytes()
        .windows(window_size)
        .enumerate()
        .find(|(_, w)| w.iter().cloned().collect::<HashSet<_>>().len() == window_size)
        .unwrap()
        .0
        + window_size
}

pub fn part_one(input: &str) -> Option<u32> {
    Some(solve(input, 4) as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(solve(input, 14) as u32)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 6);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 6);
        assert_eq!(part_one(&input), Some(7));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 6);
        assert_eq!(part_two(&input), Some(19));
    }
}
