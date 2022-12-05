use crate::utils;

pub fn day5_solution(example: bool) {
    let lines = utils::get_lines_from_input_file(5, example);

    part1(&lines);
    part2(&lines);
}

fn part1(lines: &Vec<String>) {
    let message = solve(lines, PopPolicy::PopSingle);
    utils::print_part_solution(1, message);
}

fn part2(lines: &Vec<String>) {
    let message = solve(lines, PopPolicy::PopMultiple);
    utils::print_part_solution(1, message);
}


struct Stack {
    _str: String
}
impl Stack {
    pub fn new() -> Stack {
        Self {
            _str: "".to_string()
        }
    }

    pub fn peek(&self) -> char {
        self._str.chars().last().unwrap()
    }

    pub fn push(&mut self, element: char) {
        self._str.push(element);
    }

    pub fn reverse(&mut self) {
        self._str = self._str.chars().rev().collect::<String>();
    }

    pub fn pop(&mut self) -> char {
        self._str.pop().unwrap()
    }

    pub fn pop_multiple(&mut self, quantity: u32) -> String {
        let len = self._str.len();
        let new_str = self._str[..len - quantity as usize].to_string();
        let out = self._str[len - quantity as usize ..].to_string();
        self._str = new_str;

        out
    }

    pub fn push_multiple(&mut self, elements: &String) {
        self._str += elements;
    }
}


fn construct_stacks(lines: &Vec<String>) -> Vec<Stack> {
    let mut stacks = Vec::<Stack>::new();

    let stacks_count = (lines[0].len() + 1) / 4;
    stacks.reserve_exact(stacks_count);
    for _ in 0..stacks_count {
        stacks.push(Stack::new());
    }

    let mut current_stack_number;

    for line in lines {
        current_stack_number = 0;
        let mut chars = line.chars();
        let mut cursor_pos = 0;
        while cursor_pos < line.len() {
            let element = chars.nth(1).unwrap();
            if element.is_alphabetic() {
                stacks[current_stack_number].push(element);
            } else if element.is_numeric() {
                for i in 0..stacks_count {
                    stacks[i].reverse()
                }
                return stacks;
            }
            chars.nth(1);  // advance by 2 elements
            current_stack_number += 1;
            cursor_pos += 4;
        }
    }

    stacks
}


enum PopPolicy {
    PopSingle,
    PopMultiple
}


fn solve(lines: &Vec<String>, pop_policy: PopPolicy) -> String {
    let mut lines_iter = lines.iter();
    let mut stacks = construct_stacks(lines);

    while !lines_iter.next().unwrap().is_empty() {
        continue;
    }

    for line in lines_iter {
        let mut tokens = line.split_whitespace();
        let quantity: u32 = tokens.nth(1).unwrap().parse().unwrap();
        let source: usize = tokens.nth(1).unwrap().parse().unwrap();
        let destination: usize = tokens.nth(1).unwrap().parse().unwrap();
        
        match pop_policy {
            PopPolicy::PopSingle => transfer_popping_single(&mut stacks, quantity, source, destination),
            PopPolicy::PopMultiple => transfer_popping_multiple(&mut stacks, quantity, source, destination)
        }
    }

    let mut message = String::new();
    for stack in stacks {
        message.push(stack.peek());
    }

    message
}


fn transfer_popping_single(stacks: &mut Vec<Stack>, quantity: u32, source: usize, destination: usize) {
    for _ in 0..quantity {
        let popped = stacks[source - 1].pop();
        stacks[destination - 1].push(popped);
    }
}

fn transfer_popping_multiple(stacks: &mut Vec<Stack>, quantity: u32, source: usize, destination: usize) {
    let popped = stacks[source - 1].pop_multiple(quantity);
    stacks[destination - 1].push_multiple(&popped);
}


#[cfg(test)]
mod day5_tests {
    use super::*;

    #[test]
    fn stack_construction() {
        let input = vec![
            "    [D]    ".to_string(),
            "[N] [C]    ".to_string(),
            "[Z] [M] [P]".to_string(),
            " 1   2   3 ".to_string(),
            ""           .to_string()
        ];
        let stacks = construct_stacks(&input);
        assert_eq!(stacks[0]._str, "ZN");
        assert_eq!(stacks[1]._str, "MCD");
        assert_eq!(stacks[2]._str, "P");
    }

    #[test]
    fn stack_push() {
        let mut stack = Stack::new();
        assert_eq!(stack._str, "");
        stack.push('A');
        assert_eq!(stack._str, "A");
        stack.push('B');
        assert_eq!(stack._str, "AB");
    }

    #[test]
    fn stack_push_pop_multiple() {
        let mut stack = Stack {_str: "ABC".to_string()};
        assert_eq!(stack.pop_multiple(2), "BC");
        assert_eq!(stack._str, "A");
        stack.push_multiple(&"DEF".to_string());
        assert_eq!(stack._str, "ADEF");
        assert_eq!(stack.pop_multiple(2), "EF");
        assert_eq!(stack._str, "AD");
        let initial = String::from(&stack._str);
        let popped = stack.pop_multiple(2);
        stack.push_multiple(&popped);
        assert_eq!(initial, stack._str);
    }
}