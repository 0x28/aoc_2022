use ahash::AHashMap;

#[derive(Debug)]
struct Board {
    blizzards: Vec<(i64, i64, char)>,
    width: i64,
    height: i64,
}

fn parse(input: &str) -> Board {
    let mut board = Board {
        blizzards: vec![],
        width: 0,
        height: 0,
    };

    for (y, line) in input.lines().enumerate() {
        board.width = 0;
        for (x, c) in line.char_indices() {
            match c {
                '>' | 'v' | '^' | '<' => {
                    board.blizzards.push((x as i64 - 1, y as i64 - 1, c));
                }
                _ => (),
            }

            board.width += 1;
        }

        board.height += 1;
    }

    board.width -= 2;
    board.height -= 2;

    board
}

fn collision(
    pos: &(i64, i64),
    board: &Board,
    time: i64,
    cache: &mut Cache,
) -> bool {
    let state = (pos.0, pos.1, time % board.height, time % board.width);
    if let Some(col) = cache.collision_cache.get(&state) {
        return *col;
    }

    let mut collision_detected = false;

    for &(x, y, dir) in &board.blizzards {
        match dir {
            '>' => {
                if *pos == ((x + time).rem_euclid(board.width), y) {
                    collision_detected = true;
                    break;
                }
            }
            'v' => {
                if *pos == (x, (y + time).rem_euclid(board.height)) {
                    collision_detected = true;
                    break;
                }
            }
            '^' => {
                if *pos == (x, (y - time).rem_euclid(board.height)) {
                    collision_detected = true;
                    break;
                }
            }
            '<' => {
                if *pos == ((x - time).rem_euclid(board.width), y) {
                    collision_detected = true;
                    break;
                }
            }
            _ => unreachable!(),
        }
    }

    cache.collision_cache.insert(state, collision_detected);

    collision_detected
}

struct Cache {
    dist_cache: AHashMap<(i64, i64, i64), i64>,
    collision_cache: AHashMap<(i64, i64, i64, i64), bool>,
    best_dist: i64,
}

impl Cache {
    fn new() -> Self {
        Cache {
            dist_cache: AHashMap::new(),
            collision_cache: AHashMap::new(),
            best_dist: i64::MAX,
        }
    }
}

fn walk(
    pos: (i64, i64),
    board: &Board,
    time: i64,
    cache: &mut Cache,
    goal: (i64, i64),
) -> i64 {
    let mut min_dist = i64::MAX;
    let state = (pos.0, pos.1, time);

    if pos == goal {
        return time;
    }

    if let Some(dist) = cache.dist_cache.get(&state) {
        return *dist;
    }

    if time > cache.best_dist
        || time >= 1000
        || collision(&pos, board, time, cache)
    {
        return i64::MAX;
    }

    min_dist = i64::min(min_dist, walk(pos, board, time + 1, cache, goal));

    if pos.0 < board.width - 1 && (0..board.height).contains(&pos.1) {
        min_dist = i64::min(
            min_dist,
            walk((pos.0 + 1, pos.1), board, time + 1, cache, goal),
        );
    }
    if pos.1 < board.height - 1
        || (pos.0 == board.width - 1 && pos.1 <= board.height)
    {
        min_dist = i64::min(
            min_dist,
            walk((pos.0, pos.1 + 1), board, time + 1, cache, goal),
        );
    }
    if pos.0 > 0 && (0..board.height).contains(&pos.1) {
        min_dist = i64::min(
            min_dist,
            walk((pos.0 - 1, pos.1), board, time + 1, cache, goal),
        );
    }
    if pos.1 > 0 || (pos.0 == 0 && pos.1 >= -1) {
        min_dist = i64::min(
            min_dist,
            walk((pos.0, pos.1 - 1), board, time + 1, cache, goal),
        );
    }

    if let Some(dist) = cache.dist_cache.get_mut(&state) {
        *dist = i64::min(*dist, min_dist);
    } else {
        cache.dist_cache.insert(state, min_dist);
    }

    cache.best_dist = i64::min(cache.best_dist, min_dist);

    min_dist
}

fn part1(board: &Board) -> i64 {
    walk(
        (0, -1),
        board,
        0,
        &mut Cache::new(),
        (board.width - 1, board.height),
    )
}

fn part2(board: &Board) -> i64 {
    let mut cache = Cache::new();
    let start = (0, -1);
    let goal = (board.width - 1, board.height);
    let first_dist = walk(start, board, 0, &mut cache, goal);
    cache.best_dist = i64::MAX;
    cache.dist_cache = AHashMap::default();
    let second_dist = walk(goal, board, first_dist, &mut cache, start);
    cache.best_dist = i64::MAX;
    cache.dist_cache = AHashMap::default();
    walk(start, board, second_dist, &mut cache, goal)
}

fn main() {
    let input = include_str!("../../input/input24.txt");
    let input = parse(input);
    println!("part1 = {}", part1(&input));
    println!("part2 = {}", part2(&input));
}

#[test]
fn test_day24() {
    let input = "\
#.######
#>>.<^<#
#.<..<<#
#>v.><>#
#<^v^^>#
######.#";
    let input = parse(input);

    assert_eq!(part1(&input), 18);
    assert_eq!(part2(&input), 54);
}
