use itertools::Itertools;
use std::collections::HashMap;
use std::path::PathBuf;

fn parse_fs(input: &str) -> HashMap<PathBuf, usize> {
    let mut map = HashMap::<PathBuf, usize>::new();
    let mut path = PathBuf::new();
    for line in input.lines() {
        let tokens = line.split(' ').collect_vec();
        if tokens[0] == "$" {
            if tokens[1] == "cd" {
                if tokens[2] == ".." {
                    path.pop();
                } else {
                    path.push(tokens[2]);
                }
            }
        } else if tokens[0] != "dir" {
            let size = tokens[0].parse::<usize>().unwrap();
            for p in path.ancestors() {
                *map.entry(p.to_path_buf()).or_default() += size;
            }
        }
    }

    map
}

pub fn part_one(input: &str) -> Option<u32> {
    Some(
        parse_fs(input)
            .values()
            .cloned()
            .filter(|&v| v <= 100000)
            .sum::<usize>() as u32,
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let fs = parse_fs(input);
    let unused = 70000000 - *fs.get(&PathBuf::from("/")).unwrap();
    let needed = 30000000 - unused;

    Some(fs.values().cloned().filter(|&v| v >= needed).min().unwrap() as u32)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 7);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 7);
        assert_eq!(part_one(&input), Some(95437));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 7);
        assert_eq!(part_two(&input), Some(24933642));
    }
}
