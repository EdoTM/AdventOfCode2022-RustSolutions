
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


pub mod math {
    pub fn gcd(a: u32, b: u32) -> u32 {
        let (mut a, mut b) = (a, b);
        while b != 0 {
            (a, b) = (b, a % b);
        }

        a
    }

    pub fn lcm_multiple(values: &mut impl Iterator<Item = u32>) -> Option<u64> {
        let mut _gcd = values.next()?;
        let mut prod = _gcd as u64;
        for value in values {
            _gcd = gcd(_gcd, value);
            prod *= value as u64;
        }

        Some(prod / _gcd as u64)
    }
}