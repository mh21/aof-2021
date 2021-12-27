// https://adventofcode.com/2021/day/9

pub static DATA: &str = include_str!("../data/puzzle-2021-09-input");

fn read_heights(data: &str) -> Vec<Vec<usize>> {
    data.lines()
        .map(|s| s.as_bytes().iter().map(|&c| (c - b'0') as usize).collect())
        .collect()
}

fn find_minima(heights: &[Vec<usize>]) -> Vec<(usize, usize)> {
    let mut minima = Vec::new();
    let ylen = heights.len();
    let xlen = heights[0].len();
    for y in 0..ylen {
        for x in 0..xlen {
            let v = heights[y][x];
            if (y == 0 || v < heights[y - 1][x])
                && (y == ylen - 1 || v < heights[y + 1][x])
                && (x == 0 || v < heights[y][x - 1])
                && (x == xlen - 1 || v < heights[y][x + 1])
            {
                minima.push((y, x));
            }
        }
    }
    minima
}

pub fn part_1(data: &str) -> String {
    let heights = read_heights(data);
    find_minima(&heights)
        .iter()
        .map(|&(y, x)| heights[y][x] + 1)
        .sum::<usize>()
        .to_string()
}

pub fn part_2(data: &str) -> String {
    let mut heights = read_heights(data);
    let ylen = heights.len();
    let xlen = heights[0].len();
    let minima = find_minima(&heights);
    // mark all low points with numbers > 9
    for (i, &(y, x)) in minima.iter().enumerate() {
        heights[y][x] = i + 10;
    }
    let mut found_unresolved = true;
    while found_unresolved {
        found_unresolved = false;
        for y in 0..ylen {
            for x in 0..xlen {
                if heights[y][x] < 9 {
                    found_unresolved = true;
                    heights[y][x] = match (y > 0, y < ylen - 1, x > 0, x < xlen - 1) {
                        (true, _, _, _) if heights[y - 1][x] > 9 => heights[y - 1][x],
                        (_, true, _, _) if heights[y + 1][x] > 9 => heights[y + 1][x],
                        (_, _, true, _) if heights[y][x - 1] > 9 => heights[y][x - 1],
                        (_, _, _, true) if heights[y][x + 1] > 9 => heights[y][x + 1],
                        _ => heights[y][x],
                    };
                }
            }
        }
    }
    let mut results: Vec<_> = heights
        .iter()
        .flatten()
        .filter(|&&height| height > 9)
        .fold(
            std::collections::HashMap::<usize, usize>::new(),
            |mut m, &n| {
                *m.entry(n).or_default() += 1;
                m
            },
        )
        .into_values()
        .collect();
    results.sort_unstable();
    results.iter().rev().take(3).product::<usize>().to_string()
}

#[cfg(test)]
mod tests {
    static EXAMPLE_DATA: &str = include_str!("../data/puzzle-2021-09-input-example");

    use super::*;

    #[test]
    fn test_part_1_example() {
        assert_eq!(part_1(EXAMPLE_DATA), "15");
    }

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(DATA), "541");
    }

    #[test]
    fn test_part_2_example() {
        assert_eq!(part_2(EXAMPLE_DATA), "1134");
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(DATA), "847504");
    }
}
