// https://adventofcode.com/2020/day/1
use itertools::Itertools;

pub fn part_1(filename: &str) -> String {
    std::fs::read_to_string(filename)
        .unwrap()
        .lines()
        .map(|n| n.parse::<i32>().unwrap())
        .combinations(2)
        .find(|n| n.iter().sum::<i32>() == 2020)
        .unwrap()
        .iter()
        .product::<i32>()
        .to_string()
}

pub fn part_2(filename: &str) -> String {
    std::fs::read_to_string(filename)
        .unwrap()
        .lines()
        .map(|n| n.parse::<i32>().unwrap())
        .combinations(3)
        .find(|n| n.iter().sum::<i32>() == 2020)
        .unwrap()
        .iter()
        .product::<i32>()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1_example() {
        assert_eq!(part_1("data/puzzle-2020-01-input-example"), "514579");
    }

    #[test]
    fn test_part_1() {
        assert_eq!(part_1("data/puzzle-2020-01-input"), "731731");
    }

    #[test]
    fn test_part_2_example() {
        assert_eq!(part_2("data/puzzle-2020-01-input-example"), "241861950");
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2("data/puzzle-2020-01-input"), "116115990");
    }
}
