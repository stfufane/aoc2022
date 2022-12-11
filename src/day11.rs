fn main() {
    const INPUT: &str = include_str!("../inputs/day11.txt");
    println!(
        "Monkey business after 20 rounds is {}",
        monkey_business(INPUT, 20, 3)
    );
    println!(
        "Monkey business after 10000 rounds is {}",
        monkey_business(INPUT, 10000, 1)
    );
}

struct Monkey {
    items: Vec<u64>,
    operation: Operation,
    divisible: u64,
    target: (usize, usize),
    inspections: usize,
}

// This contains the index of the monkey to whom the item is thrown and the value of the item
type ThrownItem = (usize, u64);
// Contains the operator and the value of the operand (can be None when using the item value)
type Operation = (char, Option<u64>);

impl Monkey {
    fn new(items: Vec<u64>, operation: Operation, divisible: u64, target: (usize, usize)) -> Self {
        Monkey {
            items,
            operation,
            divisible,
            target,
            inspections: 0,
        }
    }

    // Tells which monkey will receive what
    fn process(&mut self, worry: u64, lcm: u64) -> Vec<ThrownItem> {
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

            // Chill dude
            *item = (*item % lcm) / worry;

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
                    .filter_map(|i| i.parse::<u64>().ok())
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
                    lines[2].split(' ').last().unwrap().parse::<u64>().ok(),
                ),
                lines[3]
                    .split_whitespace()
                    .last()
                    .unwrap()
                    .parse::<u64>()
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
                ),
            )
        })
        .collect()
}

fn monkey_business(input: &str, nb_rounds: u16, worry: u64) -> usize {
    let mut monkeys = parse_monkeys(input);
    let lcm = monkeys.iter().map(|monkey| monkey.divisible).product();
    for _round in 0..nb_rounds {
        for m in 0..monkeys.len() {
            let thrown_items = monkeys[m].process(worry, lcm);
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
            monkey_business(include_str!("../inputs/day11_ex.txt"), 20, 3),
            10605
        );
    }

    #[test]
    fn validate_example_input_2() {
        assert_eq!(
            monkey_business(include_str!("../inputs/day11_ex.txt"), 10000, 1),
            2713310158
        );
    }
}
