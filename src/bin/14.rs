use regex::Regex;
use std::fmt::Display;
use std::ops::RangeInclusive;

#[derive(Copy, Clone, PartialEq, Eq)]
enum Element {
    Air,
    Rock,
    Sand,
}

impl Element {
    fn to_char(self) -> char {
        use Element::*;
        match self {
            Air => '.',
            Rock => '#',
            Sand => 'o',
        }
    }
}

enum Axis {
    X(usize),
    Y(usize),
}
struct CoordinateRangeIterator {
    axis: Axis,
    range: RangeInclusive<usize>,
}

impl CoordinateRangeIterator {
    fn new(start: (usize, usize), end: (usize, usize)) -> Option<Self> {
        let axis;
        let range;
        if start.0 == end.0 {
            axis = Axis::X(start.0);
            range = start.1.min(end.1)..=start.1.max(end.1);
        } else if start.1 == end.1 {
            axis = Axis::Y(start.1);
            range = start.0.min(end.0)..=start.0.max(end.0);
        } else {
            return None;
        }
        Some(Self { axis, range })
    }
}

impl Iterator for CoordinateRangeIterator {
    type Item = (usize, usize);

    fn next(&mut self) -> Option<Self::Item> {
        match (self.range.next(), &self.axis) {
            (Some(y), Axis::X(x)) => Some((*x, y)),
            (Some(x), Axis::Y(y)) => Some((x, *y)),
            _ => None,
        }
    }
}

const WIDTH: usize = 1001;

struct Cave {
    grid: Vec<[Element; WIDTH]>,
}

impl Cave {
    fn from_str(input: &str, has_floor: bool) -> Cave {
        let pattern = Regex::new(r"\d+,\d+").unwrap();
        let mut height = 0;
        let vertices: Vec<Vec<(usize, usize)>> = input
            .lines()
            .map(|line| {
                pattern
                    .find_iter(line)
                    .map(|vertex| {
                        let values: Vec<_> = vertex
                            .as_str()
                            .split(',')
                            .map(|value| value.parse::<usize>().unwrap())
                            .collect();
                        let x = values[0];
                        let y = values[1];
                        height = height.max(y + 1);
                        (x, y)
                    })
                    .collect()
            })
            .collect();

        let mut grid = vec![[Element::Air; WIDTH]; height];

        for structure_vertices in vertices {
            for window in structure_vertices.windows(2) {
                for (x, y) in CoordinateRangeIterator::new(window[0], window[1]).unwrap() {
                    grid[y][x] = Element::Rock;
                }
            }
        }

        if has_floor {
            grid.push([Element::Air; WIDTH]);
            grid.push([Element::Rock; WIDTH]);
        }

        Cave { grid }
    }

    fn drop_sand(&mut self, origin: (usize, usize)) -> bool {
        let (mut x, mut y) = origin;

        if let Element::Sand = self.grid[y][x] {
            return false;
        }

        let y_max = self.grid.len() - 1;
        let x_max = self.grid[0].len() - 1;
        while y < y_max {
            if let Element::Air = self.grid[y + 1][x] {
                y += 1;
            } else if x == 0 || x == x_max {
                panic!("Sand is falling horizontally off the grid!");
            } else if let Element::Air = self.grid[y + 1][x - 1] {
                x -= 1;
                y += 1;
            } else if let Element::Air = self.grid[y + 1][x + 1] {
                x += 1;
                y += 1;
            } else {
                self.grid[y][x] = Element::Sand;
                return true;
            }
        }
        false
    }
}

impl Display for Cave {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fn trim_grid<T, P>(mut grid: Vec<Vec<T>>, predicate: P) -> Vec<Vec<T>>
        where
            P: Fn(&T) -> bool,
        {
            let (mut front, mut back) = (usize::MAX, usize::MAX);
            for row in &grid {
                front = front.min(row.iter().take_while(|x| predicate(x)).count());
                back = back.min(row.iter().rev().take_while(|x| predicate(x)).count());
            }
            for row in &mut grid {
                row.truncate(row.len() - back);
                row.drain(..front);
            }
            grid
        }

        let grid: Vec<Vec<Element>> =
            trim_grid(self.grid.iter().map(|row| row.to_vec()).collect(), |item| {
                matches!(item, Element::Air)
            });
        f.write_str(
            &grid
                .into_iter()
                .map(|row| row.into_iter().map(Element::to_char).collect::<String>())
                .collect::<Vec<_>>()
                .join("\n"),
        )
    }
}

fn solve(input: &str, has_floor: bool) -> Option<u32> {
    let mut cave = Cave::from_str(input, has_floor);
    let mut num_sand = 0;
    while cave.drop_sand((500, 0)) {
        num_sand += 1;
    }
    Some(num_sand)
}

pub fn part_one(input: &str) -> Option<u32> {
    solve(input, false)
}

pub fn part_two(input: &str) -> Option<u32> {
    solve(input, true)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 14);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 14);
        assert_eq!(part_one(&input), Some(24));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 14);
        assert_eq!(part_two(&input), Some(93));
    }
}
