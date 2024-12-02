use std::collections::HashMap;

pub fn star_one(input: &str) -> u64 {
    let (mut f, mut s) = parse(input);

    f.sort();
    s.sort();

    f.into_iter()
        .zip(s.into_iter())
        .map(|(a, b)| a.abs_diff(b))
        .sum()
}

pub fn star_two(input: &str) -> i64 {
    let (f, s) = parse(input);

    let counts = {
        let mut counts = HashMap::new();

        for i in &s {
            let _ = counts.entry(*i).and_modify(|e| *e += 1).or_insert(1);
        }
        counts
    };

    f.into_iter()
        .map(|f| f * counts.get(&f).unwrap_or(&0))
        .sum()
}

fn parse(input: &str) -> (Vec<i64>, Vec<i64>) {
    input
        .lines()
        .map(|l| l.trim())
        .filter(|l| !l.is_empty())
        .map(|l| {
            let mut parts = l.split_whitespace();
            let f = parts
                .next()
                .expect("first item is present")
                .parse::<i64>()
                .expect("parsable number");

            let s = parts
                .next()
                .expect("second item is present")
                .parse::<i64>()
                .expect("parsable number");

            (f, s)
        })
        .unzip()
}

#[cfg(test)]
mod tests {
    use super::{star_one, star_two};
    const INPUT: &'static str = r#"
3   4
4   3
2   5
1   3
3   9
3   3
    "#;

    #[test]
    fn test_star_one() {
        assert_eq!(star_one(INPUT), 11);
    }

    #[test]
    fn test_star_two() {
        assert_eq!(star_two(INPUT), 31);
    }
}
