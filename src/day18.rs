use itertools::Itertools;
const INPUT: &str = include_str!("../inputs/day18.txt");

fn main() {
    println!("Nb visible faces = {}", process_part1(INPUT));
    println!("Nb visible faces = {}", process_part2(INPUT));
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct Cube {
    x: i32,
    y: i32,
    z: i32,
    element: Element,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Element {
    BOUNDS,
    LAVA,
    WATER,
    AIR,
}
type Coordinates = (i32, i32, i32);

impl Cube {
    fn new(coords: (i32, i32, i32), element: Element) -> Self {
        Cube {
            x: coords.0,
            y: coords.1,
            z: coords.2,
            element,
        }
    }

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
                .collect_tuple::<Coordinates>()
                .unwrap()
        })
        .map(|(x, y, z)| Cube::new((x, y, z), Element::LAVA))
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

fn neighbours(coords: Coordinates) -> Vec<Coordinates> {
    vec![
        (coords.0 - 1, coords.1, coords.2),
        (coords.0 + 1, coords.1, coords.2),
        (coords.0, coords.1 - 1, coords.2),
        (coords.0, coords.1 + 1, coords.2),
        (coords.0, coords.1, coords.2 - 1),
        (coords.0, coords.1, coords.2 + 1),
    ]
}

fn is_cube_there(cubes: &[Cube], coords: Coordinates) -> bool {
    cubes.iter().any(|c| (c.x, c.y, c.z) == coords)
}

fn is_coords_taken(cubes: &[Cube], coords: Coordinates) -> bool {
    cubes
        .iter()
        .any(|c| c.x == coords.0 && c.y == coords.1 && c.z == coords.2)
}

fn is_in_bounds(coords: Coordinates, bounds: Coordinates) -> bool {
    coords.0 >= 0
        && coords.0 <= bounds.0
        && coords.1 >= 0
        && coords.1 <= bounds.1
        && coords.2 >= 0
        && coords.2 <= bounds.2
}

fn is_water_or_bounds(cubes: &[Cube], coords: Coordinates) -> bool {
    let element = get_element(&cubes, coords);
    element == Element::WATER || element == Element::BOUNDS
}

fn get_element(cubes: &[Cube], coords: Coordinates) -> Element {
    cubes
        .iter()
        .find(|&c| c.x == coords.0 && c.y == coords.1 && c.z == coords.2)
        .map(|found| found.element)
        .unwrap_or(Element::AIR)
}

fn process_part2(input: &str) -> u32 {
    let mut cubes = parse_cubes(input);
    let x_max = cubes.iter().map(|cube| cube.x).max().unwrap() + 1;
    let y_max = cubes.iter().map(|cube| cube.y).max().unwrap() + 1;
    let z_max = cubes.iter().map(|cube| cube.z).max().unwrap() + 1;
    let bounds = (x_max, y_max, z_max);

    // Add the bounds around the droplet
    for y in -1..=y_max {
        for z in -1..=z_max {
            if !is_coords_taken(&cubes, (-1, y, z)) {
                cubes.push(Cube::new((-1, y, z), Element::BOUNDS));
            }
            if !is_coords_taken(&cubes, (x_max, y, z)) {
                cubes.push(Cube::new((x_max, y, z), Element::BOUNDS));
            }
        }
    }
    for x in -1..=x_max {
        for z in -1..=z_max {
            if !is_coords_taken(&cubes, (x, -1, z)) {
                cubes.push(Cube::new((x, -1, z), Element::BOUNDS));
            }
            if !is_coords_taken(&cubes, (x, y_max, z)) {
                cubes.push(Cube::new((x, y_max, z), Element::BOUNDS));
            }
        }
    }
    for x in -1..=x_max {
        for y in -1..=y_max {
            if !is_coords_taken(&cubes, (x, y, -1)) {
                cubes.push(Cube::new((x, y, -1), Element::BOUNDS));
            }
            if !is_coords_taken(&cubes, (x, y, z_max)) {
                cubes.push(Cube::new((x, y, z_max), Element::BOUNDS));
            }
        }
    }

    let mut water_drops: Vec<Cube> =
        vec![Cube::new((x_max - 1, y_max - 1, z_max - 1), Element::WATER)];
    // Flood the bounds with water.
    loop {
        let mut water_drop = water_drops.pop().unwrap();
        cubes.push(water_drop.clone());
        neighbours((water_drop.x, water_drop.y, water_drop.z))
            .iter()
            .filter(|&neighbour| {
                !is_cube_there(&cubes, *neighbour) && is_in_bounds(*neighbour, bounds)
            })
            .for_each(|&neighbour| {
                (water_drop.x, water_drop.y, water_drop.z) = neighbour;
                water_drops.push(water_drop.clone());
            });
        if water_drops.is_empty() {
            break;
        }
    }

    let mut faces = 0;
    for z in 0..z_max {
        for y in 0..y_max {
            for x in 0..x_max {
                if get_element(&cubes, (x, y, z)) == Element::LAVA {
                    faces += neighbours((x, y, z))
                        .iter()
                        .filter(|&neighbour| is_water_or_bounds(&cubes, *neighbour))
                        .count();
                }
            }
        }
    }

    // Print the slices
    // for x in -1..=x_max {
    //     for y in -1..y_max {
    //         for z in -1..y_max {
    //             match get_element(&cubes, (x, y, z)) {
    //                 Element::AIR => { print!("[ ]"); },
    //                 Element::BOUNDS => { print!("[-]"); },
    //                 Element::LAVA => { print!("[0]"); },
    //                 Element::WATER => { print!("[~]"); }
    //             }
    //         }
    //         println!();
    //     }
    //     println!();
    //     println!();
    // }

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
