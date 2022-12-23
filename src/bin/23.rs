use std::{
    collections::{HashMap, HashSet, VecDeque},
    hash::Hash,
};

enum Direction {
    North,
    South,
    West,
    East,
}

#[derive(Hash, PartialEq, Eq, Clone, Copy)]
struct Position {
    x: isize,
    y: isize,
}

struct Neighbourhood {
    north: Position,
    north_east: Position,
    north_west: Position,
    south: Position,
    south_east: Position,
    south_west: Position,
    west: Position,
    east: Position,
}

impl Neighbourhood {
    fn from_position(position: &Position) -> Neighbourhood {
        let Position { x, y } = *position;
        Neighbourhood {
            north: Position { x, y: y - 1 },
            north_east: Position { x: x + 1, y: y - 1 },
            north_west: Position { x: x - 1, y: y - 1 },
            south: Position { x, y: y + 1 },
            south_east: Position { x: x + 1, y: y + 1 },
            south_west: Position { x: x - 1, y: y + 1 },
            west: Position { x: x - 1, y },
            east: Position { x: x + 1, y },
        }
    }

    fn all_neighbours(&self) -> Vec<&Position> {
        vec![
            &self.north,
            &self.north_east,
            &self.north_west,
            &self.south,
            &self.south_east,
            &self.south_west,
            &self.west,
            &self.east,
        ]
    }

    fn directional_neighbours(&self, direction: &Direction) -> Vec<&Position> {
        match direction {
            Direction::North => vec![&self.north, &self.north_east, &self.north_west],
            Direction::South => vec![&self.south, &self.south_east, &self.south_west],
            Direction::West => vec![&self.west, &self.north_west, &self.south_west],
            Direction::East => vec![&self.east, &self.north_east, &self.south_east],
        }
    }
}

fn simulate(input: &str, max_rounds: usize) -> (HashSet<Position>, usize) {
    let mut occupied_positions: HashSet<Position> = input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .filter(|(_, c)| c == &'#')
                .map(move |(x, _)| Position {
                    x: x as isize,
                    y: y as isize,
                })
        })
        .collect();
    let mut proposed_positions: HashMap<Position, Vec<Position>> = HashMap::new();
    let mut directions = VecDeque::from_iter(
        [
            Direction::North,
            Direction::South,
            Direction::West,
            Direction::East,
        ]
        .iter(),
    );

    let mut round = 0;
    while round < max_rounds {
        round += 1;

        for position in &occupied_positions {
            let neighbourhood = Neighbourhood::from_position(position);
            if neighbourhood
                .all_neighbours()
                .iter()
                .all(|position| !occupied_positions.contains(position))
            {
                continue;
            }
            for direction in &directions {
                if neighbourhood
                    .directional_neighbours(direction)
                    .iter()
                    .all(|position| !occupied_positions.contains(position))
                {
                    proposed_positions
                        .entry(match direction {
                            Direction::North => neighbourhood.north,
                            Direction::South => neighbourhood.south,
                            Direction::West => neighbourhood.west,
                            Direction::East => neighbourhood.east,
                        })
                        .or_insert_with(Vec::new)
                        .push(*position);
                    break;
                }
            }
        }

        if proposed_positions.is_empty() {
            break;
        }

        for (new_position, old_positions) in &proposed_positions {
            if old_positions.len() == 1 {
                occupied_positions.remove(&old_positions[0]);
                occupied_positions.insert(*new_position);
            }
        }

        proposed_positions.clear();
        directions.rotate_left(1);
    }
    (occupied_positions, round)
}

pub fn part_one(input: &str) -> Option<u32> {
    let (positions, _) = simulate(input, 10);

    let mut x_min = isize::MAX;
    let mut x_max = isize::MIN;
    let mut y_min = isize::MAX;
    let mut y_max = isize::MIN;
    for Position { x, y } in positions.iter() {
        x_min = x_min.min(*x);
        x_max = x_max.max(*x);
        y_min = y_min.min(*y);
        y_max = y_max.max(*y);
    }

    let width = (x_max - x_min + 1) as usize;
    let height = (y_max - y_min + 1) as usize;
    Some((width * height - positions.len()) as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let (_, rounds) = simulate(input, usize::MAX);
    Some(rounds as u32)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 23);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 23);
        assert_eq!(part_one(&input), Some(110));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 23);
        assert_eq!(part_two(&input), Some(20));
    }
}
