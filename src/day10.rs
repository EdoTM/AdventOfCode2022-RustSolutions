use std::slice::Iter;

use crate::utils;

pub fn day10_solution(example: bool) {
    let lines = utils::get_lines_from_input_file(10, example);

    part1(&lines);
    part2(&lines);
}


fn part1(lines: &Vec<String>) {
    let mut cpu = Cpu::new(lines);
    let mut signal_strengths = 0i32;

    while let Some((cycle, reg)) = cpu.next_cycle() {
        if cycle >= 20 && (cycle - 20) % 40 == 0 {
            signal_strengths += cycle as i32 * reg;
        }
    }

    utils::print_part_solution(1, signal_strengths);
}

fn part2(lines: &Vec<String>) {
    let mut crt = Crt::new(lines);
    let image = crt.render();
    utils::print_part_solution(2, image);
}


type RegValue = i32;
type ClockCycle = u32;

struct Cpu<'a> {
    instructions: Iter<'a, String>,
    x_register: RegValue,
    clock_cycle: ClockCycle,
    current_instruction: Instruction
}
impl<'a> Cpu<'a> {
    pub fn new(instructions: &Vec<String>) -> Cpu {
        Cpu {
            instructions: instructions.iter(),
            x_register: 1,
            clock_cycle: 1,
            current_instruction: Instruction::Noop
        }
    } 

    pub fn next_cycle(&mut self) -> Option<(ClockCycle, RegValue)> {
        let return_value = (self.clock_cycle, self.x_register);

        if let Instruction::Addx(amount) = self.current_instruction {
            self.x_register += amount;
            self.current_instruction = Instruction::Noop;
        } else {
            let line = self.instructions.next()?;
            let instruction = Instruction::from_line(line);
            self.current_instruction = instruction;
        }
        self.clock_cycle += 1;

        Some(return_value)
    }
}


enum Instruction {
    Noop,
    Addx(i32),
}
impl Instruction {
    pub fn from_line(line: &str) -> Instruction {
        let mut tokens = line.split_whitespace();
        match tokens.next().unwrap() {
            "noop" => Self::Noop,
            "addx" => {
                let amount: i32 = tokens.next().unwrap().parse().unwrap();
                Self::Addx(amount)
            }
            _ => panic!("Invalid input line."),
        }
    }
}


struct Crt<'a> {
    cpu: Cpu<'a>,
    image_buffer: String,
}
impl<'a> Crt<'a> {
    pub fn new(instructions: &Vec<String>) -> Crt {
        Crt {
            cpu: Cpu::new(instructions),
            image_buffer: "".to_string(),
        }
    }

    pub fn render(&mut self) -> String {
        while let Some((cycle, reg)) = self.cpu.next_cycle() {
            let pixel = (cycle - 1) % 40;
            if pixel == 0 && cycle != 1 {
                self.draw_new_line();
            }
            if (reg-1 ..= reg+1).contains(&(pixel as i32)) {
                self.draw_lit_pixel();
            } else {
                self.draw_dim_pixel();
            }
        }

        self.image_buffer.clone()
    }

    fn draw_lit_pixel(&mut self) {
        self.image_buffer.push('#');
    }

    fn draw_dim_pixel(&mut self) {
        self.image_buffer.push('.');
    }

    fn draw_new_line(&mut self) {
        self.image_buffer.push('\n');
    }
}
