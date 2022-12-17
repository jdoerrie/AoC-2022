use itertools::Itertools;

enum Direction {
    Up(isize),
    Down(isize),
    Left(isize),
    Right(isize),
}

fn parse_direction(input: &str) -> Direction {
    let (dir, n) = input.split_once(' ').unwrap();
    match dir {
        "U" => Direction::Up(n.parse().unwrap()),
        "D" => Direction::Down(n.parse().unwrap()),
        "L" => Direction::Left(n.parse().unwrap()),
        "R" => Direction::Right(n.parse().unwrap()),
        _ => unreachable!(),
    }
}

type Pos = (isize, isize);

fn gen_head_moves(dirs: &[Direction]) -> Vec<Pos> {
    let mut result = vec![(0, 0)];
    for dir in dirs {
        let (i, j) = result[result.len() - 1];
        match dir {
            Direction::Up(n) => {
                for k in 1..=*n {
                    result.push((i + k, j));
                }
            }
            Direction::Down(n) => {
                for k in 1..=*n {
                    result.push((i - k, j));
                }
            }
            Direction::Left(n) => {
                for k in 1..=*n {
                    result.push((i, j - k));
                }
            }
            Direction::Right(n) => {
                for k in 1..=*n {
                    result.push((i, j + k));
                }
            }
        }
    }

    result
}

fn get_tail_move(old_tail: Pos, head: Pos) -> Pos {
    let dx = head.0 - old_tail.0;
    let dy = head.1 - old_tail.1;

    if dx.abs() <= 1 && dy.abs() <= 1 {
        old_tail
    } else {
        (old_tail.0 + dx.signum(), old_tail.1 + dy.signum())
    }
}

fn parse_directions(input: &str) -> Vec<Direction> {
    input.lines().map(parse_direction).collect()
}

pub fn part_one(input: &str) -> Option<u32> {
    let moves = gen_head_moves(parse_directions(input).as_slice());
    let mut tail_moves = vec![(0, 0)];
    for mv in moves {
        tail_moves.push(get_tail_move(tail_moves[tail_moves.len() - 1], mv));
    }

    Some(tail_moves.iter().unique().count() as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut head = gen_head_moves(parse_directions(input).as_slice());
    let mut tail = Vec::new();
    for _ in 0..9 {
        if !tail.is_empty() {
            head = tail;
        }
        tail = vec![(0, 0)];
        for mv in &head {
            tail.push(get_tail_move(tail[tail.len() - 1], *mv));
        }
    }

    Some(tail.iter().unique().count() as u32)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 9);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 9);
        assert_eq!(part_one(&input), Some(88));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 9);
        assert_eq!(part_two(&input), Some(36));
    }
}
