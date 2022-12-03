use std::collections::HashSet;

fn get_priority(char: &char) -> u32 {
    if char <= &'Z' {
        return *char as u32 - 'A' as u32 + 27;
    } else {
        return *char as u32 - 'a' as u32 + 1;
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    return Some(
        input
            .lines()
            .map(|line| {
                let chars: Vec<char> = line.chars().collect();
                let first: HashSet<&char> = chars[..chars.len() / 2].into_iter().collect();
                let second: HashSet<&char> = chars[chars.len() / 2..].into_iter().collect();
                second
                    .into_iter()
                    .map(|c| {
                        if first.get(c) != None {
                            get_priority(&c)
                        } else {
                            0
                        }
                    })
                    .sum::<u32>()
            })
            .sum(),
    );
}

pub fn part_two(input: &str) -> Option<u32> {
    let input: Vec<&str> = input.lines().collect();
    return Some(
        input
            .chunks(3)
            .map(|chunk| {
                let first: HashSet<char> = chunk.get(0).unwrap().chars().into_iter().collect();
                let second: HashSet<char> = chunk.get(1).unwrap().chars().into_iter().collect();
                let third: HashSet<char> = chunk.get(2).unwrap().chars().into_iter().collect();
                third
                    .into_iter()
                    .map(|char| {
                        if first.get(&char) != None && second.get(&char) != None {
                            get_priority(&char)
                        } else {
                            0
                        }
                    })
                    .sum::<u32>()
            })
            .sum(),
    );
}

fn main() {
    let input = &aoc::read_file("inputs", 3);
    aoc::solve!(1, part_one, input);
    aoc::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = aoc::read_file("examples", 3);
        assert_eq!(part_one(&input), Some(157));
    }

    #[test]
    fn test_part_two() {
        let input = aoc::read_file("examples", 3);
        assert_eq!(part_two(&input), Some(70));
    }
}
