fn main() {
    const INPUT: &str = include_str!("../inputs/day11.txt");
    println!(
        "Monkey business after 20 rounds is {}",
        monkey_business(INPUT)
    );
}

struct Monkey {
    items: Vec<u32>,
    operation: Operation,
    divisible: u32,
    target: (usize, usize),
    inspections: usize,
}

// This contains the index of the monkey to whom item is thrown and the value of the item
type ThrownItem = (usize, u32);
// Contains the operator and the value of the operand (can be None when using the item value)
type Operation = (char, Option<u32>);

impl Monkey {
    fn new(items: Vec<u32>, operation: Operation, divisible: u32, target: (usize, usize)) -> Self {
        Monkey {
            items,
            operation,
            divisible,
            target,
            inspections: 0,
        }
    }

    // Tells which monkey will receive what
    fn process_part1(&mut self) -> Vec<ThrownItem> {
        self.inspections += self.items.len();
        let mut thrown_items: Vec<ThrownItem> = Vec::new();
        thrown_items.reserve(self.items.len());
        self.items.iter_mut().for_each(|item| {
            let operator = match self.operation.1 {
                Some(v) => v,
                None => *item,
            };
            match self.operation.0 {
                '*' => *item *= operator,
                '+' => *item += operator,
                _ => (),
            };
            *item = ((*item as f32 / 3.0).floor()) as u32;
            thrown_items.push((
                if *item % self.divisible == 0 {
                    self.target.0
                } else {
                    self.target.1
                },
                *item,
            ));
        });
        self.items.clear();
        thrown_items
    }
}

fn parse_monkeys(input: &str) -> Vec<Monkey> {
    input
        .lines()
        .collect::<Vec<&str>>()
        .chunks(7)
        .map(|lines| {
            // Shitty parsing to get all the info from the input
            Monkey::new(
                lines[1]
                    .split([',', ' '])
                    .filter_map(|i| i.parse::<u32>().ok())
                    .collect(),
                (
                    lines[2]
                        .split(' ')
                        .rev()
                        .nth(1)
                        .unwrap()
                        .chars()
                        .next()
                        .unwrap(),
                    lines[2].split(' ').last().unwrap().parse::<u32>().ok(),
                ),
                lines[3]
                    .split_whitespace()
                    .last()
                    .unwrap()
                    .parse::<u32>()
                    .unwrap(),
                (
                    lines[4]
                        .split_whitespace()
                        .last()
                        .unwrap()
                        .parse::<usize>()
                        .unwrap(),
                    lines[5]
                        .split_whitespace()
                        .last()
                        .unwrap()
                        .parse::<usize>()
                        .unwrap(),
                )
            )
        })
        .collect()
}

fn monkey_business(input: &str) -> usize {
    let mut monkeys = parse_monkeys(input);
    for _round in 0..20 {
        for m in 0..monkeys.len() {
            let thrown_items = monkeys[m].process_part1();
            for item in thrown_items {
                monkeys[item.0].items.push(item.1);
            }
        }
    }

    monkeys.sort_by(|a, b| b.inspections.cmp(&a.inspections));
    monkeys
        .iter()
        .take(2)
        .map(|monkey| monkey.inspections)
        .product()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn validate_example_input_1() {
        assert_eq!(
            monkey_business(include_str!("../inputs/day11_ex.txt")),
            10605
        );
    }
}
