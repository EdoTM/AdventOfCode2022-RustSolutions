use crate::utils;

pub fn day4_solution(example: bool) {
    let lines = utils::get_lines_from_input_file(4, example);

    let ranges = get_ranges(&lines);
    part1(&ranges);
    part2(&ranges);
}

struct Range {
    begin: u32,
    end: u32
}
impl Range {
    pub fn from_str(s: &str) -> Range {
        let range: Vec<u32> = s
            .split("-")
            .map(|t| t.parse::<u32>().unwrap())
            .collect();
        Self {
            begin: range[0],
            end: range[1]
        }
    }
    
    pub fn fully_contains(&self, other: &Range) -> bool {
        self.begin <= other.begin && self.end >= other.end
    }

    pub fn is_disjoint(&self, other: &Range) -> bool {
        other.begin > self.end || self.begin > other.end
    }

    pub fn overlaps_with(&self, other: &Range) -> bool {
        !self.is_disjoint(other)
    }
}

fn get_ranges(lines: &Vec<String>) -> Vec<(Range, Range)> {
    let mut out = Vec::<(Range, Range)>::new();

    for line in lines {
        let ranges: Vec<&str> = line
            .split(",")
            .collect();
        let range1 = Range::from_str(ranges[0]);
        let range2 = Range::from_str(ranges[1]);
        out.push((range1, range2));
    }

    out
}


fn part1(ranges: &Vec<(Range, Range)>) -> Option<u32> {
    
    let mut total: u32 = 0;

    for range_tuple in ranges {
        let (range1, range2) = range_tuple;

        if range1.fully_contains(&range2) || range2.fully_contains(&range1) {
            total += 1;
        } 
    }

    utils::print_part_solution(1, total);
    Some(total)
}

fn part2(ranges: &Vec<(Range, Range)>) {
   
    let mut total: u32 = 0;

    for range_tuple in ranges {
        let (range1, range2) = range_tuple;

        if range1.overlaps_with(&range2) {
            total += 1;
        } 
    }

    utils::print_part_solution(2, total);
}
