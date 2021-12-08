// https://adventofcode.com/2021/day/8
use std::collections::HashSet;

struct ScrambledDigits {
    digits: Vec<HashSet<u8>>,
    examples: Vec<Vec<u8>>,
}

impl ScrambledDigits {
    fn recognize(&self, scrambled_digit: &[u8], map: &[u8]) -> usize {
        let mut descrambled: Vec<_> = scrambled_digit
            .iter()
            .map(|&d| map[d as usize] + b'a')
            .collect();
        descrambled.sort_unstable();
        match String::from_utf8(descrambled).unwrap().as_str() {
            "abcefg" => 0,
            "cf" => 1,
            "acdeg" => 2,
            "acdfg" => 3,
            "bcdf" => 4,
            "abdfg" => 5,
            "abdefg" => 6,
            "acf" => 7,
            "abcdefg" => 8,
            "abcdfg" => 9,
            _ => 255,
        }
    }

    fn solve(&self) -> Vec<u8> {
        let segments_cf = self.digits.iter().find(|d| d.len() == 2).unwrap(); // 1
        let segments_acf = self.digits.iter().find(|d| d.len() == 3).unwrap(); // 7
        let segments_bcdf = self.digits.iter().find(|d| d.len() == 4).unwrap(); // 4
        let segments_abcdfg = self // 9
            .digits
            .iter()
            .find(|d| d.len() == 6 && d.is_superset(&(segments_acf | segments_bcdf)))
            .unwrap();
        let segments_cd = self // 6 xor 0
            .digits
            .iter()
            .filter(|&d| d.len() == 6 && d != segments_abcdfg)
            .fold(HashSet::new(), |r, d| &r ^ d);
        let segments_bd = segments_bcdf - segments_cf;
        let segments_ag = segments_abcdfg - segments_bcdf;
        let segments_abcdefg = (0..7).collect::<HashSet<_>>();
        let mut mapping: Vec<u8> = vec![255; 7];
        mapping[*(segments_acf - segments_cf).iter().next().unwrap() as usize] = 0; // segment a
        mapping[*(&segments_bd - &segments_cd).iter().next().unwrap() as usize] = 1; // segment b
        mapping[*(&segments_cd & segments_cf).iter().next().unwrap() as usize] = 2; // segment c
        mapping[*(&segments_cd - segments_cf).iter().next().unwrap() as usize] = 3; // segment d
        mapping[*(&segments_abcdefg - segments_abcdfg).iter().next().unwrap() as usize] = 4; // segment e
        mapping[*(segments_cf - &segments_cd).iter().next().unwrap() as usize] = 5; // segment f
        mapping[*(&segments_ag - segments_acf).iter().next().unwrap() as usize] = 6; // segment g
        mapping
    }
}

fn read(filename: &str) -> Vec<ScrambledDigits> {
    std::fs::read_to_string(filename)
        .unwrap()
        .lines()
        .map(|l| l.split_once(" | ").unwrap())
        .map(|(digits, examples)| ScrambledDigits {
            digits: digits
                .split(' ')
                .map(|s| s.as_bytes().iter().map(|&c| c - b'a').collect())
                .collect(),
            examples: examples
                .split(' ')
                .map(|s| s.as_bytes().iter().map(|&c| c - b'a').collect())
                .collect(),
        })
        .collect()
}

pub fn part_1(filename: &str) -> String {
    let scrambled_digits = read(filename);
    let mut results = vec![0; 10];
    for digits in scrambled_digits {
        let map = digits.solve();
        for example in &digits.examples {
            results[digits.recognize(example, &map)] += 1;
        }
    }
    (results[1] + results[4] + results[7] + results[8]).to_string()
}

pub fn part_2(filename: &str) -> String {
    let scrambled_digits = read(filename);
    let mut result = 0;
    for digits in scrambled_digits {
        let map = digits.solve();
        result += digits
            .examples
            .iter()
            .rev()
            .enumerate()
            .map(|(i, v)| digits.recognize(v, &map) * 10_usize.pow(i as u32))
            .sum::<usize>();
    }
    result.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1_example() {
        assert_eq!(part_1("data/puzzle-2021-08-input-example"), "26");
    }

    #[test]
    fn test_part_1() {
        assert_eq!(part_1("data/puzzle-2021-08-input"), "247");
    }

    #[test]
    fn test_part_2_example() {
        assert_eq!(part_2("data/puzzle-2021-08-input-example"), "61229");
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2("data/puzzle-2021-08-input"), "933305");
    }
}
