use rayon::prelude::*;
use std::mem;

fn parse(input: &str) -> Vec<[i64; 4]> {
    let mut sensor_list = vec![];

    for line in input.lines() {
        let mut points = [0; 4];
        let mut idx = 0;
        let mut current_num_str = String::new();

        for c in line.chars() {
            if c.is_numeric() || c == '-' {
                current_num_str.push(c);
            } else if !current_num_str.is_empty() {
                points[idx] = current_num_str.parse().unwrap();
                idx += 1;
                current_num_str.clear();
            }
        }

        points[idx] = current_num_str.parse().unwrap();
        sensor_list.push(points);
    }

    sensor_list
}

fn manhattan_distance(x1: i64, y1: i64, x2: i64, y2: i64) -> i64 {
    x1.abs_diff(x2) as i64 + y1.abs_diff(y2) as i64
}

fn simplify_intervals(intervals: &mut Vec<(i64, i64)>) -> Vec<(i64, i64)> {
    let mut simplified = vec![];

    loop {
        'outer_loop: for interval in intervals.iter() {
            if simplified.is_empty() {
                simplified.push(*interval);
            } else {
                let mut is_disjoint = false;
                for (begin, end) in &mut simplified {
                    if interval.0 <= *begin && *end <= interval.1 {
                        *begin = interval.0; // incoming is bigger
                        *end = interval.1;
                    } else if *begin <= interval.0 && interval.1 <= *end {
                        continue 'outer_loop; // contained -> skip
                    } else if *begin <= interval.0 && interval.0 <= *end + 1 {
                        *end = interval.1; // extend right
                        break;
                    } else if *begin - 1 <= interval.1 && interval.1 <= *end {
                        *begin = interval.0; // extend left
                        break;
                    } else {
                        is_disjoint = true;
                    }
                }

                if is_disjoint {
                    simplified.push(*interval);
                }
            }
        }

        if intervals == &simplified {
            return simplified;
        }

        mem::swap(&mut simplified, intervals);
        simplified.clear();
    }
}

fn part1(ranges: &[[i64; 4]], y: i64) -> i64 {
    let mut x_ranges = vec![];

    for &[sensor_x, sensor_y, beacon_x, beacon_y] in ranges {
        let range_dist =
            manhattan_distance(sensor_x, sensor_y, beacon_x, beacon_y);
        let y_dist = sensor_y.abs_diff(y) as i64;
        if range_dist >= y_dist {
            let x_pos1 = sensor_x - (range_dist - y_dist);
            let x_pos2 = sensor_x + (range_dist - y_dist);

            x_ranges.push((x_pos1, x_pos2));
        }
    }

    simplify_intervals(&mut x_ranges)
        .iter()
        .map(|(x1, x2)| x1.abs_diff(*x2) as i64)
        .sum()
}

fn part2(ranges: &[[i64; 4]], y_max: i64) -> i64 {
    (0..=y_max)
        .into_par_iter()
        .find_map_first(|y| {
            let mut x_ranges = vec![];
            for &[sensor_x, sensor_y, beacon_x, beacon_y] in ranges {
                let range_dist =
                    manhattan_distance(sensor_x, sensor_y, beacon_x, beacon_y);
                let y_dist = sensor_y.abs_diff(y) as i64;
                if range_dist >= y_dist {
                    let x_pos1 = sensor_x - (range_dist - y_dist);
                    let x_pos2 = sensor_x + (range_dist - y_dist);

                    x_ranges.push((x_pos1, x_pos2));
                }
            }

            let intervals = simplify_intervals(&mut x_ranges);

            // find the gap
            if intervals.len() == 2 {
                Some((intervals[0].1 + 1) * 4_000_000 + y)
            } else {
                None
            }
        })
        .unwrap()
}

fn main() {
    let input = include_str!("../../input/input15.txt");
    let input = parse(input);
    println!("part1 = {}", part1(&input, 2_000_000));
    println!("part2 = {}", part2(&input, 4_000_000));
}

#[test]
fn test_day15() {
    let input = "\
Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16
Sensor at x=13, y=2: closest beacon is at x=15, y=3
Sensor at x=12, y=14: closest beacon is at x=10, y=16
Sensor at x=10, y=20: closest beacon is at x=10, y=16
Sensor at x=14, y=17: closest beacon is at x=10, y=16
Sensor at x=8, y=7: closest beacon is at x=2, y=10
Sensor at x=2, y=0: closest beacon is at x=2, y=10
Sensor at x=0, y=11: closest beacon is at x=2, y=10
Sensor at x=20, y=14: closest beacon is at x=25, y=17
Sensor at x=17, y=20: closest beacon is at x=21, y=22
Sensor at x=16, y=7: closest beacon is at x=15, y=3
Sensor at x=14, y=3: closest beacon is at x=15, y=3
Sensor at x=20, y=1: closest beacon is at x=15, y=3";
    let input = parse(input);

    assert_eq!(part1(&input, 10), 26);
    assert_eq!(part2(&input, 20), 56000011);
}
