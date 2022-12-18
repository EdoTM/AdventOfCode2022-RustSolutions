use std::{collections::HashSet, hash::Hash};

use crate::utils;

pub fn day14_solution(example: bool) {
    let lines = utils::get_lines_from_input_file(14, example);

    part1(&lines);
    part2(&lines);
}


const STARTING_POINT: Point = Point {x: 500, y: 0};

fn part1(lines: &Vec<String>) {
    let positions = get_points_from_input(lines);

    let mut simulation = SandSimulation::new(positions, STARTING_POINT);
    let sand_count = simulation.get_sand_count_before_falling_into_the_void();

    utils::print_part_solution(1, sand_count - 1);
}

fn part2(lines: &Vec<String>) {
    let positions = get_points_from_input(lines);

    let mut simulation = SandSimulation::new(positions, STARTING_POINT);
    let sand_count = simulation.get_sand_count_before_reaching_starting_point();

    utils::print_part_solution(2, sand_count);
}


struct SandSimulation {
    positions: HashSet<Point>,
    starting_point: Point,
    cur_point: Point
}
impl SandSimulation {
    pub fn new(positions: HashSet<Point>, starting_point: Point) -> Self {
        Self {
            positions,
            starting_point,
            cur_point: starting_point
        }
    }

    pub fn get_sand_count_before_falling_into_the_void(&mut self) -> u32 {
        let max_y = self.get_maximum_y();
        
        let mut sand_count = 1u32;

        while self.cur_point.y < max_y {
            let point_below = self.cur_point.plus(0, 1);
            if !self.positions.contains(&point_below) {
                self.cur_point = point_below;
                continue;
            }
        
            let point_down_left = self.cur_point.plus(-1, 1);
            if !self.positions.contains(&point_down_left) {
                self.cur_point = point_down_left;
                continue;
            }
        
            let point_down_right = self.cur_point.plus(1, 1);
            if !self.positions.contains(&point_down_right) {
                self.cur_point = point_down_right;
                continue;
            }

            self.positions.insert(self.cur_point);
            sand_count += 1;
            self.cur_point = self.starting_point;
        }

        sand_count
    }

    pub fn get_sand_count_before_reaching_starting_point(&mut self) -> u32 {
        let limit_y = self.get_maximum_y() + 2;
        
        let mut sand_count = 1u32;

        loop {
            if self.cur_point.y == limit_y - 1 {
                self.positions.insert(self.cur_point);
                sand_count += 1;
                self.cur_point = self.starting_point;
                continue;
            }

            let point_below = self.cur_point.plus(0, 1);
            if !self.positions.contains(&point_below) {
                self.cur_point = point_below;
                continue;
            }
        
            let point_down_left = self.cur_point.plus(-1, 1);
            if !self.positions.contains(&point_down_left) {
                self.cur_point = point_down_left;
                continue;
            }
        
            let point_down_right = self.cur_point.plus(1, 1);
            if !self.positions.contains(&point_down_right) {
                self.cur_point = point_down_right;
                continue;
            }

            if self.cur_point == self.starting_point {
                break;
            }

            self.positions.insert(self.cur_point);
            sand_count += 1;
            self.cur_point = self.starting_point;
        }

        sand_count
    }

    fn get_maximum_y(&self) -> u32 {
        self.positions
            .iter()
            .map(|p| p.y)
            .max()
            .unwrap()
    }
    
}


#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
struct Point {
    x: u32,
    y: u32
}
impl Point {
    pub fn from_coordinates(coords: &str) -> Self {
        let mut tokens = coords.split(",");
        let x = tokens.next().unwrap().parse().unwrap();
        let y = tokens.next().unwrap().parse().unwrap();
        
        Self {x, y}
    }

    pub fn plus(&self, x: i32, y: i32) -> Self {
        Self {
            x: (self.x as i32 + x as i32) as u32,
            y: (self.y as i32 + y as i32) as u32
        }
    }
}


fn get_points_from_input(lines: &Vec<String>) -> HashSet<Point> {
    let mut positions: HashSet<Point> = HashSet::new();

    for line in lines {
        let mut points = get_points_from_line(line);
        let mut old_point = points.next().unwrap();

        for point in points {
            for x in u32::min(old_point.x, point.x) ..= u32::max(old_point.x, point.x) {
                positions.insert(Point {x, y: point.y});
            } 
            for y in u32::min(old_point.y, point.y) ..= u32::max(old_point.y, point.y) {
                positions.insert(Point {x: point.x, y});
            } 
            old_point = point;
        }
    }

    positions
}

fn get_points_from_line<'a>(line: &'a str) -> impl Iterator<Item = Point> + 'a {
    line
        .split(" -> ")
        .map(|s| Point::from_coordinates(s))
}


