use std::cmp;

use ahash::AHashMap;

#[derive(Clone)]
struct Rock {
    structure: AHashMap<(usize, usize), char>,
    sand_x: usize,
    max_y: usize,
}

fn parse(input: &str) -> Rock {
    let mut paths = vec![];
    for line in input.lines() {
        let path: Vec<_> = line
            .split(" -> ")
            .map(|point| {
                point
                    .split(',')
                    .map(|p| p.parse().unwrap())
                    .collect::<Vec<usize>>()
            })
            .map(|pair| (pair[0], pair[1]))
            .collect();

        paths.push(path);
    }

    let mut max_y = usize::MIN;

    for path in &paths {
        for segment in path {
            max_y = cmp::max(segment.1, max_y);
        }
    }
    let mut structure = AHashMap::default();

    for path in paths {
        for segment in path.windows(2) {
            let begin_x = segment[0].0;
            let begin_y = segment[0].1;
            let end_x = segment[1].0;
            let end_y = segment[1].1;

            if begin_x == end_x {
                let start_y = cmp::min(begin_y, end_y);
                let end_y = cmp::max(begin_y, end_y);
                for y in start_y..=end_y {
                    structure.insert((y, begin_x), '#');
                }
            } else if begin_y == end_y {
                let start_x = cmp::min(begin_x, end_x);
                let end_x = cmp::max(begin_x, end_x);
                for x in start_x..=end_x {
                    structure.insert((begin_y, x), '#');
                }
            } else {
                unreachable!()
            }
        }
    }

    Rock {
        structure,
        sand_x: 500,
        max_y,
    }
}

fn part1(rock: &mut Rock) -> i64 {
    let (mut sand_x, mut sand_y) = (rock.sand_x, 0);
    let mut resting_sand = 0;

    loop {
        if sand_y >= rock.max_y {
            break;
        } else if rock.structure.get(&(sand_y + 1, sand_x)).is_none() {
            sand_y += 1;
        } else if rock.structure.get(&(sand_y + 1, sand_x - 1)).is_none() {
            sand_y += 1;
            sand_x -= 1;
        } else if rock.structure.get(&(sand_y + 1, sand_x + 1)).is_none() {
            sand_y += 1;
            sand_x += 1;
        } else {
            rock.structure.insert((sand_y, sand_x), 'O');
            resting_sand += 1;
            sand_x = rock.sand_x;
            sand_y = 0;
        }
    }

    resting_sand
}

fn part2(rock: &mut Rock) -> i64 {
    let (mut sand_x, mut sand_y) = (rock.sand_x, 0);
    let mut resting_sand = 0;
    let floor = rock.max_y + 2;

    loop {
        if sand_y + 1 == floor {
            rock.structure.insert((sand_y, sand_x), 'O');
            resting_sand += 1;
            sand_x = rock.sand_x;
            sand_y = 0;
        } else if rock.structure.get(&(sand_y + 1, sand_x)).is_none() {
            sand_y += 1;
        } else if rock.structure.get(&(sand_y + 1, sand_x - 1)).is_none() {
            sand_y += 1;
            sand_x -= 1;
        } else if rock.structure.get(&(sand_y + 1, sand_x + 1)).is_none() {
            sand_y += 1;
            sand_x += 1;
        } else if (sand_x, sand_y) == (rock.sand_x, 0) {
            resting_sand += 1;
            break;
        } else {
            rock.structure.insert((sand_y, sand_x), 'O');
            resting_sand += 1;
            sand_x = rock.sand_x;
            sand_y = 0;
        }
    }

    resting_sand
}

fn main() {
    let input = include_str!("../../input/input14.txt");
    let mut input = parse(input);
    println!("part1 = {}", part1(&mut input.clone()));
    println!("part2 = {}", part2(&mut input));
}

#[test]
fn test_day14() {
    let input = "\
498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9";
    let mut input = parse(input);

    assert_eq!(part1(&mut input.clone()), 24);
    assert_eq!(part2(&mut input), 93);
}
