pub fn star_one(input: &str) -> usize {
    let iter = parse(input);

    iter.filter(|l| valid(l, None)).count()
}

pub fn star_two(input: &str) -> usize {
    let iter = parse(input);

    let mut count = 0;
    for report in iter {
        if valid(&report, None) {
            count += 1;
            continue;
        };
        // Brute force
        let any_valid = (0..report.len()).any(|i| valid(&report, Some(i)));

        if any_valid {
            count += 1;
        }
    }

    count
}

fn parse(input: &str) -> impl Iterator<Item = Vec<i64>> + '_ {
    input
        .lines()
        .map(|l| l.trim())
        .filter(|l| !l.is_empty())
        .map(|l| {
            l.split_whitespace()
                .map(|d| d.parse::<i64>().expect("Parasable digit"))
                .collect::<Vec<_>>()
        })
}

fn valid(values: &[i64], ignored_idx: Option<usize>) -> bool {
    values
        .iter()
        .enumerate()
        .filter(|(idx, _)| Some(idx) != ignored_idx.as_ref())
        .zip(
            values
                .into_iter()
                .enumerate()
                .filter(|(idx, _)| Some(idx) != ignored_idx.as_ref())
                .skip(1),
        )
        .fold((true, None), |(valid, mut decreasing), ((_, a), (_, b))| {
            let d = a - b;
            if !valid {
                return (valid, decreasing);
            }
            let abs_diff = d.abs();

            if abs_diff < 1 || abs_diff > 3 {
                return (false, decreasing);
            }

            let is_decreasing = *decreasing.get_or_insert(d.is_positive());

            let is_valid = valid
                && ((is_decreasing && d.is_positive()) || (!is_decreasing && d.is_negative()));

            (is_valid, decreasing)
        })
        .0
}

#[cfg(test)]
mod tests {
    use super::{star_one, star_two};
    const INPUT: &'static str = r#"
7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9
81 1 2 3 4
    "#;

    #[test]
    fn test_star_one() {
        assert_eq!(star_one(INPUT), 2);
    }

    #[test]
    fn test_star_two() {
        assert_eq!(star_two(INPUT), 5)
    }

    #[test]
    fn test_star_two_fails() {
        assert_eq!(
            star_two(
                r#"
1 2 7 8 9
9 7 6 2 1
        "#
            ),
            0
        )
    }

    #[test]
    fn test_star_two_false_negatives() {
        assert_eq!(
            star_two(
                r#"
1 2 3 4 5 10
48 46 47 49 51 54 56
1 1 2 3 4 5
1 2 3 4 5 5
5 1 2 3 4 5
1 4 3 2 1
1 6 7 8 9
1 2 3 4 3
9 8 7 6 7
7 10 8 10 11
29 28 27 25 26 25 22 20
        "#
            ),
            11
        )
    }
}
