use std::collections::HashSet;

fn solve(input: &str, marker_length: usize) -> Option<u32> {
    let chars: Vec<_> = input.chars().collect();
    for (idx, window) in chars.windows(marker_length).enumerate() {
        let set: HashSet<&char> = HashSet::from_iter(window);
        if set.len() == marker_length {
            return Some((idx + marker_length) as u32);
        }
    }
    None
}

pub fn part_one(input: &str) -> Option<u32> {
    solve(input, 4)
}

pub fn part_two(input: &str) -> Option<u32> {
    solve(input, 14)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 6);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 6);
        assert_eq!(part_one(&input), Some(10));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 6);
        assert_eq!(part_two(&input), Some(29));
    }
}
