struct Cargo {
    containers: Vec<Vec<char>>,
}

impl Cargo {
    fn new(n: usize) -> Cargo {
        Cargo {
            containers: vec![Vec::<char>::new(); n],
        }
    }

    fn add(&mut self, line: &str) {
        for (i, c) in line.chars().enumerate().filter(|(_, c)| *c != ' ') {
            self.containers[i].push(c)
        }
    }
    fn move_containers(&mut self, n: usize, i: usize, j: usize) {
        for _ in 0..n {
            let c = self.containers[i].pop().unwrap();
            self.containers[j].push(c);
        }
    }

    fn move_containers_new(&mut self, n: usize, i: usize, j: usize) {
        for k in 0..n {
            let c = self.containers[i][self.containers[i].len() - n + k];
            self.containers[j].push(c);
        }

        for _ in 0..n {
            self.containers[i].pop();
        }
    }

    fn tops(&self) -> String {
        self.containers.iter().map(|v| v[v.len() - 1]).collect()
    }
}

pub fn part_one(input: &str) -> Option<String> {
    let (start, steps) = input.split_once("\n\n").unwrap();
    let n = (start.split('\n').next().unwrap().len() + 1) / 4;
    let mut cargo = Cargo::new(n);
    for con in start
        .lines()
        .rev()
        .skip(1)
        .map(|l| l.chars().skip(1).step_by(4).collect::<String>())
    {
        cargo.add(&con);
    }

    for step in steps.lines().map(|l| {
        l.split(' ')
            .skip(1)
            .step_by(2)
            .map(|i| i.parse::<usize>().unwrap())
            .collect::<Vec<_>>()
    }) {
        cargo.move_containers(step[0], step[1] - 1, step[2] - 1)
    }

    Some(cargo.tops())
}

pub fn part_two(input: &str) -> Option<String> {
    let (start, steps) = input.split_once("\n\n").unwrap();
    let n = (start.split('\n').next().unwrap().len() + 1) / 4;
    let mut cargo = Cargo::new(n);
    for con in start
        .lines()
        .rev()
        .skip(1)
        .map(|l| l.chars().skip(1).step_by(4).collect::<String>())
    {
        cargo.add(&con);
    }

    for step in steps.lines().map(|l| {
        l.split(' ')
            .skip(1)
            .step_by(2)
            .map(|i| i.parse::<usize>().unwrap())
            .collect::<Vec<_>>()
    }) {
        cargo.move_containers_new(step[0], step[1] - 1, step[2] - 1)
    }

    Some(cargo.tops())
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 5);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 5);
        assert_eq!(part_one(&input), Some(String::from("CMZ")));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 5);
        assert_eq!(part_two(&input), Some(String::from("MCD")));
    }
}
