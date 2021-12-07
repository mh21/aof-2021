// https://adventofcode.com/2020/day/1

pub fn part_1(filename: &str) -> String {
    let mut lines: Vec<_> = std::fs::read_to_string(filename)
        .unwrap()
        .lines()
        .map(|n| n.parse::<i32>().unwrap())
        .collect();
    lines.sort_unstable();
    let mut min_index = 0;
    let mut max_index = lines.len() - 1;
    while min_index < max_index {
        while lines[max_index] + lines[min_index] > 2020 {
            max_index -= 1;
        }
        if lines[max_index] + lines[min_index] == 2020 {
            return (lines[max_index] * lines[min_index]).to_string();
        }
        min_index += 1;
    }
    String::from("No matching entries found")
}

pub fn part_2(filename: &str) -> String {
    let mut lines: Vec<_> = std::fs::read_to_string(filename)
        .unwrap()
        .lines()
        .map(|n| n.parse::<i32>().unwrap())
        .collect();
    lines.sort_unstable();
    for base_index in 0..lines.len() - 2 {
        let mut min_index = base_index + 1;
        let mut max_index = lines.len() - 1;
        while min_index < max_index {
            while lines[max_index] + lines[min_index] + lines[base_index] > 2020 {
                max_index -= 1;
            }
            if lines[max_index] + lines[min_index] + lines[base_index] == 2020 {
                return (lines[max_index] * lines[min_index] * lines[base_index]).to_string();
            }
            min_index += 1;
        }
    }
    String::from("No matching entries found")
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
