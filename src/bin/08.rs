fn get_grid(input: &str) -> Vec<Vec<u32>> {
    input
        .lines()
        .map(|line| line.chars().flat_map(|c| c.to_digit(10)).collect())
        .collect()
}

const DIRS: [(i32, i32); 4] = [(0, -1), (0, 1), (-1, 0), (1, 0)];

pub fn part_one(input: &str) -> Option<u32> {
    let grid: Vec<Vec<u32>> = get_grid(input);
    let mut visible: u32 = 0;
    let len = grid.len();
    for i in 0..len {
        'outer: for j in 0..len {
            for dir in DIRS {
                for k in 1..len {
                    let ii = i as i32 + dir.0 * k as i32;
                    let jj = j as i32 + dir.1 * k as i32;
                    if ii < 0 || jj < 0 || ii >= len as i32 || jj >= len as i32 {
                        visible += 1;
                        continue 'outer;
                    }
                    if grid[ii as usize][jj as usize] >= grid[i][j] {
                        break;
                    }
                }
            }
        }
    }
    Some(visible)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut max: u32 = 0;
    let grid = get_grid(input);
    let len = grid.len();
    for i in 0..len {
        for j in 0..len {
            let mut score = 1;
            for dir in DIRS {
                let mut visible = 0;
                for k in 1..len {
                    let ii = i as i32 + dir.0 * k as i32;
                    let jj = j as i32 + dir.1 * k as i32;
                    if ii < 0 || jj < 0 || ii >= len as i32 || jj >= len as i32 {
                        break;
                    }
                    visible += 1;
                    if grid[ii as usize][jj as usize] >= grid[i][j] {
                        break;
                    }
                }
                score *= visible;
            }
            if score > max {
                max = score;
            }
        }
    }
    Some(max)
}

fn main() {
    let input = &aoc::read_file("inputs", 8);
    aoc::solve!(1, part_one, input);
    aoc::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = aoc::read_file("examples", 8);
        assert_eq!(part_one(&input), Some(21));
    }

    #[test]
    fn test_part_two() {
        let input = aoc::read_file("examples", 8);
        assert_eq!(part_two(&input), Some(8));
    }
}
