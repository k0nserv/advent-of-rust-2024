use itertools::Itertools;
use std::collections::{HashMap, HashSet};

use crate::math::Vector2;

pub fn star_one(input: &str) -> usize {
    let grid = Grid::from(input);
    grid.unique_antinodes(Some(1)).len()
}

pub fn star_two(input: &str) -> usize {
    let grid = Grid::from(input);
    grid.unique_antinodes(None).len()
}

type Frequency = char;

#[derive(Debug)]
struct Grid {
    max: Vector2<isize>,
    antennas: HashMap<Frequency, Vec<Antenna>>,
}
impl Grid {
    fn unique_antinodes(&self, limit: Option<usize>) -> HashSet<Vector2<isize>> {
        let unique: HashSet<Vector2<isize>> = self
            .antennas
            .iter()
            .flat_map(|(_, antennas)| {
                antennas.iter().combinations(2).flat_map(|pair| {
                    let mut a1 = anti_node(pair[0].location, pair[1].location);
                    let mut a2 = anti_node(pair[1].location, pair[0].location);

                    let mut t1;
                    let mut t2;
                    let (l1, l2): (
                        &mut dyn Iterator<Item = Vector2<isize>>,
                        &mut dyn Iterator<Item = Vector2<isize>>,
                    ) = if let Some(limit) = limit {
                        t1 = a1.skip(1).take(limit);
                        t2 = a2.skip(1).take(limit);

                        (&mut t1, &mut t2)
                    } else {
                        (&mut a1, &mut a2)
                    };

                    // This collect is maybe avoidable, but I cannot be bothered
                    l1.take_while(|x| self.in_bounds(*x))
                        .chain(l2.take_while(|x| self.in_bounds(*x)))
                        .collect::<Vec<_>>()
                        .into_iter()
                })
            })
            .collect();

        unique
    }

    fn in_bounds(&self, l: Vector2<isize>) -> bool {
        l.x >= 0 && l.x <= self.max.x && l.y >= 0 && l.y <= self.max.y
    }
}

fn anti_node(a1: Vector2<isize>, a2: Vector2<isize>) -> impl Iterator<Item = Vector2<isize>> {
    let d = a1 - a2;
    let first_anti_node = a2 + d * 2;
    let x = first_anti_node - a1;

    (0..).map(move |i| a1 + x * i)
}

#[derive(Debug)]
struct Antenna {
    location: Vector2<isize>,
    frequency: Frequency,
}

impl From<&str> for Grid {
    fn from(input: &str) -> Self {
        let locations = input
            .lines()
            .map(str::trim)
            .filter(|l| !l.is_empty())
            .enumerate()
            .flat_map(|(y, row)| {
                row.chars()
                    .enumerate()
                    .map(move |(x, c)| (Vector2::new(x as isize, y as isize), c))
            });

        locations.fold(
            Grid {
                max: Vector2::new(0, 0),
                antennas: HashMap::new(),
            },
            |mut grid, (location, frequency)| {
                grid.max.x = grid.max.x.max(location.x);
                grid.max.y = grid.max.y.max(location.y);

                if frequency == '.' {
                    return grid;
                }

                grid.antennas
                    .entry(frequency)
                    .or_insert_with(Vec::new)
                    .push(Antenna {
                        location,
                        frequency,
                    });

                grid
            },
        )
    }
}

#[cfg(test)]
mod tests {
    use super::{star_one, star_two};
    const INPUT: &'static str = r#"
............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............
"#;

    #[test]
    fn test_star_one() {
        assert_eq!(star_one(INPUT), 14);
    }

    #[test]
    fn test_star_two() {
        assert_eq!(star_two(INPUT), 34);
    }
}
