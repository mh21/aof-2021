// https://adventofcode.com/2021/day/10

pub static DATA: &str = include_str!("../data/puzzle-2021-10-input");

fn expected(data: char) -> Option<char> {
    match data {
        '}' => Some('{'),
        ']' => Some('['),
        '>' => Some('<'),
        ')' => Some('('),
        _ => None,
    }
}

fn read(data: &str) -> Vec<(Vec<char>, Option<char>)> {
    data.lines()
        .map(|s| {
            s.chars()
                .fold((Vec::<char>::new(), None), |(mut m, mut c), n| {
                    match (c, n) {
                        (None, '(' | '[' | '{' | '<') => m.push(n),
                        (None, _) if m.pop() != expected(n) => c = Some(n),
                        _ => {}
                    }
                    (m, c)
                })
        })
        .collect()
}
pub fn part_1(data: &str) -> String {
    read(data)
        .iter()
        .map(|(_, c)| match c {
            Some(')') => 3,
            Some(']') => 57,
            Some('}') => 1197,
            Some('>') => 25137,
            _ => 0,
        })
        .sum::<i32>()
        .to_string()
}

pub fn part_2(data: &str) -> String {
    let mut scores: Vec<i64> = read(data)
        .iter()
        .filter_map(|(m, c)| match (m.len(), c) {
            (_, Some(_)) | (0, _) => None,
            _ => Some(m.iter().rev().fold(0, |s, c| {
                s * 5
                    + match c {
                        '(' => 1,
                        '[' => 2,
                        '{' => 3,
                        '<' => 4,
                        _ => 0,
                    }
            })),
        })
        .collect();
    let index = scores.len() / 2;
    scores.select_nth_unstable(index).1.to_string()
}

#[cfg(test)]
mod tests {
    static EXAMPLE_DATA: &str = include_str!("../data/puzzle-2021-10-input-example");

    use super::*;

    #[test]
    fn test_part_1_example() {
        assert_eq!(part_1(EXAMPLE_DATA), "26397");
    }

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(DATA), "168417");
    }

    #[test]
    fn test_part_2_example() {
        assert_eq!(part_2(EXAMPLE_DATA), "288957");
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(DATA), "2802519786");
    }
}
