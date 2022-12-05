use std::collections::VecDeque;

fn solve(input: &str, flip: bool) -> Option<String> {
    let mut parts = input.split("\n\n");
    let mut stacks: Vec<VecDeque<char>> = vec![VecDeque::new(); 9];
    parts.next().unwrap().lines().for_each(|line| {
        let mut chars = line.chars().into_iter();
        let mut i = 0;
        chars.next();
        while let Some(c) = chars.next() {
            if c != ' ' && !c.is_numeric() {
                stacks[i].push_back(c);
            }
            chars.nth(2);
            i += 1;
        }
    });
    parts.next().unwrap().lines().for_each(|line| {
        let mut numbers = line.split(' ').flat_map(|num| num.parse::<u32>());
        let count = numbers.next().unwrap();
        let from = numbers.next().unwrap();
        let to = numbers.next().unwrap();
        let mut temp = VecDeque::new();
        for _ in 0..count {
            let char = stacks[from as usize - 1].pop_front();
            if flip {
                temp.push_front(char.unwrap());
            } else {
                temp.push_back(char.unwrap());
            }
        }
        temp.iter()
            .for_each(|c| stacks[to as usize - 1].push_front(*c));
    });
    return Some(
        stacks
            .iter()
            .flat_map(|stack| stack.front())
            .collect::<String>(),
    );
}

pub fn part_one(input: &str) -> Option<String> {
    solve(input, false)
}

pub fn part_two(input: &str) -> Option<String> {
    solve(input, true)
}

fn main() {
    let input = &aoc::read_file("inputs", 5);
    aoc::solve!(1, part_one, input);
    aoc::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = aoc::read_file("examples", 5);
        assert_eq!(part_one(&input), Some("CMZ".to_string()));
    }

    #[test]
    fn test_part_two() {
        let input = aoc::read_file("examples", 5);
        assert_eq!(part_two(&input), Some("MCD".to_string()));
    }
}
