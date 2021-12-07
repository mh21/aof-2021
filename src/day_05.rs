// https://adventofcode.com/2021/day/4
struct Diagram {
    size: i32,
    numbers: Vec<i32>,
}

impl Diagram {
    fn new(vents: &[Vec<i32>]) -> Diagram {
        let size = *vents.iter().map(|v| v.iter().max().unwrap()).max().unwrap() + 1;
        Diagram {
            size,
            numbers: vec![0; (size * size) as usize],
        }
    }

    fn record_vent(&mut self, x1: i32, y1: i32, x2: i32, y2: i32) {
        let (xstep, xcount) = match x1.cmp(&x2) {
            std::cmp::Ordering::Less => (1, x2 - x1),
            std::cmp::Ordering::Greater => (-1, x1 - x2),
            _ => (0, 0),
        };
        let (ystep, ycount) = match y1.cmp(&y2) {
            std::cmp::Ordering::Less => (1, y2 - y1),
            std::cmp::Ordering::Greater => (-1, y1 - y2),
            _ => (0, 0),
        };
        for i in 0..=std::cmp::max(xcount, ycount) {
            self.numbers[((y1 + i * ystep) * self.size + x1 + i * xstep) as usize] += 1;
        }
    }

    fn dangerous_areas(&self) -> usize {
        self.numbers.iter().filter(|&&n| n > 1).count()
    }
}

fn read_data(filename: &str) -> Vec<Vec<i32>> {
    let contents = std::fs::read_to_string(filename).unwrap();
    contents
        .lines()
        .map(|l| {
            l.split(" -> ")
                .map(|c| c.split(','))
                .flatten()
                .map(|n| n.parse::<i32>().unwrap())
                .collect()
        })
        .collect()
}

pub fn part_1(filename: &str) -> String {
    let vents: Vec<_> = read_data(filename)
        .into_iter()
        .filter(|v| (v[0] == v[2]) || v[1] == v[3])
        .collect();
    let mut diagram = Diagram::new(&vents);
    for vent in vents.iter() {
        diagram.record_vent(vent[0], vent[1], vent[2], vent[3]);
    }
    diagram.dangerous_areas().to_string()
}

pub fn part_2(filename: &str) -> String {
    let vents: Vec<_> = read_data(filename);
    let mut diagram = Diagram::new(&vents);
    for vent in vents.iter() {
        diagram.record_vent(vent[0], vent[1], vent[2], vent[3]);
    }
    diagram.dangerous_areas().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1_example() {
        assert_eq!(part_1("data/puzzle-05-input-example"), "5");
    }

    #[test]
    fn test_part_1() {
        assert_eq!(part_1("data/puzzle-05-input"), "5145");
    }

    #[test]
    fn test_part_2_example() {
        assert_eq!(part_2("data/puzzle-05-input-example"), "12");
    }
    #[test]
    fn test_part_2() {
        assert_eq!(part_2("data/puzzle-05-input"), "16518");
    }
}
