use aoc_2022::input_file;
use std::fs;

fn main() {
    let input = fs::read_to_string(input_file("input01.txt")).unwrap();
    let groups: Vec<Vec<u64>> = input
        .split("\n\n")
        .map(|g| g.lines().flat_map(str::parse).collect())
        .collect();

    let mut group_sums: Vec<u64> =
        groups.iter().map(|g| g.iter().sum()).collect();
    group_sums.sort();

    println!("part1 = {}", group_sums.last().unwrap());
    println!("part2 = {}", group_sums.iter().rev().take(3).sum::<u64>());
}
