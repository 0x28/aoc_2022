use rand::{rngs::SmallRng, Rng, SeedableRng};
use rayon::prelude::*;

#[derive(Debug)]
struct Blueprint {
    id: i64,
    ore_robot: i64,
    clay_robot: i64,
    obsidian_robot: (i64, i64),
    geode_robot: (i64, i64),
}

#[derive(Clone, Debug)]
enum Robot {
    Ore = 0,
    Clay = 1,
    Obsidian = 2,
    Geode = 3,
}

fn parse(input: &str) -> Vec<Blueprint> {
    let mut prints = vec![];

    for line in input.lines() {
        let nums = line.split(|c: char| !c.is_numeric());
        let mut numbers = vec![];

        for num in nums {
            if num.is_empty() {
                continue;
            }

            numbers.push(num.parse::<i64>().unwrap());
        }

        prints.push(Blueprint {
            id: numbers[0],
            ore_robot: numbers[1],
            clay_robot: numbers[2],
            obsidian_robot: (numbers[3], numbers[4]),
            geode_robot: (numbers[5], numbers[6]),
        })
    }

    prints
}

fn maybe_buy(
    blueprint: &Blueprint,
    robot: &Robot,
    ore: &mut i64,
    clay: &mut i64,
    obsidian: &mut i64,
) -> bool {
    match robot {
        Robot::Ore if blueprint.ore_robot <= *ore => {
            *ore -= blueprint.ore_robot;
            true
        }
        Robot::Clay if blueprint.clay_robot <= *ore => {
            *ore -= blueprint.clay_robot;
            true
        }
        Robot::Obsidian
            if blueprint.obsidian_robot.0 <= *ore
                && blueprint.obsidian_robot.1 <= *clay =>
        {
            *ore -= blueprint.obsidian_robot.0;
            *clay -= blueprint.obsidian_robot.1;
            true
        }
        Robot::Geode
            if blueprint.geode_robot.0 <= *ore
                && blueprint.geode_robot.1 <= *obsidian =>
        {
            *ore -= blueprint.geode_robot.0;
            *obsidian -= blueprint.geode_robot.1;
            true
        }
        _ => false,
    }
}

fn simulate(blueprint: &Blueprint, buylist: &[Robot], minutes: usize) -> i64 {
    let mut ore_robots = 1;
    let mut clay_robots = 0;
    let mut obsidian_robots = 0;
    let mut geode_robots = 0;
    let mut buy_idx = 0;

    let mut ore = 0;
    let mut clay = 0;
    let mut obsidian = 0;
    let mut geodes = 0;

    for _ in 0..minutes {
        let can_buy = buy_idx < buylist.len()
            && maybe_buy(
                blueprint,
                &buylist[buy_idx],
                &mut ore,
                &mut clay,
                &mut obsidian,
            );

        ore += ore_robots;
        clay += clay_robots;
        obsidian += obsidian_robots;
        geodes += geode_robots;

        if can_buy {
            match buylist[buy_idx] {
                Robot::Ore => ore_robots += 1,
                Robot::Clay => clay_robots += 1,
                Robot::Obsidian => obsidian_robots += 1,
                Robot::Geode => geode_robots += 1,
            }
            buy_idx += 1;
        }
    }

    geodes
}

fn generate_buylist(minutes: usize) -> Vec<Robot> {
    let mut buylist = vec![];
    let robots = [Robot::Ore, Robot::Clay, Robot::Obsidian, Robot::Geode];
    let mut rng = SmallRng::from_entropy();

    for _ in 0..minutes {
        buylist.push(robots[rng.gen_range(0..robots.len())].clone());
    }

    buylist
}

fn repopulate(population: &mut Vec<(Vec<Robot>, i64)>) {
    let survivor_size = 50;
    let aliens = 10;
    let mut rng = SmallRng::from_entropy();
    let robots = [Robot::Ore, Robot::Clay, Robot::Obsidian, Robot::Geode];

    for idx in survivor_size..population.len() - aliens {
        let left_idx = rng.gen_range(0..survivor_size);
        let right_idx = rng.gen_range(0..survivor_size);

        for element_idx in 0..population[idx].0.len() {
            population[idx].1 = -1; // reset
            population[idx].0[element_idx] = if rng.gen_bool(0.4) {
                robots[rng.gen_range(0..robots.len())].clone()
            } else if rng.gen_bool(0.5) {
                population[left_idx].0[element_idx].clone()
            } else {
                population[right_idx].0[element_idx].clone()
            };
        }
    }

    for idx in population.len() - aliens..population.len() {
        population[idx]
            .0
            .fill_with(|| robots[rng.gen_range(0..robots.len())].clone());
    }
}

fn evolve_solution(blueprint: &Blueprint, minutes: usize) -> i64 {
    let mut population = (0..1000)
        .map(|_| (generate_buylist(minutes), 0))
        .collect::<Vec<_>>();

    for _gen in 0..2000 {
        for individual in population.iter_mut() {
            if individual.1 < 0 {
                individual.1 = simulate(blueprint, &individual.0, minutes);
            }
        }

        population.sort_by(|l, r| r.1.cmp(&l.1));
        repopulate(&mut population);
    }

    population[0].1
}

fn approx(blueprints: &[Blueprint]) {
    let mut best_part1 = vec![0; blueprints.len() + 1];
    let mut best_part2 = vec![1; 4];

    for iteration in 1.. {
        let p1 = blueprints
            .iter()
            .par_bridge()
            .map(|blueprint| (blueprint.id, evolve_solution(blueprint, 24)))
            .collect::<Vec<_>>();

        let p2 = blueprints
            .iter()
            .take(3)
            .par_bridge()
            .map(|blueprint| (blueprint.id, evolve_solution(blueprint, 32)))
            .collect::<Vec<_>>();

        for (id, score) in p2 {
            best_part2[id as usize] = i64::max(best_part2[id as usize], score);
        }

        for (id, score) in p1 {
            best_part1[id as usize] =
                i64::max(best_part1[id as usize], id * score);
        }

        println!(
            "iteration {iteration}:\n\
             * best result for part1: {}\n\
             * best result for part2: {}\n",
            best_part1.iter().sum::<i64>(),
            best_part2.iter().product::<i64>()
        );
    }
}

fn main() {
    let input = parse(include_str!("../../input/input19.txt"));

    // NOTE: This is a evolutionary solution. It isn't always correct but it's
    // enough to solve this problem after some time. Part 2 can take some
    // time...
    approx(&input);
}

#[test]
fn test_day19() {
    let input = "\
Blueprint 1: \
  Each ore robot costs 4 ore. \
  Each clay robot costs 2 ore. \
  Each obsidian robot costs 3 ore and 14 clay. \
  Each geode robot costs 2 ore and 7 obsidian.
Blueprint 2: \
  Each ore robot costs 2 ore. \
  Each clay robot costs 3 ore. \
  Each obsidian robot costs 3 ore and 8 clay. \
  Each geode robot costs 3 ore and 12 obsidian.";
    let input = parse(input);

    assert_eq!(
        simulate(
            &input[0],
            &[
                Robot::Clay,
                Robot::Clay,
                Robot::Clay,
                Robot::Obsidian,
                Robot::Clay,
                Robot::Obsidian,
                Robot::Geode,
                Robot::Geode
            ],
            24
        ),
        9
    );
    assert_eq!(
        simulate(
            &input[0],
            &[
                Robot::Ore,
                Robot::Clay,
                Robot::Clay,
                Robot::Clay,
                Robot::Clay,
                Robot::Clay,
                Robot::Clay,
                Robot::Clay,
                Robot::Obsidian,
                Robot::Obsidian,
                Robot::Obsidian,
                Robot::Obsidian,
                Robot::Geode,
                Robot::Obsidian,
                Robot::Geode,
                Robot::Geode,
                Robot::Geode,
                Robot::Geode,
                Robot::Geode,
                Robot::Geode,
                Robot::Geode,
                Robot::Geode
            ],
            32
        ),
        56
    );
}
