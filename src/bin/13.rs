use std::{cmp::Ordering, fmt::Display};

#[derive(PartialEq, Eq)]
enum Node {
    Internal { children: Vec<Node> },
    Leaf { value: usize },
}

impl Node {
    fn from_str(input: &str) -> Node {
        use Node::*;

        if input.is_empty() {
            Internal {
                children: Vec::new(),
            }
        } else if input.starts_with('[') {
            debug_assert!(input.ends_with(']'));

            let mut children = Vec::new();
            let mut child = String::new();
            let mut bracket_count: usize = 0;

            for c in input.chars().skip(1).take(input.len() - 2) {
                if c == ',' && bracket_count == 0 {
                    children.push(Node::from_str(&child));
                    child = String::new();
                } else {
                    if c == '[' {
                        bracket_count += 1;
                    } else if c == ']' {
                        bracket_count -= 1;
                    }
                    child.push(c);
                }
            }

            debug_assert!(bracket_count == 0);
            children.push(Node::from_str(&child));

            Internal { children }
        } else {
            Leaf {
                value: input.parse().unwrap(),
            }
        }
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        use Node::*;
        match (self, other) {
            (Leaf { value: self_value }, Leaf { value: other_value }) => {
                self_value.partial_cmp(other_value)
            }
            (Leaf { value }, Internal { .. }) => {
                let new_self = Internal {
                    children: vec![Leaf { value: *value }],
                };
                new_self.partial_cmp(other)
            }
            (Internal { .. }, Leaf { value }) => {
                let new_other = Internal {
                    children: vec![Leaf { value: *value }],
                };
                self.partial_cmp(&new_other)
            }
            (
                Internal {
                    children: self_children,
                },
                Internal {
                    children: other_children,
                },
            ) => {
                for (self_child, other_child) in self_children.iter().zip(other_children.iter()) {
                    match self_child.partial_cmp(other_child) {
                        None => return None,
                        Some(ordering) => match ordering {
                            Ordering::Less | Ordering::Greater => return Some(ordering),
                            _ => (),
                        },
                    }
                }
                self_children.len().partial_cmp(&other_children.len())
            }
        }
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use Node::*;
        match self {
            Leaf { value } => write!(f, "{value}"),
            Internal { children } => {
                write!(f, "[")?;
                for (idx, child) in children.iter().enumerate() {
                    child.fmt(f)?;
                    if idx < children.len() - 1 {
                        write!(f, ",")?;
                    }
                }
                write!(f, "]")
            }
        }
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    Some(
        input
            .split("\n\n")
            .enumerate()
            .filter_map(|(idx, pair)| {
                let mut nodes = pair.lines().map(Node::from_str);
                let left = nodes.next().unwrap();
                let right = nodes.next().unwrap();

                if let Ordering::Less = left.cmp(&right) {
                    Some(idx + 1)
                } else {
                    None
                }
            })
            .sum::<usize>() as u32,
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let packets: Vec<Node> = input
        .lines()
        .filter_map(|line| {
            if !line.is_empty() {
                Some(Node::from_str(line))
            } else {
                None
            }
        })
        .collect();
    let divider_packets: Vec<Node> = [2, 6]
        .into_iter()
        .map(|value| Node::Internal {
            children: vec![Node::Internal {
                children: vec![Node::Leaf { value }],
            }],
        })
        .collect();

    let mut all_packets: Vec<&Node> = packets.iter().chain(divider_packets.iter()).collect();
    all_packets.sort();

    Some(
        all_packets
            .iter()
            .enumerate()
            .filter_map(|(idx, packet)| {
                if divider_packets.contains(packet) {
                    Some(idx + 1)
                } else {
                    None
                }
            })
            .product::<usize>() as u32,
    )
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 13);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 13);
        assert_eq!(part_one(&input), Some(13));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 13);
        assert_eq!(part_two(&input), Some(140));
    }
}
