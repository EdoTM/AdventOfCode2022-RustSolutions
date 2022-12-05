use crate::utils;

pub fn day2_solution(example: bool) {
    let lines = utils::get_lines_from_input_file(2, example);

    part1(&lines);
    part2(&lines);
}

#[derive(PartialEq, Clone, Copy)]
enum Shape {
    Rock = 1,
    Paper = 2,
    Scissors = 3
}
impl Shape {
    pub fn from_code(code: &str) -> Shape {
        match code {
            "A" | "X" => Shape::Rock,
            "B" | "Y" => Shape::Paper,
            "C" | "Z" => Shape::Scissors,
            _ => panic!("Invalid code.")
        } 
    }
    pub fn wins_over(&self, other: &Shape) -> bool {
        other.get_winning_over() == *self
    }
    pub fn score(&self) -> u8 {
        *self as u8
    }
    pub fn get_winning_over(&self) -> Shape {
        match self {
            Self::Rock => Shape::Paper,
            Self::Paper => Shape::Scissors,
            Self::Scissors => Shape::Rock
        }
    }
    pub fn get_losing_over(&self) -> Shape {
        match self {
            Self::Rock => Shape::Scissors,
            Self::Paper => Shape::Rock,
            Self::Scissors => Shape::Paper
        }
    }
}


fn part1(lines: &Vec<String>) {
    let mut total_score = 0u32;
    for line in lines {
        let shapes = line
            .split_whitespace()
            .collect::<Vec<&str>>();
        let opponent_shape = Shape::from_code(shapes[0]);
        let own_shape = Shape::from_code(shapes[1]);
        total_score += calculate_score(&own_shape, &opponent_shape);        
    }
    utils::print_part_solution(1, total_score);
}

fn calculate_score(own_shape: &Shape, opponent_shape: &Shape) -> u32 {
    if own_shape.wins_over(opponent_shape) {
        own_shape.score() as u32 + 6
    } else if own_shape == opponent_shape {
        own_shape.score() as u32 + 3
    } else {
        own_shape.score() as u32
    }
}


fn part2(lines: &Vec<String>) {
    let mut total_score = 0u32;
    for line in lines {
        let shapes = line
            .split_whitespace()
            .collect::<Vec<&str>>();
        let opponent_shape = Shape::from_code(shapes[0]);
        let outcome = shapes[1];
        let own_shape: Shape;
        match outcome {
            "X" => own_shape = opponent_shape.get_losing_over(),
            "Y" => own_shape = opponent_shape,
            "Z" => own_shape = opponent_shape.get_winning_over(),
            _ => panic!()
        }
        total_score += calculate_score(&own_shape, &opponent_shape);
    }
    utils::print_part_solution(1, total_score);
}
