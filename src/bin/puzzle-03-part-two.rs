// https://adventofcode.com/2021/day/3
use std::fs;

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

fn main() {
    let filename = "data/puzzle-03-input";
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    let numbers: Vec<Vec<_>> = contents
        .lines()
        .map(|s| s.chars().map(|c| c.to_digit(10).unwrap() as i32).collect())
        .collect();

    let oxygen = decimal(filter(&numbers, |c, s| c >= (s + 1) / 2));
    let co2_scrubber = decimal(filter(&numbers, |c, s| c < (s + 1) / 2));
    println!("Oxygen: {}", oxygen);
    println!("CO2 scrubber: {}", co2_scrubber);
    println!("Life support rating: {}", oxygen * co2_scrubber);
}
