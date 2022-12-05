import argparse

DAY_SRC_TEMPLATE = """use crate::utils;

pub fn day{0}_solution(example: bool) {{
    let lines = utils::get_lines_from_input_file({0}, example);

    part1(&lines);
    part2(&lines);
}}

fn part1(lines: &Vec<String>) {{
    // TODO Solve part 1
    utils::print_part_solution(1, "TODO");
}}

fn part2(lines: &Vec<String>) {{
    // TODO Solve part 2
    utils::print_part_solution(2, "TODO");
}}
"""

def main():
    parser = argparse.ArgumentParser(description="Create a new day")
    parser.add_argument("day", type=int, help="The day to create")
    args = parser.parse_args()

    day = args.day
    if day < 1 or day > 25:
        print("Day must be between 1 and 25")
        return
    
    day_src = DAY_SRC_TEMPLATE.format(day)

    try:
        with open(f"src/day{day}.rs", "x") as f:
            f.write(day_src)

        with open(f"examples/day{day}_example.txt", "x") as f:
            f.write("")

        with open(f"inputs/day{day}_input.txt", "x") as f:
            f.write("")
    except FileExistsError:
        print("Day already exists, aborting.")
        exit(1)
    
    with open("src/main.rs", "r") as f:
        lines = f.readlines()

    
    for i in range(len(lines)):
        if lines[i].strip() == "":
            lines.insert(i, f"mod day{day};\n")
            break
    for i in range(len(lines)):
        if lines[i].strip().startswith(f"{day-1} =>"):
            lines.insert(i+1, f"        {day} => day{day}::day{day}_solution(example),\n")
            break

    with open("src/main.rs", "w") as f:
        f.writelines(lines)

    print(f"Created day {day}!")

if __name__ == '__main__':
    main()