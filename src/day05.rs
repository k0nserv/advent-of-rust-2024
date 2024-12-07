use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};

type Rules = HashMap<usize, HashSet<usize>>;

pub fn star_one(input: &str) -> usize {
    let (rules, updates) = parse(input);

    // For all updates
    updates
        .filter(|update| validate(&rules, update))
        .map(|update| update[update.len() / 2])
        .sum()
}

pub fn star_two(input: &str) -> usize {
    let (rules, updates) = parse(input);

    let bad = updates.filter(|update| !validate(&rules, update));

    bad.map(|mut update| {
        update.sort_by(|a, b| {
            let a_before_b = rules.get(a).map(|o| o.contains(b)).unwrap_or(false);
            if a_before_b {
                return Ordering::Greater;
            }

            let b_before_a = rules.get(b).map(|o| o.contains(a)).unwrap_or(false);

            if b_before_a {
                return Ordering::Less;
            }

            unreachable!("Should have total order")
        });

        update[update.len() / 2]
    })
    .sum()
}

/// Validate the update according to the rules.
///
/// Returns the first index that violates the rule if any
fn validate(rules: &Rules, update: &[usize]) -> bool {
    // Only those that are valid
    update.iter().enumerate().all(|(i, page)| {
        let backward = (0..i).all(|b| {
            rules
                .get(&update[b])
                .map(|r| r.contains(page))
                .unwrap_or(true)
                && rules
                    .get(page)
                    .map(|r| !r.contains(&update[b]))
                    .unwrap_or(true)
        });

        if !backward {
            return false;
        }

        let forward = ((i + 1)..update.len()).all(|a| {
            rules
                .get(page)
                .map(|r| r.contains(&update[a]))
                .unwrap_or(true)
                && rules
                    .get(&update[a])
                    .map(|r| !r.contains(page))
                    .unwrap_or(true)
        });

        forward
    })
}

fn parse(input: &str) -> (Rules, impl Iterator<Item = Vec<usize>> + '_) {
    let (rules, updates) = input
        .split_once("\n\n")
        .expect("Two sections separated by \\n\\n");
    let rules = {
        let mut result: Rules = Default::default();
        let iter = rules
            .lines()
            .map(str::trim)
            .filter(|l| !l.is_empty())
            .map(|l| {
                let (key, value) = l.split_once('|').expect("rule separated by |");
                let key = key.parse().expect("parsable key");
                let value = value.parse().expect("parsable key");

                (key, value)
            });

        for (key, value) in iter {
            result.entry(key).or_default().insert(value);
        }

        result
    };

    let updates = updates
        .lines()
        .map(str::trim)
        .filter(|l| !l.is_empty())
        .map(|l| {
            l.split(',')
                .map(|s| s.parse::<usize>().expect("parsable page number"))
                .collect::<Vec<_>>()
        });

    (rules, updates)
}

#[cfg(test)]
mod tests {
    use super::{star_one, star_two};
    const INPUT: &'static str = r#"
47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47"#;

    #[test]
    fn test_star_one() {
        assert_eq!(star_one(INPUT), 143);
    }

    #[test]
    fn test_star_two() {
        assert_eq!(star_two(INPUT), 123);
    }
}
