use std::collections::VecDeque;

use crate::utils;

pub fn day6_solution(example: bool) {
    let lines = utils::get_lines_from_input_file(6, example);

    if example {
        for line in &lines {
            println!("{:-^20}", "");
            part1(line);
            part2(line);
        }
    } else {
        let line = &lines[0];
        part1(line);
        part2(line);
    }
}


fn get_char_count_after_distinct(count: usize, line: &String) -> u32 {
    let mut char_count = 0u32;

    let mut current_window = VecDeque::<char>::new();
    current_window.reserve_exact(count);

    for c in line.chars() {
        char_count += 1;

        while current_window.contains(&c) {
            current_window.pop_front();
        }
        current_window.push_back(c);

        if current_window.len() == count {
            break;
        }
    }

    char_count
}


fn part1(line: &String) {
    let char_count = get_char_count_after_distinct(4, line);
    utils::print_part_solution(1, char_count);
}


fn part2(line: &String) {
    let char_count = get_char_count_after_distinct(14, line);
    utils::print_part_solution(2, char_count);
}
