use serde_json::{json, Value};
use std::cmp::Ordering;

fn main() {
    const INPUT: &str = include_str!("../inputs/day13_ex.txt");
    println!("Part 1 result is {}", process_part1(INPUT));
    println!("Part 2 is {}", process_part2(INPUT));
}

fn cmp_pair(left: &Value, right: &Value) -> Option<bool> {
    let mut idx = 0;

    // Left is empty and right is not.
    if left.as_array().unwrap().is_empty() && !right.as_array().unwrap().is_empty() {
        return Some(true);
    }

    while idx
        < left
            .as_array()
            .unwrap()
            .len()
            .min(right.as_array().unwrap().len())
    {
        let mut valid: Option<bool> = None;
        let lv = left.as_array().unwrap().get(idx).unwrap();
        let rv = right.as_array().unwrap().get(idx).unwrap();
        if lv.is_i64() && rv.is_i64() {
            valid = match lv.as_i64().unwrap().cmp(&rv.as_i64().unwrap()) {
                Ordering::Greater => Some(false),
                Ordering::Equal => None,
                Ordering::Less => Some(true),
            };
            println!("Compared 2 ints and result is {:?}", valid);
        }

        if lv.is_array() && rv.is_array() {
            valid = cmp_pair(lv, rv);
        } else if lv.is_array() && !rv.is_array() {
            let rva = json!([rv]);
            valid = cmp_pair(lv, &rva);
        } else if !lv.is_array() && rv.is_array() {
            let lva = json!([lv]);
            valid = cmp_pair(&lva, rv);
        }

        if let Some(result) = valid {
            return Some(result);
        }
        idx += 1;
    }

    if idx < left.as_array().unwrap().len() {
        Some(false)
    } else if idx < right.as_array().unwrap().len() {
        Some(true)
    } else {
        None
    }
}

fn process_part1(input: &str) -> i32 {
    input
        .lines()
        .collect::<Vec<&str>>()
        .chunks(3)
        .map(|lines| -> (Value, Value) {
            (
                serde_json::from_str(lines[0]).unwrap(),
                serde_json::from_str(lines[1]).unwrap(),
            )
        })
        .enumerate()
        .filter(|(_, pair)| cmp_pair(&pair.0, &pair.1).unwrap())
        .map(|(index, _)| (index + 1) as i32)
        .sum()
}

fn process_part2(_input: &str) -> i32 {
    0
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn validate_example_input_1() {
        assert_eq!(process_part1(include_str!("../inputs/day13_ex.txt")), 13);
    }

    #[test]
    fn validate_example_input_2() {
        assert_eq!(process_part2(include_str!("../inputs/day13_ex.txt")), 140);
    }
}
