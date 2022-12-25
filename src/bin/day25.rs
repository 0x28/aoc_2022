fn parse(input: &str) -> Vec<&str> {
    input.lines().collect()
}

fn snafu2num(snafu: &str) -> i64 {
    let mut num = 0;
    for c in snafu.chars() {
        let val = match c {
            '-' => -1,
            '=' => -2,
            '0' => 0,
            '1' => 1,
            '2' => 2,
            _ => unreachable!(),
        };

        num = num * 5 + val;
    }

    num
}

fn num2snafu(mut num: i64) -> String {
    let mut snafu_num = String::new();

    while num > 0 {
        let val = num % 5;
        num /= 5;

        match val {
            0 => snafu_num.push('0'),
            1 => snafu_num.push('1'),
            2 => snafu_num.push('2'),
            3 => {
                snafu_num.push('=');
                num += 1;
            }
            4 => {
                snafu_num.push('-');
                num += 1;
            }
            _ => unreachable!(),
        }
    }

    snafu_num.chars().rev().collect()
}

fn solve(puzzle: &[&str]) -> String {
    num2snafu(puzzle.iter().map(|s| snafu2num(s)).sum())
}

fn main() {
    let input = include_str!("../../input/input25.txt");
    let input = parse(input);
    println!("answer = {}", solve(&input));
}

#[test]
fn test_day25() {
    let input = "\
1=-0-2
12111
2=0=
21
2=01
111
20012
112
1=-1=
1-12
12
1=
122";
    let input = parse(input);

    assert_eq!(snafu2num("1=-0-2"), 1747);
    assert_eq!(snafu2num("12111"), 906);
    assert_eq!(snafu2num("2=0="), 198);
    assert_eq!(snafu2num("21"), 11);
    assert_eq!(snafu2num("2=01"), 201);
    assert_eq!(snafu2num("111"), 31);
    assert_eq!(snafu2num("20012"), 1257);
    assert_eq!(snafu2num("112"), 32);
    assert_eq!(snafu2num("1=-1="), 353);
    assert_eq!(snafu2num("1-12"), 107);
    assert_eq!(snafu2num("12"), 7);
    assert_eq!(snafu2num("1="), 3);
    assert_eq!(snafu2num("122"), 37);

    assert_eq!(num2snafu(1), "1");
    assert_eq!(num2snafu(2), "2");
    assert_eq!(num2snafu(3), "1=");
    assert_eq!(num2snafu(4), "1-");
    assert_eq!(num2snafu(5), "10");
    assert_eq!(num2snafu(6), "11");
    assert_eq!(num2snafu(7), "12");
    assert_eq!(num2snafu(8), "2=");
    assert_eq!(num2snafu(9), "2-");
    assert_eq!(num2snafu(10), "20");
    assert_eq!(num2snafu(15), "1=0");
    assert_eq!(num2snafu(20), "1-0");
    assert_eq!(num2snafu(2022), "1=11-2");
    assert_eq!(num2snafu(12345), "1-0---0");
    assert_eq!(num2snafu(314159265), "1121-1110-1=0");

    assert_eq!(solve(&input), "2=-1=0");
}
