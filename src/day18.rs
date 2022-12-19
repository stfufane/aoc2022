use itertools::Itertools;
const INPUT: &str = include_str!("../inputs/day18.txt");

fn main() {
    println!("Nb visible faces = {}", process_part1(INPUT));
    println!("Nb visible faces = {}", process_part2(INPUT));
}

#[derive(Debug, PartialEq, Eq)]
struct Cube {
    x: i32,
    y: i32,
    z: i32,
}

impl Cube {
    fn is_adjacent(&self, other: &Cube) -> bool {
        other.x.abs_diff(self.x) + other.y.abs_diff(self.y) + other.z.abs_diff(self.z) == 1
    }
}

fn parse_cubes(input: &str) -> Vec<Cube> {
    input
        .lines()
        .map(|line| {
            line.split(',')
                .map(|s| s.parse::<i32>().unwrap())
                .collect_tuple::<(i32, i32, i32)>()
                .unwrap()
        })
        .map(|(x, y, z)| Cube { x, y, z })
        .collect::<Vec<Cube>>()
}

fn process_part1(input: &str) -> u32 {
    let cubes = parse_cubes(input);
    let mut faces = 0;
    for cube in cubes.iter() {
        faces += 6 - cubes.iter().filter(|other| other.is_adjacent(cube)).count();
    }
    faces as u32
}

fn add_surrounding_cube(
    surrounding_cubes: &mut Vec<Cube>,
    cubes: &[Cube],
    cube: Cube,
    first_loop: bool,
) {
    let cube_already_there = cubes
        .iter()
        .any(|c| c.x == cube.x && c.y == cube.y && c.z == cube.z);
    if first_loop
        // || (cube_already_there
        //     && cubes
        //         .iter()
        //         .filter(|other| other.is_adjacent(&cube))
        //         .count()
        //         == 0)
        || (!cube_already_there
            && surrounding_cubes.iter().any(|c| c.is_adjacent(&cube))
            && !surrounding_cubes
                .iter()
                .any(|c| c.x == cube.x && c.y == cube.y && c.z == cube.z))
    {
        surrounding_cubes.push(cube);
    }
}

fn process_part2(input: &str) -> u32 {
    let cubes = parse_cubes(input);
    let mut surrounding_cubes: Vec<Cube> = Vec::new();

    let mut x_left = cubes.iter().map(|cube| cube.x).min().unwrap() - 1;
    let mut x_right = cubes.iter().map(|cube| cube.x).max().unwrap() + 1;
    let mut y_bottom = cubes.iter().map(|cube| cube.y).min().unwrap() - 1;
    let mut y_top = cubes.iter().map(|cube| cube.y).max().unwrap() + 1;
    let mut z_front = cubes.iter().map(|cube| cube.z).min().unwrap() - 1;
    let mut z_rear = cubes.iter().map(|cube| cube.z).max().unwrap() + 1;
    let mut faces = 0;

    let mut first_loop = true;
    loop {
        // First add to the left edge and right edge
        for y in y_bottom..=y_top {
            for z in z_front..=z_rear {
                add_surrounding_cube(
                    &mut surrounding_cubes,
                    &cubes,
                    Cube { x: x_left, y, z },
                    first_loop,
                );
                add_surrounding_cube(
                    &mut surrounding_cubes,
                    &cubes,
                    Cube { x: x_right, y, z },
                    first_loop,
                );
            }
        }
        // Then the top and bottom
        for x in x_left..=x_right {
            for z in z_front..=z_rear {
                add_surrounding_cube(
                    &mut surrounding_cubes,
                    &cubes,
                    Cube { x, y: y_bottom, z },
                    first_loop,
                );
                add_surrounding_cube(
                    &mut surrounding_cubes,
                    &cubes,
                    Cube { x, y: y_top, z },
                    first_loop,
                );
            }
        }
        // Finally the front and rear
        for x in x_left..=x_right {
            for y in y_bottom..=y_top {
                add_surrounding_cube(
                    &mut surrounding_cubes,
                    &cubes,
                    Cube { x, y, z: z_front },
                    first_loop,
                );
                add_surrounding_cube(
                    &mut surrounding_cubes,
                    &cubes,
                    Cube { x, y, z: z_rear },
                    first_loop,
                );
            }
        }
        first_loop = false;
        if x_left <= x_right {
            x_left += 1;
            x_right -= 1;
        }
        if y_bottom <= y_top {
            y_bottom += 1;
            y_top -= 1;
        }
        if z_front <= z_rear {
            z_front += 1;
            z_rear -= 1;
        }
        if x_left > x_right && y_bottom > y_top && z_front > z_rear {
            break;
        }
    }

    // Print slice by slice
    // for x in cubes.iter().map(|cube| cube.x).min().unwrap() - 1
    //     ..=cubes.iter().map(|cube| cube.x).max().unwrap() + 1
    // {
    //     for y in cubes.iter().map(|cube| cube.y).min().unwrap() - 1
    //         ..=cubes.iter().map(|cube| cube.y).max().unwrap() + 1
    //     {
    //         for z in cubes.iter().map(|cube| cube.z).min().unwrap() - 1
    //             ..=cubes.iter().map(|cube| cube.z).max().unwrap() + 1
    //         {
    //             if surrounding_cubes
    //                 .iter()
    //                 .any(|cube| cube == &Cube { x, y, z })
    //             {
    //                 print!("[-]");
    //             } else if cubes.iter().any(|cube| cube == &Cube { x, y, z }) {
    //                 print!("[0]");
    //             } else {
    //                 print!("[ ]");
    //             }
    //         }
    //         println!();
    //     }
    //     println!();
    //     println!();
    // }

    for cube in cubes.iter() {
        faces += surrounding_cubes
            .iter()
            .filter(|other| other.is_adjacent(cube))
            .count();
    }
    faces as u32
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE_DATA: &str = "2,2,2
1,2,2
3,2,2
2,1,2
2,3,2
2,2,1
2,2,3
2,2,4
2,2,6
1,2,5
3,2,5
2,1,5
2,3,5";

    #[test]
    fn validate_example_input_1() {
        assert_eq!(process_part1(EXAMPLE_DATA), 64);
    }

    #[test]
    fn validate_example_input_2() {
        assert_eq!(process_part2(EXAMPLE_DATA), 58);
    }
}
