use std::collections::VecDeque;

#[derive(Debug, Clone)]
enum Operand {
    Old,
    Number(u64),
}

impl Operand {
    fn resolve(&self, old: u64) -> u64 {
        match self {
            Operand::Number(n) => *n,
            Operand::Old => old,
        }
    }
}

#[derive(Debug, Clone)]
enum Expr {
    Mul(Operand, Operand),
    Add(Operand, Operand),
}

impl Expr {
    fn eval(&self, old: u64) -> u64 {
        match self {
            Expr::Mul(a, b) => a.resolve(old) * b.resolve(old),
            Expr::Add(a, b) => a.resolve(old) + b.resolve(old),
        }
    }
}

#[derive(Debug, Clone)]
struct Monkey {
    items: VecDeque<u64>,
    operation: Expr,
    test_divisor: u64,
    true_monkey: usize,
    false_monkey: usize,
    inspect_count: u64,
}

fn parse_expr(input: &str) -> Expr {
    let input = input
        .chars()
        .skip_while(|&c| c != '=')
        .skip(2)
        .collect::<String>();

    let input = input.split_ascii_whitespace().collect::<Vec<_>>();

    fn parse_operand(input: &str) -> Operand {
        match input {
            "old" => Operand::Old,
            n => Operand::Number(n.parse().unwrap()),
        }
    }

    match input.as_slice() {
        [o1, "+", o2] => Expr::Add(parse_operand(o1), parse_operand(o2)),
        [o1, "*", o2] => Expr::Mul(parse_operand(o1), parse_operand(o2)),
        _ => unreachable!(),
    }
}

fn extract_numbers<T>(input: &str) -> VecDeque<T>
where
    T: std::str::FromStr,
    T::Err: std::fmt::Debug,
{
    let mut numbers = VecDeque::new();
    let mut num = String::new();

    for c in input.chars() {
        if c.is_numeric() {
            num.push(c);
        } else if !num.is_empty() {
            numbers.push_back(num.parse::<T>().unwrap());
            num.clear();
        }
    }

    if !num.is_empty() {
        numbers.push_back(num.parse::<T>().unwrap());
    }

    numbers
}

fn parse(input: &str) -> Vec<Monkey> {
    let mut monkeys = vec![];

    for monkey in input.split("\n\n") {
        let mut attrs = monkey.lines();
        attrs.next();

        let items: VecDeque<u64> = extract_numbers(attrs.next().unwrap());
        let operation = parse_expr(attrs.next().unwrap());
        let test_divisor = extract_numbers(attrs.next().unwrap())[0];
        let true_monkey = extract_numbers::<usize>(attrs.next().unwrap())[0];
        let false_monkey = extract_numbers::<usize>(attrs.next().unwrap())[0];

        monkeys.push(Monkey {
            items,
            operation,
            test_divisor,
            false_monkey,
            true_monkey,
            inspect_count: 0,
        });
    }

    monkeys
}

fn part1(monkeys: &mut [Monkey]) -> u64 {
    for _ in 0..20 {
        for idx in 0..monkeys.len() {
            while !monkeys[idx].items.is_empty() {
                let mut item = monkeys[idx].items.pop_front().unwrap();
                monkeys[idx].inspect_count += 1;
                item = monkeys[idx].operation.eval(item);
                item /= 3;

                let dest = if item % monkeys[idx].test_divisor == 0 {
                    monkeys[idx].true_monkey
                } else {
                    monkeys[idx].false_monkey
                };

                monkeys.get_mut(dest).unwrap().items.push_back(item);
            }
        }
    }

    monkeys.sort_by(|m1, m2| u64::cmp(&m2.inspect_count, &m1.inspect_count));
    monkeys.iter().take(2).map(|m| m.inspect_count).product()
}

fn part2(monkeys: &mut [Monkey]) -> u64 {
    let prod: u64 = monkeys.iter().map(|m| m.test_divisor).product();

    for _ in 0..10_000 {
        for idx in 0..monkeys.len() {
            while !monkeys[idx].items.is_empty() {
                let mut item = monkeys[idx].items.pop_front().unwrap();
                monkeys[idx].inspect_count += 1;
                item = monkeys[idx].operation.eval(item) % prod;

                let dest = if item % monkeys[idx].test_divisor == 0 {
                    monkeys[idx].true_monkey
                } else {
                    monkeys[idx].false_monkey
                };

                monkeys.get_mut(dest).unwrap().items.push_back(item);
            }
        }
    }

    monkeys.sort_by(|m1, m2| u64::cmp(&m2.inspect_count, &m1.inspect_count));
    monkeys.iter().take(2).map(|m| m.inspect_count).product()
}

fn main() {
    let input = include_str!("../../input/input11.txt");
    let mut input = parse(input);
    println!("part1 = {}", part1(&mut input.clone()));
    println!("part2 = {}", part2(&mut input));
}

#[test]
fn test_day11() {
    let input = "\
Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1";
    let mut input = parse(input);

    assert_eq!(part1(&mut input.clone()), 10605);
    assert_eq!(part2(&mut input), 2713310158);
}
