use std::collections::HashMap;

use crate::utils;

type DirStack<'a> = Vec<&'a str>;
type Sizes = HashMap<String, u32>;

pub fn day7_solution(example: bool) {
    let lines = utils::get_lines_from_input_file(7, example);
    
    let sizes: Sizes = get_directory_sizes(&lines);
    part1(&sizes);
    part2(&sizes);
}


fn part1(directory_sizes: &Sizes) {
    let size_of_dirs_with_size_at_most_100k: u32 =
        directory_sizes
            .values()
            .filter(|&&v| v <= 100_000)
            .sum::<u32>();

    utils::print_part_solution(1, size_of_dirs_with_size_at_most_100k);
}


fn part2(sizes: &Sizes) {
    const TOTAL_DISK_SPACE: u32 = 70_000_000;
    const MINIMUM_UNUSED_SPACE: u32 = 30_000_000;
    const MAXIMUM_USABLE_SPACE: u32 = TOTAL_DISK_SPACE - MINIMUM_UNUSED_SPACE;
    
    let used_space: u32 = sizes["/"];
    let minimum_space_to_free = used_space - MAXIMUM_USABLE_SPACE;

    let smallest_directory_to_delete = sizes
        .values()
        .filter(|&&size| size >= minimum_space_to_free)
        .min()
        .unwrap();

    utils::print_part_solution(2, smallest_directory_to_delete);
}


fn get_directory_sizes(lines: &Vec<String>) -> Sizes {
    let mut current_dir: DirStack = vec![""];
    let mut dir_sizes: Sizes = HashMap::new();

    for line in lines {
        if line.starts_with("$ cd") {
            match &line[5..] {
                "/" => current_dir = vec![""],
                ".." => _ = current_dir.pop(),
                dir => current_dir.push(dir),
            }
        } else if line_starts_with_number(line) {
            let file_size = get_file_size_from_line(line);
            update_file_size(&current_dir, &mut dir_sizes, file_size);
        }
    }

    dir_sizes
}

fn line_starts_with_number(line: &String) -> bool {
    line.chars().next().unwrap().is_numeric()
}

fn get_file_size_from_line(line: &String) -> u32 {
    line
        .split_whitespace()
        .next().unwrap()
        .parse().unwrap()
}

fn update_file_size(current_dir: &DirStack, dir_sizes: &mut Sizes, file_size: u32) {
    let mut absolute_position = String::new();
    for &directory in current_dir {
        absolute_position = absolute_position + directory + "/";
        let previous_value = dir_sizes
            .entry(absolute_position.to_string())
            .or_insert(0);
        *previous_value += file_size
    }
}
