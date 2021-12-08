// https://adventofcode.com/2021/day/1
use std::fs;

fn read(filename: &str) -> Vec<i32> {
    fs::read_to_string(filename)
        .unwrap()
        .lines()
        .map(|s| s.parse().unwrap())
        .collect()
}
pub fn part_1(filename: &str) -> String {
    read(filename)
        .windows(2)
        .filter(|n| n[1] > n[0])
        .count()
        .to_string()
}

pub fn part_2(filename: &str) -> String {
    read(filename)
        .windows(4)
        .filter(|n| n[1..4].iter().sum::<i32>() > n[0..3].iter().sum::<i32>())
        .count()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1_example() {
        assert_eq!(part_1("data/puzzle-2021-01-input-example"), "7");
    }

    #[test]
    fn test_part_1() {
        assert_eq!(part_1("data/puzzle-2021-01-input"), "1711");
    }

    #[test]
    fn test_part_2_example() {
        assert_eq!(part_2("data/puzzle-2021-01-input-example"), "5");
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2("data/puzzle-2021-01-input"), "1743");
    }
}
