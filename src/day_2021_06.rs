// https://adventofcode.com/2021/day/6

fn run(filename: &str, days: usize) -> String {
    let mut fishes = std::fs::read_to_string(filename)
        .unwrap()
        .trim()
        .split(',')
        .map(|n| n.parse::<i32>().unwrap())
        .fold(
            std::collections::HashMap::<i32, usize>::new(),
            |mut m, n| {
                *m.entry(n).or_default() += 1;
                m
            },
        );
    for _ in 0..days {
        let mut new_fishes: std::collections::HashMap<_, _> =
            fishes.iter().map(|(k, v)| ((k + 8) % 9, *v)).collect();
        *new_fishes.entry(6).or_default() += *new_fishes.entry(8).or_default();
        fishes = new_fishes;
    }
    fishes.values().sum::<usize>().to_string()
}

pub fn part_1(filename: &str) -> String {
    run(filename, 80)
}

pub fn part_2(filename: &str) -> String {
    run(filename, 256)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1_example() {
        assert_eq!(part_1("data/puzzle-2021-06-input-example"), "5934");
    }

    #[test]
    fn test_part_1() {
        assert_eq!(part_1("data/puzzle-2021-06-input"), "350605");
    }

    #[test]
    fn test_part_2_example() {
        assert_eq!(part_2("data/puzzle-2021-06-input-example"), "26984457539");
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2("data/puzzle-2021-06-input"), "1592778185024");
    }
}
