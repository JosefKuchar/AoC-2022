pub fn part_one(input: &str) -> Option<i32> {
    let mut x: i32 = 1;
    let mut cycles = 0;
    let mut strength = 0;
    input.lines().for_each(|line| {
        let mut parts = line.split_whitespace();
        let instruction = parts.next().unwrap();
        let instruction = match instruction {
            "noop" => (1, 0),
            "addx" => (2, parts.next().unwrap().parse::<i32>().unwrap()),
            _ => (0, 0),
        };
        for _ in 0..instruction.0 {
            cycles += 1;
            if ((cycles + 20) % 40) == 0 {
                strength += x * cycles;
            }
        }
        x += instruction.1;
    });
    Some(strength)
}

pub fn part_two(input: &str) -> Option<String> {
    let mut x: i32 = 1;
    let mut cycles = 0;
    let mut buffer: String = String::new();
    input.lines().for_each(|line| {
        let mut parts = line.split_whitespace();
        let instruction = parts.next().unwrap();
        let instruction = match instruction {
            "noop" => (1, 0),
            "addx" => (2, parts.next().unwrap().parse::<i32>().unwrap()),
            _ => (0, 0),
        };
        for _ in 0..instruction.0 {
            cycles += 1;
            if (x - (cycles - 1) % 40).abs() <= 1 {
                buffer.push('#');
            } else {
                buffer.push('.');
            }
            if cycles % 40 == 0 {
                buffer.push('\n');
            }
        }
        x += instruction.1;
    });
    return Some(buffer);
}

fn main() {
    let input = &aoc::read_file("inputs", 10);
    aoc::solve!(1, part_one, input);
    aoc::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = aoc::read_file("examples", 10);
        assert_eq!(part_one(&input), Some(13140));
    }

    #[test]
    fn test_part_two() {
        let input = aoc::read_file("examples", 10);
        assert_eq!(
            part_two(&input),
            Some(String::from(
                "##..##..##..##..##..##..##..##..##..##..\n\
                 ###...###...###...###...###...###...###.\n\
                 ####....####....####....####....####....\n\
                 #####.....#####.....#####.....#####.....\n\
                 ######......######......######......####\n\
                 #######.......#######.......#######.....\n"
            ))
        );
    }
}
