mod utils;
mod day1;
mod day2;
mod day3;
mod day4;
mod day5;

fn main() {
    let arg = std::env::args().nth(1);
    match arg {
        None => {
            println!("Usage: aoc2022 <day> [ex]");
            println!("If you are using cargo: cargo run <day> [ex]");
            std::process::exit(1);
        }
        _ => ()
    }
    let arg = arg.unwrap();
    let day = arg.parse::<i32>().unwrap();
    let example = std::env::args().nth(2) == Some("ex".to_string());
    match day {
        1 => day1::day1_solution(example),
        2 => day2::day2_solution(example),
        3 => day3::day3_solution(example),
        4 => day4::day4_solution(example),
        5 => day5::day5_solution(example),
        _ => println!("Day {} not implemented", day),
    }
}