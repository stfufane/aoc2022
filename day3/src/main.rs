use std::{fs::File, io::BufRead};

fn main() {
    // Part 1
    let mut priorities_pt1: Vec<char> = Vec::new();
    for line in std::io::BufReader::new(File::open("input.txt").unwrap())
        .lines()
        .flatten()
    {
        let first_pocket: Vec<char> = line.chars().take(line.len() / 2).collect();
        let mut second_pocket: Vec<char> = line.chars().rev().take(line.len() / 2).collect();
        second_pocket.sort();
        second_pocket.dedup();
        for item in second_pocket.iter() {
            if first_pocket.contains(&item) {
                priorities_pt1.push(item.to_owned());
            }
        }
    }

    let total_pt1: usize = priorities_pt1.iter().map(|item| {
        match item {
            'a'..='z' => {
                *item as usize - 96
            },
            'A'..='Z' => {
                *item as usize - 64 + 26
            },
            _ => 0
        }
    }).sum();
    println!("Total of duplicates = {}", total_pt1);
}
