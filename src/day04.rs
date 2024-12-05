use std::collections::HashSet;

use crate::math::Vector2;

pub fn star_one(input: &str) -> usize {
    const NEEDLE: [char; 4] = ['X', 'M', 'A', 'S'];
    let grid: Vec<Vec<char>> = input
        .lines()
        .map(str::trim)
        .filter(|l| !l.is_empty())
        .map(|l| l.chars().collect())
        .collect();

    let mut covered: HashSet<((usize, usize), (isize, isize))> = HashSet::new();
    let mut run = |pos, dir| -> usize {
        if covered.contains(&(pos, dir)) {
            return 0;
        }

        covered.insert((pos, dir));
        let mut iter = GridIterator::new(&grid, (pos.0 as isize, pos.1 as isize), dir);
        return iter.count_needles(&NEEDLE);
    };

    let y_max = grid.len() - 1;
    let x_max = grid[0].len() - 1;
    let mut count = 0;
    for x in 0..grid[0].len() {
        //Down
        count += run((x, 0), (0, 1));
        // Up
        count += run((x, y_max), (0, -1));

        // Down left
        count += run((x, 0), (-1, 1));
        // Down right
        count += run((x, 0), (1, 1));

        // Up left
        count += run((x, y_max), (-1, -1));
        //Up right
        count += run((x, y_max), (1, -1));
    }

    for y in 0..grid.len() {
        // Right
        count += run((0, y), (1, 0));
        // Left
        count += run((x_max, y), (-1, 0));

        // Down right
        count += run((0, y), (1, 1));
        // Up right
        count += run((0, y), (1, -1));

        //Up left
        count += run((x_max, y), (-1, -1));

        // Down left
        count += run((x_max, y), (-1, 1));
    }

    count
}

pub fn star_two(input: &str) -> usize {
    let grid: Vec<Vec<char>> = input
        .lines()
        .map(str::trim)
        .filter(|l| !l.is_empty())
        .map(|l| l.chars().collect())
        .collect();

    // All the 'A's
    let needles = grid
        .iter()
        .enumerate()
        .flat_map(|(y, row)| row.iter().enumerate().map(move |(x, c)| ((x, y), *c)))
        .filter(|(_, c)| *c == 'A');

    const NEEDLE: [char; 3] = ['M', 'A', 'S'];
    // Have As, check surrounding characters
    // M.S
    // .A.
    // M.S
    needles
        .filter(|((x, y), _)| {
            let x = *x as isize;
            let y = *y as isize;

            // 1.2
            // .A.
            // 4.3
            let count = [
                ((x - 1, y - 1), (1, 1)),   // 1, down right
                ((x + 1, y - 1), (-1, 1)),  // 2, down left
                ((x + 1, y + 1), (-1, -1)), // 3, up left
                ((x - 1, y + 1), (1, -1)),  // 4, up right
            ]
            .iter()
            .filter(|(pos, dir)| GridIterator::new(&grid, *pos, *dir).find_needle(&NEEDLE))
            .count();

            if count > 2 {}

            count == 2
        })
        .count()
}

struct GridIterator<'a> {
    grid: &'a [Vec<char>],
    direction: Vector2<isize>,
    current: Vector2<isize>,
}

impl GridIterator<'_> {
    fn new(
        grid: &[Vec<char>],
        start: (isize, isize),
        direction: (isize, isize),
    ) -> GridIterator<'_> {
        GridIterator {
            grid,
            direction: Vector2::new(direction.0, direction.1),
            current: Vector2::new(start.0, start.1),
        }
    }

    fn count_needles(&mut self, needle: &[char]) -> usize {
        self.fold((0, 0), |(count, nidx), c| {
            if c != needle[nidx] {
                if c != needle[0] {
                    return (count, 0);
                } else {
                    return (count, 1);
                }
            }
            let nidx = if nidx == needle.len() - 1 {
                0
            } else {
                nidx + 1
            };
            let found: usize = (nidx == 0).into();

            (count + found, nidx)
        })
        .0
    }

    fn find_needle(&mut self, needle: &[char]) -> bool {
        // NB: cannot use `Iterator::all` because self might be shorter than needle which results
        // in false positive.
        let res = self.take(needle.len()).zip(needle.iter()).try_fold(
            (true, needle.len()),
            |(valid, remain), (c, n)| {
                if !valid {
                    return None;
                }

                Some((valid && c == *n, remain - 1))
            },
        );

        res.map(|(valid, i)| valid && i == 0).unwrap_or(false)
    }
}

impl Iterator for GridIterator<'_> {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current.y < 0 || self.current.y as usize >= self.grid.len() {
            return None;
        }

        if self.current.x < 0 || self.current.x as usize >= self.grid[self.current.y as usize].len()
        {
            return None;
        }

        let next = self.current + self.direction;
        let c = self.grid[self.current.y as usize][self.current.x as usize];
        self.current = next;

        Some(c)
    }
}

#[cfg(test)]
mod tests {
    use super::{star_one, star_two};
    const INPUT: &'static str = r#"
....XXMAS.
.SAMXMS...
...S..A...
..A.A.MS.X
XMASAMX.MM
X.....XA.A
S.S.S.S.SS
.A.A.A.A.A
..M.M.M.MM
.X.X.XMASX
"#;

    const INPUT2: &'static str = r#"
.M.S......
..A..MSMS.
.M.S.MAA..
..A.ASMSM.
.M.S.M....
..........
S.S.S.S.S.
.A.A.A.A..
M.M.M.M.M.
..........
"#;

    #[test]
    fn test_star_one() {
        assert_eq!(star_one(INPUT), 18);
    }

    #[test]
    fn test_star_two() {
        assert_eq!(star_two(INPUT2), 9);
    }
}
