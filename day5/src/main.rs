use itertools::Itertools;

fn main() {
    // Cut the input in 2 for easier parsing.
    const CARGO: &str = include_str!("../cargo.txt");
    const MOVES: &str = include_str!("../moves.txt");

    let mut stacks: Vec<Vec<char>> = Vec::new();
    stacks.resize(9, Vec::new());
    CARGO.lines().rev().for_each(|line| {
        for stack in 0..9 {
            let package = line.chars().nth(stack * 4 + 1).unwrap();
            if package != ' ' {
                stacks[stack].push(package);
            }
        }
    });

    type Move = (usize, usize, usize);
    let moves: Vec<Move> = MOVES.lines()
        .map(|line| line.split(' ')
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

    // Rearranging for part 1.
    for (num, from, to) in moves.iter() {
        // Taking items one by one.
        for _i in 0..*num+1 {
            let from_stack = stacks.get_mut(*from).unwrap().pop().unwrap();
            stacks.get_mut(*to).unwrap().push(from_stack);
        }
    }

    // Rearranging for part 2.
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
