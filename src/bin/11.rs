use std::collections::VecDeque;

#[derive(Debug, Clone)]
enum Target {
    Number(u64),
    Old,
}

#[derive(Debug, Clone)]
struct Monkey {
    items: VecDeque<u64>,
    operation: String,
    number: Target,
    test: (u64, u64, u64),
    inspected: u64,
}

fn get_monkeys(input: &str) -> Vec<Monkey> {
    input
        .split("\n\n")
        .map(|line| {
            let mut lines = line.lines();
            let items: VecDeque<u64> = lines
                .nth(1)
                .unwrap()
                .split([',', ' '])
                .flat_map(|s| s.parse::<u64>())
                .collect();
            let mut op = lines.next().unwrap().split(' ');
            let operation = op.nth(6).unwrap();
            let number = match op.next().unwrap() {
                "old" => Target::Old,
                n => Target::Number(n.parse().unwrap()),
            };
            let test1: u64 = lines
                .next()
                .unwrap()
                .split(' ')
                .nth(5)
                .unwrap()
                .parse()
                .unwrap();
            let test2: u64 = lines
                .next()
                .unwrap()
                .split(' ')
                .nth(9)
                .unwrap()
                .parse()
                .unwrap();
            let test3: u64 = lines
                .next()
                .unwrap()
                .split(' ')
                .nth(9)
                .unwrap()
                .parse()
                .unwrap();
            Monkey {
                items: items,
                operation: operation.to_string(),
                number: number,
                test: (test1, test2, test3),
                inspected: 0,
            }
        })
        .collect()
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut monkeys: Vec<Monkey> = get_monkeys(input);
    for _ in 0..20 {
        for i in 0..monkeys.len() {
            loop {
                let result;
                {
                    let monkey = &mut monkeys[i];
                    if let Some(item) = monkey.items.pop_front() {
                        monkey.inspected += 1;
                        let number = match monkey.number {
                            Target::Old => item,
                            Target::Number(n) => n,
                        };
                        result = match monkey.operation.as_str() {
                            "*" => item * number,
                            "+" => item + number,
                            _ => unreachable!(),
                        } / 3;
                    } else {
                        break;
                    }
                }
                {
                    let monkey = monkeys[i].clone();
                    if result % monkey.test.0 == 0 {
                        monkeys[monkey.test.1 as usize].items.push_back(result);
                    } else {
                        monkeys[monkey.test.2 as usize].items.push_back(result);
                    }
                }
            }
        }
    }
    let mut inspections: Vec<u64> = monkeys.iter().map(|m| m.inspected).collect();
    inspections.sort();
    Some(inspections[inspections.len() - 1] * inspections[inspections.len() - 2])
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut monkeys = get_monkeys(input);
    let lcm = monkeys
        .iter()
        .map(|m| m.test.0)
        .reduce(|a, b| a * b)
        .unwrap();
    for _ in 0..10000 {
        for i in 0..monkeys.len() {
            loop {
                let result;
                {
                    let monkey = &mut monkeys[i];
                    if let Some(item) = monkey.items.pop_front() {
                        monkey.inspected += 1;
                        let number = match monkey.number {
                            Target::Old => item,
                            Target::Number(n) => n,
                        };
                        result = match monkey.operation.as_str() {
                            "*" => item * number,
                            "+" => item + number,
                            _ => unreachable!(),
                        } % lcm;
                    } else {
                        break;
                    }
                }
                {
                    let monkey = monkeys[i].clone();
                    if result % monkey.test.0 == 0 {
                        monkeys[monkey.test.1 as usize].items.push_back(result);
                    } else {
                        monkeys[monkey.test.2 as usize].items.push_back(result);
                    }
                }
            }
        }
    }
    let mut inspections: Vec<u64> = monkeys.iter().map(|m| m.inspected).collect();
    inspections.sort();
    Some(inspections[inspections.len() - 1] * inspections[inspections.len() - 2])
}

fn main() {
    let input = &aoc::read_file("inputs", 11);
    aoc::solve!(1, part_one, input);
    aoc::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = aoc::read_file("examples", 11);
        assert_eq!(part_one(&input), Some(10605));
    }

    #[test]
    fn test_part_two() {
        let input = aoc::read_file("examples", 11);
        assert_eq!(part_two(&input), Some(2713310158));
    }
}
