use std::collections::VecDeque;

const TARGET: char = 'E';

fn bfs(mountain: &Vec<Vec<char>>, start_pos: (i32, i32)) -> i32 {
    let mut visited = vec![vec![None; mountain[0].len()]; mountain.len()];

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
            return path_taken.len() as i32 - 1;
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

fn can_climb(from: char, to: char) -> bool {
    from == 'S' || // We can go anywhere from the start
    from == 'z' && to == TARGET || // Reaching the summit, yay !
    to <= from && to != TARGET || // Moving to a lower spot
    match (to as u8).checked_sub(from as u8) { 
        Some(v) => v <= 1, // Moving to a reachable spot
        None => false,
    }
}

fn get_mountain(input: &str) -> Vec<Vec<char>> {
    let mut mountain: Vec<Vec<char>> = Vec::new();
    input.lines().for_each(|line| {
        mountain.push(line.chars().collect());
    });
    mountain
}

fn get_start(mountain: &[Vec<char>]) -> (i32, i32) {
    for (y, row) in mountain.iter().enumerate() {
        if let Some(x) = row.iter().position(|c| *c == 'S') {
            return (y as i32, x as i32);
        }
    }
    (0, 0)
}

fn get_starts(mountain: &[Vec<char>]) -> Vec<(i32, i32)> {
    let mut starts: Vec<(i32, i32)> = Vec::new();
    for (y, row) in mountain.iter().enumerate() {
        let mut line_starts = row.iter()
            .enumerate()
            .filter(|(_, &c)| c == 'a')
            .map(|(x, _)| (y as i32, x as i32))
            .collect::<Vec<(i32, i32)>>();
        starts.append(&mut line_starts);
    }
    starts
}

fn best_path(mountain: &Vec<Vec<char>>) -> i32 {
    get_starts(mountain).iter().map(|pos| {
        bfs(mountain, *pos)
    }).filter(|path| *path > 0).min().unwrap()
}

fn main() {
    let mountain = get_mountain(include_str!("../inputs/day12.txt"));

    println!("Shortest path for part 1 is : {}", bfs(&mountain, get_start(&mountain)));
    println!("Shortest path for part 2 is : {}", best_path(&mountain));
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE_DATA: &str = "Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi";

    #[test]
    fn validate_example_input_1() {
        let mountain = get_mountain(EXAMPLE_DATA);
        assert_eq!(
            bfs(&mountain, get_start(&mountain)),
            31
        );
    }

    #[test]
    fn validate_example_input_2() {
        let mountain = get_mountain(EXAMPLE_DATA);
        assert_eq!(
            best_path(&mountain),
            29
        );
    }
}
