use std::cmp::{max, min};
use std::ops::Add;

///             ^ (1, 0)
///             |
/// (0, -1) <---+---> (0, 1)
///             |
///             v (-1, 0)
#[derive(Clone, Copy)]
enum Fragment {
    U(i64),
    D(i64),
    R(i64),
    L(i64),
}

impl Fragment {
    fn from_str(s: &str) -> Fragment {
        use Fragment::*;
        let (dir, num) = s.split_at(1);
        let num: i64 = num.parse().unwrap();
        match dir {
            "U" => U(num),
            "D" => D(num),
            "R" => R(num),
            "L" => L(num),
            _ => unreachable!(),
        }
    }
}

#[allow(clippy::suspicious_arithmetic_impl)]
impl Add<(i64, i64, i64)> for Fragment {
    type Output = (i64, i64, i64);

    fn add(self, rhs: (i64, i64, i64)) -> (i64, i64, i64) {
        use Fragment::*;
        match self {
            U(num) => (rhs.0 + num, rhs.1, rhs.2 + num),
            D(num) => (rhs.0 - num, rhs.1, rhs.2 + num),
            R(num) => (rhs.0, rhs.1 + num, rhs.2 + num),
            L(num) => (rhs.0, rhs.1 - num, rhs.2 + num),
        }
    }
}

fn between(x: i64, a: i64, b: i64) -> bool {
    min(a, b) <= x && x <= max(a, b)
}

fn intersection(
    a1: (i64, i64, i64),
    a2: (i64, i64, i64),
    b1: (i64, i64, i64),
    b2: (i64, i64, i64),
) -> Option<(i64, i64)> {
    if a1.0 == a2.0 && b1.1 == b2.1 && between(a1.0, b1.0, b2.0) && between(b1.1, a1.1, a2.1) {
        Some((b1.1, a1.0))
    } else if a1.1 == a2.1 && b1.0 == b2.0 && between(a1.1, b1.1, b2.1) && between(b1.0, a1.0, a2.0)
    {
        Some((b1.0, a1.1))
    } else {
        None
    }
}

// Each tuple is the (x, y) intersection coordinate followed by the number of steps to get there.
fn intersections(a: &str, b: &str) -> Vec<(i64, i64, i64)> {
    let mut v = Vec::new();
    let mut a_pos = (0, 0, 0);
    for a_frag in a.trim().split(',').map(Fragment::from_str) {
        let a_pos_next = a_frag + a_pos;
        let mut b_pos = (0, 0, 0);
        for b_frag in b.trim().split(',').map(Fragment::from_str) {
            let b_pos_next = b_frag + b_pos;
            if let Some(ix) = intersection(a_pos, a_pos_next, b_pos, b_pos_next) {
                if (ix.0, ix.1) != (0, 0) {
                    use Fragment::*;
                    let a_steps = a_pos.2
                        + match a_frag {
                            U(_) | D(_) => (a_pos.0 - ix.0).abs(),
                            L(_) | R(_) => (a_pos.1 - ix.1).abs(),
                        };
                    let b_steps = b_pos.2
                        + match b_frag {
                            U(_) | D(_) => (b_pos.0 - ix.0).abs(),
                            L(_) | R(_) => (b_pos.1 - ix.1).abs(),
                        };
                    v.push((ix.0, ix.1, a_steps + b_steps));
                }
            }
            b_pos = b_pos_next;
        }
        a_pos = a_pos_next;
    }
    v
}

#[test]
fn test_intersections() {
    assert_eq!(
        intersections("R8,U5,L5,D3", "U7,R6,D4,L4"),
        vec![(6, 5, 30), (3, 3, 40)]
    );
}

fn main() {
    let input = util::read_input();
    let mut lines = input.lines();
    let a = lines.next().unwrap();
    let b = lines.next().unwrap();
    let intersections = intersections(&a, &b);
    println!(
        "part 1: {}",
        intersections
            .iter()
            .map(|(x, y, _)| x.abs() + y.abs())
            .min()
            .unwrap()
    );
    println!(
        "part 2: {}",
        intersections.iter().map(|(_, _, s)| s).min().unwrap()
    );
}
