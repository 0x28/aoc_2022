use std::{cmp::Ordering, iter::Peekable, str::Chars};

#[derive(Debug, Clone, Eq, PartialEq)]
enum Packet {
    List(Vec<Packet>),
    Integer(u32),
}

fn parse_packet(input: &mut Peekable<Chars>) -> Packet {
    match input.peek() {
        Some('[') => {
            input.next();
            let mut sub = vec![];

            while let Some(lookahead) = input.peek() {
                match lookahead {
                    ']' => {
                        input.next();
                        break;
                    }
                    ',' => {
                        input.next();
                    }
                    _ => sub.push(parse_packet(input)),
                }
            }

            Packet::List(sub)
        }
        Some(n) if n.is_numeric() => {
            let mut num_str = String::new();

            while let Some(n) = input.peek() {
                if n.is_numeric() {
                    num_str.push(*n);
                    input.next();
                } else {
                    break;
                }
            }

            Packet::Integer(num_str.parse().unwrap())
        }
        _ => unreachable!(),
    }
}

fn parse(input: &str) -> Vec<(Packet, Packet)> {
    let groups = input.split("\n\n").collect::<Vec<_>>();
    let mut packets = vec![];

    for group in groups {
        let mut lines = group.lines();
        let first = parse_packet(&mut lines.next().unwrap().chars().peekable());
        let second =
            parse_packet(&mut lines.next().unwrap().chars().peekable());

        packets.push((first, second));
    }

    packets
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Packet::List(left), Packet::List(right)) => {
                let mut left = left.iter();
                let mut right = right.iter();
                let mut ordering = Ordering::Equal;

                while ordering == Ordering::Equal {
                    ordering = match (left.next(), right.next()) {
                        (None, None) => return Ordering::Equal,
                        (None, Some(_)) => return Ordering::Less,
                        (Some(_), None) => return Ordering::Greater,
                        (Some(l), Some(r)) => l.cmp(r),
                    };
                }

                ordering
            }
            (list @ Packet::List(_), int @ Packet::Integer(_)) => {
                list.cmp(&Packet::List(vec![int.clone()]))
            }
            (int @ Packet::Integer(_), list @ Packet::List(_)) => {
                Packet::List(vec![int.clone()]).cmp(list)
            }
            (Packet::Integer(l), Packet::Integer(r)) => l.cmp(r),
        }
    }
}

fn part1(pairs: &[(Packet, Packet)]) -> usize {
    let mut sum = 0;
    for (idx, (left, right)) in pairs.iter().enumerate() {
        if left.cmp(right) == Ordering::Less {
            sum += idx + 1;
        }
    }
    sum
}

fn part2(pairs: &[(Packet, Packet)]) -> usize {
    let make_divider =
        |n| Packet::List(vec![Packet::List(vec![Packet::Integer(n)])]);
    let mut packets = vec![make_divider(2), make_divider(6)];

    for (left, right) in pairs.iter() {
        packets.push(left.clone());
        packets.push(right.clone());
    }

    packets.sort_unstable();
    (packets.binary_search(&make_divider(2)).unwrap() + 1)
        * (packets.binary_search(&make_divider(6)).unwrap() + 1)
}

fn main() {
    let input = include_str!("../../input/input13.txt");
    let input = parse(input);
    println!("part1 = {}", part1(&input));
    println!("part2 = {}", part2(&input));
}

#[test]
fn test_day13() {
    let input = "\
[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]";
    let input = parse(input);

    assert_eq!(part1(&input), 13);
    assert_eq!(part2(&input), 140);
}
