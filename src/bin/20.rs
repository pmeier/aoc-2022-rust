fn decrypt(input: &str, key: i64, num_repeats: usize) -> i64 {
    let mut indexed_mixed_file: Vec<(usize, i64)> = input
        .lines()
        .map(|line| line.parse::<i64>().unwrap() * key)
        .enumerate()
        .collect();
    let cycle = indexed_mixed_file.len();
    let shift_cycle = (cycle - 1) as i64;
    for indexed_shift in indexed_mixed_file
        .clone()
        .into_iter()
        .cycle()
        .take(cycle * num_repeats)
    {
        let index = indexed_mixed_file
            .iter()
            .position(|item| item == &indexed_shift)
            .unwrap();
        indexed_mixed_file.remove(index);
        indexed_mixed_file.insert(
            ((index as i64 + indexed_shift.1).rem_euclid(shift_cycle)) as usize,
            indexed_shift,
        );
    }
    let mixed_file: Vec<i64> = indexed_mixed_file
        .into_iter()
        .map(|(_, value)| value)
        .collect();
    let sentinel_index = mixed_file.iter().position(|value| value == &0).unwrap();
    [1_000, 2_000, 3_000]
        .into_iter()
        .map(|offset| {
            let index = (sentinel_index + offset) % cycle;
            mixed_file[index]
        })
        .sum()
}

pub fn part_one(input: &str) -> Option<i64> {
    Some(decrypt(input, 1, 1))
}

pub fn part_two(input: &str) -> Option<i64> {
    Some(decrypt(input, 811_589_153, 10))
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 20);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 20);
        assert_eq!(part_one(&input), Some(3));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 20);
        assert_eq!(part_two(&input), Some(1_623_178_306));
    }
}
