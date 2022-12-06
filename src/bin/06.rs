use std::collections::HashSet;

pub fn solve(input: &str, count: usize) -> Option<u32> {
    let chars = input.chars().collect::<Vec<char>>();
    let mut windows = chars.windows(count);
    let mut i = count as u32;
    while let Some(w) = windows.next() {
        let set: HashSet<&char> = HashSet::from_iter(w);
        if set.len() == count {
            break;
        }
        i += 1;
    }
    return Some(i);
}

pub fn part_one(input: &str) -> Option<u32> {
    solve(input, 4)
}

pub fn part_two(input: &str) -> Option<u32> {
    solve(input, 14)
}

fn main() {
    let input = &aoc::read_file("inputs", 6);
    aoc::solve!(1, part_one, input);
    aoc::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        assert_eq!(part_one("bvwbjplbgvbhsrlpgdmjqwftvncz"), Some(5));
        assert_eq!(part_one("nppdvjthqldpwncqszvftbrmjlhg"), Some(6));
        assert_eq!(part_one("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), Some(10));
        assert_eq!(part_one("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), Some(11));
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two("mjqjpqmgbljsphdztnvjfqwrcgsmlb"), Some(19));
        assert_eq!(part_two("bvwbjplbgvbhsrlpgdmjqwftvncz"), Some(23));
        assert_eq!(part_two("nppdvjthqldpwncqszvftbrmjlhg"), Some(23));
        assert_eq!(part_two("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), Some(29));
        assert_eq!(part_two("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), Some(26));
    }
}
