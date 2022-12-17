fn main() {
    const INPUT: &str = include_str!("../inputs/day10.txt");

    println!("Signal strength for part 1 is {}", signal_strength(INPUT));
    println!("{}", draw_crt(INPUT));
}

// Part 1
fn signal_strength(input: &str) -> u32 {
    let mut x = 1;
    let mut cycles: Vec<i32> = vec![x];
    input.lines().for_each(|line| match line {
        "noop" => cycles.push(x),
        _ => {
            let value: i32 = line
                .split_whitespace()
                .nth(1)
                .unwrap()
                .parse::<i32>()
                .unwrap();
            cycles.extend_from_slice(&[x, x + value]);
            x += value;
        }
    });
    [20, 60, 100, 140, 180, 220]
        .iter()
        .map(|&cycle| *cycles.get((cycle - 1) as usize).unwrap() as u32 * cycle)
        .sum()
}

// Part 2
fn draw_crt(input: &str) -> String {
    let mut result: String = String::new();
    let mut cycles: Vec<char> = vec!['.'; 240];
    let mut x: i32 = 1;
    let mut addx: i32 = 0;
    let mut read_next = true;
    let mut instructions = input.lines();

    for (idx, cycle) in cycles.iter_mut().enumerate() {
        let position = (idx % 40) as i32;
        if [x - 1, x, x + 1].contains(&position) {
            *cycle = '#';
        }
        result.push(*cycle);
        if position == 39 {
            result += "\n";
        }

        if !read_next {
            read_next = true;
            x += addx;
            continue;
        }
        let instruction = instructions.next().unwrap();
        if instruction != "noop" {
            addx = instruction
                .split_whitespace()
                .nth(1)
                .unwrap()
                .parse::<i32>()
                .unwrap();
            read_next = false; // Skip the next instruction.
        }
    }
    result
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn validate_example_input_1() {
        assert_eq!(
            signal_strength(include_str!("../inputs/day10_ex.txt")),
            13140
        );
    }

    #[test]
    fn validate_example_input_2() {
        assert_eq!(
            draw_crt(include_str!("../inputs/day10_ex.txt")),
"##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######.....
"
        )
    }
}
