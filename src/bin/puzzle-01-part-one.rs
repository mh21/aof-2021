// https://adventofcode.com/2021/day/1
use std::fs;

fn main() {
    let filename = "data/puzzle-01-input";
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
    println!("Increases: {}", count);
}
