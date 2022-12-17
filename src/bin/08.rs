use itertools::Itertools;
use std::cmp::max;

pub fn part_one(input: &str) -> Option<u32> {
    let bytes = input.lines().map(str::as_bytes).collect_vec();
    let m = bytes.len();
    let n = bytes[0].len();

    let mut visible = vec![vec![false; n]; m];
    let mut max_l = vec![vec![0; n]; m];
    let mut max_u = vec![vec![0; n]; m];
    let mut max_r = vec![vec![0; n]; m];
    let mut max_d = vec![vec![0; n]; m];

    for i in 0..m {
        for j in 0..n {
            if i == 0 || j == 0 {
                visible[i][j] = true;
                max_l[i][j] = bytes[i][j];
                max_u[i][j] = bytes[i][j];
            } else {
                visible[i][j] |= bytes[i][j] > max_l[i - 1][j];
                visible[i][j] |= bytes[i][j] > max_u[i][j - 1];

                max_l[i][j] = max(bytes[i][j], max_l[i - 1][j]);
                max_u[i][j] = max(bytes[i][j], max_u[i][j - 1]);
            }
        }
    }

    for i in (0..m).rev() {
        for j in (0..n).rev() {
            if i == m - 1 || j == n - 1 {
                visible[i][j] = true;
                max_r[i][j] = bytes[i][j];
                max_d[i][j] = bytes[i][j];
            } else {
                visible[i][j] |= bytes[i][j] > max_r[i + 1][j];
                visible[i][j] |= bytes[i][j] > max_d[i][j + 1];

                max_r[i][j] = max(bytes[i][j], max_r[i + 1][j]);
                max_d[i][j] = max(bytes[i][j], max_d[i][j + 1]);
            }
        }
    }

    Some(
        visible
            .iter()
            .flat_map(|r| r.iter())
            .filter(|&b| *b)
            .count() as u32,
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let trees = input
        .lines()
        .map(|l| l.chars().map(|c| (c as usize) - 48).collect_vec())
        .collect_vec();
    let m = trees.len();
    let n = trees[0].len();

    let mut scene_l = vec![vec![[0; 10]; n]; m];
    let mut scene_u = vec![vec![[0; 10]; n]; m];
    let mut scene_r = vec![vec![[0; 10]; n]; m];
    let mut scene_d = vec![vec![[0; 10]; n]; m];

    // scene_d[i][j][k] = scene score of a tree of size `k` at pos (i, j) looking in direction d.
    for i in 0..m {
        for j in 0..n {
            for k in 0..10 {
                scene_u[i][j][k] = if i == 0 {
                    0
                } else if k <= trees[i - 1][j] {
                    1
                } else {
                    scene_u[i - 1][j][k] + 1
                };

                scene_l[i][j][k] = if j == 0 {
                    0
                } else if k <= trees[i][j - 1] {
                    1
                } else {
                    scene_l[i][j - 1][k] + 1
                };
            }
        }
    }

    for i in (0..m).rev() {
        for j in (0..n).rev() {
            for k in 0..10 {
                scene_d[i][j][k] = if i == m - 1 {
                    0
                } else if k <= trees[i + 1][j] {
                    1
                } else {
                    scene_d[i + 1][j][k] + 1
                };

                scene_r[i][j][k] = if j == n - 1 {
                    0
                } else if k <= trees[i][j + 1] {
                    1
                } else {
                    scene_r[i][j + 1][k] + 1
                };
            }
        }
    }

    Some(
        (0..m)
            .cartesian_product(0..n)
            .map(|(i, j)| {
                let k = trees[i][j];
                scene_l[i][j][k] * scene_r[i][j][k] * scene_u[i][j][k] * scene_d[i][j][k]
            })
            .max()
            .unwrap() as u32,
    )
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 8);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 8);
        assert_eq!(part_one(&input), Some(21));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 8);
        assert_eq!(part_two(&input), Some(8));
    }
}
