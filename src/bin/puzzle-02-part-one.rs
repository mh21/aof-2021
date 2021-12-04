// https://adventofcode.com/2021/day/2
use std::fs;

struct Movement {
    direction: String,
    units: i32,
}

fn main() {
    let filename = "data/puzzle-02-input";
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    let movements = contents.lines().map(|s| {
        let mut parts = s.split(' ');
        Movement {
            direction: parts.next().unwrap().to_string(),
            units: parts.next().unwrap().parse::<i32>().unwrap(),
        }
    });
    let mut horizontal = 0;
    let mut depth = 0;
    for movement in movements {
        match movement.direction.as_str() {
            "down" => depth += movement.units,
            "up" => depth -= movement.units,
            "forward" => horizontal += movement.units,
            _ => println!("Unknown movement {}", movement.direction),
        }
    }
    println!("Final horizontal position: {}", horizontal);
    println!("Final depth: {}", depth);
    println!("Product: {}", depth * horizontal);
}
