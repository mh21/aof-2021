// https://adventofcode.com/2021/day/2
struct Movement {
    direction: String,
    units: i32,
}

fn read(filename: &str) -> Vec<Movement> {
    std::fs::read_to_string(filename)
        .unwrap()
        .lines()
        .map(|s| {
            let (direction, units) = s.split_once(' ').unwrap();
            Movement {
                direction: direction.to_string(),
                units: units.parse::<i32>().unwrap(),
            }
        })
        .collect()
}

pub fn part_1(filename: &str) -> String {
    let mut horizontal = 0;
    let mut depth = 0;
    for movement in read(filename) {
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
    let mut horizontal = 0;
    let mut depth = 0;
    let mut aim = 0;
    for movement in read(filename) {
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
