fn part_one_checker(a1: u32, a2: u32, b1: u32, b2: u32) -> bool {
    return (a1 <= b1 && a2 >= b2) || (b1 <= a1 && b2 >= a2);
}

fn part_two_checker(a1: u32, a2: u32, b1: u32, b2: u32) -> bool {
    return (a2 >= b1 && a2 <= b2)
        || (a1 >= b1 && a1 <= b2)
        || (b2 >= a1 && b2 <= a2)
        || (b1 >= a1 && b1 <= a2);
}

pub fn solve(input: &str, checker: &dyn Fn(u32, u32, u32, u32) -> bool) -> Option<u32> {
    Some(
        input
            .lines()
            .filter(|line| {
                let mut nums = line
                    .split([',', '-'])
                    .map(|str| str.parse::<u32>().unwrap_or(0));
                let a1 = nums.next().unwrap();
                let a2 = nums.next().unwrap();
                let b1 = nums.next().unwrap();
                let b2 = nums.next().unwrap();
                return checker(a1, a2, b1, b2);
            })
            .count() as u32,
    )
}

pub fn part_one(input: &str) -> Option<u32> {
    solve(input, &part_one_checker)
}

pub fn part_two(input: &str) -> Option<u32> {
    solve(input, &part_two_checker)
}

fn main() {
    let input = &aoc::read_file("inputs", 4);
    aoc::solve!(1, part_one, input);
    aoc::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = aoc::read_file("examples", 4);
        assert_eq!(part_one(&input), Some(2));
    }

    #[test]
    fn test_part_two() {
        let input = aoc::read_file("examples", 4);
        assert_eq!(part_two(&input), Some(4));
    }
}
