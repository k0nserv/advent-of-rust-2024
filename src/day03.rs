use regex::Regex;

pub fn star_one(input: &str) -> i64 {
    let re = Regex::new(r"mul\((\d+)\s*,\s*(\d+)\)").unwrap();

    re.captures_iter(input)
        .map(|c| {
            let lhs = c.get(1).expect("left capture");
            let rhs = c.get(2).expect("right capture");

            let lhs = lhs.as_str().parse::<i64>().expect("lhs parse");
            let rhs = rhs.as_str().parse::<i64>().expect("rhs parse");

            lhs * rhs
        })
        .sum()
}

pub fn star_two(input: &str) -> i64 {
    let re = Regex::new(r"(?:mul\((\d+)\s*,\s*(\d+)\)|do\(\)|don't\(\))").unwrap();

    re.captures_iter(input)
        .fold((true, 0), |(active, sum), c| {
            let op = c.get(0).expect("operation");
            if op.as_str().starts_with("don't") {
                return (false, sum);
            }

            if op.as_str().starts_with("do") {
                return (true, sum);
            }

            if !active {
                return (active, sum);
            }

            let lhs = c.get(1).expect("left capture");
            let rhs = c.get(2).expect("right capture");

            let lhs = lhs.as_str().parse::<i64>().expect("lhs parse");
            let rhs = rhs.as_str().parse::<i64>().expect("rhs parse");

            (active, sum + lhs * rhs)
        })
        .1
}

#[cfg(test)]
mod tests {
    use super::{star_one, star_two};
    const INPUT: &'static str = r#"
xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))
    "#;

    #[test]
    fn test_star_one() {
        assert_eq!(star_one(INPUT), 161);
    }

    #[test]
    fn test_star_two() {
        assert_eq!(
            star_two("xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))"),
            48
        );
    }
}
