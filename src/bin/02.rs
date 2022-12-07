#[derive(Debug, Clone, Copy)]
enum Shape {
    Rock,
    Paper,
    Scissors,
}

fn score(shape: &Shape) -> u32 {
    match shape {
        Shape::Rock => 1,
        Shape::Paper => 2,
        Shape::Scissors => 3,
    }
}

fn result(lhs: &Shape, rhs: &Shape) -> u32 {
    match (lhs, rhs) {
        (Shape::Rock, Shape::Rock) => 3,
        (Shape::Rock, Shape::Paper) => 6,
        (Shape::Rock, Shape::Scissors) => 0,
        (Shape::Paper, Shape::Rock) => 0,
        (Shape::Paper, Shape::Paper) => 3,
        (Shape::Paper, Shape::Scissors) => 6,
        (Shape::Scissors, Shape::Rock) => 6,
        (Shape::Scissors, Shape::Paper) => 0,
        (Shape::Scissors, Shape::Scissors) => 3,
    }
}

fn opponent(input: &str) -> Shape {
    match input {
        "A" => Shape::Rock,
        "B" => Shape::Paper,
        "C" => Shape::Scissors,
        _ => panic!("Unknown: {}", input),
    }
}

fn strat(input: &str) -> Shape {
    match input {
        "X" => Shape::Rock,
        "Y" => Shape::Paper,
        "Z" => Shape::Scissors,
        _ => panic!("Unknown: {}", input),
    }
}

fn choose(opponent: &Shape, input: &str) -> Shape {
    match (opponent, input) {
        (_, "Y") => *opponent,
        (Shape::Rock, "X") => Shape::Scissors,
        (Shape::Rock, "Z") => Shape::Paper,
        (Shape::Paper, "X") => Shape::Rock,
        (Shape::Paper, "Z") => Shape::Scissors,
        (Shape::Scissors, "X") => Shape::Paper,
        (Shape::Scissors, "Z") => Shape::Rock,
        _ => panic!("Not Handled"),
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .map(|l| {
                let (x, y) = l.split_once(' ').unwrap();
                result(&opponent(x), &strat(y)) + score(&strat(y))
            })
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .map(|l| {
                let (x, y) = l.split_once(' ').unwrap();
                result(&opponent(x), &choose(&opponent(x), y)) + score(&choose(&opponent(x), y))
            })
            .sum(),
    )
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 2);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_one(&input), Some(15));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_two(&input), Some(12));
    }
}
