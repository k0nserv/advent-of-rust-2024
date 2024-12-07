use std::collections::HashSet;

use crate::math::Vector2;

pub fn star_one(input: &str) -> usize {
    let mut grid = Grid::from(input);

    grid.run_until_stuck_or_out_of_bounds()
        .expect("out of bounds")
        .len()
}

pub fn star_two(input: &str) -> usize {
    let grid = Grid::from(input);
    let visited = {
        let mut grid = grid.clone();

        grid.run_until_stuck_or_out_of_bounds()
            .expect("out of bounds")
    };

    // Brute force is good enough, try every location visited
    visited
        .into_iter()
        .filter(|x| {
            let mut grid = grid.clone();
            // Add obstruction
            if !grid.obstruct(*x) {
                return false;
            }

            grid.run_until_stuck_or_out_of_bounds().is_none()
        })
        .count()
}

#[derive(Clone)]
struct Grid {
    grid: Vec<Vec<Location>>,
    guard_location: Option<Vector2<isize>>,
}

impl Grid {
    fn run_until_stuck_or_out_of_bounds(&mut self) -> Option<HashSet<Vector2<isize>>> {
        let mut locations = HashSet::new();
        while let Some(guard_location) = self.guard_location() {
            if !locations.insert((guard_location, self.guard().direction)) {
                return None;
            }

            self.tick();
        }

        locations
            .into_iter()
            .map(|(l, _)| l)
            .collect::<HashSet<_>>()
            .into()
    }
    fn guard_location(&self) -> Option<Vector2<isize>> {
        self.guard_location
    }

    fn obstruct(&mut self, at: Vector2<isize>) -> bool {
        // Cannot obstruct guard
        if at == self.guard_location.expect("guard") {
            return false;
        }

        self.grid[at.y as usize][at.x as usize] = Location::Obstruction;
        true
    }

    fn tick(&mut self) {
        let Some(guard_location) = self.guard_location else {
            return;
        };
        let guard_direction = self.guard().direction;
        let next = guard_location + guard_direction;

        if self.out_of_bounds(next) {
            self.guard_location = None;
            return;
        }

        if self.grid[next.y as usize][next.x as usize] == Location::Obstruction {
            self.guard_mut().direction = guard_direction.rotate_right();
            return;
        }

        self.grid[guard_location.y as usize][guard_location.x as usize] = Location::Empty;
        self.guard_location = Some(next);
        self.grid[next.y as usize][next.x as usize] = Location::Guard(Guard {
            direction: guard_direction,
        });
    }

    fn out_of_bounds(&self, next: Vector2<isize>) -> bool {
        next.x < 0
            || next.y < 0
            || next.y as usize >= self.grid.len()
            || next.x as usize >= self.grid[0].len()
    }

    fn guard(&self) -> &Guard {
        let guard_location = self.guard_location.expect("guard");
        &self.grid[guard_location.y as usize][guard_location.x as usize]
            .as_guard()
            .unwrap_or_else(|| {
                panic!(
                    "Expected guard at {guard_location:?} found {:?}",
                    self.grid[guard_location.y as usize][guard_location.x as usize]
                )
            })
    }

    fn guard_mut(&mut self) -> &mut Guard {
        let guard_location = self.guard_location.expect("guard");
        self.grid[guard_location.y as usize][guard_location.x as usize]
            .as_guard_mut()
            .unwrap()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Location {
    Empty,
    Obstruction,
    Guard(Guard),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Guard {
    direction: Vector2<isize>,
}

impl Location {
    fn is_guard(&self) -> bool {
        matches!(self, Self::Guard { .. })
    }

    fn as_guard(&self) -> Option<&Guard> {
        match self {
            Self::Guard(g) => Some(g),
            _ => None,
        }
    }

    fn as_guard_mut(&mut self) -> Option<&mut Guard> {
        match self {
            Self::Guard(g) => Some(g),
            _ => None,
        }
    }
}

impl From<&str> for Grid {
    fn from(input: &str) -> Self {
        let grid: Vec<Vec<Location>> = input
            .lines()
            .map(str::trim)
            .filter(|l| !l.is_empty())
            .map(|l| l.chars().map(Into::into).collect())
            .collect();
        let guard_location = grid
            .iter()
            .enumerate()
            .find_map(|(y, row)| {
                row.iter()
                    .enumerate()
                    .find_map(|(x, l)| l.is_guard().then(|| Vector2::new(x as isize, y as isize)))
            })
            .expect("guard");

        Self {
            grid,
            guard_location: Some(guard_location),
        }
    }
}

impl From<char> for Location {
    fn from(value: char) -> Self {
        match value {
            '.' => Self::Empty,
            '#' => Self::Obstruction,
            '^' => Self::Guard(Guard {
                direction: Vector2::new(0, -1),
            }),
            _ => unreachable!("Invalid location {value}"),
        }
    }
}

trait Vector2Ext {
    const UP: Self;
    const DOWN: Self;
    const LEFT: Self;
    const RIGHT: Self;

    fn rotate_right(&self) -> Self;
}

impl Vector2Ext for Vector2<isize> {
    const UP: Self = Vector2::new(0, -1);
    const DOWN: Self = Vector2::new(0, 1);
    const LEFT: Self = Vector2::new(-1, 0);
    const RIGHT: Self = Vector2::new(1, 0);

    fn rotate_right(&self) -> Self {
        match *self {
            Self::UP => Self::RIGHT,
            Self::RIGHT => Self::DOWN,
            Self::DOWN => Self::LEFT,
            Self::LEFT => Self::UP,
            _ => unreachable!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{star_one, star_two};
    const INPUT: &'static str = r#"
....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...
"#;

    #[test]
    fn test_star_one() {
        assert_eq!(star_one(INPUT), 41);
    }

    #[test]
    fn test_star_two() {
        assert_eq!(star_two(INPUT), 6);
    }
}
