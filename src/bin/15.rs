use std::collections::HashSet;

use regex::Regex;

#[derive(PartialEq, Eq, Hash)]
struct Position(isize, isize);

impl Position {
    fn manhattan_distance(&self, other: &Position) -> usize {
        ((self.0 - other.0).abs() + (self.1 - other.1).abs()) as usize
    }
}

struct Sensor {
    position: Position,
    coverage_distance: usize,
}

impl Sensor {
    fn new(position: Position, nearest_beacon_position: &Position) -> Sensor {
        let coverage_distance = position.manhattan_distance(nearest_beacon_position);
        Sensor {
            position,
            coverage_distance,
        }
    }

    fn coverage_interval(&self, y: isize) -> Option<(isize, isize)> {
        let range = self.coverage_distance as isize - (self.position.1 - y).abs();
        if range < 0 {
            None
        } else {
            Some((self.position.0 - range, self.position.0 + range))
        }
    }
}

fn parse(input: &str) -> (Vec<Sensor>, HashSet<Position>) {
    let mut sensors = Vec::new();
    let mut beacon_positions = HashSet::new();

    let pattern: Regex = Regex::new(r"-?\d+").unwrap();
    for line in input.lines() {
        let coordinates: Vec<_> = pattern
            .find_iter(line)
            .map(|value| value.as_str().parse::<isize>().unwrap())
            .collect();
        let sensor_position = Position(coordinates[0], coordinates[1]);
        let beacon_position = Position(coordinates[2], coordinates[3]);
        sensors.push(Sensor::new(sensor_position, &beacon_position));
        beacon_positions.insert(beacon_position);
    }

    (sensors, beacon_positions)
}

fn merge_intervals(mut intervals: Vec<(isize, isize)>) -> Vec<(isize, isize)> {
    intervals.sort();
    let mut iter = intervals.into_iter();
    let mut merged_intervals = Vec::new();
    let mut prev_interval = match iter.next() {
        Some(interval) => interval,
        None => return merged_intervals,
    };
    for cur_interval in iter {
        if cur_interval.0 <= prev_interval.1 {
            prev_interval = (prev_interval.0, prev_interval.1.max(cur_interval.1));
        } else {
            merged_intervals.push(prev_interval);
            prev_interval = cur_interval;
        }
    }
    if merged_intervals.is_empty() || merged_intervals.iter().last().unwrap() != &prev_interval {
        merged_intervals.push(prev_interval);
    }
    merged_intervals
}

fn part_one_core(input: &str, y: isize) -> Option<u32> {
    let (sensors, beacons) = parse(input);
    let beacons_in_scan: u32 = beacons
        .iter()
        .map(|position| (position.1 == y) as u32)
        .sum();

    let intervals = merge_intervals(
        sensors
            .iter()
            .filter_map(|sensor| sensor.coverage_interval(y))
            .collect(),
    );

    Some(
        intervals
            .iter()
            .map(|(start, end)| (end - start + 1) as u32)
            .sum::<u32>()
            - beacons_in_scan,
    )
}

pub fn part_one(input: &str) -> Option<u32> {
    part_one_core(input, 2_000_000)
}

pub fn part_two_core(input: &str, y_max: isize) -> Option<u64> {
    let (sensors, _) = parse(input);
    for y in 0..=y_max {
        let intervals = merge_intervals(
            sensors
                .iter()
                .filter_map(|sensor| sensor.coverage_interval(y))
                .map(|interval| (interval.0.clamp(0, y_max), interval.1.clamp(0, y_max)))
                .collect(),
        );
        if intervals.len() > 1 {
            debug_assert!(intervals.len() == 2);
            let x = intervals[0].1 + 1;
            return Some(x as u64 * 4_000_000 + y as u64);
        }
    }
    None
}

pub fn part_two(input: &str) -> Option<u64> {
    part_two_core(input, 4_000_000)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 15);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 15);
        assert_eq!(part_one_core(&input, 10), Some(26));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 15);
        assert_eq!(part_two_core(&input, 20), Some(56000011));
    }
}
