use std::collections::VecDeque;

use crate::utils;

pub fn day11_solution(example: bool) {
    let lines = utils::get_lines_from_input_file(11, example);

    part1(&lines);
    part2(&lines);
}


fn part1(lines: &Vec<String>) {
    let mut monkeys = get_monkeys_from_lines(lines);

    for _ in 0..20 {
        for i in 0..monkeys.len() {
            while let Some(item) = monkeys[i].process_next_item() {
                let item = item / 3;
                test_item_and_give_to_next_monkey(&mut monkeys, i, item);
            }
        }
    }

    let monkey_business = calculate_monkey_business(&monkeys);
    utils::print_part_solution(1, monkey_business);
}

fn part2(lines: &Vec<String>) {
    let mut monkeys = get_monkeys_from_lines(lines);

    for _ in 0..10_000 {
        for i in 0..monkeys.len() {
            let lcm = get_lcm_of_all_divisors(&monkeys);
            while let Some(item) = monkeys[i].process_next_item() {
                let mod_item = item % lcm;
                test_item_and_give_to_next_monkey(&mut monkeys, i, mod_item);
            }
        }
    }

    let monkey_business = calculate_monkey_business(&monkeys);
    utils::print_part_solution(2, monkey_business);
}


fn test_item_and_give_to_next_monkey(monkeys: &mut Vec<Monkey>, monkey_id: usize, item: Item) {
    let monkey = &monkeys[monkey_id];
    let next_monkey;
    if item % monkey.divisor == 0 {
        next_monkey = monkey.monkey_true;
    } else {
        next_monkey = monkey.monkey_false;
    }
    monkeys[next_monkey].insert_item(item);
}

fn calculate_monkey_business(monkeys: &Vec<Monkey>) -> u64 {
    let (a, b) = get_top_2_inspected_items_count(monkeys);
    
    a as u64 * b as u64
}

fn get_top_2_inspected_items_count(monkeys: &Vec<Monkey>) -> (u32, u32) {
    let mut top_2 = (0, 0);
    let (a, b) = &mut top_2;
    let mut min = 0;
    
    for monkey in monkeys {
        let count = monkey.inspected_items_count;
        if count > min {
            if *a == min {
                *a = count; 
            } else {
                *b = count;
            }
            min = u32::min(*a, *b);
        }
    }

    top_2
}

fn get_lcm_of_all_divisors(monkeys: &Vec<Monkey>) -> Item {
    utils::math::lcm_multiple(
        &mut monkeys
            .iter()
            .map(|m| m.divisor as u32)
    )
    .unwrap() as Item
}


fn get_monkeys_from_lines(lines: &Vec<String>) -> Vec<Monkey> {
    let mut monkeys: Vec<Monkey> = Vec::new();

    for line in lines {
        if line.starts_with("Monkey") {
            monkeys.push(Monkey::new_empty());
            continue;
        }
        if line.is_empty() {
            continue;
        }
        let current_monkey = monkeys.last_mut().unwrap();
        let mut tokens = line.trim().split_whitespace();

        match tokens.next().unwrap() {
            "Starting" => parse_starting_items(current_monkey, &mut tokens),
            "Operation:" => current_monkey.operation = Operation::from_line(line),
            "Test:" => parse_test(current_monkey, &mut tokens),
            "If" => parse_condition(current_monkey, &mut tokens),
            _ => ()
        }
    }

    monkeys
}

fn parse_starting_items<'a>(monkey: &mut Monkey, tokens: &mut impl Iterator<Item = &'a str>) {
    monkey.items = tokens
        .filter_map(
            |s| s
                .replace(',', "")
                .parse::<Item>()
                .ok()
            )
        .collect();
}

fn parse_test<'a>(monkey: &mut Monkey, tokens: &mut impl Iterator<Item = &'a str>) {
    let divisor = tokens.last().unwrap();
    monkey.divisor = divisor.parse().unwrap();
}

fn parse_condition<'a>(monkey: &mut Monkey, tokens: &mut impl Iterator<Item = &'a str>) {
    let condition = tokens.next().unwrap();
    let monkey_id = tokens.last().unwrap().parse().unwrap();
    match condition {
        "true:" => monkey.monkey_true = monkey_id,
        "false:" => monkey.monkey_false = monkey_id,
        _ => panic!("Could not parse condition")
    }
}


type Item = u64;
struct Monkey {
    items: VecDeque<Item>,
    operation: Operation,
    divisor: Item,
    monkey_true: usize,
    monkey_false: usize,

    inspected_items_count: u32,
}
impl Monkey {
    pub fn new_empty() -> Monkey {
        Self {
            items: VecDeque::new(),
            operation: Operation::Add(0),
            divisor: 0,
            monkey_true: 0,
            monkey_false: 0,
            inspected_items_count: 0,
        }
    }

    pub fn insert_item(&mut self, item: Item) {
        self.items.push_back(item);
    }

    pub fn process_next_item(&mut self) -> Option<Item> {
        let item = self.items.pop_front()?;
        self.inspected_items_count += 1;

        Some(self.operation.compute(item))
    }
}


enum Operation {
    Add(Item),
    Multiply(Item),
    Square
}
impl Operation {
    pub fn compute(&self, old: Item) -> Item {
        match self {
            Self::Add(amount) => old + amount,
            Self::Multiply(amount) => old * amount,
            Self::Square => old * old
        }
    }

    pub fn from_line(line: &str) -> Self {
        let right_operand = line.split_whitespace().last().unwrap();

        if line.contains('+') {
            if let Ok(amount) = right_operand.parse() {
                return Self::Add(amount);
            } else if line.ends_with("old") {
                return Self::Multiply(2)
            }
        } else if line.contains('*') {
            if let Ok(amount) = right_operand.parse() {
                return Self::Multiply(amount);
            } else if line.ends_with("old") {
                return Self::Square;
            }
        } 

        panic!("Could not parse operation.")
    }
}
