use aoc2022::{Range, RangeStack};
use itertools::Itertools;
use std::collections::HashSet;

fn main() {
    const INPUT: &str = include_str!("../inputs/day15.txt");
    println!(
        "Line {} has {} positions that cannot contain a beacon",
        2_000_000,
        process_part1(INPUT, 2_000_000)
    );
    println!(
        "The tuning frequency of the beacon is {}",
        process_part2(INPUT, 4_000_000)
    );
}

type Coordinate = (i32, i32);
type SensorRawData = (i32, i32, i32, i32);
struct SensorData {
    sensor: Coordinate,
    beacon: Coordinate,
}

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
            sensor: (raw_data.0, raw_data.1),
            beacon: (raw_data.2, raw_data.3),
        })
        .collect()
}

fn ranges_for_line(sensors_data: &[SensorData], line: i32, max_xy: i32) -> RangeStack {
    let mut ranges: Vec<Range> = Vec::new();
    sensors_data.iter().for_each(|data| {
        let distance =
            (data.beacon.0 - data.sensor.0).abs() + (data.beacon.1 - data.sensor.1).abs();
        if (data.sensor.1 >= line && data.sensor.1 - distance < line)
            || (data.sensor.1 <= line && data.sensor.1 + distance > line)
        {
            // Calculate the ranges of values that are on this line.
            let nb_x_on_line = distance - (data.sensor.1 - line).abs();
            if max_xy == 0 { // Values for part 1
                ranges.push(aoc2022::Range::new(
                    data.sensor.0 - nb_x_on_line,
                    data.sensor.0 + nb_x_on_line + 1,
                ));
            } else { // Values for part 2, capped at 0..max_xy
                ranges.push(aoc2022::Range::new(
                    (data.sensor.0 - nb_x_on_line).max(0),
                    (data.sensor.0 + nb_x_on_line).min(max_xy),
                ));
            }
        }
    });
    let range_stack: RangeStack = ranges.iter().collect();
    range_stack
}

fn process_part1(input: &str, line: i32) -> usize {
    let sensors_data = parse_sensors(input);
    let all_beacons: HashSet<Coordinate> = HashSet::from_iter(
        sensors_data
            .iter()
            .map(|data| data.beacon)
            .collect::<Vec<Coordinate>>(),
    );

    ranges_for_line(&sensors_data, line, 0)
        .ranges
        .iter()
        .map(|range| range.size())
        .sum::<usize>()
        - all_beacons
            .iter()
            .filter(|&coordinates| coordinates.1 == line)
            .count()
}

fn process_part2(input: &str, max_xy: i32) -> u64 {
    // Find the first line that has a hole
    let sensors_data = parse_sensors(input);
    for line in (0..max_xy).rev() {
        let range_stack = ranges_for_line(&sensors_data, line, max_xy);
        if range_stack.ranges.len() == 1 {
            continue;
        }
        return (range_stack.ranges[0].end + 1) as u64 * 4_000_000 + line as u64;
    }
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
        assert_eq!(process_part2(EXAMPLE_DATA, 20), 56_000_011);
    }
}
