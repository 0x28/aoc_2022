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

fn step(pos: &mut (i64, i64), dir: (i64, i64), by: i64, field: &Field) {
    for _ in 0..by {
        let mut next_pos = *pos;
        next_pos = (next_pos.0 + dir.0, next_pos.1 + dir.1);
        match field.map.get(&next_pos) {
            Some('.') => (),
            Some('#') => return,
            Some(_) => unreachable!(),
            None => match dir {
                (-1, 0) => {
                    for x in (field.min_x..=field.max_x).rev() {
                        if field.map.contains_key(&(x, next_pos.1)) {
                            next_pos = (x, next_pos.1);
                            break;
                        }
                    }
                }
                (1, 0) => {
                    for x in field.min_x..=field.max_x {
                        if field.map.contains_key(&(x, next_pos.1)) {
                            next_pos = (x, next_pos.1);
                            break;
                        }
                    }
                }
                (0, 1) => {
                    for y in field.min_y..=field.max_y {
                        if field.map.contains_key(&(next_pos.0, y)) {
                            next_pos = (next_pos.0, y);
                            break;
                        }
                    }
                }
                (0, -1) => {
                    for y in (field.min_y..=field.max_y).rev() {
                        if field.map.contains_key(&(next_pos.0, y)) {
                            next_pos = (next_pos.0, y);
                            break;
                        }
                    }
                }
                _ => unreachable!(),
            },
        }

        if let Some('#') = field.map.get(&next_pos) {
            return;
        } else {
            *pos = next_pos;
        }
    }
}

fn part1((field, instructions): &(Field, Vec<Instruction>)) -> i64 {
    let mut pos = (0, 0);
    let mut dir = (1, 0);

    for x in field.min_x..=field.max_x {
        if field.map.contains_key(&(x, field.min_y)) {
            pos = (x, field.min_y);
            break;
        }
    }
    // field.display(pos);

    for ins in instructions {
        match ins {
            Instruction::Left => {
                dir = (dir.1, -dir.0);
            }
            Instruction::Right => {
                dir = (-dir.1, dir.0);
            }
            Instruction::Forward(by) => {
                step(&mut pos, dir, *by, field);
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

fn main() {
    let input = include_str!("../../input/input22.txt");
    let input = parse(input);
    println!("part1 = {}", part1(&input));
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

    assert_eq!(part1(&input), 6032);
}
