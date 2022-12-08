use std::cmp;

fn parse(input: &str) -> Vec<Vec<u32>> {
    input
        .lines()
        .map(|l| l.chars().flat_map(|c| c.to_digit(10)).collect())
        .collect()
}

fn part1(puzzle: &Vec<Vec<u32>>) -> usize {
    let visible = |pos_x: usize, pos_y: usize| {
        let height = puzzle[pos_y][pos_x];

        (0..pos_x).all(|x| height > puzzle[pos_y][x])
            || (pos_x + 1..puzzle[0].len()).all(|x| height > puzzle[pos_y][x])
            || (0..pos_y).all(|y| height > puzzle[y][pos_x])
            || (pos_y + 1..puzzle.len()).all(|y| height > puzzle[y][pos_x])
    };

    let mut count = 0;
    for y in 0..puzzle.len() {
        for x in 0..puzzle[0].len() {
            if visible(x, y) {
                count += 1;
            }
        }
    }

    count
}

fn count_trees(
    height: u32,
    puzzle: &[Vec<u32>],
    x_range: impl Iterator<Item = usize> + Clone,
    y_range: impl Iterator<Item = usize>,
) -> usize {
    let mut dist = 0;
    for y in y_range {
        for x in x_range.clone() {
            dist += 1;

            if height <= puzzle[y][x] {
                return dist;
            }
        }
    }

    dist
}

fn part2(puzzle: &Vec<Vec<u32>>) -> usize {
    let mut best_score = 0;
    let score = |pos_x: usize, pos_y: usize| {
        let height = puzzle[pos_y][pos_x];

        let mut res = 1;

        res *= count_trees(height, puzzle, (0..pos_x).rev(), pos_y..=pos_y);
        res *= count_trees(
            height,
            puzzle,
            pos_x + 1..puzzle[0].len(),
            pos_y..=pos_y,
        );
        res *= count_trees(height, puzzle, pos_x..=pos_x, (0..pos_y).rev());
        res *=
            count_trees(height, puzzle, pos_x..=pos_x, pos_y + 1..puzzle.len());

        res
    };

    for y in 0..puzzle.len() {
        for x in 0..puzzle[0].len() {
            best_score = cmp::max(best_score, score(x, y));
        }
    }

    best_score
}

fn main() {
    let input = include_str!("../../input/input08.txt");
    let input = parse(input);
    println!("part1 = {}", part1(&input));
    println!("part2 = {}", part2(&input));
}

#[test]
fn test_day08() {
    let input = "\
30373
25512
65332
33549
35390";
    let input = parse(input);

    assert_eq!(part1(&input), 21);
    assert_eq!(part2(&input), 8);
}
