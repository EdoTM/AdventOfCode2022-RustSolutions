use std::collections::{VecDeque, HashSet};

use crate::utils;

pub fn day12_solution(example: bool) {
    let lines = utils::get_lines_from_input_file(12, example);

    let grid = create_grid_from_lines(&lines);
    part1(&grid);
    part2(&grid);
}


fn part1(grid: &Grid) {
    let shortest_path = get_shortest_path_from_start_to_end(grid);
    utils::print_part_solution(1, shortest_path);
}

fn part2(grid: &Grid) {
    let shortest_path = get_shortest_path_from_end_to_a_low_point(grid);
    utils::print_part_solution(2, shortest_path);
}


enum SearchType {
    Normal,
    Inverse
}

fn get_shortest_path_from_start_to_end(grid: &Grid) -> u32 {
    get_shortest_path(grid, grid.start, END, SearchType::Normal)
}

fn get_shortest_path_from_end_to_a_low_point(grid: &Grid) -> u32 {
    get_shortest_path(grid, grid.end, 'a', SearchType::Inverse)
}


fn create_grid_from_lines(lines: &Vec<String>) -> Grid {
    let mut grid: Vec<Vec<char>> = Vec::new();

    for line in lines {
        let row = line
            .chars()
            .collect();
        grid.push(row);
    }

    Grid::new(grid)
}


type Node = (usize, usize);

struct Grid {
    _columns: Vec<Vec<char>>,
    size: (usize, usize),
    start: Node,
    end: Node
}

const START: char = 'S';
const END: char = 'E';

#[derive(PartialEq)]
enum BfsSearchItem {
    Value(Node),
    IncreaseDistance
}

fn get_shortest_path(grid: &Grid, start_node: Node, end_value: char, search_type: SearchType) -> u32 {
    use BfsSearchItem::*;

    let mut nodes_to_visit: VecDeque<BfsSearchItem> = VecDeque::new();
    let mut visited_nodes: HashSet<Node> = HashSet::new(); 
    
    nodes_to_visit.push_back(Value(start_node));
    nodes_to_visit.push_back(IncreaseDistance);

    let mut path_len = 0u32;

    while let Some(item) = nodes_to_visit.pop_front() {
        match item {
        IncreaseDistance => {
            nodes_to_visit.push_back(IncreaseDistance);
            path_len += 1;
            continue;
        },
        Value(node) => {
            visited_nodes.insert(node);

            for neighbor in grid.get_neighbors_of(node) {
                if visited_nodes.contains(&neighbor) || nodes_to_visit.contains(&Value(neighbor)) {
                    continue;
                }
                match search_type {
                    SearchType::Normal => if !is_reachable_from(node, neighbor, grid) { 
                        continue; 
                    },
                    SearchType::Inverse => if !is_reachable_from(neighbor, node, grid) {
                        continue;
                    }
                }
                if grid[neighbor] == end_value {
                    return path_len + 1;
                }
                nodes_to_visit.push_back(Value(neighbor));
            }
        }
        }
    }

    path_len
}

fn is_reachable_from(from: Node, node: Node, grid: &Grid) -> bool {
    let e_from = get_elevation(grid[from]);
    let e_to = get_elevation(grid[node]);

    e_to <= e_from + 1
}

fn get_elevation(c: char) -> u8 {
    match c {
        START => get_elevation('a'),
        END => get_elevation('z'),
        _ => c as u8 - 'a' as u8
    }
}


impl Grid {
    pub fn new(columns: Vec<Vec<char>>) -> Self {
        let size = (columns.len(), columns[0].len());
        let (start, end) = Self::get_starting_and_ending_positions(&columns, size);
        
        Self {
            _columns: columns,
            size,
            start,
            end
        }
    }

    fn get_starting_and_ending_positions(columns: &Vec<Vec<char>>, (size_x, size_y): Node) -> (Node, Node) {
        let mut start = None;
        let mut end = None;

        for i in 0..size_x {
            for j in 0..size_y {
                match columns[i][j] {
                    START => start = Some((i, j)),
                    END => end = Some((i, j)),
                    _ => continue
                }
            }
        }

        (start.expect("There was no starting position in the grid."),
        end.expect("There was no ending position in the grid."))

    }

    pub fn get_neighbors_of(&self, node: Node) -> impl Iterator<Item = Node> {
        let mut neighbors: Vec<Node> = Vec::new();
        let (size_x, size_y) = self.size;

        if node.0 > 0 {
            neighbors.push((node.0 - 1, node.1));
        }
        if node.0 < size_x - 1 {
            neighbors.push((node.0 + 1, node.1));
        }
        if node.1 > 0 {
            neighbors.push((node.0, node.1 - 1));
        }
        if node.1 < size_y - 1 {
            neighbors.push((node.0, node.1 + 1));
        }
    
        neighbors.into_iter()
    }
}
impl std::ops::Index<Node> for Grid {
    type Output = char;

    fn index(&self, index: Node) -> &Self::Output {
        let (i, j) = index;

        &self._columns[i][j]
    }
}
