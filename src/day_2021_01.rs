// https://adventofcode.com/2021/day/1
use std::fs;

pub fn part_1(filename: &str) -> String {
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    let lines = contents.lines();
    let mut first = true;
    let mut count = 0;
    let mut last = 0;
    for line in lines {
        let next = line.parse::<i32>().unwrap();
        if !first && next > last {
            count += 1;
        }
        last = next;
        first = false;
    }
    count.to_string()
}

pub fn part_2(filename: &str) -> String {
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    let lines: Vec<i32> = contents
        .lines()
        .map(|s| s.parse::<i32>().unwrap())
        .collect();
    let mut first = true;
    let mut count = 0;
    let mut last = 0;
    for window in lines.windows(3) {
        let next: i32 = window.iter().sum();
        if !first && next > last {
            count += 1;
        }
        last = next;
        first = false;
    }
    count.to_string()
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
