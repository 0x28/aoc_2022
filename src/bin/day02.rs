use aoc_2022::input_file;
use std::fs;

#[derive(Clone, PartialEq)]
enum Play {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

#[derive(PartialEq)]
enum GameResult {
    Lose = 0,
    Draw = 3,
    Win = 6,
}

fn game(opponent: &Play, you: &Play) -> GameResult {
    match (opponent, you) {
        (Play::Paper, Play::Scissors) => GameResult::Win,
        (Play::Scissors, Play::Rock) => GameResult::Win,
        (Play::Rock, Play::Paper) => GameResult::Win,
        _ if opponent == you => GameResult::Draw,
        _ => GameResult::Lose,
    }
}

fn parse_play(input: u8) -> Play {
    match input {
        b'A' | b'X' => Play::Rock,
        b'B' | b'Y' => Play::Paper,
        b'C' | b'Z' => Play::Scissors,
        _ => unreachable!(),
    }
}

fn parse(input: &str) -> Vec<(Play, Play)> {
    input
        .lines()
        .map(|l| {
            let words: Vec<_> =
                l.split(' ').map(|s| parse_play(s.as_bytes()[0])).collect();
            (words[0].clone(), words[1].clone())
        })
        .collect()
}

fn part1(puzzle: &[(Play, Play)]) -> i64 {
    let mut score = 0;
    for (opponent, you) in puzzle {
        score += *you as i64;

        score += game(opponent, you) as i64
    }

    score
}

fn part2(puzzle: &[(Play, Play)]) -> i64 {
    let mut score = 0;

    for (opponent, you) in puzzle {
        let result = match you {
            Play::Rock => GameResult::Lose,
            Play::Paper => GameResult::Draw,
            Play::Scissors => GameResult::Win,
        };

        for play in [Play::Rock, Play::Paper, Play::Scissors] {
            if game(opponent, &play) == result {
                score += play as i64 + result as i64;
                break;
            }
        }
    }

    score
}

fn main() {
    let input = fs::read_to_string(input_file("input02.txt")).unwrap();
    let input = parse(&input);
    println!("part1 = {}", part1(&input));
    println!("part2 = {}", part2(&input));
}

#[test]
fn test_day() {
    let input = "\
A Y
B X
C Z";
    let input = parse(input);

    assert_eq!(part1(&input), 15);
    assert_eq!(part2(&input), 12);
}
