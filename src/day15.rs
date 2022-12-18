use itertools::Itertools;
use std::{collections::HashSet, hash::Hash};

fn main() {
    const INPUT: &str = include_str!("../inputs/day15.txt");
    println!(
        "Line {} has {} positions that cannot contain a beacon",
        2_000_000,
        process_part1(INPUT, 2_000_000)
    );
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Coordinate {
    x: i32,
    y: i32,
}

#[derive(Debug)]
struct SensorData {
    sensor: Coordinate,
    beacon: Coordinate,
}

type SensorRawData = (i32, i32, i32, i32);
fn parse_sensors(input: &str) -> Vec<SensorData> {
    input
        .lines()
        .map(|line| {
            line.split([',', '=', ':'])
                .filter_map(|s| {
                    // Get rid of non number characters
                    match s.parse::<i32>() {
                        Ok(v) => Some(v),
                        Err(_) => None,
                    }
                })
                .collect_tuple::<SensorRawData>()
                .unwrap()
        })
        .collect::<Vec<SensorRawData>>()
        .iter()
        .map(|raw_data| SensorData {
            sensor: Coordinate {
                x: raw_data.0,
                y: raw_data.1,
            },
            beacon: Coordinate {
                x: raw_data.2,
                y: raw_data.3,
            },
        })
        .collect()
}

fn process_part1(input: &str, line: i32) -> usize {
    let sensors_data = parse_sensors(input);
    let mut no_beacons_here: Vec<i32> = Vec::new();
    sensors_data.iter().for_each(|data| {
        let distance =
            (data.beacon.x - data.sensor.x).abs() + (data.beacon.y - data.sensor.y).abs();
        if (data.sensor.y > line && data.sensor.y - distance < line)
            || (data.sensor.y < line && data.sensor.y + distance > line)
        {
            let nb_x_on_line = (distance - (data.sensor.y - line).abs()).abs();
            no_beacons_here.extend(data.sensor.x - nb_x_on_line..data.sensor.x + nb_x_on_line + 1);
        }
    });
    let all_beacons: HashSet<Coordinate> = HashSet::from_iter(
        sensors_data
            .iter()
            .map(|data| data.beacon)
            .collect::<Vec<Coordinate>>(),
    );
    let unique_on_line: HashSet<i32> = HashSet::from_iter(no_beacons_here.iter().cloned());
    unique_on_line.len()
        - all_beacons
            .iter()
            .filter(|&coordinates| coordinates.y == line)
            .count()
}

fn process_part2(_input: &str) -> u32 {
    0
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE_DATA: &str = "Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16
Sensor at x=13, y=2: closest beacon is at x=15, y=3
Sensor at x=12, y=14: closest beacon is at x=10, y=16
Sensor at x=10, y=20: closest beacon is at x=10, y=16
Sensor at x=14, y=17: closest beacon is at x=10, y=16
Sensor at x=8, y=7: closest beacon is at x=2, y=10
Sensor at x=2, y=0: closest beacon is at x=2, y=10
Sensor at x=0, y=11: closest beacon is at x=2, y=10
Sensor at x=20, y=14: closest beacon is at x=25, y=17
Sensor at x=17, y=20: closest beacon is at x=21, y=22
Sensor at x=16, y=7: closest beacon is at x=15, y=3
Sensor at x=14, y=3: closest beacon is at x=15, y=3
Sensor at x=20, y=1: closest beacon is at x=15, y=3";

    #[test]
    fn validate_example_input_1() {
        assert_eq!(process_part1(EXAMPLE_DATA, 10), 26);
    }

    #[test]
    fn validate_example_input_2() {
        assert_eq!(process_part2(EXAMPLE_DATA), 0);
    }
}
