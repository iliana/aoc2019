use intcode::{PollExt, Runner};
use std::collections::HashMap;

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn turn(&self, input: i64) -> Direction {
        use Direction::*;
        match (self, input) {
            (Up, 0) => Left,
            (Up, 1) => Right,
            (Down, 0) => Right,
            (Down, 1) => Left,
            (Left, 0) => Down,
            (Left, 1) => Up,
            (Right, 0) => Up,
            (Right, 1) => Down,
            _ => unreachable!(),
        }
    }

    fn forward(&self, pos: (isize, isize)) -> (isize, isize) {
        use Direction::*;
        match self {
            Up => (pos.0, pos.1 - 1),
            Down => (pos.0, pos.1 + 1),
            Left => (pos.0 - 1, pos.1),
            Right => (pos.0 + 1, pos.1),
        }
    }
}

fn run_robot(input: &[i64], start_color: i64) -> HashMap<(isize, isize), i64> {
    let mut pos = (0, 0);
    let mut dir = Direction::Up;
    let mut map = HashMap::new();
    map.insert((0, 0), start_color);
    let mut program = [0; 1280];
    program[..input.len()].copy_from_slice(input);
    let mut runner = Runner::new(&mut program);
    loop {
        // input current color
        let input = map.get(&pos).cloned().unwrap_or(0);
        runner.input(input);
        // output new color
        let color = match runner.next() {
            Some(x) => x.unwrap(),
            None => break,
        };
        map.insert(pos, color);
        // output direction
        let new_dir = runner.next().unwrap().unwrap();
        dir = dir.turn(new_dir);
        pos = dir.forward(pos);
    }
    map
}

fn main() {
    let input = util::read_intcode();

    let map = run_robot(&input, 0);
    println!("part 1: {}", map.len());

    println!("part 2:");
    let map = run_robot(&input, 1);
    let x_min = *map.keys().map(|(x, _)| x).min().unwrap();
    let x_max = *map.keys().map(|(x, _)| x).max().unwrap();
    let y_min = *map.keys().map(|(_, y)| y).min().unwrap();
    let y_max = *map.keys().map(|(_, y)| y).max().unwrap();
    for y in y_min..=y_max {
        for x in x_min..=x_max {
            print!("{}", match map.get(&(x, y)).cloned().unwrap_or(0) {
                0 => " ",
                1 => "#",
                _ => unreachable!(),
            });
        }
        println!();
    }
}
