extern crate clap;
mod day_2021_01;
mod day_2021_02;
mod day_2021_03;
mod day_2021_04;
mod day_2021_05;
mod day_2021_06;
mod day_2021_07;

fn run(puzzle: &str) -> Option<String> {
    match puzzle {
        "2021/1/1" => Some(day_2021_01::part_1("data/puzzle-2021-01-input")),
        "2021/1/2" => Some(day_2021_01::part_2("data/puzzle-2021-01-input")),
        "2021/2/1" => Some(day_2021_02::part_1("data/puzzle-2021-02-input")),
        "2021/2/2" => Some(day_2021_02::part_2("data/puzzle-2021-02-input")),
        "2021/3/1" => Some(day_2021_03::part_1("data/puzzle-2021-03-input")),
        "2021/3/2" => Some(day_2021_03::part_2("data/puzzle-2021-03-input")),
        "2021/4/1" => Some(day_2021_04::part_1("data/puzzle-2021-04-input")),
        "2021/4/2" => Some(day_2021_04::part_2("data/puzzle-2021-04-input")),
        "2021/5/1" => Some(day_2021_05::part_1("data/puzzle-2021-05-input")),
        "2021/5/2" => Some(day_2021_05::part_2("data/puzzle-2021-05-input")),
        "2021/6/1" => Some(day_2021_06::part_1("data/puzzle-2021-06-input")),
        "2021/6/2" => Some(day_2021_06::part_2("data/puzzle-2021-06-input")),
        "2021/7/1" => Some(day_2021_07::part_1("data/puzzle-2021-07-input")),
        "2021/7/2" => Some(day_2021_07::part_2("data/puzzle-2021-07-input")),
        _ => {
            println!("Unknown puzzle");
            None
        }
    }
}

fn main() {
    let matches = clap::App::new("aof-rust")
        .arg(
            clap::Arg::with_name("PUZZLE")
                .help("Puzzle to run ({year}/{day}/{1,2})")
                .required(true)
                .index(1),
        )
        .get_matches();
    let result = run(matches.value_of("PUZZLE").unwrap());
    println!("Result: {}", result.unwrap());
}
