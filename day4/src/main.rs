use itertools::Itertools;

fn main() {
    const INPUT: &str = include_str!("../input.txt");

    type Sections = (u8, u8, u8, u8);

    // Parse once with a multi split
    let all_sections: Vec<Sections> = INPUT
        .lines()
        .map(|line| {
            line.split(&[',', '-'])
                .map(|s| s.parse().unwrap())
                .collect_tuple()
                .unwrap()
        })
        .collect_vec();

    // Part 1 : Check the sections totally overlap
    let full_overlaps: usize = all_sections.iter().map(|&section| -> usize {
        let (s1_start, s1_end, s2_start, s2_end) = section;
        ((s1_start <= s2_start && s1_end >= s2_end) ||
        (s2_start <= s1_start && s2_end >= s1_end))
        .into() 
    }).sum();

    // Part 2 : Check the sections partially overlapp
    let partial_overlaps: usize = all_sections.iter().map(|&section| -> usize {
        let (s1_start, s1_end, s2_start, s2_end) = section;
        ((s1_end >= s2_start && s1_start <= s2_start) || 
        (s2_end >= s1_start && s2_start <= s1_start))
        .into()
    }).sum();

    println!("Full overlaps = {}", full_overlaps);
    println!("Partial overlaps = {}", partial_overlaps);
}
