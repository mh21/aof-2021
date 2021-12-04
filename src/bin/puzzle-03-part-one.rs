// https://adventofcode.com/2021/day/3
use std::fs;

fn main() {
    let filename = "data/puzzle-03-input";
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    let numbers: Vec<Vec<_>> = contents
        .lines()
        .map(|s| s.chars().map(|c| c.to_digit(10).unwrap() as i32).collect())
        .collect();
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
    println!("Power: {}", gamma * epsilon);
}
