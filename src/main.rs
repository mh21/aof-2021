extern crate clap;
mod day_01;
mod day_02;
mod day_03;

fn run(puzzle: &str) -> Option<String> {
    match puzzle {
        "1/1" => Some(day_01::part_1("data/puzzle-01-input")),
        "1/2" => Some(day_01::part_2("data/puzzle-01-input")),
        "2/1" => Some(day_02::part_1("data/puzzle-02-input")),
        "2/2" => Some(day_02::part_2("data/puzzle-02-input")),
        "3/1" => Some(day_03::part_1("data/puzzle-03-input")),
        "3/2" => Some(day_03::part_2("data/puzzle-03-input")),
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
