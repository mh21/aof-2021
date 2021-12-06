// https://adventofcode.com/2021/day/4
use std::fs;

struct Board {
    size: usize,
    numbers: Vec<i32>,
}

impl Board {
    fn check_win(&self) -> bool {
        (0..self.size).any(|r| {
            self.numbers
                .iter()
                .skip(r * self.size)
                .take(self.size)
                .all(|&n| n == -1)
        }) || (0..self.size).any(|c| {
            self.numbers
                .iter()
                .skip(c)
                .step_by(self.size)
                .all(|&n| n == -1)
        })
    }

    fn sum_unmarked(&self) -> i32 {
        self.numbers.iter().filter(|&&n| n != -1).sum()
    }

    fn cross_out(&mut self, number: i32) {
        if let Some(position) = self.numbers.iter().position(|&n| n == number) {
            self.numbers[position] = -1;
        }
    }
}

fn read_data(filename: &str) -> (Vec<i32>, Vec<Board>) {
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    let mut lines = contents.lines();
    let drawn_numbers: Vec<i32> = lines
        .next()
        .unwrap()
        .split(',')
        .map(|n| n.parse::<i32>().unwrap())
        .collect();
    let mut boards = Vec::new();
    while let Some(_) = lines.next() {
        let numbers: Vec<i32> = lines
            .by_ref()
            .take(5)
            .map(|s| s.split_whitespace())
            .flatten()
            .map(|n| n.parse::<i32>().unwrap())
            .collect();
        boards.push(Board { size: 5, numbers });
    }
    (drawn_numbers, boards)
}

pub fn part_1(filename: &str) -> String {
    let (drawn_numbers, mut boards) = read_data(filename);
    for number in drawn_numbers {
        println!("Drawing: {}", number);
        for board in boards.iter_mut() {
            board.cross_out(number);
        }
        if let Some(board) = boards.iter().find(|b| b.check_win()) {
            println!("Winning board found!");
            let sum_unmarked = board.sum_unmarked();
            println!("Sum of unmarked numbers: {:?}", sum_unmarked);
            return (number * sum_unmarked).to_string();
        }
    }
    String::from("no winning board found")
}

pub fn part_2(filename: &str) -> String {
    let (drawn_numbers, mut boards) = read_data(filename);
    let mut last_winning_board: Option<Board> = None;
    let mut last_winning_number = -1;
    for number in drawn_numbers {
        println!("Drawing: {}", number);
        for board in boards.iter_mut() {
            board.cross_out(number);
        }
        while let Some(position) = boards.iter().position(|b| b.check_win()) {
            last_winning_board = Some(boards.remove(position));
            last_winning_number = number;
        }
    }
    let sum_unmarked = last_winning_board.unwrap().sum_unmarked();
    println!("Sum of unmarked numbers: {:?}", sum_unmarked);
    (last_winning_number * sum_unmarked).to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1_example() {
        assert_eq!(part_1("data/puzzle-04-input-example"), "4512");
    }

    #[test]
    fn test_part_1() {
        assert_eq!(part_1("data/puzzle-04-input"), "89001");
    }

    #[test]
    fn test_part_2_example() {
        assert_eq!(part_2("data/puzzle-04-input-example"), "1924");
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2("data/puzzle-04-input"), "7296");
    }
}
