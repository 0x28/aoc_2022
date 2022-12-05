use std::collections::VecDeque;

#[derive(Debug, Clone)]
struct Puzzle {
    stacks: Vec<VecDeque<char>>,
    moves: Vec<(usize, usize, usize)>,
}

fn parse(input: &str) -> Puzzle {
    let num_stacks = input.lines().next().unwrap().len() / 4 + 1;
    let mut stacks = vec![VecDeque::new(); num_stacks];
    let mut moves = vec![];

    let parts: Vec<_> = input.split("\n\n").collect();

    for line in parts[0].lines() {
        let mut line = line.chars().enumerate().peekable();

        while let Some(&(i, c)) = line.peek() {
            if c == '[' {
                line.next();
                stacks[i / 4].push_front(line.next().unwrap().1);
                line.next();
                line.next();
            } else {
                line.next();
            }
        }
    }

    for line in parts[1].lines() {
        let words: Vec<_> = line.split(' ').collect();
        moves.push((
            words[1].parse().unwrap(),
            words[3].parse::<usize>().unwrap() - 1,
            words[5].parse::<usize>().unwrap() - 1,
        ));
    }

    Puzzle { stacks, moves }
}

fn part1(mut puzzle: Puzzle) -> String {
    for &(count, from, to) in &puzzle.moves {
        for _ in 0..count {
            let elem = puzzle.stacks[from].pop_back().unwrap();
            puzzle.stacks[to].push_back(elem);
        }
    }

    puzzle.stacks.iter().map(|s| s.iter().last().unwrap()).collect()
}

fn part2(mut puzzle: Puzzle) -> String {
    let mut tmp_stack = vec![];
    for &(count, from, to) in &puzzle.moves {
        for _ in 0..count {
            let elem = puzzle.stacks[from].pop_back().unwrap();
            tmp_stack.push(elem);
        }

        for _ in 0..count {
            puzzle.stacks[to].push_back(tmp_stack.pop().unwrap());
        }
    }

    puzzle.stacks.iter().map(|s| s.iter().last().unwrap()).collect()
}

fn main() {
    let input = include_str!("../../input/input05.txt");
    let input = parse(input);
    println!("part1 = {}", part1(input.clone()));
    println!("part2 = {}", part2(input));
}

#[test]
fn test_day05() {
    let input = "    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";
    let input = parse(input);

    assert_eq!(part1(input.clone()), "CMZ");
    assert_eq!(part2(input), "MCD");
}
