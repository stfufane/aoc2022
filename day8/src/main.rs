use take_until::TakeUntilExt;

fn main() {
    const INPUT: &str = include_str!("../input.txt");

    let line_len: usize = INPUT.lines().next().unwrap().len();
    let values: Vec<char> = INPUT.lines().flat_map(|s| s.chars()).collect();

    let mut lines: Vec<Vec<char>> = Vec::new();
    let mut cols: Vec<Vec<char>> = Vec::new();
    cols.resize(line_len, Vec::new());

    INPUT.lines().for_each(|line| {
        lines.push(line.chars().collect());
    });

    for (pos, value) in values.iter().enumerate() {
        cols[pos % line_len].push(*value);
    }

    let mut nb_visible = 0;
    let mut area: usize = 0;
    for (pos, value) in values.iter().enumerate() {
        // No need to calculate for first line, last line and edges
        if pos < line_len
            || pos > (values.len() - line_len)
            || pos % line_len == 0
            || pos % line_len == line_len - 1
        {
            nb_visible += 1;
            continue;
        }
        let current_line = pos / line_len;
        let current_col = pos % line_len;

        // Part 1 : check if the tree is visible.
        if value
            > lines[current_line][0..current_col]
                .into_iter()
                .max()
                .unwrap()
            || value
                > lines[current_line][current_col + 1..line_len]
                    .into_iter()
                    .max()
                    .unwrap()
            || value
                > cols[current_col][0..current_line]
                    .into_iter()
                    .max()
                    .unwrap()
            || value
                > cols[current_col][current_line + 1..line_len]
                    .into_iter()
                    .max()
                    .unwrap()
        {
            nb_visible += 1;
        }

        // Part 2 : Calculate how many trees I can see from here
        let left_trees = lines[current_line][0..current_col]
            .into_iter()
            .rev()
            .take_until(|t| t >= &value)
            .count();
        let right_trees = lines[current_line][current_col + 1..line_len]
            .into_iter()
            .take_until(|t| t >= &value)
            .count();
        let top_trees = cols[current_col][0..current_line]
            .into_iter()
            .rev()
            .take_until(|t| t >= &value)
            .count();
        let bottom_trees = cols[current_col][current_line + 1..line_len]
            .into_iter()
            .take_until(|t| t >= &value)
            .count();
        area = area.max(left_trees * right_trees * top_trees * bottom_trees);
    }

    println!("There are {} visible trees", nb_visible);
    println!("The top area is {}", area);
}
