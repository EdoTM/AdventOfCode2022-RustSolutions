use std::ops::Index;

use crate::utils;



pub fn day8_solution(example: bool) {
    let lines = utils::get_lines_from_input_file(8, example);

    let grid = create_grid(&lines);
    part1(&grid);
    part2(&grid);
}

fn part1(grid: &Grid) {
    let mut visible_trees = 0u32;

    for i in 0..grid.rows {
        for j in 0..grid.columns {
            if is_visible(i, j, grid) {
                visible_trees += 1;
            }
        }
    }

    utils::print_part_solution(1, visible_trees);
}

fn part2(grid: &Grid) {
    let mut max_score = 0u32;

    for i in 0..grid.rows {
        for j in 0..grid.columns {
            let score = get_scenic_score(i, j, grid);
            if score > max_score {
                max_score = score;
            }
        }
    }

    utils::print_part_solution(2, max_score);
}


struct Grid {
    rows: u32,
    columns: u32,
    _grid: Vec<Vec<u8>>,
} 
impl Index<usize> for Grid {
    type Output = Vec<u8>;

    fn index(&self, index: usize) -> &Self::Output {
        &self._grid[index]
    }
}


fn create_grid(lines: &Vec<String>) -> Grid {
    let rows_count = lines.len();
    let columns_count = lines[0].len();

    let mut rows: Vec<Vec<u8>> = Vec::new();
    rows.reserve_exact(rows_count);
    
    for line in lines {
        let mut column: Vec<u8> = Vec::new();
        column.reserve_exact(columns_count);

        for c in line.chars() {
            let digit = c.to_digit(10).unwrap() as u8;
            column.push(digit);
        }

        rows.push(column);
    }

    Grid {
        _grid: rows,
        rows: rows_count as u32,
        columns: columns_count as u32
    }
}


fn is_on_boundary(i: u32, j: u32, grid: &Grid) -> bool {
    j == 0 || j == grid.columns - 1 || i == 0 || i == grid.rows - 1
}

fn is_visible(i: u32, j: u32, grid: &Grid) -> bool {
    if is_on_boundary(i, j, grid) {
        return true;
    }

    let i = i as usize;
    let j = j as usize;

    is_visible_from_top(i, j, grid)
    || is_visible_from_bottom(i, j, grid)
    || is_visible_from_left(i, j, grid)
    || is_visible_from_right(i, j, grid)
}

fn is_visible_from_top(i: usize, j: usize, grid: &Grid) -> bool {
    let elem = grid[i][j];
    for k in 0..i {
        if grid[k][j] >= elem {
            return false;
        }
    }
    
    true
}

fn is_visible_from_bottom(i: usize, j: usize, grid: &Grid) -> bool {
    let elem = grid[i][j];
    for k in i+1 .. grid.rows as usize {
        if grid[k][j] >= elem {
            return false;
        }
    }
    
    true
}

fn is_visible_from_left(i: usize, j: usize, grid: &Grid) -> bool {
    let elem = grid[i][j];
    for k in 0..j {
        if grid[i][k] >= elem {
            return false;
        }
    }

    true
}

fn is_visible_from_right(i: usize, j: usize, grid: &Grid) -> bool {
    let elem = grid[i][j];
    for k in j+1 .. grid.columns as usize {
        if grid[i][k] >= elem {
            return false;
        }
    }

    true
}


fn get_scenic_score(i: u32, j: u32, grid: &Grid) -> u32 {
    if is_on_boundary(i, j, grid) {
        return 0;
    }

    let i = i as usize;
    let j = j as usize;

    get_top_scenic_score(i, j, grid)
    * get_bottom_scenic_score(i, j, grid)
    * get_left_scenic_score(i, j, grid)
    * get_right_scenic_score(i, j, grid) 
}

fn get_top_scenic_score(i: usize, j: usize, grid: &Grid) -> u32 {
    let elem = grid[i][j];
    let mut view_dist = 0;

    for k in (0..i).rev() {
        view_dist += 1;
        if grid[k][j] >= elem {
            break;
        }
    }

    view_dist
}

fn get_bottom_scenic_score(i: usize, j: usize, grid: &Grid) -> u32 {
    let elem = grid[i][j];
    let mut view_dist = 0;

    for k in i+1 .. grid.rows as usize {
        view_dist += 1;
        if grid[k][j] >= elem {
            break;
        }
    }

    view_dist
}

fn get_left_scenic_score(i: usize, j: usize, grid: &Grid) -> u32 {
    let elem = grid[i][j];
    let mut view_dist = 0;

    for k in (0..j).rev() {
        view_dist += 1;
        if grid[i][k] >= elem {
            break;
        }
    }

    view_dist
}

fn get_right_scenic_score(i: usize, j: usize, grid: &Grid) -> u32 {
    let elem = grid[i][j];
    let mut view_dist = 0;

    for k in j+1 .. grid.columns as usize {
        view_dist += 1;
        if grid[i][k] >= elem {
            break;
        }
    }

    view_dist
}
