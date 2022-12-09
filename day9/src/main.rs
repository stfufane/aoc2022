use std::collections::HashSet;

#[derive(Hash,PartialEq, Eq, Clone,Debug)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Self {
        Point { x, y }
    }

    fn translate(&mut self, instruction: &char) {
        match instruction {
            'U' => self.translate_y(-1),
            'D' => self.translate_y(1),
            'L' => self.translate_x(-1),
            'R' => self.translate_x(1),
            _ => ()
        }
    } 

    fn translate_x(&mut self, delta: i32) {
        self.x += delta;
    }

    fn translate_y(&mut self, delta: i32) {
        self.y += delta;
    }

    fn get_closer(&mut self, other: &Point) {
        if (other.x - self.x).abs() >= 1 {
            self.translate_x(if other.x > self.x { 1 } else { -1 });
        } 
        if (other.y - self.y).abs() >= 1 {
            self.translate_y(if other.y > self.y { 1 } else { -1 });
        }
    } 

    fn is_too_far(&self, other: &Point) -> bool {
        (other.x - self.x).abs() > 1 || (other.y - self.y).abs() > 1
    }
}

fn main() {
    let mut t = Point::new(0, 0);
    let mut h = Point::new(0, 0);
    // Init the visited positions with the initial position.
    let mut visited: HashSet<Point> = HashSet::from([h.clone()]);

    const INPUT: &str = include_str!("../input.txt");

    // Part 1 : The rope has only one head and one tail.
    INPUT.lines().for_each(|line| {
        let instruction = line.chars().nth(0).unwrap();
        let moves = line.split_whitespace().nth(1).unwrap().parse::<u8>().unwrap();

        for _ in 0..moves {
            h.translate(&instruction);
            if t.is_too_far(&h) {
                t.get_closer(&h);
                visited.insert(t.clone());
            }
        }
    });

    println!("Nb visited on part 1 : {}", visited.len());
}
