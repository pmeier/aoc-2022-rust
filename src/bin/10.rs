use std::collections::VecDeque;

enum Instruction {
    NoOp,
    AddX(isize),
}

impl Instruction {
    fn from_str(input: &str) -> Instruction {
        use Instruction::*;
        match input.trim() {
            "noop" => NoOp,
            addx => AddX(addx.split(' ').rev().next().unwrap().parse().unwrap()),
        }
    }
}

struct Instructions {
    stack: VecDeque<Instruction>,
}

impl Instructions {
    fn from_str(input: &str) -> Instructions {
        Instructions {
            stack: input.lines().map(Instruction::from_str).collect(),
        }
    }
}

impl Iterator for Instructions {
    type Item = Instruction;

    fn next(&mut self) -> Option<Self::Item> {
        self.stack.pop_front()
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut cycle: usize = 0;
    let mut x: isize = 1;
    let mut last_report = 0;
    let mut report;
    let mut signal_strengths = 0;
    for instruction in Instructions::from_str(input) {
        use Instruction::*;
        let (duration, x_diff) = match instruction {
            NoOp => (1, 0),
            AddX(x_diff) => (2, x_diff),
        };

        cycle += duration;

        report = (cycle + 20) / 40;
        if report > last_report {
            signal_strengths += (last_report * 40 + 20) * x as usize;
            last_report = report;
        }

        x += x_diff;
    }
    Some(signal_strengths as u32)
}

pub fn part_two(input: &str) -> Option<String> {
    let mut instructions = Instructions::from_str(input);

    let mut x: isize = 1;
    let mut position: usize = 0;
    let mut display = Vec::new();

    let mut duration = 1;
    let mut x_diff: isize = 0;

    loop {
        duration -= 1;

        if duration == 0 {
            x += x_diff;
            (duration, x_diff) = match instructions.next() {
                None => break,
                Some(instruction) => match instruction {
                    Instruction::NoOp => (1, 0),
                    Instruction::AddX(x_diff) => (2, x_diff),
                },
            };
        }

        let cursor = (position % 40) as isize;
        display.push(if (x - cursor).abs() <= 1 { '#' } else { '.' });
        position += 1;
    }

    Some(
        display
            .chunks(40)
            .map(|row| row.iter().collect::<String>())
            .collect::<Vec<_>>()
            .join("\n"),
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
        let expected = "
        ##..##..##..##..##..##..##..##..##..##..\n\
        ###...###...###...###...###...###...###.\n\
        ####....####....####....####....####....\n\
        #####.....#####.....#####.....#####.....\n\
        ######......######......######......####\n\
        #######.......#######.......#######.....\n\
        "
        .trim()
        .to_owned();
        assert_eq!(part_two(&input), Some(expected));
    }
}
