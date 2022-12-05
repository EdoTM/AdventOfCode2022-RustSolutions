use std::collections::HashSet;

use crate::utils;

pub fn day3_solution(example: bool) {
    let lines = utils::get_lines_from_input_file(3, example);

    part1(&lines);
    part2(&lines);
}

fn get_priority(letter: char) -> u8 {
    if letter.is_ascii_lowercase() {
        letter as u8 - 'a' as u8 + 1
    } else {
        letter as u8 - 'A' as u8 + 27
    }
}

fn part1(lines: &Vec<String>) {
    let mut total = 0u32;

    for line in lines {
        let len = line.len();

        let first_half = &line[..len/2];
        let second_half = &line[len/2..];

        let first_half_set: HashSet<char> = first_half.chars().collect();
        let second_half_set: HashSet<char> = second_half.chars().collect();

        let intersection = first_half_set
            .intersection(&second_half_set)
            .next()
            .unwrap();
        let priority = get_priority(*intersection);
        total += priority as u32;
    }

    utils::print_part_solution(1, total);
} 

fn part2(lines: &Vec<String>) {
    let mut i = 0usize;
    let input_len = lines.len();
    let mut total = 0u32;

    while i < input_len {
        let set1: HashSet<char> = lines[i].chars().collect();
        let set2: HashSet<char> = lines[i + 1].chars().collect();
        let set3: HashSet<char> = lines[i + 2].chars().collect();
        i += 3;

        let intersection = *set1
            .intersection(&set2)
            .cloned()  // it would be better without clone
            .collect::<HashSet<_>>()
            .intersection(&set3)
            .next()
            .unwrap();

        let priority = get_priority(intersection);
        total += priority as u32;       
    }

    utils::print_part_solution(2, total);
}