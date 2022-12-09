use std::collections::HashSet;

fn get_step(num: i32) -> i32 {
    if num > 0 {
        1
    } else if num < 0 {
        -1
    } else {
        0
    }
}

fn solve(input: &str, len: usize) -> Option<u32> {
    let mut visited: HashSet<(i32, i32)> = HashSet::new();
    let mut rope: Vec<(i32, i32)> = vec![(0, 0); len];
    visited.insert(rope[len - 1]);
    input.lines().filter(|line| *line != "").for_each(|line| {
        let mut parts = line.split(' ');
        let dir = parts.next().unwrap();
        let steps = parts.next().unwrap().parse::<u32>().unwrap();
        let dir = match dir {
            "L" => (-1, 0),
            "R" => (1, 0),
            "D" => (0, -1),
            "U" => (0, 1),
            _ => unreachable!(),
        };
        for _ in 0..steps {
            rope[0].0 += dir.0;
            rope[0].1 += dir.1;
            let mut prev = rope[0];
            for i in 1..len {
                let diff = (prev.0 - rope[i].0, prev.1 - rope[i].1);
                if diff.0.abs() > 1 || diff.1.abs() > 1 {
                    rope[i].0 += get_step(diff.0);
                    rope[i].1 += get_step(diff.1);
                }
                prev = rope[i];
            }
            visited.insert(rope[len - 1]);
        }
    });
    Some(visited.len() as u32)
}

pub fn part_one(input: &str) -> Option<u32> {
    solve(input, 2)
}

pub fn part_two(input: &str) -> Option<u32> {
    solve(input, 10)
}

fn main() {
    let input = &aoc::read_file("inputs", 9);
    aoc::solve!(1, part_one, input);
    aoc::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = aoc::read_file("examples", 9);
        assert_eq!(part_one(&input), Some(13));
    }

    #[test]
    fn test_part_two() {
        let input = aoc::read_file("examples", 9);
        assert_eq!(part_two(&input), Some(1));
    }
}
