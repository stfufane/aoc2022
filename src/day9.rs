use std::collections::HashSet;

#[derive(Hash, PartialEq, Eq, Clone, Debug)]
struct Knot {
    x: i32,
    y: i32,
}

impl Knot {
    fn new(x: i32, y: i32) -> Self {
        Knot { x, y }
    }

    fn translate(&mut self, instruction: &char) {
        match instruction {
            'U' => self.translate_y(-1),
            'D' => self.translate_y(1),
            'L' => self.translate_x(-1),
            'R' => self.translate_x(1),
            _ => (),
        }
    }

    fn translate_x(&mut self, delta: i32) {
        self.x += delta;
    }

    fn translate_y(&mut self, delta: i32) {
        self.y += delta;
    }

    fn get_closer(&mut self, other: &Knot) {
        if (other.x - self.x).abs() >= 1 {
            self.translate_x(if other.x > self.x { 1 } else { -1 });
        }
        if (other.y - self.y).abs() >= 1 {
            self.translate_y(if other.y > self.y { 1 } else { -1 });
        }
    }

    fn is_too_far(&self, other: &Knot) -> bool {
        (other.x - self.x).abs() > 1 || (other.y - self.y).abs() > 1
    }
}

fn nb_visited(input: &str, nb_knots: usize) -> usize {
    // Init the visited positions with the initial position.
    let mut visited: HashSet<Knot> = HashSet::from([Knot::new(0, 0)]);
    // The rope consists of n knots all starting at the origin.
    let mut knots = vec![Knot::new(0, 0); nb_knots];
    input.lines().for_each(|line| {
        let instruction = line.chars().next().unwrap();
        let moves = line
            .split_whitespace()
            .nth(1)
            .unwrap()
            .parse::<u8>()
            .unwrap();

        for _ in 0..moves {
            // Move the head first following the instruction
            knots.get_mut(0).unwrap().translate(&instruction);
            // Then iterate all the remaining knots to move them according to the previous one.
            for k in 1..knots.len() {
                let prev_knot = &knots[k - 1].clone();
                if knots[k].is_too_far(prev_knot) {
                    knots[k].get_closer(prev_knot);
                }
                // Add the last knot to the set of visited positions.
                if k == knots.len() - 1 {
                    visited.insert(knots.last().unwrap().clone());
                }
            }
        }
    });
    visited.len()
}

fn main() {
    const INPUT: &str = include_str!("../inputs/day9.txt");

    println!("Nb visited on part 1 : {}", nb_visited(INPUT, 2));
    println!("Nb visited on part 2 : {}", nb_visited(INPUT, 10));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn validate_example_input_1() {
        assert_eq!(nb_visited(include_str!("../inputs/day9_ex1.txt"), 2), 13);
    }

    #[test]
    fn validate_example_input_2() {
        assert_eq!(nb_visited(include_str!("../inputs/day9_ex2.txt"), 10), 36);
    }
}
