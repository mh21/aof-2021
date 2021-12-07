// https://adventofcode.com/2021/day/7

pub fn run(filename: &str, map_fuel: fn(i32) -> i32) -> String {
    let crabs: Vec<_> = std::fs::read_to_string(filename)
        .unwrap()
        .lines()
        .next()
        .unwrap()
        .split(',')
        .map(|n| n.parse::<i32>().unwrap())
        .collect();
    let min = *crabs.iter().min().unwrap();
    let max = *crabs.iter().max().unwrap();
    let mut min_pos = 0;
    let mut min_fuel = 0;
    for pos in min..=max {
        let fuel = crabs.iter().map(|n| map_fuel((n - pos).abs())).sum();
        if pos == min || fuel < min_fuel {
            min_pos = pos;
            min_fuel = fuel;
        }
    }
    println!("Minimum position: {}", min_pos);
    min_fuel.to_string()
}

pub fn part_1(filename: &str) -> String {
    run(filename, |n| n)
}

pub fn part_2(filename: &str) -> String {
    run(filename, |n| n * (n + 1) / 2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1_example() {
        assert_eq!(part_1("data/puzzle-2021-07-input-example"), "37");
    }

    #[test]
    fn test_part_1() {
        assert_eq!(part_1("data/puzzle-2021-07-input"), "336131");
    }

    #[test]
    fn test_part_2_example() {
        assert_eq!(part_2("data/puzzle-2021-07-input-example"), "168");
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2("data/puzzle-2021-07-input"), "92676646");
    }
}
