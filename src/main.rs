mod utils;
mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;

fn main() {
    let day = parse_day();
    let example = std::env::args().nth(2) == Some("ex".to_string());

    match day {
        1 => day1::day1_solution(example),
        2 => day2::day2_solution(example),
        3 => day3::day3_solution(example),
        4 => day4::day4_solution(example),
        5 => day5::day5_solution(example),
        6 => day6::day6_solution(example),
        _ => {
            println!("Day {} not implemented", day);
            std::process::exit(1);
        }
    }
}

fn parse_day() -> u32 {
    let arg;
    match std::env::args().nth(1) {
        None => {
            println!("Usage: aoc2022 <day> [ex]");
            println!("With cargo: cargo run <day> [ex]");
            std::process::exit(1);
        },
        Some(v) => arg = v
    }

    match arg.parse::<u32>() {
        Ok(day) => day,
        Err(_) => {
            println!("Day must be an integer from 1 to 25.");
            std::process::exit(1);
        }
    }
}