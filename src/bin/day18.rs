use ahash::AHashSet;

fn parse(input: &str) -> AHashSet<[i64; 3]> {
    let mut cubes = AHashSet::default();
    for line in input.lines() {
        let mut cube = [0; 3];
        let mut elems = line.split(',');

        for side in cube.iter_mut() {
            *side = elems.next().unwrap().parse::<i64>().unwrap();
        }

        cubes.insert(cube);
    }
    cubes
}

fn dirs(&[x, y, z]: &[i64; 3]) -> Vec<[i64; 3]> {
    vec![
        [x + 1, y, z],
        [x - 1, y, z],
        [x, y + 1, z],
        [x, y - 1, z],
        [x, y, z + 1],
        [x, y, z - 1],
    ]
}

fn part1(cubes: &AHashSet<[i64; 3]>) -> i64 {
    let mut total_area = 0;

    for cube in cubes {
        for dir in dirs(cube) {
            if !cubes.contains(&dir) {
                total_area += 1;
            }
        }
    }

    total_area
}

fn part2(cubes: &AHashSet<[i64; 3]>) -> i64 {
    let mut total_area = 0;
    let mut water = AHashSet::default();
    let mut expanded_water = AHashSet::default();

    let mut max_x = i64::MIN;
    let mut min_x = i64::MAX;
    let mut max_y = i64::MIN;
    let mut min_y = i64::MAX;
    let mut max_z = i64::MIN;
    let mut min_z = i64::MAX;

    for &[x, y, z] in cubes {
        max_x = i64::max(max_x, x);
        min_x = i64::min(min_x, x);
        max_y = i64::max(max_y, y);
        min_y = i64::min(min_y, y);
        max_z = i64::max(max_z, z);
        min_z = i64::min(min_z, z);
    }

    water.insert([min_x - 1, min_y - 1, min_z - 1]);

    loop {
        let mut new_water = AHashSet::default();
        for w in water.iter().copied() {
            if expanded_water.contains(&w) {
                continue;
            }

            for [x, y, z] in dirs(&w) {
                if x <= max_x + 1
                    && x >= min_x - 1
                    && y <= max_y + 1
                    && y >= min_y - 1
                    && z <= max_z + 1
                    && z >= min_z - 1
                    && !cubes.contains(&[x, y, z])
                {
                    new_water.insert([x, y, z]);
                }
            }

            expanded_water.insert(w);
        }

        let old_water_count = water.len();
        water.extend(new_water.iter());

        if old_water_count == water.len() {
            break;
        }
    }

    for w in &water {
        for dir in dirs(w) {
            if cubes.contains(&dir) {
                total_area += 1;
            }
        }
    }

    total_area
}

fn main() {
    let input = include_str!("../../input/input18.txt");
    let input = parse(input);
    println!("part1 = {}", part1(&input));
    println!("part2 = {}", part2(&input));
}

#[test]
fn test_day18() {
    let input = "\
2,2,2
1,2,2
3,2,2
2,1,2
2,3,2
2,2,1
2,2,3
2,2,4
2,2,6
1,2,5
3,2,5
2,1,5
2,3,5";
    let input = parse(input);

    assert_eq!(part1(&input), 64);
    assert_eq!(part2(&input), 58);
}
