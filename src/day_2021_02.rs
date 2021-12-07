// https://adventofcode.com/2021/day/2
use std::fs;

struct Movement {
    direction: String,
    units: i32,
}

pub fn part_1(filename: &str) -> String {
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
    (depth * horizontal).to_string()
}

pub fn part_2(filename: &str) -> String {
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
    let mut aim = 0;
    for movement in movements {
        match movement.direction.as_str() {
            "down" => aim += movement.units,
            "up" => aim -= movement.units,
            "forward" => {
                horizontal += movement.units;
                depth += aim * movement.units;
            }
            _ => println!("Unknown movement {}", movement.direction),
        }
    }
    println!("Final horizontal position: {}", horizontal);
    println!("Final depth: {}", depth);
    (depth * horizontal).to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1_example() {
        assert_eq!(part_1("data/puzzle-2021-02-input-example"), "150");
    }

    #[test]
    fn test_part_1() {
        assert_eq!(part_1("data/puzzle-2021-02-input"), "1936494");
    }

    #[test]
    fn test_part_2_example() {
        assert_eq!(part_2("data/puzzle-2021-02-input-example"), "900");
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2("data/puzzle-2021-02-input"), "1997106066");
    }
}
