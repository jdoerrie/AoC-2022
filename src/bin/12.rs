use std::collections::{HashMap, VecDeque};

use itertools::Itertools;

type Pair = (usize, usize);

fn get_height(c: u8) -> isize {
    match c {
        b'S' => 0,
        b'E' => 25,
        c => (c - b'a') as isize,
    }
}

fn get_next(bytes: &[&[u8]], (i, j): Pair) -> Vec<Pair> {
    let m = bytes.len() as isize;
    let n = bytes[0].len() as isize;

    let h = get_height(bytes[i][j]);
    [(-1, 0), (1, 0), (0, -1), (0, 1)]
        .map(|(dx, dy)| (i as isize + dx, j as isize + dy))
        .iter()
        .filter(|&(x, y)| (0..m).contains(x) && (0..n).contains(y))
        .map(|&(i, j)| (i as usize, j as usize))
        .filter(|&(i, j)| (h - get_height(bytes[i][j])) <= 1)
        .collect()
}

fn bfs(bytes: &[&[u8]], beg: Pair) -> HashMap<Pair, u32> {
    let mut q = VecDeque::new();
    let mut costs = HashMap::new();
    costs.insert(beg, 0);
    q.push_back((beg, 0));

    while let Some((p, c)) = q.pop_front() {
        for n in get_next(bytes, p) {
            if let std::collections::hash_map::Entry::Vacant(e) = costs.entry(n) {
                e.insert(c + 1);
                q.push_back((n, c + 1));
            }
        }
    }
    costs
}

pub fn part_one(input: &str) -> Option<u32> {
    let bytes = input.lines().map(str::as_bytes).collect_vec();
    let m = bytes.len();
    let n = bytes[0].len();

    let find_pos = |c| {
        (0..m)
            .cartesian_product(0..n)
            .find(|&(i, j)| bytes[i][j] == c)
            .unwrap()
    };

    let beg = find_pos(b'E');
    let end = find_pos(b'S');

    bfs(&bytes[..], beg).get(&end).copied()
}

pub fn part_two(input: &str) -> Option<u32> {
    let bytes = input.lines().map(str::as_bytes).collect_vec();
    let m = bytes.len();
    let n = bytes[0].len();

    let beg = (0..m)
        .cartesian_product(0..n)
        .find(|&(i, j)| bytes[i][j] == b'E')
        .unwrap();

    let costs = bfs(&bytes[..], beg);
    costs
        .keys()
        .filter(|&(i, j)| get_height(bytes[*i][*j]) == 0)
        .map(|k| costs.get(k).unwrap())
        .min()
        .copied()
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 12);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 12);
        assert_eq!(part_one(&input), Some(31));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 12);
        assert_eq!(part_two(&input), Some(29));
    }
}
