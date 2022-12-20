fn parse(input: &str) -> Vec<i64> {
    input.lines().flat_map(|num| num.parse().ok()).collect()
}

fn solve(nums: &[i64], key: i64, iter: usize) -> i64 {
    let mut indices = (0..nums.len()).collect::<Vec<_>>();

    for _ in 0..iter {
        for (idx, num) in nums.iter().map(|n| n * key).enumerate() {
            let old_pos = indices.iter().position(|&i| i == idx).unwrap();
            indices.remove(old_pos);
            let new_pos =
                (old_pos as i64 + num).rem_euclid(indices.len() as i64);
            indices.insert(new_pos as usize, idx);
        }
    }

    let org_zero_idx = nums.iter().position(|&i| i == 0).unwrap();
    let zero_idx = indices.iter().position(|&i| i == org_zero_idx).unwrap();

    [1000, 2000, 3000]
        .into_iter()
        .map(|idx| key * nums[indices[(zero_idx + idx) % indices.len()]])
        .sum()
}

fn main() {
    let input = include_str!("../../input/input20.txt");
    let input = parse(input);
    println!("part1 = {}", solve(&input, 1, 1));
    println!("part2 = {}", solve(&input, 811589153, 10));
}

#[test]
fn test_day20() {
    let input = "\
1
2
-3
3
-2
0
4";
    let input = parse(input);

    assert_eq!(solve(&input, 1, 1), 3);
    assert_eq!(solve(&input, 811589153, 10), 1623178306);
}
