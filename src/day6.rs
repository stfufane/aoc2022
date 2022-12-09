use std::collections::HashSet;

// This solution feels totally not idiomatic, got to find something cleaner.
fn find_index_after_n_unique(input: &str, n: usize) -> usize {
    for i in 0..(input.len() - n) { 
        if input.get(i..i+n).unwrap().chars().collect::<HashSet<char>>().len() == n {
            return i + n;
        }
    }
    input.len()
}

fn main() {
    const INPUT: &str = include_str!("../inputs/day6.txt");

    println!("Start of packet is at {}", find_index_after_n_unique(INPUT, 4));
    println!("Start of message is at {}", find_index_after_n_unique(INPUT, 14));
}
