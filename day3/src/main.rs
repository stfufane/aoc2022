use std::{fs::File, io::BufRead, collections::HashSet};

fn priority_value(letter: &char) -> usize {
    match letter {
        'a'..='z' => {
            *letter as usize - 96
        },
        'A'..='Z' => {
            *letter as usize - 64 + 26
        },
        _ => 0
    }
}

fn main() {
    // Part 1
    let mut priorities_pt1: usize = 0;
    for line in std::io::BufReader::new(File::open("input.txt").unwrap())
        .lines()
        .flatten()
    {
        let first_pocket: HashSet<char> = line.chars().take(line.len() / 2).collect();
        let second_pocket: HashSet<char> = line.chars().rev().take(line.len() / 2).collect();
        for item in second_pocket.iter() {
            if first_pocket.contains(item) {
                priorities_pt1 += priority_value(item);
                break;
            }
        }
    }
    
    // Part 2
    let mut priorities_pt2: usize = 0;
    for line in std::io::BufReader::new(File::open("input.txt").unwrap())
        .lines()
        .flatten()
        .collect::<Vec<String>>()
        .chunks(3)
    {
        for item in line[0].chars().collect::<HashSet<char>>().iter() {
            if line[1].chars().collect::<HashSet<char>>().contains(item) && line[2].chars().collect::<HashSet<char>>().contains(item) {
                priorities_pt2 += priority_value(item);
                break;
            }
        }
    }

    println!("Total of priorities for part 1 = {}", priorities_pt1);
    println!("Total of grouped priorities for part 2 = {}", priorities_pt2);
}
