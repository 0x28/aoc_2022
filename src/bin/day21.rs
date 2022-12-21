use ahash::AHashMap;

#[derive(Debug, Clone, PartialEq)]
enum Expression {
    Add(Box<Expression>, Box<Expression>),
    Sub(Box<Expression>, Box<Expression>),
    Mul(Box<Expression>, Box<Expression>),
    Div(Box<Expression>, Box<Expression>),
    Num(i64),
    Human,
}

impl Expression {
    fn op(&self) -> fn(i64, i64) -> i64 {
        match self {
            Self::Add(_, _) => |a, b| a + b,
            Self::Sub(_, _) => |a, b| a - b,
            Self::Mul(_, _) => |a, b| a * b,
            Self::Div(_, _) => |a, b| a / b,
            _ => unreachable!(),
        }
    }
}

fn parse(input: &str) -> Box<Expression> {
    let mut expressions = AHashMap::default();

    for line in input.lines() {
        let mut split = line.split(':');
        let left = split.next().unwrap();
        let right = split.next().unwrap();

        let mut expr = right.split_ascii_whitespace();
        let lhs = expr.next();
        let op = expr.next();
        let rhs = expr.next();

        if left == "humn" {
            expressions.insert(left.to_string(), vec!["humn"]);
        } else if let (Some(lhs), Some(op), Some(rhs)) = (lhs, op, rhs) {
            expressions.insert(left.to_string(), vec![op, lhs, rhs]);
        } else {
            expressions.insert(left.to_string(), vec!["num", lhs.unwrap()]);
        }
    }

    fn create_tree(
        expr: &[&str],
        expressions: &AHashMap<String, Vec<&str>>,
    ) -> Box<Expression> {
        match expr {
            ["humn"] => Box::new(Expression::Human),
            ["+", l, r] => Box::new(Expression::Add(
                create_tree(expressions.get(*l).unwrap(), expressions),
                create_tree(expressions.get(*r).unwrap(), expressions),
            )),
            ["-", l, r] => Box::new(Expression::Sub(
                create_tree(expressions.get(*l).unwrap(), expressions),
                create_tree(expressions.get(*r).unwrap(), expressions),
            )),
            ["*", l, r] => Box::new(Expression::Mul(
                create_tree(expressions.get(*l).unwrap(), expressions),
                create_tree(expressions.get(*r).unwrap(), expressions),
            )),
            ["/", l, r] => Box::new(Expression::Div(
                create_tree(expressions.get(*l).unwrap(), expressions),
                create_tree(expressions.get(*r).unwrap(), expressions),
            )),
            ["num", n] => Box::new(Expression::Num(n.parse().unwrap())),
            _ => unreachable!(),
        }
    }

    create_tree(expressions.get("root").unwrap(), &expressions)
}

fn simplify(expr: &Expression) -> Expression {
    match expr {
        Expression::Add(l, r)
        | Expression::Sub(l, r)
        | Expression::Mul(l, r)
        | Expression::Div(l, r) => {
            let left = simplify(l);
            let right = simplify(r);

            if let (Expression::Num(left_num), Expression::Num(right_num)) =
                (&left, &right)
            {
                Expression::Num(expr.op()(*left_num, *right_num))
            } else {
                match expr {
                    Expression::Add(_, _) => {
                        Expression::Add(Box::new(left), Box::new(right))
                    }
                    Expression::Sub(_, _) => {
                        Expression::Sub(Box::new(left), Box::new(right))
                    }
                    Expression::Mul(_, _) => {
                        Expression::Mul(Box::new(left), Box::new(right))
                    }
                    Expression::Div(_, _) => {
                        Expression::Div(Box::new(left), Box::new(right))
                    }
                    _ => unreachable!(),
                }
            }
        }
        e => e.clone(),
    }
}

fn solve(equation: &Expression) -> i64 {
    let (mut left, right) = if let Expression::Add(left, right) = equation {
        (left, right)
    } else {
        unreachable!()
    };

    let mut right = if let Expression::Num(n) = right.as_ref() {
        *n
    } else {
        unreachable!()
    };

    loop {
        match left.as_ref() {
            Expression::Add(lhs, rhs) => match (lhs.as_ref(), rhs.as_ref()) {
                (_, Expression::Num(nr)) => {
                    right -= nr;
                    left = lhs;
                }
                (Expression::Num(nl), _) => {
                    right -= nl;
                    left = rhs;
                }
                _ => unreachable!(),
            },
            Expression::Sub(lhs, rhs) => match (lhs.as_ref(), rhs.as_ref()) {
                (_, Expression::Num(nr)) => {
                    right += nr;
                    left = lhs;
                }
                (Expression::Num(nl), _) => {
                    right = -(right - nl);
                    left = rhs;
                }
                _ => unreachable!(),
            },
            Expression::Mul(lhs, rhs) => match (lhs.as_ref(), rhs.as_ref()) {
                (_, Expression::Num(nr)) => {
                    right /= nr;
                    left = lhs;
                }
                (Expression::Num(nl), _) => {
                    right /= nl;
                    left = rhs;
                }
                _ => unreachable!(),
            },
            Expression::Div(lhs, rhs) => match (lhs.as_ref(), rhs.as_ref()) {
                (_, Expression::Num(nr)) => {
                    right *= nr;
                    left = lhs;
                }
                (Expression::Num(nl), _) => {
                    right *= nl;
                    left = rhs;
                }
                _ => unreachable!(),
            },
            _ => {
                return right;
            }
        }
    }
}

fn part2(expr: &mut Expression) -> i64 {
    solve(&simplify(expr))
}

fn main() {
    let input = include_str!("../../input/input21.txt");
    let mut input = parse(input);
    println!();
    // NOTE: for part1 see day21.hs
    println!("part2 = {}", part2(&mut input));
}

#[test]
fn test_day21() {
    let input = "\
root: pppw + sjmn
dbpl: 5
cczh: sllz + lgvd
zczc: 2
ptdq: humn - dvpt
dvpt: 3
lfqf: 4
humn: 5
ljgn: 2
sjmn: drzm * dbpl
sllz: 4
pppw: cczh / lfqf
lgvd: ljgn * ptdq
drzm: hmdt - zczc
hmdt: 32";
    let mut input = parse(input);

    assert_eq!(part2(&mut input), 301);
}
