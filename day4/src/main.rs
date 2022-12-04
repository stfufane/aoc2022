fn main() {
    const INPUT: &str = include_str!("../input.txt");

    // Part 1 
    // List of ranges that are overlap completely
    let full_overlaps: usize = INPUT
        .lines()
        .map(|line| line.split_once(',').unwrap())
        .map(|(l, r)| vec![l.split_once('-').unwrap(), r.split_once('-').unwrap()])
        .map(|ranges| -> usize {
            ((ranges[0].0.parse::<i32>().unwrap() <= ranges[1].0.parse::<i32>().unwrap()
                && ranges[0].1.parse::<i32>().unwrap() >= ranges[1].1.parse::<i32>().unwrap())
                || (ranges[1].0.parse::<i32>().unwrap() <= ranges[0].0.parse::<i32>().unwrap()
                    && ranges[1].1.parse::<i32>().unwrap() >= ranges[0].1.parse::<i32>().unwrap()))
            .into()
        })
        .sum();

    // Part 2
    // List of ranges that overlap partially
    let partial_overlaps: usize = INPUT
        .lines()
        .map(|line| line.split_once(',').unwrap())
        .map(|(l, r)| vec![l.split_once('-').unwrap(), r.split_once('-').unwrap()])
        .map(|ranges| -> usize {
            ((ranges[0].1.parse::<i32>().unwrap() >= ranges[1].0.parse::<i32>().unwrap()
                && ranges[0].0.parse::<i32>().unwrap() <= ranges[1].0.parse::<i32>().unwrap())
                || (ranges[1].1.parse::<i32>().unwrap() >= ranges[0].0.parse::<i32>().unwrap()
                    && ranges[1].0.parse::<i32>().unwrap() <= ranges[0].0.parse::<i32>().unwrap()))
            .into()
        })
        .sum();

    println!("Full overlaps = {}", full_overlaps);
    println!("Partial overlaps = {}", partial_overlaps);
}
