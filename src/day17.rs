use core::time;
use std::{
    ops::{Add, AddAssign},
    thread,
};

fn main() {
    const INPUT: &str = include_str!("../inputs/day17.txt");
    println!("The tower is {} units tall", process_part1(INPUT));
}

#[derive(Debug, PartialEq, Eq)]
enum Direction {
    Left,
    Right,
    Down,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Space {
    Rock,
    Air,
}

#[derive(Debug)]
enum Shape {
    Minus,
    Plus,
    J,
    Bar,
    Square,
}

#[derive(Debug, Clone, Copy)]
struct Coord(i32, i32);

impl AddAssign for Coord {
    fn add_assign(&mut self, other: Self) {
        *self = Coord(self.0 + other.0, self.1 + other.1);
    }
}

impl Add for Coord {
    type Output = Coord;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0, self.1 + rhs.1)
    }
}

type Tower = Vec<Vec<Space>>;

fn parse_gas(input: &str) -> Vec<Direction> {
    input
        .chars()
        .filter_map(|c| {
            match c {
                '<' => Some(Direction::Left),
                '>' => Some(Direction::Right),
                _ => None,
            }
        })
        .collect::<Vec<Direction>>()
}

fn shape_coordinates(shape: &Shape) -> Vec<Coord> {
    match shape {
        Shape::Minus => vec![Coord(0, 0), Coord(1, 0), Coord(2, 0), Coord(3, 0)],
        Shape::Plus => vec![
            Coord(1, 0),
            Coord(0, 1),
            Coord(1, 1),
            Coord(2, 1),
            Coord(1, 2),
        ],
        Shape::J => vec![
            Coord(0, 0),
            Coord(1, 0),
            Coord(2, 0),
            Coord(2, 1),
            Coord(2, 2),
        ],
        Shape::Bar => vec![Coord(0, 0), Coord(0, 1), Coord(0, 2), Coord(0, 3)],
        Shape::Square => vec![Coord(0, 0), Coord(1, 0), Coord(0, 1), Coord(1, 1)],
    }
}

fn direction_coordinates(direction: &Direction) -> Coord {
    match direction {
        Direction::Left => Coord(-1, 0),
        Direction::Right => Coord(1, 0),
        Direction::Down => Coord(0, -1),
    }
}

fn step_shape(idx: u32) -> Shape {
    match idx % 5 {
        0 => Shape::Minus,
        1 => Shape::Plus,
        2 => Shape::J,
        3 => Shape::Bar,
        4 => Shape::Square,
        _ => unreachable!(),
    }
}

fn move_shape(
    shape_coord: &mut Coord,
    shape_type: &Shape,
    tower: &Tower,
    direction: &Direction,
) -> bool {
    let new_shape_coord = *shape_coord + direction_coordinates(direction);
    // Check if any of the shape parts are somewhere they can't be (over the edges or an other rock)
    let can_move = !shape_coordinates(shape_type).iter().any(|&coord| {
        let shape_part_coord = new_shape_coord + coord;
        shape_part_coord.0 < 0
            || shape_part_coord.0 > 6
            || (*direction == Direction::Down && shape_coord.1 == 0)
            || tower[shape_part_coord.1 as usize][shape_part_coord.0 as usize] == Space::Rock
    });
    if can_move {
        *shape_coord = new_shape_coord;
    }
    can_move
}

fn _draw_tower(tower: &Tower) {
    print!("\x1B[2J");
    for line in tower.iter().rev() {
        print!("|");
        for space in line.iter() {
            match space {
                Space::Air => print!("."),
                Space::Rock => print!("#"),
            }
        }
        print!("|");
        println!()
    }
    thread::sleep(time::Duration::from_millis(1000));
}

fn process_part1(input: &str) -> u32 {
    let gas = parse_gas(input);
    let mut gas_iter = gas.iter().cycle();
    let mut summit = 0_u32;
    let mut tower: Vec<Vec<Space>> = vec![vec![Space::Air; 7]; 3];
    tower.reserve(2022);

    for rock in 0..2022 {
        // Spawn the next rock.
        let shape = step_shape(rock);
        let mut shape_coord = Coord(2, summit as i32 + 3);

        // Add new lines to the tower
        if (shape_coord.1 + 5) >= tower.len() as i32 {
            for _ in 0..=(tower.len() + 5 - shape_coord.1 as usize) {
                tower.push(vec![Space::Air; 7]);
            }
        }
        // While the rock can move, retrieve the jet direction, apply it and then move down.
        // draw_tower(&tower);
        loop {
            let next_gas = gas_iter.next().unwrap();
            move_shape(&mut shape_coord, &shape, &tower, next_gas);
            if !move_shape(&mut shape_coord, &shape, &tower, &Direction::Down) {
                // If the rock cannot go down anymore, "print" it to the tower and calculate the new summit.
                shape_coordinates(&shape).iter().for_each(|part| {
                    let shape_part_coord = shape_coord + *part;
                    tower[shape_part_coord.1 as usize][shape_part_coord.0 as usize] = Space::Rock;
                });
                summit = tower
                    .iter()
                    .enumerate()
                    .map(|(y, line)| {
                        if line.iter().any(|&space| space == Space::Rock) {
                            y
                        } else {
                            0
                        }
                    })
                    .max()
                    .unwrap() as u32 + 1;
                break;
            }
        }
    }
    summit
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE_DATA: &str = ">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>";


    #[test]
    fn validate_example_input_1() {
        assert_eq!(process_part1(EXAMPLE_DATA), 3068);
    }

    // #[test]
    // fn validate_example_input_2() {
    //     assert_eq!(process_part2(EXAMPLE_DATA), 1514285714288);
    // }
}
