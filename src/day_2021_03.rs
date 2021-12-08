// https://adventofcode.com/2021/day/3

fn decimal(binary: &[i32]) -> i32 {
    return binary
        .iter()
        .enumerate()
        .map(|(pos, i)| i * 2i32.pow((binary.len() - pos - 1) as u32))
        .sum();
}

fn filter(numbers: &[Vec<i32>], filter_fn: fn(usize, usize) -> bool) -> &Vec<i32> {
    let measurements = numbers[0].len();
    let mut filtered_numbers: Vec<&Vec<i32>> = numbers.iter().collect();
    for measurement in 0..measurements {
        let selected = filter_fn(
            filtered_numbers
                .iter()
                .filter(|ms| ms[measurement] == 1)
                .count(),
            filtered_numbers.len(),
        ) as i32;
        filtered_numbers = filtered_numbers
            .iter()
            .filter(|ms| ms[measurement] == selected)
            .copied()
            .collect();
        if filtered_numbers.len() <= 1 {
            break;
        }
    }
    filtered_numbers[0]
}

fn read(filename: &str) -> Vec<Vec<i32>> {
    std::fs::read_to_string(filename)
        .unwrap()
        .lines()
        .map(|s| s.chars().map(|c| c.to_digit(10).unwrap() as i32).collect())
        .collect()
}

pub fn part_1(filename: &str) -> String {
    let numbers = read(filename);
    let number_count = numbers.len();
    let measurements = numbers[0].len();
    println!("Number of measurements: {}", measurements);
    let gamma: i32 = (0..measurements)
        .map(|i| numbers.iter().filter(|ms| ms[i] == 1).count()) // number of "1" bits
        .map(|i| (i > number_count / 2) as i32) // most common bit as int
        .enumerate()
        .map(|(pos, i)| i * 2i32.pow((measurements - pos - 1) as u32))
        .sum();
    let epsilon = 2i32.pow(measurements as u32) - 1 - gamma;
    println!("Gamma: {}", gamma);
    println!("Epsilon: {}", epsilon);
    (gamma * epsilon).to_string()
}

pub fn part_2(filename: &str) -> String {
    let numbers = read(filename);
    let oxygen = decimal(filter(&numbers, |c, s| c >= (s + 1) / 2));
    let co2_scrubber = decimal(filter(&numbers, |c, s| c < (s + 1) / 2));
    println!("Oxygen: {}", oxygen);
    println!("CO2 scrubber: {}", co2_scrubber);
    (oxygen * co2_scrubber).to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1_example() {
        assert_eq!(part_1("data/puzzle-2021-03-input-example"), "198");
    }

    #[test]
    fn test_part_1() {
        assert_eq!(part_1("data/puzzle-2021-03-input"), "3633500");
    }

    #[test]
    fn test_part_2_example() {
        assert_eq!(part_2("data/puzzle-2021-03-input-example"), "230");
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2("data/puzzle-2021-03-input"), "4550283");
    }
}
