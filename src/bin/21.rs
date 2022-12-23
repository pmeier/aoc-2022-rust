use std::collections::HashMap;

#[derive(Hash, PartialEq, Eq)]
enum Side {
    Left,
    Right,
}

#[derive(Hash, PartialEq, Eq, Debug)]
enum BinaryOperation {
    Addition,
    Subtraction,
    Multiplication,
    Division,
}

impl BinaryOperation {
    fn from_str(input: &str) -> Option<BinaryOperation> {
        use BinaryOperation::*;
        match input {
            "+" => Some(Addition),
            "-" => Some(Subtraction),
            "*" => Some(Multiplication),
            "/" => Some(Division),
            _ => None,
        }
    }

    fn perform(&self, left: i64, right: i64) -> i64 {
        use BinaryOperation::*;
        match self {
            Addition => left + right,
            Subtraction => left - right,
            Multiplication => left * right,
            Division => left / right,
        }
    }

    fn perform_inverse(&self, result: i64, input: i64, side: Side) -> i64 {
        use BinaryOperation::*;
        match self {
            Addition => result - input,
            Subtraction => match side {
                Side::Left => input - result,
                Side::Right => input + result,
            },
            Multiplication => result / input,
            Division => match side {
                Side::Left => input / result,
                Side::Right => input * result,
            },
        }
    }
}

fn parse(
    input: &str,
) -> (
    HashMap<&str, (&str, BinaryOperation, &str)>,
    HashMap<&str, i64>,
) {
    let mut operations = HashMap::new();
    let mut results = HashMap::new();

    for line in input.lines() {
        let parts = line.split(": ").collect::<Vec<_>>();
        debug_assert!(parts.len() == 2);
        let id = parts[0];
        let operation_or_result = parts[1];

        if let Ok(result) = operation_or_result.parse::<i64>() {
            results.insert(id, result);
        } else {
            let parts = operation_or_result.split_whitespace().collect::<Vec<_>>();
            debug_assert!(parts.len() == 3);
            let operation = BinaryOperation::from_str(parts[1]).unwrap();
            operations.insert(id, (parts[0], operation, parts[2]));
        }
    }

    (operations, results)
}

macro_rules! unwrap_or_continue {
    ($opt: expr) => {
        match $opt {
            Some(v) => v,
            None => {
                continue;
            }
        }
    };
}

fn compute<'a>(
    operations: &mut HashMap<&'a str, (&'a str, BinaryOperation, &'a str)>,
    results: &mut HashMap<&'a str, i64>,
) {
    let mut performed = Vec::new();
    while !operations.is_empty() {
        for (result_id, (left_id, operation, right_id)) in operations.iter() {
            let left = unwrap_or_continue!(results.get(left_id));
            let right = unwrap_or_continue!(results.get(right_id));
            results.insert(result_id, operation.perform(*left, *right));
            performed.push(result_id.clone());
        }

        if performed.is_empty() {
            break;
        }

        for id in performed.iter() {
            operations.remove(id);
        }
        performed.clear();
    }
}

fn get_left_or_right<'a>(
    left_id: &'a str,
    right_id: &'a str,
    results: &'a HashMap<&'a str, i64>,
) -> Option<(&'a str, i64, Side)> {
    let other_id;
    let result;
    let side;
    if let Some(value) = results.get(left_id) {
        other_id = right_id;
        result = *value;
        side = Side::Left;
    } else if let Some(value) = results.get(right_id) {
        other_id = left_id;
        result = *value;
        side = Side::Right;
    } else {
        return None;
    }
    Some((other_id, result, side))
}

fn query<'a>(
    mut id: &'a str,
    mut result: i64,
    target_id: &'a str,
    operations: &mut HashMap<&'a str, (&'a str, BinaryOperation, &'a str)>,
    results: &'a mut HashMap<&'a str, i64>,
) -> Option<i64> {
    while !operations.is_empty() {
        let (left_id, operation, right_id) = operations.remove(id).expect(id);
        let (other_id, input, side) = get_left_or_right(left_id, right_id, results).unwrap();
        id = other_id;
        result = operation.perform_inverse(result, input, side);
        if id == target_id {
            return Some(result);
        }
    }
    None
}

pub fn part_one(input: &str) -> Option<i64> {
    let (mut operations, mut results) = parse(input);
    compute(&mut operations, &mut results);
    results.get("root").copied()
}

pub fn part_two(input: &str) -> Option<i64> {
    let (mut operations, mut results) = parse(input);

    let (left_id, _, right_id) = operations.remove("root").unwrap();

    const TARGET_ID: &str = "humn";
    results.remove(TARGET_ID);

    compute(&mut operations, &mut results);

    let (other_id, result, side) = get_left_or_right(left_id, right_id, &results).unwrap();
    // let other_id = match side {
    //     Side::Left => right_id,
    //     Side::Right => left_id,
    // };
    query(other_id, result, "humn", &mut operations, &mut results)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 21);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 21);
        assert_eq!(part_one(&input), Some(152));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 21);
        assert_eq!(part_two(&input), Some(301));
    }
}
