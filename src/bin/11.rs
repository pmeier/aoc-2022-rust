use std::collections::VecDeque;

#[derive(Debug)]
enum Operation {
    Add(usize),
    Multiply(usize),
    Square,
}

impl Operation {
    fn from_str(input: &str) -> Operation {
        use Operation::*;

        fn extract_num(input: &str, sep: char) -> usize {
            input
                .split(sep)
                .rev()
                .next()
                .unwrap()
                .trim()
                .parse()
                .unwrap()
        }

        let operation_str = input.split('=').rev().next().unwrap().trim();
        if operation_str == "old * old" {
            Square
        } else if operation_str.starts_with("old *") {
            Multiply(extract_num(operation_str, '*'))
        } else if operation_str.starts_with("old +") {
            Add(extract_num(operation_str, '+'))
        } else {
            unreachable!()
        }
    }

    fn perform(&self, input: usize) -> usize {
        use Operation::*;
        match self {
            Add(summand) => input + summand,
            Multiply(multiplicand) => input * multiplicand,
            Square => input * input,
        }
    }
}

#[derive(Debug)]
struct Test {
    divisible_by: usize,
    idx_true: usize,
    idx_false: usize,
}

impl Test {
    fn from_strs<'a, I>(strs: I) -> Test
    where
        I: Iterator<Item = &'a str>,
    {
        let params: Vec<_> = strs
            .take(3)
            .map(|line| line.split(' ').rev().next().unwrap().parse().unwrap())
            .collect();
        Test {
            divisible_by: params[0],
            idx_true: params[1],
            idx_false: params[2],
        }
    }
    fn perform(&self, input: usize) -> usize {
        if input % self.divisible_by == 0 {
            self.idx_true
        } else {
            self.idx_false
        }
    }
}

#[derive(Debug)]
struct Item {
    worry_level: usize,
}

impl Item {
    fn inspect(&mut self, operation: &Operation, relief: &bool) {
        let mut new_worry_level = operation.perform(self.worry_level);

        if *relief {
            new_worry_level /= 3;
        }

        const MAX_WORRY_LEVEL: usize = 2 * 3 * 5 * 7 * 11 * 13 * 17 * 19;
        new_worry_level %= MAX_WORRY_LEVEL;

        self.worry_level = new_worry_level;
    }
}

#[derive(Debug)]
struct Monkey {
    items: VecDeque<Item>,
    operation: Operation,
    test: Test,
    num_inspections: usize,
}

impl Monkey {
    fn new(items: VecDeque<Item>, operation: Operation, test: Test) -> Monkey {
        Monkey {
            items,
            operation,
            test,
            num_inspections: 0,
        }
    }
    fn inspect_and_throw(&mut self, relief: bool) -> (Item, usize) {
        let mut item = self.items.pop_front().unwrap();

        item.inspect(&self.operation, &relief);
        self.num_inspections += 1;

        let other_idx = self.test.perform(item.worry_level);

        (item, other_idx)
    }
}

fn parse(input: &str) -> Vec<Monkey> {
    input
        .split("\n\n")
        .map(|block| {
            let mut lines = block.lines().skip(1);

            let items: VecDeque<Item> = lines
                .next()
                .unwrap()
                .split(':')
                .rev()
                .next()
                .unwrap()
                .trim()
                .split(", ")
                .map(|worry_level| Item {
                    worry_level: worry_level.parse().unwrap(),
                })
                .collect();
            let operation = Operation::from_str(lines.next().unwrap());
            let test = Test::from_strs(lines);

            Monkey::new(items, operation, test)
        })
        .collect()
}

fn solve(input: &str, rounds: usize, relief: bool) -> u64 {
    let mut monkeys = parse(input);

    for _ in 0..rounds {
        for monkey in &mut monkeys {
            while !monkey.items.is_empty() {
                let (item, idx) = monkey.inspect_and_throw(relief);
                // FIXME E0499
                monkeys[idx].items.push_back(item);
            }
        }
    }

    let mut num_inspections = monkeys
        .iter()
        .map(|monkey| monkey.num_inspections as u64)
        .collect::<Vec<_>>();
    num_inspections.sort();
    num_inspections.iter().take(2).product::<u64>()
}

pub fn part_one(input: &str) -> Option<u64> {
    Some(solve(input, 20, true))
}

pub fn part_two(input: &str) -> Option<u64> {
    Some(solve(input, 10_000, false))
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 11);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 11);
        assert_eq!(part_one(&input), Some(10605));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 11);
        assert_eq!(part_two(&input), Some(2713310158));
    }
}
