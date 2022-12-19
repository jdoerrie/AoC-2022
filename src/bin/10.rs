use itertools::Itertools;

enum Instruction {
    NoOp,
    AddX(i64),
}

fn parse_line(input: &str) -> Instruction {
    match input {
        "noop" => Instruction::NoOp,
        addx if addx.starts_with("addx") => {
            Instruction::AddX(addx.split_once(' ').unwrap().1.parse().unwrap())
        }
        _ => unreachable!(),
    }
}

// Returns a vector where vec[i] corresponds to the value of X after the i'th cycle.
fn get_registers(input: &str) -> Vec<i64> {
    std::iter::once(vec![1])
        .chain(input.lines().map(parse_line).scan(1, |x, ins| {
            let ret;
            match ins {
                Instruction::NoOp => {
                    ret = vec![*x];
                }
                Instruction::AddX(val) => {
                    ret = vec![*x, *x + val];
                    *x += val;
                }
            };
            Some(ret)
        }))
        .flatten()
        .collect()
}

pub fn part_one(input: &str) -> Option<i64> {
    let registers = get_registers(input);
    Some(
        (20..=registers.len())
            .step_by(40)
            .map(|i| (i as i64) * registers[i - 1])
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<String> {
    let registers = get_registers(input);

    const WIDTH: usize = 40;
    const HEIGHT: usize = 6;
    let mut monitor = [[b'.'; WIDTH]; HEIGHT];

    for cycle in 0..WIDTH * HEIGHT {
        let x = registers[cycle];
        let c = if (((cycle % WIDTH) as i64) - x).abs() <= 1 {
            b'#'
        } else {
            b'.'
        };
        monitor[cycle / WIDTH][cycle % WIDTH] = c;
    }

    Some(
        monitor
            .iter()
            .map(|row| std::str::from_utf8(row.as_slice()).unwrap())
            .intersperse("\n")
            .collect(),
    )
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 10);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 10);
        assert_eq!(part_one(&input), Some(13140));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 10);
        let output = String::from("##..##..##..##..##..##..##..##..##..##..\n###...###...###...###...###...###...###.\n####....####....####....####....####....\n#####.....#####.....#####.....#####.....\n######......######......######......####\n#######.......#######.......#######.....");
        assert_eq!(part_two(&input), Some(output));
    }
}
