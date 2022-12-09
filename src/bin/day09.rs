use std::collections::HashSet;

enum Move {
    Right,
    Left,
    Up,
    Down,
}

fn parse(input: &str) -> Vec<(Move, i64)> {
    let mut moves = vec![];
    for line in input
        .lines()
        .map(|l| l.split_ascii_whitespace().collect::<Vec<_>>())
    {
        moves.push(match line.as_slice() {
            ["R", dist] => (Move::Right, dist.parse().unwrap()),
            ["L", dist] => (Move::Left, dist.parse().unwrap()),
            ["U", dist] => (Move::Up, dist.parse().unwrap()),
            ["D", dist] => (Move::Down, dist.parse().unwrap()),
            _ => unreachable!(),
        })
    }
    moves
}

fn solve(puzzle: &[(Move, i64)], mut tail: Vec<(i64, i64)>) -> usize {
    let mut head_pos = (0, 0);
    let mut visited = HashSet::from([(0, 0)]);

    for (dir, dist) in puzzle {
        for _ in 0..*dist {
            match dir {
                Move::Right => head_pos.0 += 1,
                Move::Left => head_pos.0 -= 1,
                Move::Up => head_pos.1 += 1,
                Move::Down => head_pos.1 -= 1,
            }

            let mut prev = head_pos;
            for tail_pos in tail.iter_mut() {
                let x_dist = prev.0 - tail_pos.0;
                let y_dist = prev.1 - tail_pos.1;

                if i64::abs(x_dist) > 1 || i64::abs(y_dist) > 1 {
                    tail_pos.0 += i64::signum(x_dist);
                    tail_pos.1 += i64::signum(y_dist);
                }

                prev = *tail_pos;
            }

            visited.insert(tail[tail.len() - 1]);
        }
    }

    visited.len()
}

fn part1(puzzle: &[(Move, i64)]) -> usize {
    solve(puzzle, vec![(0, 0)])
}

fn part2(puzzle: &[(Move, i64)]) -> usize {
    solve(puzzle, vec![(0, 0); 9])
}

fn main() {
    let input = include_str!("../../input/input09.txt");
    let input = parse(input);
    println!("part1 = {}", part1(&input));
    println!("part2 = {}", part2(&input));
}

#[test]
fn test_day09() {
    let input = "\
R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2";
    let input = parse(input);

    assert_eq!(part1(&input), 13);
    assert_eq!(part2(&input), 1);

    let input = "\
R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20";
    let input = parse(input);

    assert_eq!(part2(&input), 36);
}
