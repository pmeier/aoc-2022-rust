use std::collections::HashSet;

#[derive(Copy, Clone)]
enum Movement {
    Right,
    Left,
    Up,
    Down,
}

#[derive(Copy, Clone, Hash, Eq, PartialEq)]
struct Position(isize, isize);

#[derive(Copy, Clone)]
struct PositionDiff(isize, isize);

impl PositionDiff {
    fn from_movement(movement: Movement) -> PositionDiff {
        use Movement::*;
        let (x, y) = match movement {
            Right => (1, 0),
            Left => (-1, 0),
            Up => (0, 1),
            Down => (0, -1),
        };
        PositionDiff(x, y)
    }
    fn from_positions(position1: Position, position2: Position) -> PositionDiff {
        PositionDiff(position1.0 - position2.0, position1.1 - position2.1)
    }

    fn chebyshev_distance(&self) -> usize {
        std::cmp::max(self.0.unsigned_abs(), self.1.unsigned_abs())
    }
}

#[derive(Copy, Clone)]
struct Knot {
    position: Position,
}

impl Knot {
    fn pull_with(&mut self, position_diff: PositionDiff) {
        self.position = Position(
            self.position.0 + position_diff.0.clamp(-1, 1),
            self.position.1 + position_diff.1.clamp(-1, 1),
        );
    }
}

struct Rope {
    knots: Vec<Knot>,
}

impl Rope {
    fn new(num_knots: usize) -> Rope {
        debug_assert!(num_knots >= 2);
        Rope {
            knots: vec![
                Knot {
                    position: Position(0, 0)
                };
                num_knots
            ],
        }
    }

    fn tail(&self) -> &Knot {
        self.knots.last().unwrap()
    }

    fn move_head(&mut self, movement: Movement) {
        {
            let head = &mut self.knots[0];
            head.pull_with(PositionDiff::from_movement(movement));
        }
        for idx in 0..self.knots.len() - 1 {
            let predecessor = self.knots[idx];
            let mut successor = self.knots[idx + 1];

            let position_diff =
                PositionDiff::from_positions(predecessor.position, successor.position);
            if position_diff.chebyshev_distance() != 2 {
                break;
            }

            successor.pull_with(position_diff);
        }
    }
}

fn solve(input: &str, num_knots: usize) -> u32 {
    let mut rope = Rope::new(num_knots);
    let mut visited_by_tail: HashSet<Position> = HashSet::new();
    visited_by_tail.insert(rope.tail().position);

    for movement in input.lines().flat_map(|line| {
        let instruction: Vec<&str> = line.split_whitespace().collect();
        debug_assert!(instruction.len() == 2);
        use Movement::*;
        std::iter::repeat(match instruction[0] {
            "R" => Right,
            "L" => Left,
            "U" => Up,
            "D" => Down,
            _ => unreachable!(),
        })
        .take(instruction[1].parse().unwrap())
    }) {
        rope.move_head(movement);
        visited_by_tail.insert(rope.tail().position);
    }
    visited_by_tail.len() as u32
}

pub fn part_one(input: &str) -> Option<u32> {
    Some(solve(input, 2))
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(solve(input, 10))
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
        assert_eq!(part_one(&input), Some(13));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 9);
        assert_eq!(part_two(&input), Some(1));
    }
}
