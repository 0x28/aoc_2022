use ahash::AHashMap;
#[derive(Debug)]
enum Instruction {
    Left,
    Right,
    Forward(i64),
}

#[derive(Debug)]
struct Field {
    max_x: i64,
    min_x: i64,
    max_y: i64,
    min_y: i64,
    map: AHashMap<(i64, i64), char>,
}

impl Field {
    fn new() -> Self {
        Field {
            max_x: i64::MIN,
            min_x: i64::MAX,
            max_y: i64::MIN,
            min_y: i64::MAX,
            map: AHashMap::default(),
        }
    }

    #[allow(unused)]
    fn display(&self, pos: (i64, i64)) {
        for y in self.min_y..=self.max_y {
            for x in self.min_x..=self.max_x {
                if (x, y) == pos {
                    print!("@");
                } else {
                    print!("{}", self.map.get(&(x, y)).unwrap_or(&' '));
                }
            }
            println!();
        }

        println!();
    }
}

fn parse(input: &str) -> (Field, Vec<Instruction>) {
    let mut field = Field::new();
    let mut ins = vec![];

    let mut lines = input.lines();
    for (y, line) in lines.by_ref().enumerate() {
        if line.is_empty() {
            break;
        }

        for (x, c) in line.chars().enumerate() {
            if c == ' ' {
                continue;
            }

            let x = (x + 1) as i64;
            let y = (y + 1) as i64;

            field.max_x = i64::max(x, field.max_x);
            field.min_x = i64::min(x, field.min_x);
            field.max_y = i64::max(y, field.max_y);
            field.min_y = i64::min(y, field.min_y);
            field.map.insert((x, y), c);
        }
    }

    let mut num_str = String::new();

    for c in lines.next().unwrap().chars() {
        if c.is_numeric() {
            num_str.push(c);
        } else {
            if !num_str.is_empty() {
                ins.push(Instruction::Forward(num_str.parse().unwrap()));
                num_str.clear();
            }

            ins.push(match c {
                'L' => Instruction::Left,
                'R' => Instruction::Right,
                _ => unreachable!(),
            })
        }
    }

    if !num_str.is_empty() {
        ins.push(Instruction::Forward(num_str.parse().unwrap()));
    }

    (field, ins)
}

fn part1_warp(pos: &mut (i64, i64), dir: &mut (i64, i64), field: &Field) {
    match dir {
        (-1, 0) => {
            for x in (field.min_x..=field.max_x).rev() {
                if field.map.contains_key(&(x, pos.1)) {
                    *pos = (x, pos.1);
                    break;
                }
            }
        }
        (1, 0) => {
            for x in field.min_x..=field.max_x {
                if field.map.contains_key(&(x, pos.1)) {
                    *pos = (x, pos.1);
                    break;
                }
            }
        }
        (0, 1) => {
            for y in field.min_y..=field.max_y {
                if field.map.contains_key(&(pos.0, y)) {
                    *pos = (pos.0, y);
                    break;
                }
            }
        }
        (0, -1) => {
            for y in (field.min_y..=field.max_y).rev() {
                if field.map.contains_key(&(pos.0, y)) {
                    *pos = (pos.0, y);
                    break;
                }
            }
        }
        _ => unreachable!(),
    }
}

fn part1(input: &(Field, Vec<Instruction>)) -> i64 {
    solve(input, Box::new(part1_warp))
}

type WarpFn = Box<dyn Fn(&mut (i64, i64), &mut (i64, i64), &Field)>;

fn step(
    pos: &mut (i64, i64),
    dir: &mut (i64, i64),
    by: i64,
    field: &Field,
    warp: &WarpFn,
) {
    for _ in 0..by {
        let mut next_pos = *pos;
        let mut next_dir = *dir;
        next_pos = (next_pos.0 + dir.0, next_pos.1 + dir.1);
        match field.map.get(&next_pos) {
            Some('.') => (),
            Some('#') => return,
            Some(_) => unreachable!(),
            None => {
                next_pos = *pos;
                warp(&mut next_pos, &mut next_dir, field);
            }
        }

        if let Some('#') = field.map.get(&next_pos) {
            return;
        } else {
            *pos = next_pos;
            *dir = next_dir;
        }
    }
}

fn solve(
    (field, instructions): &(Field, Vec<Instruction>),
    warp: WarpFn,
) -> i64 {
    let mut pos = (0, 0);
    let mut dir = (1, 0);

    for x in field.min_x..=field.max_x {
        if field.map.contains_key(&(x, field.min_y)) {
            pos = (x, field.min_y);
            break;
        }
    }

    for ins in instructions {
        match ins {
            Instruction::Left => {
                dir = (dir.1, -dir.0);
            }
            Instruction::Right => {
                dir = (-dir.1, dir.0);
            }
            Instruction::Forward(by) => {
                step(&mut pos, &mut dir, *by, field, &warp);
            }
        }
    }

    let facing = match dir {
        (1, 0) => 0,
        (0, 1) => 1,
        (-1, 0) => 2,
        (0, -1) => 3,
        _ => unreachable!(),
    };

    1000 * pos.1 + 4 * pos.0 + facing
}

fn part2(input: &(Field, Vec<Instruction>), warp: WarpFn) -> i64 {
    solve(input, warp)
}

fn remap(x: i64, a: i64, b: i64, c: i64, d: i64) -> i64 {
    x * (d - c) / (b - a) + c - a * (d - c) / (b - a)
}

#[allow(unused)]
fn test_warp(pos: &mut (i64, i64), dir: &mut (i64, i64), _: &Field) {
    if (1..=4).contains(&pos.0) && (5..=8).contains(&pos.1) {
        match dir {
            (0, -1) => {
                *pos = (remap(pos.0, 1, 4, 8, 12), 1);
                *dir = (0, 1);
            }
            (-1, 0) => {
                *pos = (remap(pos.1, 5, 8, 16, 13), 12);
                *dir = (0, -1);
            }
            (0, 1) => {
                *pos = (remap(pos.0, 1, 4, 12, 9), 12);
                *dir = (0, -1);
            }
            _ => unreachable!(),
        }
    } else if (5..=8).contains(&pos.0) && (5..=8).contains(&pos.1) {
        match dir {
            (0, -1) => {
                *pos = (9, remap(pos.0, 5, 8, 1, 4));
                *dir = (1, 0);
            }
            (0, 1) => {
                *pos = (9, remap(pos.0, 5, 8, 12, 9));
                *dir = (1, 0);
            }
            _ => unreachable!(),
        }
    } else if (9..=12).contains(&pos.0) && (5..=8).contains(&pos.1) {
        *pos = (remap(pos.1, 5, 8, 16, 13), 9);
        *dir = (0, 1);
    } else if (9..=12).contains(&pos.0) && (1..=4).contains(&pos.1) {
        match dir {
            (0, -1) => {
                *pos = (remap(pos.0, 9, 12, 4, 1), 5);
                *dir = (0, 1);
            }
            (-1, 0) => {
                *pos = (remap(pos.1, 1, 4, 5, 8), 5);
                *dir = (0, 1);
            }
            (1, 0) => {
                *pos = (16, remap(pos.1, 1, 4, 12, 9));
                *dir = (-1, 0);
            }
            _ => unreachable!(),
        }
    } else if (9..=12).contains(&pos.0) && (9..=12).contains(&pos.1) {
        match dir {
            (-1, 0) => {
                *pos = (remap(pos.1, 9, 12, 8, 5), 8);
                *dir = (0, -1);
            }
            (0, 1) => {
                *pos = (remap(pos.0, 9, 12, 4, 1), 8);
                *dir = (0, -1);
            }
            _ => unreachable!(),
        }
    } else if (13..=16).contains(&pos.0) && (9..=12).contains(&pos.1) {
        match dir {
            (0, -1) => {
                *pos = (12, remap(pos.0, 13, 16, 8, 5));
                *dir = (-1, 0);
            }
            (1, 0) => {
                *pos = (12, remap(pos.1, 9, 12, 4, 1));
                *dir = (-1, 0);
            }
            (0, 1) => {
                *pos = (1, remap(pos.0, 13, 16, 8, 5));
                *dir = (1, 0);
            }
            _ => unreachable!(),
        }
    }
}

fn part2_warp(pos: &mut (i64, i64), dir: &mut (i64, i64), _: &Field) {
    if (51..=100).contains(&pos.0) && (1..=50).contains(&pos.1) {
        match dir {
            (0, -1) => {
                *pos = (1, remap(pos.0, 51, 100, 151, 200));
                *dir = (1, 0);
            }
            (-1, 0) => {
                *pos = (1, remap(pos.1, 1, 50, 150, 101));
                *dir = (1, 0);
            }
            _ => unreachable!(),
        }
    } else if (101..=150).contains(&pos.0) && (1..=50).contains(&pos.1) {
        match dir {
            (0, -1) => {
                *pos = (remap(pos.0, 101, 150, 1, 50), 200);
                *dir = (0, -1);
            }
            (1, 0) => {
                *pos = (100, remap(pos.1, 1, 50, 150, 101));
                *dir = (-1, 0);
            }
            (0, 1) => {
                *pos = (100, remap(pos.0, 101, 150, 51, 100));
                *dir = (-1, 0);
            }
            _ => unreachable!(),
        }
    } else if (51..=100).contains(&pos.0) && (51..=100).contains(&pos.1) {
        match dir {
            (-1, 0) => {
                *pos = (remap(pos.1, 51, 100, 1, 50), 101);
                *dir = (0, 1);
            }
            (1, 0) => {
                *pos = (remap(pos.1, 51, 100, 101, 150), 50);
                *dir = (0, -1);
            }
            _ => unreachable!(),
        }
    } else if (51..=100).contains(&pos.0) && (101..=150).contains(&pos.1) {
        match dir {
            (1, 0) => {
                *pos = (150, remap(pos.1, 101, 150, 50, 1));
                *dir = (-1, 0);
            }
            (0, 1) => {
                *pos = (50, remap(pos.0, 51, 100, 151, 200));
                *dir = (-1, 0);
            }
            _ => unreachable!(),
        }
    } else if (1..=50).contains(&pos.0) && (101..=150).contains(&pos.1) {
        match dir {
            (0, -1) => {
                *pos = (51, remap(pos.0, 1, 50, 51, 100));
                *dir = (1, 0);
            }
            (-1, 0) => {
                *pos = (51, remap(pos.1, 101, 150, 50, 1));
                *dir = (1, 0);
            }
            _ => unreachable!(),
        }
    } else if (1..=50).contains(&pos.0) && (151..=200).contains(&pos.1) {
        match dir {
            (-1, 0) => {
                *pos = (remap(pos.1, 151, 200, 51, 100), 1);
                *dir = (0, 1);
            }
            (0, 1) => {
                *pos = (remap(pos.0, 1, 50, 101, 150), 1);
                *dir = (0, 1);
            }
            (1, 0) => {
                *pos = (remap(pos.1, 151, 200, 51, 100), 150);
                *dir = (0, -1);
            }
            _ => unreachable!(),
        }
    }
}

fn main() {
    let input = include_str!("../../input/input22.txt");
    let input = parse(input);
    println!("part1 = {}", part1(&input));
    println!("part2 = {}", part2(&input, Box::new(part2_warp)));
}

#[test]
fn test_day22() {
    let input = "        ...#
        .#..
        #...
        ....
...#.......#
........#...
..#....#....
..........#.
        ...#....
        .....#..
        .#......
        ......#.

10R5L5R10L4R5L5";
    let input = parse(input);
    let input2 = parse(include_str!("../../input/input22.txt"));

    assert_eq!(part1(&input), 6032);
    assert_eq!(part1(&input2), 65368);

    assert_eq!(part2(&input, Box::new(test_warp)), 5031);
    assert_eq!(part2(&input2, Box::new(part2_warp)), 156166);
}
