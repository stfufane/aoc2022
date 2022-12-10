fn main() {
    const INPUT: &str = include_str!("../inputs/day10.txt");
    
    println!("Signal strength for part 1 is {}", signal_strength(INPUT));
}

fn signal_strength(input: &str) -> u32 {
    let mut x = 1;
    let mut cycles: Vec<i32> = vec![x];
    input.lines().for_each(|line| {
        match line {
            "noop" => { cycles.push(x) },
            _ => {
                let value: i32 = line.split_whitespace().nth(1).unwrap().parse::<i32>().unwrap();
                cycles.extend_from_slice(&[x, x + value]);
                x += value;
            }
        }
    });
    [20, 60, 100, 140, 180, 220].iter().map(|&cycle| {
        *cycles.get((cycle - 1) as usize).unwrap() as u32 * cycle
    }).sum()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn validate_example_input_1() {
        assert_eq!(signal_strength(include_str!("../inputs/day10_ex.txt")), 13140);
    }
}
