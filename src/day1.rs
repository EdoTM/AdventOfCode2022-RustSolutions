use crate::utils;

pub fn day1_solution(example: bool) {
    let lines = utils::get_lines_from_input_file(1, example);
    part1(&lines);
    part2(&lines);
}

fn part1(lines: &Vec<String>) {
    let mut max: u32 = 0;
    let mut local_max: u32 = 0;
    for line in lines {
        if line.is_empty() {
            if local_max > max {
                max = local_max;
            }
            local_max = 0;
        } else {
            local_max += line.parse::<u32>().unwrap();
        }
    }
    utils::print_part_solution(1, max);
}

fn part2(lines: &Vec<String>) {
    let mut top_3: [u32; 3] = [0; 3];
    let mut local_max: u32 = 0;
    for line in lines {
        if line.is_empty() {
            sub_if_greater(&mut top_3, local_max);
            local_max = 0;
        } else {
            local_max += line.parse::<u32>().unwrap();
        }
        println!("{:?}", top_3);
    }
    sub_if_greater(&mut top_3, local_max);
    utils::print_part_solution(2, top_3.iter().sum::<u32>());
}

fn sub_if_greater(top_3: &mut [u32; 3], num: u32) {
    let &min = top_3.iter().min().unwrap();
    if num < min {
        return;
    }
    for i in 0..3 {
        if top_3[i] == min {
            top_3[i] = num;
            return;
        }
    }

}