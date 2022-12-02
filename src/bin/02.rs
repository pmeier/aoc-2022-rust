use std::str::FromStr;

enum HandShape {
    Rock,
    Paper,
    Scissors,
}

enum Outcome {
    Win,
    Defeat,
    Draw,
}

#[derive(Debug)]
enum StrategyParseFailure {
    NumColumns(usize),
    FirstColumn(String),
    SecondColumn(String),
}

#[derive(Debug)]
struct StrategyParseError(StrategyParseFailure);

enum First {
    A,
    B,
    C,
}

impl FromStr for First {
    type Err = StrategyParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" => Ok(First::A),
            "B" => Ok(First::B),
            "C" => Ok(First::C),
            _ => Err(StrategyParseError(StrategyParseFailure::FirstColumn(s.to_owned()))),
        }
    }
}

enum Second {
    X,
    Y,
    Z,
}

impl FromStr for Second {
    type Err = StrategyParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "X" => Ok(Second::X),
            "Y" => Ok(Second::Y),
            "Z" => Ok(Second::Z),
            _ => Err(StrategyParseError(StrategyParseFailure::SecondColumn(s.to_owned()))),
        }
    }
}

struct Score {
    value: u32,
}

impl Score {
    fn from_player_and_outcome(player: &HandShape, outcome: &Outcome) -> Score {
        let hand_shape_score = match player {
            HandShape::Rock => 1,
            HandShape::Paper => 2,
            HandShape::Scissors => 3,
        };

        let outcome_score = match outcome {
            Outcome::Win => 6,
            Outcome::Defeat => 0,
            Outcome::Draw => 3,
        };

        Score {
            value: hand_shape_score + outcome_score,
        }
    }

    fn parse_strategy(s: &str) -> Result<(First, Second), StrategyParseError> {
        let columns: Vec<&str> = s.split_whitespace().collect();
        if columns.len() != 2 {
            return Err(StrategyParseError(StrategyParseFailure::NumColumns(columns.len())));
        }

        Ok((columns[0].parse::<First>()?, columns[1].parse::<Second>()?))
    }

    fn from_encrypted_strategy(s: &str) -> Result<Score, StrategyParseError> {
        let (first, second) = Score::parse_strategy(s)?;

        let opponent = match first {
            First::A => HandShape::Rock,
            First::B => HandShape::Paper,
            First::C => HandShape::Scissors,
        };

        let player = match second {
            Second::X => HandShape::Rock,
            Second::Y => HandShape::Paper,
            Second::Z => HandShape::Scissors,
        };

        let outcome = match player {
            HandShape::Rock => match opponent {
                HandShape::Rock => Outcome::Draw,
                HandShape::Paper => Outcome::Defeat,
                HandShape::Scissors => Outcome::Win,
            },
            HandShape::Paper => match opponent {
                HandShape::Rock => Outcome::Win,
                HandShape::Paper => Outcome::Draw,
                HandShape::Scissors => Outcome::Defeat,
            },
            HandShape::Scissors => match opponent {
                HandShape::Rock => Outcome::Defeat,
                HandShape::Paper => Outcome::Win,
                HandShape::Scissors => Outcome::Draw,
            },
        };

        Ok(Score::from_player_and_outcome(&player, &outcome))
    }

    fn from_decrypted_strategy(s: &str) -> Result<Score, StrategyParseError> {
        let (first, second) = Score::parse_strategy(s)?;

        let opponent = match first {
            First::A => HandShape::Rock,
            First::B => HandShape::Paper,
            First::C => HandShape::Scissors,
        };

        let outcome = match second {
            Second::X => Outcome::Defeat,
            Second::Y => Outcome::Draw,
            Second::Z => Outcome::Win,
        };

        let player = match outcome {
            Outcome::Defeat => match opponent {
                HandShape::Rock => HandShape::Scissors,
                HandShape::Paper => HandShape::Rock,
                HandShape::Scissors => HandShape::Paper,
            },
            Outcome::Draw => match opponent {
                HandShape::Rock => HandShape::Rock,
                HandShape::Paper => HandShape::Paper,
                HandShape::Scissors => HandShape::Scissors,
            },
            Outcome::Win => match opponent {
                HandShape::Rock => HandShape::Paper,
                HandShape::Paper => HandShape::Scissors,
                HandShape::Scissors => HandShape::Rock,
            },
        };

        Ok(Score::from_player_and_outcome(&player, &outcome))
    }
}

fn solve<P>(input: &str, parse: P) -> Option<u32>
where
    P: Fn(&str) -> Result<Score, StrategyParseError>,
{
    Some(input.lines().map(|line| parse(line).unwrap().value).sum())
}

pub fn part_one(input: &str) -> Option<u32> {
    solve(input, Score::from_encrypted_strategy)
}

pub fn part_two(input: &str) -> Option<u32> {
    solve(input, Score::from_decrypted_strategy)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 2);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_one(&input), Some(15));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_two(&input), Some(12));
    }
}
