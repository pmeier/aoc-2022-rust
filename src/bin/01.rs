fn top_k_total_calories(input: &str, k: usize) -> u32 {
    let mut total_calories = input
        .split("\n\n")
        .into_iter()
        .map(|input_per_elf| {
            input_per_elf
                .lines()
                .map(|line| {
                    line.parse::<u32>()
                        .expect(&format!("Can't parse '{line}' into an u32!"))
                })
                .sum::<u32>()
        })
        .collect::<Vec<_>>();
    total_calories.sort();
    total_calories.iter().rev().take(k).sum()
}

pub fn part_one(input: &str) -> Option<u32> {
    Some(top_k_total_calories(input, 1))
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(top_k_total_calories(input, 3))
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 1);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_one(&input), Some(24000));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_two(&input), Some(45000));
    }
}
