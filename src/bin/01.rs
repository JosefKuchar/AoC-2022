fn get_calories(input: &str) -> Vec<u32> {
    let mut calories: Vec<u32> = input
        .split("\n\n")
        .map(|group| {
            group
                .split("\n")
                .map(|num| num.parse::<u32>().unwrap_or(0))
                .sum()
        })
        .collect();
    calories.sort_unstable();
    calories
}

pub fn part_one(input: &str) -> Option<u32> {
    let calories = get_calories(input);
    Some(calories[calories.len() - 1])
}

pub fn part_two(input: &str) -> Option<u32> {
    let calories = get_calories(input);
    let len = calories.len();
    Some(calories[len - 1] + calories[len - 2] + calories[len - 3])
}

fn main() {
    let input = &aoc::read_file("inputs", 1);
    aoc::solve!(1, part_one, &input);
    aoc::solve!(2, part_two, &input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = aoc::read_file("examples", 1);
        assert_eq!(part_one(&input), Some(24000));
    }

    #[test]
    fn test_part_two() {
        let input = aoc::read_file("examples", 1);
        assert_eq!(part_two(&input), Some(45000));
    }
}
