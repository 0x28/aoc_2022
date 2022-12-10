enum Instruction {
    Add(i64),
    Noop,
}

fn parse(input: &str) -> Vec<Instruction> {
    let mut instructions = vec![];

    for line in input.lines() {
        let line: Vec<_> = line.split_ascii_whitespace().collect();

        instructions.push(match line.as_slice() {
            ["addx", operand] => Instruction::Add(operand.parse().unwrap()),
            ["noop"] => Instruction::Noop,
            _ => unreachable!(),
        });
    }

    instructions
}

struct Handheld {
    clock: i64,
    register: i64,
    strength: i64,
    screen: [[char; 40]; 6],
}

impl Handheld {
    fn new() -> Handheld {
        let mut h = Handheld {
            clock: 0,
            register: 1,
            strength: 0,
            screen: [[' '; 40]; 6],
        };

        h.tick();
        h
    }

    fn tick(&mut self) {
        self.update_screen();
        self.clock += 1;

        if (20..=220).step_by(40).any(|e| self.clock == e) {
            self.strength += self.register * self.clock;
        }
    }

    fn interpret(&mut self, instructions: &[Instruction]) {
        for ins in instructions {
            match ins {
                Instruction::Add(v) => {
                    self.tick();
                    self.register += v;
                    self.tick();
                }
                Instruction::Noop => self.tick(),
            }
        }
    }

    fn update_screen(&mut self) {
        let y_pos = self.clock / 40;
        let x_pos = self.clock % 40;

        if let Some(pos) = self
            .screen
            .get_mut(y_pos as usize)
            .and_then(|r| r.get_mut(x_pos as usize))
        {
            *pos = if i64::abs(self.register - x_pos) <= 1 {
                '#'
            } else {
                '.'
            };
        }
    }

    fn show_screen(&self) {
        for row in self.screen {
            for pixel in row {
                print!("{}", pixel);
            }

            println!();
        }
    }
}

fn solve(instructions: &[Instruction]) -> i64 {
    let mut cpu = Handheld::new();

    cpu.interpret(instructions);
    cpu.show_screen();
    cpu.strength
}

fn main() {
    let input = include_str!("../../input/input10.txt");
    let input = parse(input);
    println!("part1 = {}", solve(&input));
}

#[test]
fn test_day10() {
    let input = "\
addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop";
    let input = parse(input);

    assert_eq!(solve(&input), 13140);
}
