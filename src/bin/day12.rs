use std::{cmp::Reverse, collections::BinaryHeap};

struct Map {
    heights: Vec<Vec<u8>>,
    start: (usize, usize),
    end: (usize, usize),
}

impl Map {
    fn height_at(&self, x: usize, y: usize) -> Option<i64> {
        self.heights
            .get(y)
            .and_then(|r| r.get(x).map(|&h| h as i64))
    }
}

fn parse(input: &str) -> Map {
    let mut heights: Vec<Vec<_>> = input
        .lines()
        .map(|l| l.chars().map(|c| c as u8).collect())
        .collect();

    let mut end = (0, 0);
    let mut start = (0, 0);

    for y in 0..heights.len() {
        for x in 0..heights[0].len() {
            heights[y][x] = match heights[y][x] as char {
                'S' => {
                    start = (x, y);
                    b'a'
                }
                'E' => {
                    end = (x, y);
                    b'z'
                }
                c => c as u8,
            };
        }
    }

    Map {
        heights,
        start,
        end,
    }
}

type Node = (usize, (usize, usize));

fn neighbors((x, y): (usize, usize), map: &Map) -> Vec<Node> {
    let mut neighbors = vec![];
    let current_height = map.height_at(x, y).unwrap();

    for (hori, vert) in [
        (x, y.wrapping_sub(1)),
        (x.wrapping_sub(1), y),
        (x, y + 1),
        (x + 1, y),
    ] {
        if let Some(new_height) = map.height_at(hori, vert) {
            if new_height - current_height <= 1 {
                neighbors.push((1, (hori, vert)));
            }
        }
    }
    neighbors
}

fn dijkstra(map: &Map, start: &(usize, usize)) -> Option<usize> {
    let width = map.heights[0].len();
    let mut dist: Vec<_> =
        (0..map.heights.len() * width).map(|_| usize::MAX).collect();

    let to_idx = |(x, y)| y * width + x;

    let mut heap = BinaryHeap::<Reverse<Node>>::new();

    dist[to_idx(*start)] = 0;
    heap.push(Reverse((0, *start)));

    while let Some(Reverse((cost, position))) = heap.pop() {
        if position == map.end {
            return Some(cost);
        }

        if cost > dist[to_idx(position)] {
            continue;
        }

        for (ncost, npos) in neighbors(position, map) {
            let next_cost = cost + ncost;
            let next = Reverse((next_cost, npos));

            if next_cost < dist[to_idx(npos)] {
                heap.push(next);
                dist[to_idx(npos)] = next_cost;
            }
        }
    }

    None
}

fn part1(map: &Map) -> usize {
    dijkstra(map, &map.start).unwrap()
}

fn part2(map: &Map) -> usize {
    let mut apos = vec![];
    for y in 0..map.heights.len() {
        for x in 0..map.heights[0].len() {
            if map.heights[y][x] == b'a' {
                apos.push((x, y));
            }
        }
    }

    apos.iter().flat_map(|p| dijkstra(map, p)).min().unwrap()
}

fn main() {
    let input = include_str!("../../input/input12.txt");
    let input = parse(input);
    println!("part1 = {}", part1(&input));
    println!("part2 = {}", part2(&input));
}

#[test]
fn test_day12() {
    let input = "\
Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi";
    let input = parse(input);

    assert_eq!(part1(&input), 31);
    assert_eq!(part2(&input), 29);
}
