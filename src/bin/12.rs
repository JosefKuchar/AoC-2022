use pathfinding::prelude::dijkstra;

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Pos(i32, i32);

const DIRS: [(i32, i32); 4] = [(0, -1), (0, 1), (-1, 0), (1, 0)];

fn transform(c: char) -> i32 {
    (match c {
        'E' => 'z',
        'S' => 'a',
        _ => c,
    }) as i32
}

impl Pos {
    fn successors(&self, map: &Vec<Vec<char>>) -> Vec<(Pos, usize)> {
        let &Pos(i, j) = self;
        let mut result = Vec::new();
        for dir in DIRS {
            let ii = i + dir.0;
            let jj = j + dir.1;
            if ii < 0 || jj < 0 || ii >= map.len() as i32 || jj >= map[0].len() as i32 {
                continue;
            }
            if transform(map[ii as usize][jj as usize]) - transform(map[i as usize][j as usize])
                <= 1
            {
                result.push((Pos(ii, jj), 1));
            }
        }
        result
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let map: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let mut start = Pos(0, 0);
    let mut end = Pos(0, 0);
    for i in 0..map.len() {
        for j in 0..map[i].len() {
            if map[i][j] == 'S' {
                start = Pos(i as i32, j as i32);
            }
            if map[i][j] == 'E' {
                end = Pos(i as i32, j as i32);
            }
        }
    }
    Some(
        dijkstra(&start, |p| p.successors(&map), |p| *p == end)
            .unwrap()
            .1 as u32,
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let map: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let mut end = Pos(0, 0);
    for i in 0..map.len() {
        for j in 0..map[i].len() {
            if map[i][j] == 'E' {
                end = Pos(i as i32, j as i32);
            }
        }
    }
    let mut min = std::u32::MAX;
    for i in 0..map.len() {
        for j in 0..map[i].len() {
            if map[i][j] == 'a' {
                let result = dijkstra(
                    &Pos(i as i32, j as i32),
                    |p| p.successors(&map),
                    |p| *p == end,
                );
                if let Some(result) = result {
                    if (result.1 as u32) < min {
                        min = result.1 as u32;
                    }
                }
            }
        }
    }
    Some(min as u32)
}

fn main() {
    let input = &aoc::read_file("inputs", 12);
    aoc::solve!(1, part_one, input);
    aoc::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = aoc::read_file("examples", 12);
        assert_eq!(part_one(&input), Some(31));
    }

    #[test]
    fn test_part_two() {
        let input = aoc::read_file("examples", 12);
        assert_eq!(part_two(&input), None);
    }
}
