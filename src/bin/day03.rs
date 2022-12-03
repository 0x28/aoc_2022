use std::collections::HashSet;

fn parse_part1(input: &str) -> Vec<(HashSet<u8>, HashSet<u8>)> {
    let mut backpack = vec![];

    for line in input.lines() {
        let (left, right) = line.split_at(line.len() / 2);

        backpack.push((left.bytes().collect(), right.bytes().collect()))
    }

    backpack
}

fn parse_part2(input: &str) -> Vec<HashSet<u8>> {
    input.lines().map(|l| l.bytes().collect()).collect()
}

fn priority(c: u8) -> u8 {
    if c.is_ascii_lowercase() {
        c - b'a' + 1
    } else if c.is_ascii_uppercase() {
        c - b'A' + 27
    } else {
        0
    }
}

fn part1(puzzle: &[(HashSet<u8>, HashSet<u8>)]) -> i64 {
    let mut total_priority = 0;

    for (left, right) in puzzle {
        for c in left.iter() {
            if right.contains(c) {
                total_priority += priority(*c) as i64;
                break;
            }
        }
    }

    total_priority
}

fn part2(puzzle: &[HashSet<u8>]) -> i64 {
    let mut total_priority = 0;

    for group in puzzle.chunks(3) {
        for c in &group[0] {
            if group[1].contains(c) && group[2].contains(c) {
                total_priority += priority(*c) as i64;
                break;
            }
        }
    }

    total_priority
}

fn main() {
    let input = include_str!("../../input/input03.txt");
    println!("part1 = {}", part1(&parse_part1(input)));
    println!("part2 = {}", part2(&parse_part2(input)));
}

#[test]
fn test_day03() {
    let input = "\
vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";
    assert_eq!(part1(&parse_part1(input)), 157);
    assert_eq!(part2(&parse_part2(input)), 70);
}
