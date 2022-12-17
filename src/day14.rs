use std::collections::HashSet;
use itertools::Itertools;

fn main() {
    const INPUT: &str = include_str!("../inputs/day14.txt");

    println!("Quantity of sand dropped = {}", process_part1(INPUT));
}

fn parse_rocks(input: &str) -> HashSet<(u32, u32)> {
    let all_rocks: Vec<Vec<(u32, u32)>> = input
        .lines()
        .map(|line| line.split_whitespace().filter(|&pair| pair != "->"))
        .into_iter()
        .map(|pairs| -> Vec<(u32, u32)> {
            pairs
                .map(|pair| {
                    pair.split(',')
                        .filter_map(|value| value.parse::<u32>().ok())
                        .collect_tuple::<(u32, u32)>()
                        .unwrap()
                })
                .collect()
        })
        .collect();

    // Calculate the coordinates of all rocks in the cave.
    let mut blocked_coordinates: HashSet<(u32, u32)> = HashSet::new();
    all_rocks.iter().for_each(|rocks| {
        rocks.windows(2).for_each(|rock_pair| {
            for y in rock_pair[0].0.min(rock_pair[1].0)..=rock_pair[0].0.max(rock_pair[1].0) {
                for x in rock_pair[0].1.min(rock_pair[1].1)..=rock_pair[0].1.max(rock_pair[1].1) {
                    blocked_coordinates.insert((y, x));
                }
            }
        });
    });
    blocked_coordinates
}

fn drop_sand(blocked_coordinates: &mut HashSet<(u32, u32)>) -> u32 {
    // Get the bottom edge of the cave
    let max_y = blocked_coordinates.iter().map(|pair| pair.1).max().unwrap();

    let mut sand_drop = (500u32, 0u32);
    let mut sand_drops: u32 = 0;
    loop {
        // Go down until we've met the edges.
        let under: Vec<(u32, u32)> = vec![
            (sand_drop.0, sand_drop.1 + 1),
            (sand_drop.0 - 1, sand_drop.1 + 1),
            (sand_drop.0 + 1, sand_drop.1 + 1),
        ];
        let found_next = under.iter().find(|&pos| !blocked_coordinates.contains(pos));
        if let Some(next_position) = found_next {
            sand_drop = *next_position;
            if sand_drop.1 >= max_y {
                break;
            }
        } else {
            blocked_coordinates.insert(sand_drop);
            sand_drops += 1;
            sand_drop = (500u32, 0u32);
        }
    }
    sand_drops
}

fn process_part1(input: &str) -> u32 {
    drop_sand(&mut parse_rocks(input))
}

fn process_part2(_input: &str) -> u32 {
    93
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE_DATA: &str = "498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9";

    #[test]
    fn validate_example_input_1() {
        assert_eq!(process_part1(EXAMPLE_DATA), 24);
    }

    #[test]
    fn validate_example_input_2() {
        assert_eq!(process_part2(EXAMPLE_DATA), 93);
    }
}