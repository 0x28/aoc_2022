enum Dir {
    Left,
    Right,
}

fn parse(input: &str) -> Vec<Dir> {
    input
        .chars()
        .flat_map(|c| match c {
            '<' => Some(Dir::Left),
            '>' => Some(Dir::Right),
            _ => None,
        })
        .collect()
}

#[derive(Clone)]
struct Rock {
    position: (usize, usize),
    blocks: Vec<(usize, usize)>,
}

impl Rock {
    fn left(&mut self, chamber: &Chamber) {
        for &(x, y) in &self.blocks {
            if let Some(x) = (self.position.0 + x).checked_sub(1) {
                if chamber.field[self.position.1 - y][x] != '.' {
                    return;
                }
            } else {
                return;
            }
        }

        self.position.0 -= 1;
    }

    fn right(&mut self, chamber: &Chamber) {
        let width = chamber.field[0].len();

        for &(x, y) in &self.blocks {
            let x = self.position.0 + x + 1;
            if x >= width {
                return;
            }

            if chamber.field[self.position.1 - y][x] != '.' {
                return;
            }
        }

        self.position.0 += 1;
    }

    fn down(&mut self, chamber: &Chamber) -> bool {
        for &(x, y) in &self.blocks {
            if let Some(y) = self.position.1.checked_sub(y + 1) {
                if chamber.field[y][self.position.0 + x] != '.' {
                    return true;
                }
            } else {
                return true;
            }
        }

        self.position.1 -= 1;
        false
    }

    fn place(&self, chamber: &mut Chamber) -> usize {
        let mut max = 0;

        for (mut x, mut y) in &self.blocks {
            x += self.position.0;
            y = self.position.1 - y;

            max = usize::max(max, y);

            chamber.field[y][x] = '#';
        }

        max + 1
    }

    fn height(&self) -> usize {
        self.blocks.iter().map(|(_, py)| *py).max().unwrap() + 1
    }
}

fn make_rocks() -> Vec<Rock> {
    vec![
        // vbar
        Rock {
            position: (0, 0),
            blocks: vec![(0, 0), (1, 0), (2, 0), (3, 0)],
        },
        // cross
        Rock {
            position: (0, 0),
            blocks: vec![(1, 0), (0, 1), (1, 1), (2, 1), (1, 2)],
        },
        // L
        Rock {
            position: (0, 0),
            blocks: vec![(2, 0), (2, 1), (0, 2), (1, 2), (2, 2)],
        },
        // I
        Rock {
            position: (0, 0),
            blocks: vec![(0, 0), (0, 1), (0, 2), (0, 3)],
        },
        // Box
        Rock {
            position: (0, 0),
            blocks: vec![(0, 0), (1, 0), (0, 1), (1, 1)],
        },
    ]
}

struct Chamber {
    field: Vec<[char; 7]>,
}

impl Chamber {
    #[allow(unused)]
    fn show(&self, rock: &Rock) {
        let mut rock_pos = rock
            .blocks
            .iter()
            .map(|(px, py)| (rock.position.0 + px, rock.position.1 - py));

        for y in (0..self.field.len()).rev() {
            for x in 0..self.field[0].len() {
                if rock_pos.any(|pos| pos == (x, y)) {
                    print!("@");
                } else {
                    print!("{}", self.field[y][x]);
                }
            }
            println!();
        }
        println!();
    }
}

fn part1(dirs: &[Dir]) -> usize {
    let mut chamber = Chamber {
        field: vec![['.'; 7]; 4],
    };
    let possible_rocks = make_rocks();
    let mut dirs = dirs.iter().cycle();
    let mut count = 0;
    let mut max_height = 0;

    for rock in possible_rocks.iter().cycle() {
        chamber
            .field
            .resize(max_height + rock.height() + 3, ['.'; 7]);

        let mut rock = rock.clone();
        rock.position.0 = 2;
        rock.position.1 = chamber.field.len() - 1;

        for wind in dirs.by_ref() {
            // chamber.show(&rock);

            match wind {
                Dir::Left => rock.left(&chamber),
                Dir::Right => rock.right(&chamber),
            }

            if rock.down(&chamber) {
                max_height = usize::max(rock.place(&mut chamber), max_height);
                break;
            }

            // chamber.show(&rock);
        }

        count += 1;
        if count >= 2022 {
            break;
        }
    }

    max_height
}

fn main() {
    let input = include_str!("../../input/input17.txt");
    let input = parse(input);
    println!("part1 = {}", part1(&input));
    // println!("part2 = {}", part2(&input));
}

#[test]
fn test_day17() {
    let input = ">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>";
    let input = parse(input);

    assert_eq!(part1(&input), 3068);
    // assert_eq!(part2(&input), 0);
}
