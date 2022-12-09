use std::collections::HashSet;

use crate::utils;

pub fn day9_solution(example: bool) {
    let lines = utils::get_lines_from_input_file(9, example);

    if example {
        for example_lines in lines.split(String::is_empty) {
            println!("{:-^20}", "");
            part1(&example_lines.to_vec());
            part2(&example_lines.to_vec());
        }
    } else {
        part1(&lines);
        part2(&lines);
    }
}


fn part1(lines: &Vec<String>) {
    let mut rope = Rope::new(2);

    perform_movements(lines, &mut rope);
    let positions_count = rope.tail_history.len();

    utils::print_part_solution(1, positions_count);
}

fn part2(lines: &Vec<String>) {
    let mut rope = Rope::new(10);

    perform_movements(lines, &mut rope);
    let positions_count = rope.tail_history.len();

    utils::print_part_solution(2, positions_count);
}


fn perform_movements(lines: &Vec<String>, rope: &mut Rope) {
    for line in lines {
        let mut tokens = line.split_whitespace();
        let direction_code = tokens.next().unwrap();
        let amount: u8 = tokens.next().unwrap().parse().unwrap();

        let direction = Direction::from_direction_code(direction_code);
        rope.move_to(&direction, amount);
    }
}


enum Direction {
    Up,
    Down,
    Left,
    Right
}
impl Direction {
    pub fn from_direction_code(code: &str) -> Direction {
        match code {
            "U" => Self::Up,
            "D" => Self::Down,
            "L" => Self::Left,
            "R" => Self::Right,
            _ => panic!("Invalid direction!"),
        }
    }
}


type Coord = i16;
type Position = (Coord, Coord);

struct Rope {
    knot_positions: Vec<Position>,
    tail_history: HashSet<Position>,
}
impl Rope {
    pub fn new(knot_count: u8) -> Rope {
        let mut tail_history: HashSet<Position> = HashSet::new();
        tail_history.insert((0, 0));

        let mut knot_positions: Vec<Position> = Vec::new();
        knot_positions.reserve_exact(knot_count as usize);
        for _ in 0..knot_count {
            knot_positions.push((0, 0));
        }

        Self {
            knot_positions,
            tail_history,
        }
    }

    pub fn move_to(&mut self, direction: &Direction, amount: u8) {
        for _ in 0..amount {
            self.process_move(direction);
        }
    }


    fn process_move(&mut self, direction: &Direction) {
        self.update_head_position(direction);

        for knot in 1..self.knot_positions.len() as usize {
            let old_tail_pos = self.knot_positions[knot];

            self.update_tail_position(knot-1, knot);

            if self.position_has_not_changed(knot, &old_tail_pos) {
                return;
            }
        }
        
        let tail_position = self.knot_positions.last().unwrap().clone();
        self.tail_history.insert(tail_position);
    }

    fn update_head_position(&mut self, direction: &Direction) {
        let (mut x, mut y) = self.knot_positions[0];
        match direction {
            Direction::Up => y += 1,
            Direction::Down => y -= 1,
            Direction::Left => x -= 1,
            Direction::Right => x += 1,
        }
        self.knot_positions[0] = (x, y);
    }

    fn update_tail_position(&mut self, head: usize, tail: usize) {
        if self.knots_are_touching(tail, tail - 1) {
            return;
        }
        let (headx, heady) = self.knot_positions[head];
        let (mut tailx, mut taily) = self.knot_positions[tail];

        if tailx != headx {
            tailx += if tailx < headx { 1 } else { -1 };
        }
        if taily != heady {
            taily += if taily < heady { 1 } else { -1 };
        }

        self.knot_positions[tail] = (tailx, taily);
    }

    fn knots_are_touching(&self, tail: usize, head: usize) -> bool {
        let (headx, heady) = self.knot_positions[tail];
        let (tailx, taily) = self.knot_positions[head];
        
        (headx - 1 ..= headx + 1).contains(&tailx) &&
        (heady - 1 ..= heady + 1).contains(&taily)
    }

    fn position_has_not_changed(&self, knot: usize, old_position: &Position) -> bool {
        old_position == &self.knot_positions[knot]
    }
}
