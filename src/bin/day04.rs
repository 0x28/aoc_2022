use std::ops::RangeInclusive;

fn parse(input: &str) -> Vec<(RangeInclusive<i64>, RangeInclusive<i64>)> {
    let mut ranges = vec![];
    for line in input.lines() {
        if let [a1, a2, b1, b2] =
            line.split(['-', ',']).collect::<Vec<_>>().as_slice()
        {
            ranges.push((
                a1.parse().unwrap()..=a2.parse().unwrap(),
                b1.parse().unwrap()..=b2.parse().unwrap(),
            ));
        }
    }

    ranges
}

fn fully_contains(
    left: &RangeInclusive<i64>,
    right: &RangeInclusive<i64>,
) -> bool {
    left.contains(right.start()) && left.contains(right.end())
}

fn overlaps(left: &RangeInclusive<i64>, right: &RangeInclusive<i64>) -> bool {
    left.contains(right.start()) || left.contains(right.end())
}

fn part1(puzzle: &[(RangeInclusive<i64>, RangeInclusive<i64>)]) -> usize {
    puzzle
        .iter()
        .filter(|(left, right)| {
            fully_contains(left, right) || fully_contains(right, left)
        })
        .count()
}

fn part2(puzzle: &[(RangeInclusive<i64>, RangeInclusive<i64>)]) -> usize {
    puzzle
        .iter()
        .filter(|(left, right)| overlaps(left, right) || overlaps(right, left))
        .count()
}

fn main() {
    let input = include_str!("../../input/input04.txt");
    let input = parse(input);
    println!("part1 = {}", part1(&input));
    println!("part2 = {}", part2(&input));
}

#[test]
fn test_day04() {
    let input = "\
2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";
    let input = parse(input);

    assert_eq!(part1(&input), 2);
    assert_eq!(part2(&input), 4);
}
