use std::collections::HashSet;

// This solution feels totally not idiomatic, got to find something cleaner.
fn find_index_after_n_unique(input: &str, n: usize) -> usize {
    let mut i: usize = 0;
    while i < input.len() - n {
        let slice = input.get(i..i+n).unwrap();
        let unique: HashSet<char> = slice.chars().collect();
        if unique.len() == n {
            return i + n;
        }
        i += 1;
    }
    input.len()
}

fn main() {
    const INPUT: &str = include_str!("../input.txt");

    println!("Start of packet is at {}", find_index_after_n_unique(INPUT, 4));
    println!("Start of message is at {}", find_index_after_n_unique(INPUT, 14));
}
