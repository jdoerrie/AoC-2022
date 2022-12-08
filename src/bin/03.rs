use itertools::Itertools;
use std::collections::HashSet;

fn to_set(input: &str) -> HashSet<char> {
    input.chars().collect()
}

fn prio(c: char) -> u32 {
    match c {
        'a'..='z' => (c as u32) - 96,
        'A'..='Z' => (c as u32) - 38,
        _ => panic!("Unsupported: {c}"),
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .map(|l| l.split_at(l.len() / 2))
            .map(|(l, r)| prio(*to_set(l).intersection(&to_set(r)).next().unwrap()))
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .chunks(3)
            .into_iter()
            .map(|mut c| {
                prio(
                    *to_set(c.next().unwrap())
                        .intersection(&to_set(c.next().unwrap()))
                        .cloned()
                        .collect::<HashSet<char>>()
                        .intersection(&to_set(c.next().unwrap()))
                        .next()
                        .unwrap(),
                )
            })
            .sum(),
    )
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 3);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 3);
        assert_eq!(part_one(&input), Some(157));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 3);
        assert_eq!(part_two(&input), Some(70));
    }
}
