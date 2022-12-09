fn main() {
    let mut totals: Vec<i32> = vec![0];
    const INPUT: &str = include_str!("../inputs/day1.txt");
    for line in INPUT.lines() {
        if line.is_empty() {
            totals.push(0);
        } else if let Some(last) = totals.last_mut() {
            *last += line.parse().unwrap_or(0);
        }
    }
    // Sort descending
    totals.sort_by(|a, b| b.cmp(a));

    // Answer to part 1 :
    println!("Carrying most has {}", totals.first().unwrap());

    // Answer to part 2 :
    let best_three: i32 = totals.iter().take(3).sum();
    println!("Sum of best 3 = {}", best_three);
}
