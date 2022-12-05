
pub fn get_lines_from_input_file(day: u8, example: bool) -> Vec<String> {
    let filename: String;
    match example {
        false => filename = format!("inputs/day{}_input.txt", day),
        true => filename = format!("examples/day{}_example.txt", day)
    }
    println!("Reading input from {}", filename);
    std::fs::read_to_string(filename)
        .expect("Something went wrong reading the file")
        .lines()
        .map(str::to_string)
        .collect()
}

pub fn print_part_solution<T: std::fmt::Display>(part: u8, solution: T) {
    println!("Part {} solution:", part);
    println!("{}", solution);
}