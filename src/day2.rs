struct Game {
    p1: char,
    p2: char,
}

/**
 * Win = 6 points, Draw = 3 points, Lose = 0 point
 * A = Rock (1 point), B = Paper (2 points), C = Scissor (3 points)
 * X = Rock (1 point), Y = Paper (2 points), Z = Scissor (3 points) for part 1
 * X = Lose (0 point), Y = Draw (3 points), Z = Win (6 points) for part 2
 */
impl Game {
    // Calculate how many points I win when playing rock, paper or scissor
    pub fn points_st1(&self) -> i32 {
        match self.p2 {
            'X' => 1 + match self.p1 { 'A' => 3, 'B' => 0, 'C' => 6, _ => 0 },
            'Y' => 2 + match self.p1 { 'A' => 6, 'B' => 3, 'C' => 0, _ => 0 },
            'Z' => 3 + match self.p1 { 'A' => 0, 'B' => 6, 'C' => 3, _ => 0 },
            _ => 0
        }
    }

    // Calculate what I should play to win, draw, lose.
    pub fn points_st2(&self) -> i32 {
        match self.p2 {
            'X' => match self.p1 { 'A' => 3, 'B' => 1, 'C' => 2, _ => 0 },
            'Y' => 3 + match self.p1 { 'A' => 1, 'B' => 2, 'C' => 3, _ => 0 },
            'Z' => 6 + match self.p1 { 'A' => 2, 'B' => 3, 'C' => 1, _ => 0 },
            _ => 0
        }
    }
}

fn main() {
    let mut games: Vec<Game> = Vec::new();
    const INPUT: &str = include_str!("../inputs/day2.txt");
    for line in INPUT
        .lines()
    {
        games.push(Game {
            p1: line.chars().next().unwrap(),
            p2: line.chars().nth(2).unwrap(),
        });
    }

    let mut total_points_st1: i32 = 0;
    let mut total_points_st2: i32 = 0;
    for game in games.iter() {
        total_points_st1 += game.points_st1();
        total_points_st2 += game.points_st2();
    }
    println!("Total points : strategy 1 : {}, strategy 2 : {}", total_points_st1, total_points_st2);
}
