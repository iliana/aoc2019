use gcd::Gcd;
use itertools::iproduct;
use std::borrow::Cow;
use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};
use std::f64;

fn vector(start: (usize, usize), end: (usize, usize)) -> (isize, isize) {
    let dx = (end.0 as isize) - (start.0 as isize);
    let dx_abs = dx.abs() as usize;
    let dy = (end.1 as isize) - (start.1 as isize);
    let dy_abs = dy.abs() as usize;
    let gcd = dx_abs.gcd(dy_abs) as isize;
    (dx / gcd, dy / gcd)
}

#[cfg(test)]
#[test]
fn test_vector() {
    assert_eq!(vector((2, 4), (4, 6)), (1, 1));
    assert_eq!(vector((2, 4), (4, 7)), (2, 3));
    assert_eq!(vector((2, 4), (0, 2)), (-1, -1));
    assert_eq!(vector((2, 4), (0, 1)), (-2, -3));
    assert_eq!(
        VectorIter::new((0, 4), (4, 2)).collect::<Vec<_>>(),
        vec![(2, 3)]
    );
}

struct VectorIter {
    current: (usize, usize),
    end: Option<(usize, usize)>,
    dimensions: Option<(usize, usize)>,
    vector: (isize, isize),
}

impl VectorIter {
    fn new(start: (usize, usize), end: (usize, usize)) -> VectorIter {
        VectorIter {
            current: start,
            end: Some(end),
            dimensions: None,
            vector: vector(start, end),
        }
    }

    fn infinite(
        start: (usize, usize),
        vector: (isize, isize),
        width: usize,
        height: usize,
    ) -> VectorIter {
        VectorIter {
            current: start,
            end: None,
            dimensions: Some((width, height)),
            vector,
        }
    }
}

impl Iterator for VectorIter {
    type Item = (usize, usize);

    fn next(&mut self) -> Option<(usize, usize)> {
        let new = (
            self.current.0 as isize + self.vector.0,
            self.current.1 as isize + self.vector.1,
        );
        match (self.end, self.dimensions) {
            (Some(end), _) if new == (end.0 as isize, end.1 as isize) => None,
            (None, Some(dim))
                if new.0 < 0 || new.0 >= dim.0 as isize || new.1 < 0 || new.1 >= dim.1 as isize =>
            {
                None
            }
            _ => {
                self.current = (new.0 as usize, new.1 as usize);
                Some(self.current)
            }
        }
    }
}

/// (0, -1) = 0
/// (1, 0) = pi/2
/// (0, 1) = pi
/// (-1, 0) = 3pi/2
fn vector_angle(vector: (isize, isize)) -> f64 {
    let angle = (vector.0 as f64).atan2(vector.1 as f64 * -1f64);
    if angle < 0f64 {
        angle + (2f64 * f64::consts::PI)
    } else {
        angle
    }
}

#[cfg(test)]
#[test]
fn test_vector_angle() {
    assert_eq!(vector_angle((0, -1)), 0f64);
    assert_eq!(vector_angle((1, -1)), f64::consts::FRAC_PI_4);
    assert_eq!(vector_angle((1, 0)), f64::consts::FRAC_PI_2);
    assert_eq!(vector_angle((1, 1)), 3f64 * f64::consts::FRAC_PI_4);
    assert_eq!(vector_angle((0, 1)), f64::consts::PI);
    assert_eq!(vector_angle((-1, 1)), 5f64 * f64::consts::FRAC_PI_4);
    assert_eq!(vector_angle((-1, 0)), 3f64 * f64::consts::FRAC_PI_2);
    assert_eq!(vector_angle((-1, -1)), 7f64 * f64::consts::FRAC_PI_4);
}

fn parse_field(s: &str) -> Vec<Cow<[u8]>> {
    s.lines().map(|s| s.trim().as_bytes().into()).collect()
}

trait Field: Sized {
    fn width(&self) -> usize;
    fn height(&self) -> usize;
    fn pos_iter(&self) -> Box<dyn Iterator<Item = (usize, usize)>>;
    fn get(&self, pos: (usize, usize)) -> bool;
    fn is_empty(&self) -> bool;
    fn best_position(&self) -> ((usize, usize), usize);
    fn slopes(&self, start: (usize, usize)) -> Vec<(isize, isize)>;
}

trait FieldMut: Field {
    fn unset(&mut self, pos: (usize, usize));
    fn vaporize(self, start: (usize, usize)) -> Vaporize<Self>;
}

impl<T> Field for Vec<T>
where
    T: AsRef<[u8]>,
{
    fn width(&self) -> usize {
        self[0].as_ref().len()
    }

    fn height(&self) -> usize {
        self.len()
    }

    fn pos_iter(&self) -> Box<dyn Iterator<Item = (usize, usize)>> {
        Box::new(iproduct!(0..self.width(), 0..self.height()))
    }

    fn get(&self, pos: (usize, usize)) -> bool {
        match self[pos.1].as_ref()[pos.0] {
            b'#' => true,
            b'.' => false,
            _ => unreachable!(),
        }
    }

    fn is_empty(&self) -> bool {
        self.pos_iter().all(|pos| !self.get(pos))
    }

    fn best_position(&self) -> ((usize, usize), usize) {
        // useless hashing...
        let mut visible = HashMap::new();

        for start in self.pos_iter() {
            if self.get(start) {
                visible.insert(
                    start,
                    self.pos_iter()
                        .filter(|end| start != *end && self.get(*end))
                        .filter(|end| VectorIter::new(start, *end).all(|x| !self.get(x)))
                        .count(),
                );
            }
        }

        visible.into_iter().max_by_key(|&(_, count)| count).unwrap()
    }

    fn slopes(&self, start: (usize, usize)) -> Vec<(isize, isize)> {
        let slopes = self
            .pos_iter()
            .filter_map(|end| {
                if start == end {
                    None
                } else {
                    Some(vector(start, end))
                }
            })
            .collect::<HashSet<_>>();
        let mut slopes = slopes.into_iter().collect::<Vec<_>>();
        slopes.sort_by(|a, b| {
            vector_angle(*a)
                .partial_cmp(&vector_angle(*b))
                .unwrap_or(Ordering::Equal)
        });
        slopes
    }
}

impl FieldMut for Vec<Cow<'_, [u8]>> {
    fn unset(&mut self, pos: (usize, usize)) {
        let row = &mut self[pos.1];
        let row = row.to_mut();
        row[pos.0] = b'.';
    }

    fn vaporize(mut self, start: (usize, usize)) -> Vaporize<Self> {
        self.unset(start);
        Vaporize {
            start,
            slopes: self.slopes(start),
            slopes_idx: 0,
            field: self,
        }
    }
}

#[cfg(test)]
#[test]
fn test() {
    let field = parse_field(
        "......#.#.
         #..#.#....",
    );
    assert_eq!(field.width(), 10);
    assert_eq!(field.height(), 2);
    assert!(!field.is_empty());
    assert!(field.get((0, 1)));
    assert!(!field.get((1, 0)));

    let field = parse_field(
        ".#..#
         .....
         #####
         ....#
         ...##",
    );
    assert_eq!(field.best_position(), ((3, 4), 8));

    let field = parse_field(
        "......#.#.
         #..#.#....
         ..#######.
         .#.#.###..
         .#..#.....
         ..#....#.#
         #..#....#.
         .##.#..###
         ##...#..#.
         .#....####",
    );
    assert_eq!(field.best_position(), ((5, 8), 33));

    let field = parse_field(
        "#.#...#.#.
         .###....#.
         .#....#...
         ##.#.#.#.#
         ....#.#.#.
         .##..###.#
         ..#...##..
         ..##....##
         ......#...
         .####.###.",
    );
    assert_eq!(field.best_position(), ((1, 2), 35));

    let field = parse_field(
        ".#..##.###...#######
         ##.############..##.
         .#.######.########.#
         .###.#######.####.#.
         #####.##.#.##.###.##
         ..#####..#.#########
         ####################
         #.####....###.#.#.##
         ##.#################
         #####.##.###..####..
         ..######..##.#######
         ####.##.####...##..#
         .#####..#.######.###
         ##...#.##########...
         #.##########.#######
         .####.#.###.###.#.##
         ....##.##.###..#####
         .#.#.###########.###
         #.#.#.#####.####.###
         ###.##.####.##.#..##",
    );
    assert_eq!(field.best_position(), ((11, 13), 210));
    assert_eq!(field.clone().vaporize((11, 13)).nth(0), Some((11, 12)));
    assert_eq!(field.clone().vaporize((11, 13)).nth(1), Some((12, 1)));
    assert_eq!(field.clone().vaporize((11, 13)).nth(2), Some((12, 2)));
    assert_eq!(field.clone().vaporize((11, 13)).nth(199), Some((8, 2)));
}

struct Vaporize<T: FieldMut> {
    start: (usize, usize),
    field: T,
    slopes: Vec<(isize, isize)>,
    slopes_idx: usize,
}

impl<T: FieldMut> Iterator for Vaporize<T> {
    type Item = (usize, usize);

    fn next(&mut self) -> Option<(usize, usize)> {
        if self.field.is_empty() {
            None
        } else {
            loop {
                let slope = self.slopes[self.slopes_idx];
                self.slopes_idx = (self.slopes_idx + 1) % self.slopes.len();
                if let Some(pos) =
                    VectorIter::infinite(self.start, slope, self.field.width(), self.field.height())
                        .find(|pos| self.field.get(*pos))
                {
                    self.field.unset(pos);
                    break Some(pos);
                }
            }
        }
    }
}

fn main() {
    let input = util::read_input();
    let field = parse_field(&input);
    let (pos, count) = field.best_position();
    println!("part 1: {}", count);
    let (x, y) = field.vaporize(pos).nth(199).unwrap();
    println!("part 2: {}", x * 100 + y);
}
