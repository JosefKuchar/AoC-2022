use std::{
    cell::RefCell,
    collections::HashMap,
    rc::{Rc, Weak},
};

#[derive(Debug, PartialEq)]
enum NodeType {
    Dir(String, u32),
    File(String, u32),
}

#[derive(Debug)]
struct Node {
    node_type: NodeType,
    parent: RefCell<Weak<Node>>,
    children: RefCell<HashMap<String, Rc<Node>>>,
}

impl Node {
    fn new() -> Node {
        return Node {
            node_type: NodeType::Dir(String::from("/"), 0),
            children: RefCell::new(HashMap::new()),
            parent: RefCell::new(Weak::new()),
        };
    }
}

fn sum_dirs(root: Rc<Node>, sums: &mut Vec<u32>) -> u32 {
    let mut sum = 0;
    root.children.borrow().iter().for_each(|(_, v)| {
        sum += match v.node_type {
            NodeType::Dir(_, _) => sum_dirs(Rc::clone(v), sums),
            NodeType::File(_, size) => size,
        };
    });
    match root.node_type {
        NodeType::Dir(_, _) => sums.push(sum),
        NodeType::File(_, _) => {}
    };
    return sum;
}

pub fn get_sums(input: &str) -> Vec<u32> {
    let root = Rc::new(Node::new());
    let mut current = Rc::clone(&root);

    input.lines().for_each(|line| {
        let command: Vec<&str> = line.split(" ").collect();
        match command[0] {
            "$" => {
                match command[1] {
                    "cd" => {
                        let path = command[2];
                        match path {
                            ".." => {
                                let current_c = Rc::clone(&current);
                                current = Rc::clone(&current_c.parent.borrow().upgrade().unwrap());
                            }
                            "/" => {
                                current = Rc::clone(&root);
                            }
                            _ => {
                                let current_c = Rc::clone(&current);
                                current = Rc::clone(current_c.children.borrow().get(path).unwrap());
                            }
                        }
                    }
                    "ls" => {}
                    _ => unreachable!(),
                };
            }
            "dir" => {
                let name = command[1];
                let node = Rc::new(Node {
                    node_type: NodeType::Dir(name.to_string(), 0),
                    children: RefCell::new(HashMap::new()),
                    parent: RefCell::new(Rc::downgrade(&current)),
                });
                current
                    .children
                    .borrow_mut()
                    .insert(name.to_string(), Rc::clone(&node));
            }
            num => {
                let size = num.parse::<u32>().unwrap();
                let name = command[1];
                let node = Rc::new(Node {
                    node_type: NodeType::File(name.to_string(), size),
                    children: RefCell::new(HashMap::new()),
                    parent: RefCell::new(Rc::downgrade(&current)),
                });
                current
                    .children
                    .borrow_mut()
                    .insert(name.to_string(), Rc::clone(&node));
            }
        };
    });
    let mut sums = Vec::new();
    sum_dirs(root, &mut sums);
    return sums;
}

pub fn part_one(input: &str) -> Option<u32> {
    let sums = get_sums(input);
    Some(sums.iter().filter(|sum| **sum <= 100000).sum::<u32>())
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut sums = get_sums(input);
    sums.sort();
    let total = sums[sums.len() - 1];
    Some(
        *sums
            .iter()
            .find(|size| 70000000 - total as isize + **size as isize >= 30000000)
            .unwrap(),
    )
}

fn main() {
    let input = &aoc::read_file("inputs", 7);
    aoc::solve!(1, part_one, input);
    aoc::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = aoc::read_file("examples", 7);
        assert_eq!(part_one(&input), Some(95437));
    }

    #[test]
    fn test_part_two() {
        let input = aoc::read_file("examples", 7);
        assert_eq!(part_two(&input), Some(24933642));
    }
}
