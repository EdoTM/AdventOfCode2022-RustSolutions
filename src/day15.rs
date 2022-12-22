use std::collections::HashSet;

use crate::utils;

pub fn day15_solution(example: bool) {
    let lines = utils::get_lines_from_input_file(15, example);

    let datapoints = get_datapoints_from_input(&lines);
    if example {
        part1(&datapoints, 10);
        part2(&datapoints, 20);
    } else {
        part1(&datapoints, 2_000_000);
        part2(&datapoints, 4_000_000);
    }
}

fn part1(datapoints: &Vec<DataPoint>, y_coord: i32) {
    let mut checked_positions: HashSet<i32> = HashSet::new();
    let mut beacons_positions: HashSet<i32> = HashSet::new();
    
    for datapoint in datapoints {
        if beacon_is_at_y(datapoint, y_coord) {
            beacons_positions.insert(datapoint.closest_beacon.0);
        }

        let (sensor_x, sensor_y) = datapoint.sensor_position;
        let distance_from_beacon = datapoint.get_distance_from_beacon();
        
        let distance_y = sensor_y.abs_diff(y_coord);
        if distance_y > distance_from_beacon {
            continue;
        }

        let remaining_distance = (distance_from_beacon - distance_y) as i32;
        checked_positions.extend(
            sensor_x - remaining_distance ..= sensor_x + remaining_distance
        );
    }

    let positions_without_beacon = checked_positions
        .difference(&beacons_positions)
        .count();

    utils::print_part_solution(1, positions_without_beacon);
}

fn beacon_is_at_y(datapoint: &DataPoint, y_coord: i32) -> bool {
    datapoint.closest_beacon.1 == y_coord
}


type Point = (i32, i32);

fn part2(datapoints: &Vec<DataPoint>, max_coord: i32) {
    let mut point = (0i32, 0i32);

    while let Some(new_point) = get_next_point(&point, datapoints) {
        if new_point.0 > max_coord {
            point.0 = 0;
            point.1 += 1;
            continue;
        }
        point = new_point;
    }

    let tuning_frequency = point.0 as u128 * 4_000_000 + point.1 as u128;

    utils::print_part_solution(2, tuning_frequency);
}

fn get_next_point(point: &Point, datapoints: &Vec<DataPoint>) -> Option<Point> {
    for datapoint in datapoints {
        let distance_sensor_beacon = datapoint.get_distance_from_beacon();
        let distance_point_sensor = get_manhattan_distance(point, &datapoint.sensor_position);
        if distance_point_sensor > distance_sensor_beacon {
            continue;
        }
        let delta_y = point.1.abs_diff(datapoint.sensor_position.1) as i32;
        let new_x = datapoint.sensor_position.0 + distance_sensor_beacon as i32 - delta_y + 1;
        return Some((new_x, point.1));
    }
    return None;
}

fn get_manhattan_distance(point1: &Point, point2: &Point) -> u32 {
    let delta_x = point1.0.abs_diff(point2.0);
    let delta_y = point1.1.abs_diff(point2.1);
    
    delta_x + delta_y
}


fn get_datapoints_from_input(lines: &Vec<String>) -> Vec<DataPoint> {
    lines
        .iter()
        .map(
            |line| line
                    .split(&[',', ' ', ':', '='])
                    .filter_map(|s| s.parse::<i32>().ok())
        )
        .map(DataPoint::from_iter)
        .collect()
}


struct DataPoint {
    sensor_position: Point,
    closest_beacon: Point
}
impl DataPoint {
    pub fn from_iter(mut iter: impl Iterator<Item = i32>) -> Self {
        Self {
            sensor_position: (iter.next().unwrap(), iter.next().unwrap()),
            closest_beacon: (iter.next().unwrap(), iter.next().unwrap()),
        }
    }

    pub fn get_distance_from_beacon(&self) -> u32 {
        get_manhattan_distance(&self.closest_beacon, &self.sensor_position)
    }
}
