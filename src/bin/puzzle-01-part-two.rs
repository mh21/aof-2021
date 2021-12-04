// https://adventofcode.com/2021/day/1
use std::fs;

fn main() {
    let filename = "data/puzzle-01-input";
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
    println!("Averaged increases: {}", count);
}
