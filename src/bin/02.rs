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

enum FirstColumn {
    A,
    B,
    C,
}

impl FromStr for FirstColumn {
    type Err = StrategyParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use FirstColumn::{A, B, C};

        match s {
            "A" => Ok(A),
            "B" => Ok(B),
            "C" => Ok(C),
            _ => Err(StrategyParseError(StrategyParseFailure::FirstColumn(
                s.to_owned(),
            ))),
        }
    }
}

enum SecondColumn {
    X,
    Y,
    Z,
}

impl FromStr for SecondColumn {
    type Err = StrategyParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use SecondColumn::{X, Y, Z};

        match s {
            "X" => Ok(X),
            "Y" => Ok(Y),
            "Z" => Ok(Z),
            _ => Err(StrategyParseError(StrategyParseFailure::SecondColumn(
                s.to_owned(),
            ))),
        }
    }
}

struct Score {
    value: u32,
}

impl Score {
    fn from_player_and_outcome(player: &HandShape, outcome: &Outcome) -> Score {
        use HandShape::{Paper, Rock, Scissors};
        use Outcome::{Defeat, Draw, Win};

        let hand_shape_score = match player {
            Rock => 1,
            Paper => 2,
            Scissors => 3,
        };

        let outcome_score = match outcome {
            Win => 6,
            Defeat => 0,
            Draw => 3,
        };

        Score {
            value: hand_shape_score + outcome_score,
        }
    }

    fn parse_strategy(s: &str) -> Result<(FirstColumn, SecondColumn), StrategyParseError> {
        let columns: Vec<&str> = s.split_whitespace().collect();
        if columns.len() != 2 {
            return Err(StrategyParseError(StrategyParseFailure::NumColumns(
                columns.len(),
            )));
        }

        Ok((columns[0].parse()?, columns[1].parse()?))
    }

    fn from_encrypted_strategy(s: &str) -> Result<Score, StrategyParseError> {
        use FirstColumn::{A, B, C};
        use HandShape::{Paper, Rock, Scissors};
        use Outcome::{Defeat, Draw, Win};
        use SecondColumn::{X, Y, Z};

        let (first, second) = Score::parse_strategy(s)?;

        let opponent = match first {
            A => Rock,
            B => Paper,
            C => Scissors,
        };

        let player = match second {
            X => Rock,
            Y => Paper,
            Z => Scissors,
        };

        let outcome = match (&player, &opponent) {
            (Rock, Scissors) | (Paper, Rock) | (Scissors, Paper) => Win,
            (Rock, Paper) | (Paper, Scissors) | (Scissors, Rock) => Defeat,
            (Rock, Rock) | (Paper, Paper) | (Scissors, Scissors) => Draw,
        };

        Ok(Score::from_player_and_outcome(&player, &outcome))
    }

    fn from_decrypted_strategy(s: &str) -> Result<Score, StrategyParseError> {
        use FirstColumn::{A, B, C};
        use HandShape::{Paper, Rock, Scissors};
        use Outcome::{Defeat, Draw, Win};
        use SecondColumn::{X, Y, Z};

        let (first, second) = Score::parse_strategy(s)?;

        let opponent = match first {
            A => Rock,
            B => Paper,
            C => Scissors,
        };

        let outcome = match second {
            X => Defeat,
            Y => Draw,
            Z => Win,
        };

        let player = match (&outcome, &opponent) {
            (Win, Scissors) | (Defeat, Paper) | (Draw, Rock) => Rock,
            (Win, Rock) | (Defeat, Scissors) | (Draw, Paper) => Paper,
            (Win, Paper) | (Defeat, Rock) | (Draw, Scissors) => Scissors,
        };

        Ok(Score::from_player_and_outcome(&player, &outcome))
    }
}

fn solve<P>(input: &str, parse: P) -> Result<u32, StrategyParseError>
where
    P: Fn(&str) -> Result<Score, StrategyParseError>,
{
    let mut total = 0;
    for line in input.lines() {
        total += parse(line)?.value;
    }
    Ok(total)
}

pub fn part_one(input: &str) -> Option<u32> {
    Some(solve(input, Score::from_encrypted_strategy).unwrap())
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(solve(input, Score::from_decrypted_strategy).unwrap())
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
