use itertools::{repeat_n, Itertools};

pub fn star_one(input: &str) -> usize {
    let cases = input
        .lines()
        .map(str::trim)
        .filter(|l| !l.is_empty())
        .map(Case::from);

    // Worst case complexity is O(m * 2^(n-1)) where m is the number of cases and n is the number of values
    // Longest input is 12 values for a total 2^11 = 2048 possible combinations
    // For m of 850 cases, this is 1.7 million, brute force is fine

    cases
        .filter(|c| {
            repeat_n([Op::Add, Op::Mul].into_iter(), c.values.len() - 1)
                .multi_cartesian_product()
                .any(|ops| c.is_correct(ops.into_iter()))
        })
        .map(|c| c.expected)
        .sum()
}

pub fn star_two(input: &str) -> usize {
    let cases = input
        .lines()
        .map(str::trim)
        .filter(|l| !l.is_empty())
        .map(Case::from);

    // Worst case complexity is O(m * 3^(n-1)) where m is the number of cases and n is the number of values
    // Longest input is 12 values for a total 3^11 = 177147 possible combinations
    // For m of 850 cases, this is 150 million, brute force is fine

    cases
        .filter(|c| {
            repeat_n(
                [Op::Add, Op::Mul, Op::Concat].into_iter(),
                c.values.len() - 1,
            )
            .multi_cartesian_product()
            .any(|ops| c.is_correct(ops.into_iter()))
        })
        .map(|c| dbg!(c.expected))
        .sum()
}

#[derive(Debug)]
struct Case {
    expected: usize,
    values: Vec<usize>,
}

impl Case {
    fn is_correct(&self, ops: impl Iterator<Item = Op>) -> bool {
        self.eval(ops) == self.expected
    }

    fn eval(&self, ops: impl Iterator<Item = Op>) -> usize {
        self.values
            .iter()
            .skip(1)
            .zip(ops)
            .fold(self.values[0], |acc, (v, op)| match op {
                Op::Add => acc + v,
                Op::Mul => acc * v,
                Op::Concat => acc * 10_usize.pow(v.ilog10() + 1) + v,
            })
    }
}

#[derive(Debug, Copy, Clone)]
enum Op {
    Add,
    Mul,
    Concat,
}

impl From<&str> for Case {
    fn from(value: &str) -> Self {
        let mut parts = value.split(": ");
        let expected = parts
            .next()
            .expect("two parts separated by :")
            .parse()
            .expect("parse expected");
        let values = parts
            .next()
            .unwrap()
            .split_whitespace()
            .map(|v| v.parse().expect("parse value"))
            .collect();

        Self { expected, values }
    }
}

#[cfg(test)]
mod tests {
    use crate::day07::Op;

    use super::{star_one, star_two, Case};
    const INPUT: &'static str = r#"
190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20
"#;

    #[test]
    fn test_star_one() {
        assert_eq!(star_one(INPUT), 3749);
    }

    #[test]
    fn test_star_two() {
        assert_eq!(star_two(INPUT), 11387);
    }

    #[test]
    fn test_is_correct() {
        let case = Case {
            expected: 190,
            values: vec![10, 19],
        };

        assert!(case.is_correct([Op::Mul].iter().copied()));

        let case = Case {
            expected: 3267,
            values: vec![81, 40, 27],
        };

        assert!(case.is_correct([Op::Add, Op::Mul].iter().copied()));

        let case = Case {
            expected: 156,
            values: vec![15, 6],
        };

        assert!(case.is_correct([Op::Concat].iter().copied()));

        let case = Case {
            expected: 12345,
            values: vec![12, 345],
        };

        assert!(case.is_correct([Op::Concat].iter().copied()));

        let case = Case {
            expected: 192,
            values: vec![17, 8, 14],
        };

        assert!(case.is_correct([Op::Concat, Op::Add].iter().copied()));

        let case = Case {
            expected: 7290,
            values: vec![6, 8, 6, 15],
        };

        assert!(case.is_correct([Op::Mul, Op::Concat, Op::Mul].iter().copied()));
    }
}
