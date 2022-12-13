use std::collections::VecDeque;

const TARGET: char = 'E';

fn bfs(input: &str) -> i32 {
    let mut mountain: Vec<Vec<char>> = Vec::new();
    input.lines().for_each(|line| {
        mountain.push(line.chars().collect());
    });
    let mut visited = vec![vec![None; mountain[0].len()]; mountain.len()];
    
    let start_pos = get_start(&mountain);
    mountain[(start_pos.0) as usize][(start_pos.1) as usize] = 'a';

    let mut queue: VecDeque<(i32, i32)> = VecDeque::new();
    queue.push_back(start_pos);

    // Put some filler into the first location
    visited[start_pos.0 as usize][start_pos.1 as usize] = Some(start_pos);

    while let Some((y, x)) = queue.pop_front() {
        // Check if this position is the target
        if mountain[y as usize][x as usize] == TARGET {
            let mut path_taken = Vec::new();
            path_taken.push((y, x));

            let mut prev_x = x;
            let mut prev_y = y;
            while prev_x != start_pos.1 || prev_y != start_pos.0 {
                let (py, px) = visited[prev_y as usize][prev_x as usize].unwrap();
                path_taken.push((px, py));
                prev_y = py;
                prev_x = px;
            }

            return path_taken.len() as i32 - 1; //Some(path_taken.into_iter().rev().collect());
        }

        // Iterate over adjacent offsets
        for (dy, dx) in &[(1, 0), (0, 1), (-1, 0), (0, -1)] {
            // Check if offset is within bounds
            if x + dx < 0
                || y + dy < 0
                || (y + dy) as usize >= mountain.len()
                || (x + dx) as usize >= mountain[0].len()
            {
                continue;
            }

            // Check if offset points to valid location
            if !can_climb(
                    mountain[y as usize][x as usize],
                    mountain[(y + dy) as usize][(x + dx) as usize]
                )
            {
                continue;
            }

            if visited[(y + dy) as usize][(x + dx) as usize].is_some() {
                continue;
            }

            visited[(y + dy) as usize][(x + dx) as usize] = Some((y, x));
            queue.push_back((y + dy, x + dx));
        }
    }
    0
}

fn get_start(data: &Vec<Vec<char>>) -> (i32, i32) {
    for y in 0..data.len() {
        let pos = data[y].iter().position(|c| *c == 'S');
        if pos.is_some() {
            return (y as i32, pos.unwrap() as i32);
        }
    }
    (0, 0)
}

fn can_climb(from: char, to: char) -> bool {
    from == 'z' && to == TARGET || // Reaching the summit, yay !
    to <= from && to != TARGET || // Moving to a lower spot
    match (to as u8).checked_sub(from as u8) { 
        Some(v) => v <= 1, // Moving to a reachable spot
        None => false,
    }
}

fn main() {
    const INPUT: &str = include_str!("../inputs/day12.txt");

    println!("Shortest path is : {:?}", bfs(INPUT));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn validate_example_input_1() {
        assert_eq!(
            bfs(include_str!("../inputs/day12_ex.txt")),
            31
        );
    }

    println!("Found: {:?}", b);
    println!("Size is {}", b.unwrap().len() - 1);
}
