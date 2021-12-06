extern crate clap;
mod day_01;
mod day_02;
mod day_03;

fn run(puzzle: &str) -> Option<String> {
    match puzzle {
        "1/1" => Some(day_01::part_1()),
        "1/2" => Some(day_01::part_2()),
        "2/1" => Some(day_02::part_1()),
        "2/2" => Some(day_02::part_2()),
        "3/1" => Some(day_03::part_1()),
        "3/2" => Some(day_03::part_2()),
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
