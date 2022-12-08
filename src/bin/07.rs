use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::{Rc, Weak};

struct Node {
    pub value: u64,
    pub parent: Option<Weak<RefCell<Node>>>,
    pub children: Vec<Rc<RefCell<Node>>>,
}

impl Node {
    fn new() -> Node {
        Node {
            value: 0,
            parent: None,
            children: vec![],
        }
    }
}

struct UsedDiskSpaceIterator {
    stack: VecDeque<Rc<RefCell<Node>>>,
}

impl UsedDiskSpaceIterator {
    fn from_file_system(file_system: &FileSystem) -> UsedDiskSpaceIterator {
        let mut stack = VecDeque::new();
        stack.push_back(Rc::clone(&file_system.root));
        UsedDiskSpaceIterator { stack }
    }
}

impl Iterator for UsedDiskSpaceIterator {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        match self.stack.pop_front() {
            Some(node) => {
                for child in node.borrow().children.iter() {
                    self.stack.push_back(Rc::clone(child));
                }
                Some(node.borrow().value)
            }
            None => None,
        }
    }
}

struct FileSystem {
    root: Rc<RefCell<Node>>,
}

impl FileSystem {
    fn from_history(history: &str) -> FileSystem {
        let root = Rc::new(RefCell::new(Node::new()));
        let mut node = Rc::clone(&root);

        let mut lines = history.lines().skip(1);
        let mut line = lines.next();

        while line.is_some() {
            let mut parts = line.unwrap().split(' ').skip(1);
            match parts.next().unwrap() {
                "cd" => {
                    match parts.next().unwrap() {
                        ".." => {
                            if let Some(parent) = node.borrow().parent.as_ref() {
                                parent.upgrade().unwrap().borrow_mut().value += node.borrow().value;
                            }
                            node = Rc::clone(
                                &Rc::clone(&node)
                                    .borrow()
                                    .parent
                                    .as_ref()
                                    .unwrap()
                                    .upgrade()
                                    .unwrap(),
                            );
                        }
                        _ => {
                            let child = Rc::new(RefCell::new(Node::new()));
                            node.borrow_mut().children.push(Rc::clone(&child));
                            child.borrow_mut().parent = Some(Rc::downgrade(&node));
                            node = child;
                        }
                    }
                    line = lines.next()
                }
                "ls" => {
                    line = lines.next();
                    while line.is_some() {
                        let first_part = line.unwrap().split(' ').next().unwrap();
                        if first_part == "$" {
                            break;
                        } else if first_part != "dir" {
                            node.borrow_mut().value += first_part.parse::<u64>().unwrap();
                        }
                        line = lines.next();
                    }
                }
                _ => unreachable!(),
            };
        }
        loop {
            let parent = match node.borrow().parent.as_ref() {
                Some(weak_parent) => {
                    let parent = weak_parent.upgrade().unwrap();
                    parent.borrow_mut().value += node.borrow().value;
                    parent
                }
                None => break,
            };
            node = parent;
        }

        FileSystem { root }
    }

    fn used_disk_space_iter(&self) -> UsedDiskSpaceIterator {
        UsedDiskSpaceIterator::from_file_system(self)
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    Some(
        FileSystem::from_history(input)
            .used_disk_space_iter()
            .filter(|uds| uds < &100000)
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u64> {
    let file_system = FileSystem::from_history(input);
    let used_disk_space = file_system.root.borrow().value;

    let min_disk_space_to_free = used_disk_space - 40_000_000;

    Some(
        FileSystem::from_history(input)
            .used_disk_space_iter()
            .filter(|uds| uds >= &min_disk_space_to_free)
            .min()
            .unwrap(),
    )
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 7);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 7);
        assert_eq!(part_one(&input), None);
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 7);
        assert_eq!(part_two(&input), None);
    }
}
