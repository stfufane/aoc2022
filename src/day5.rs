use itertools::Itertools;

type Move = (usize, usize, usize);

fn main() {
    // Cut the input in 2 for easier parsing.
    const CARGO: &str = include_str!("../inputs/day5_cargo.txt");
    const MOVES: &str = include_str!("../inputs/day5_moves.txt");

    let mut stacks: Vec<Vec<char>> = Vec::new();
    stacks.resize(9, Vec::new());
    CARGO.lines().rev().for_each(|line| {
        for (idx, stack) in stacks.iter_mut().enumerate().take(9) {
            let package = line.chars().nth(idx * 4 + 1).unwrap();
            if !package.is_whitespace()  {
                stack.push(package);
            }
        }
    });

    let moves: Vec<Move> = MOVES.lines()
        .map(|line| line.split_whitespace()
        .filter_map(|s| { 
            // Get rid of non number characters
            match s.parse::<usize>() {
                Ok(v) => Some(v - 1), // -1 for cleaner indexing in loops
                Err(_) => None
            }
        })
        .collect_tuple()
        .unwrap()
    ).collect();

    println!("Part 1");
    arrange_part1(stacks.clone(), &moves);
    println!("\nPart 2");
    arrange_part2(stacks.clone(), &moves);
}

fn arrange_part1(mut stacks: Vec<Vec<char>>, moves: &[Move]) {
    for (num, from, to) in moves.iter() {
        // Taking items one by one.
        for _i in 0..*num+1 {
            let from_stack = stacks.get_mut(*from).unwrap().pop().unwrap();
            stacks.get_mut(*to).unwrap().push(from_stack);
        }
    }
    for stack in stacks.iter() {
        print!("{}", stack.last().unwrap());
    }
}

fn arrange_part2(mut stacks: Vec<Vec<char>>, moves: &[Move]) {
    for (num, from, to) in moves.iter() {
        let from_stack = stacks.get_mut(*from).unwrap();
        // Taking a block of items.
        let top_packets = from_stack.split_off(from_stack.len() - (*num + 1));
        stacks.get_mut(*to).unwrap().append(&mut top_packets.to_owned());
    }

    for stack in stacks.iter() {
        print!("{}", stack.last().unwrap());
    }
}