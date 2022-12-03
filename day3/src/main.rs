use std::{fs::File, io::BufRead};

fn count(priorities: &Vec<char>) -> usize {
    priorities.iter().map(|item| {
        match item {
            'a'..='z' => {
                *item as usize - 96
            },
            'A'..='Z' => {
                *item as usize - 64 + 26
            },
            _ => 0
        }
    }).sum()
}

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
    
    let mut priorities_pt2: Vec<char> = Vec::new();
    let mut group: Vec<Vec<char>> = Vec::new();
    for line in std::io::BufReader::new(File::open("input.txt").unwrap())
        .lines()
        .flatten()
    {
        if group.len() == 2 {
            let mut current_line: Vec<char> = line.chars().collect();
            current_line.retain(|&item| {
                group.get(0).unwrap().contains(&item) && group.get(1).unwrap().contains(&item)
            });
            current_line.dedup();
            priorities_pt2.append(&mut current_line);
            group.clear();
        } else {
            group.push(line.chars().collect());
        }
    }

    println!("Total of priorities for part 1 = {}", count(&priorities_pt1));
    println!("Total of grouped priorities for part 2 = {}", count(&priorities_pt2));
}
