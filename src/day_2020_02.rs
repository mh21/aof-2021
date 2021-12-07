// https://adventofcode.com/2020/day/2

#[derive(Debug)]
struct Password {
    min: usize,
    max: usize,
    letter: char,
    password: String,
}

impl Password {
    pub fn new(line: &str) -> Password {
        let pieces: Vec<_> = line.split(|c| c == '-' || c == ' ' || c == ':').collect();
        Password {
            min: pieces[0].parse::<usize>().unwrap(),
            max: pieces[1].parse::<usize>().unwrap(),
            letter: pieces[2].chars().next().unwrap(),
            password: pieces[4].to_string(),
        }
    }

    pub fn is_valid_part_1(&self) -> bool {
        (self.min..=self.max).contains(&self.password.chars().filter(|c| *c == self.letter).count())
    }

    pub fn is_valid_part_2(&self) -> bool {
        self.password
            .chars()
            .enumerate()
            .filter(|(i, c)| (*i + 1 == self.min || *i + 1 == self.max) && *c == self.letter)
            .count()
            == 1
    }
}

pub fn part_1(filename: &str) -> String {
    std::fs::read_to_string(filename)
        .unwrap()
        .lines()
        .map(Password::new)
        .filter(|p| p.is_valid_part_1())
        .count()
        .to_string()
}

pub fn part_2(filename: &str) -> String {
    std::fs::read_to_string(filename)
        .unwrap()
        .lines()
        .map(Password::new)
        .filter(|p| p.is_valid_part_2())
        .count()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1_example() {
        assert_eq!(part_1("data/puzzle-2020-02-input-example"), "2");
    }

    #[test]
    fn test_part_1() {
        assert_eq!(part_1("data/puzzle-2020-02-input"), "655");
    }

    #[test]
    fn test_part_2_example() {
        assert_eq!(part_2("data/puzzle-2020-02-input-example"), "1");
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2("data/puzzle-2020-02-input"), "673");
    }
}
