use ahash::{AHashMap, AHashSet};

fn parse(input: &str) -> AHashSet<(i64, i64)> {
    let mut map = AHashSet::default();

    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c == '#' {
                map.insert((x as i64, y as i64));
            }
        }
    }

    map
}

#[derive(Debug)]
enum Dir {
    North,
    East,
    South,
    West,
}

impl Dir {
    fn adj(&self, &(x, y): &(i64, i64)) -> [(i64, i64); 3] {
        match self {
            Dir::North => [(x - 1, y - 1), (x, y - 1), (x + 1, y - 1)],
            Dir::East => [(x + 1, y - 1), (x + 1, y), (x + 1, y + 1)],
            Dir::South => [(x - 1, y + 1), (x, y + 1), (x + 1, y + 1)],
            Dir::West => [(x - 1, y - 1), (x - 1, y), (x - 1, y + 1)],
        }
    }
}

fn count_empty_space(map: &AHashSet<(i64, i64)>) -> i64 {
    let mut min_x = i64::MAX;
    let mut max_x = i64::MIN;
    let mut min_y = i64::MAX;
    let mut max_y = i64::MIN;

    for (x, y) in map {
        min_x = i64::min(*x, min_x);
        max_x = i64::max(*x, max_x);
        min_y = i64::min(*y, min_y);
        max_y = i64::max(*y, max_y);
    }

    (max_x - min_x + 1) * (max_y - min_y + 1) - map.len() as i64
}

fn alone(map: &AHashSet<(i64, i64)>, &(x, y): &(i64, i64)) -> bool {
    [
        (x - 1, y - 1),
        (x, y - 1),
        (x + 1, y - 1),
        (x + 1, y),
        (x + 1, y + 1),
        (x, y + 1),
        (x - 1, y + 1),
        (x - 1, y),
    ]
    .iter()
    .all(|p| map.get(p).is_none())
}

fn solve(map: &AHashSet<(i64, i64)>) -> (i64, i64) {
    let mut dir_order = [Dir::North, Dir::South, Dir::West, Dir::East]
        .iter()
        .cycle();
    let mut map = map.clone();

    let mut empty_space_round_10 = 0;
    let mut last_round = 0;

    for round in 1.. {
        let mut proposed_moves =
            AHashMap::<(i64, i64), Vec<(i64, i64)>>::default();
        let mut no_move = true;

        for elf in &map {
            if alone(&map, elf) {
                continue;
            }

            for dir in dir_order.clone().take(4) {
                let adj = dir.adj(elf);
                if adj.iter().all(|p| map.get(p).is_none()) {
                    if let Some(elf_list) = proposed_moves.get_mut(&adj[1]) {
                        elf_list.push(*elf);
                    } else {
                        proposed_moves.insert(adj[1], vec![*elf]);
                    }
                    break;
                }
            }
        }

        for (dest, elf) in proposed_moves {
            if elf.len() == 1 {
                map.remove(&elf[0]);
                map.insert(dest);
                no_move = false;
            }
        }

        dir_order.next();

        if round == 10 {
            empty_space_round_10 = count_empty_space(&map);
        }

        if no_move {
            last_round = round;
            break;
        }
    }

    (empty_space_round_10, last_round)
}

fn main() {
    let input = include_str!("../../input/input23.txt");
    let input = parse(input);
    let (part1, part2) = solve(&input);
    println!("part1 = {}", part1);
    println!("part2 = {}", part2);
}

#[test]
fn test_day23() {
    let input = "\
....#..
..###.#
#...#.#
.#...##
#.###..
##.#.##
.#..#..";
    let input = parse(input);

    assert_eq!(solve(&input), (110, 20));
}
