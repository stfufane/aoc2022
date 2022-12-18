use itertools::Itertools;

fn main() {
    println!("Nb visible faces = {}", process_part1(include_str!("../inputs/day18.txt")));
}

#[derive(Debug)]
struct Cube {
    x: u32,
    y: u32,
    z: u32,
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
                .map(|s| s.parse::<u32>().unwrap())
                .collect_tuple::<(u32, u32, u32)>()
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

    // #[test]
    // fn validate_example_input_2() {
    //     assert_eq!(process_part2(EXAMPLE_DATA), 0);
    // }
}
