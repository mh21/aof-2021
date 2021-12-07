extern crate clap;
mod day_01;
mod day_02;
mod day_03;
mod day_04;
mod day_05;
mod day_06;

fn run(puzzle: &str) -> Option<String> {
    match puzzle {
        "1/1" => Some(day_01::part_1("data/puzzle-01-input")),
        "1/2" => Some(day_01::part_2("data/puzzle-01-input")),
        "2/1" => Some(day_02::part_1("data/puzzle-02-input")),
        "2/2" => Some(day_02::part_2("data/puzzle-02-input")),
        "3/1" => Some(day_03::part_1("data/puzzle-03-input")),
        "3/2" => Some(day_03::part_2("data/puzzle-03-input")),
        "4/1" => Some(day_04::part_1("data/puzzle-04-input")),
        "4/2" => Some(day_04::part_2("data/puzzle-04-input")),
        "5/1" => Some(day_05::part_1("data/puzzle-05-input")),
        "5/2" => Some(day_05::part_2("data/puzzle-05-input")),
        "6/1" => Some(day_06::part_1("data/puzzle-06-input")),
        "6/2" => Some(day_06::part_2("data/puzzle-06-input")),
        _ => {
            println!("Unknown puzzle");
            None
        }
    }
}

fn main() {
    let matches = clap::App::new("aof-2021")
        .arg(
            clap::Arg::with_name("PUZZLE")
                .help("Puzzle to run (day/{1,2})")
                .required(true)
                .index(1),
        )
        .get_matches();
    let result = run(matches.value_of("PUZZLE").unwrap());
    println!("Result: {}", result.unwrap());
}
