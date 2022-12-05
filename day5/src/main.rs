use std::collections::VecDeque;

use itertools::Itertools;

fn main() {
    // Cut the input in 2 for easier parsing.
    const CARGO: &str = include_str!("../cargo.txt");
    const MOVES: &str = include_str!("../moves.txt");

    let mut stacks: Vec<VecDeque<char>> = Vec::new();
    stacks.resize(9, VecDeque::new());

    CARGO.lines().for_each(|line| {
        for stack in 0..9 {
            let package = line.chars().nth(stack * 4 + 1).unwrap();
            if package != ' ' {
                // Reading from top to bottom, so I insert at the beginning.
                stacks[stack].push_front(package);
            }
        }
    });

    type Move = (u8, u8, u8);
    let moves: Vec<Move> = MOVES.lines()
        .map(|line| line.split(' ')
        .filter_map(|s| { 
            // Get rid of non number characters
            match s.parse() {
                Ok(v) => Some(v),
                Err(_) => None
            }
        })
        .collect_tuple()
        .unwrap()
    ).collect();

    // Rearranging for part 1.
    for (num, from, to) in moves.iter() {
        // Taking items one by one.
        for _i in 0..*num {
            let from_stack = stacks.get_mut(*from as usize - 1).unwrap().pop_back().unwrap();
            stacks.get_mut(*to as usize - 1).unwrap().push_back(from_stack);
        }
    }

    // Rearranging for part 2.
    for (num, from, to) in moves.iter() {
        let from_stack = stacks.get_mut(*from as usize - 1).unwrap();
        // Taking a block of items.
        let top_packets = from_stack.split_off(from_stack.len() - *num as usize);
        stacks.get_mut(*to as usize - 1).unwrap().append(&mut top_packets.to_owned());
    }

    for stack in stacks.iter() {
        print!("{}", stack.back().unwrap());
    }
}
