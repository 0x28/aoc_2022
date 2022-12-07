use std::collections::HashMap;

fn parse(input: &str) -> Vec<(Vec<String>, usize)> {
    let mut files = vec![];
    let mut lines = input.lines().peekable();

    let mut cwd = vec![];

    while let Some(line) = lines.peek() {
        let words = line.split_ascii_whitespace().collect::<Vec<_>>();
        lines.next();
        match words.as_slice() {
            ["$", "cd", ".."] => {
                cwd.pop();
            }
            ["$", "cd", "/"] => {}
            ["$", "cd", dir] => {
                cwd.push(dir.to_string());
            }
            ["$", "ls"] => {
                while let Some(output) = lines.peek() {
                    if output.starts_with('$') {
                        break;
                    }

                    let entry =
                        output.split_ascii_whitespace().collect::<Vec<_>>();

                    if let (Ok(size), file) = (entry[0].parse(), entry[1]) {
                        let mut entry = cwd.clone();
                        entry.extend([file.to_owned()]);
                        files.push((entry, size));
                    }

                    lines.next();
                }
            }
            _ => unreachable!(),
        }
    }

    files
}

fn dir_sizes(files: &[(Vec<String>, usize)]) -> HashMap<String, usize> {
    let mut sizes = HashMap::new();

    for (path, size) in files {
        for dir in 0..path.len() - 1 {
            let dir = path[0..dir + 1].join("/");
            if let Some(entry) = sizes.get_mut(&dir) {
                *entry += size;
            } else {
                sizes.insert(dir, *size);
            }
        }
    }

    sizes
}

fn part1(files: &[(Vec<String>, usize)]) -> usize {
    dir_sizes(files)
        .iter()
        .map(|(_, &size)| if size <= 100_000 { size } else { 0 })
        .sum()
}

fn part2(files: &[(Vec<String>, usize)]) -> usize {
    let mut sizes =
        dir_sizes(files).iter().map(|(_, &s)| s).collect::<Vec<_>>();
    let total_size: usize = files.iter().map(|(_, s)| s).sum();

    sizes.sort_unstable();

    for size in sizes.iter() {
        if total_size - size <= 40000000 {
            return *size;
        }
    }

    unreachable!()
}

fn main() {
    let input = include_str!("../../input/input07.txt");
    let input = parse(input);
    println!("part1 = {}", part1(&input));
    println!("part2 = {}", part2(&input));
}

#[test]
fn test_day07() {
    let input = "\
$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k";
    let input = parse(input);

    assert_eq!(part1(&input), 95437);
    assert_eq!(part2(&input), 24933642);
}
