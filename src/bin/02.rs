#[derive(PartialEq)]
enum Shape {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

#[derive(PartialEq)]
enum Outcome {
    Win = 6,
    Draw = 3,
    Lose = 0,
}

pub fn part_one(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .map(|line| {
                let mut chars = line.chars();
                let oponnent: Shape = match chars.nth(0) {
                    Some('A') => Shape::Rock,
                    Some('B') => Shape::Paper,
                    Some('C') => Shape::Scissors,
                    _ => panic!("Invalid type"),
                };
                let player: Shape = match chars.nth(1) {
                    Some('X') => Shape::Rock,
                    Some('Y') => Shape::Paper,
                    Some('Z') => Shape::Scissors,
                    _ => panic!("Invalid type"),
                };
                let res = if player == oponnent {
                    Outcome::Draw
                } else if (player == Shape::Rock && oponnent == Shape::Scissors)
                    || (player == Shape::Paper && oponnent == Shape::Rock)
                    || (player == Shape::Scissors && oponnent == Shape::Paper)
                {
                    Outcome::Win
                } else {
                    Outcome::Lose
                };
                return res as u32 + player as u32;
            })
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .map(|line| {
                let mut chars = line.chars();
                let shape: Shape = match chars.nth(0) {
                    Some('A') => Shape::Rock,
                    Some('B') => Shape::Paper,
                    Some('C') => Shape::Scissors,
                    _ => panic!("Invalid type"),
                };
                let outcome: Outcome = match chars.nth(1) {
                    Some('X') => Outcome::Lose,
                    Some('Y') => Outcome::Draw,
                    Some('Z') => Outcome::Win,
                    _ => panic!("Invalid type"),
                };
                let res = match outcome {
                    Outcome::Draw => shape,
                    Outcome::Win => match shape {
                        Shape::Rock => Shape::Paper,
                        Shape::Paper => Shape::Scissors,
                        Shape::Scissors => Shape::Rock,
                    },
                    Outcome::Lose => match shape {
                        Shape::Rock => Shape::Scissors,
                        Shape::Paper => Shape::Rock,
                        Shape::Scissors => Shape::Paper,
                    },
                };
                return res as u32 + outcome as u32;
            })
            .sum(),
    )
}

fn main() {
    let input = &aoc::read_file("inputs", 2);
    aoc::solve!(1, part_one, input);
    aoc::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = aoc::read_file("examples", 2);
        assert_eq!(part_one(&input), Some(15));
    }

    #[test]
    fn test_part_two() {
        let input = aoc::read_file("examples", 2);
        assert_eq!(part_two(&input), Some(12));
    }
}
