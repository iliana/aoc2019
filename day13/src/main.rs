use intcode::{PollExt, Runner};
use std::cmp::Ordering;
use std::collections::HashMap;
use std::task::Poll;

#[derive(Default)]
struct State {
    board: HashMap<(i64, i64), i64>,
    score: i64,
}

impl State {
    fn is_empty(&self) -> bool {
        !self.board.values().any(|id| *id == 2)
    }

    fn paddle(&self) -> (i64, i64) {
        *self.board.iter().find(|((_, _), id)| **id == 3).unwrap().0
    }

    fn ball(&self) -> (i64, i64) {
        *self.board.iter().find(|((_, _), id)| **id == 4).unwrap().0
    }
}

fn update_state(runner: &mut Runner, state: &mut State) {
    while let Some(Poll::Ready(x)) = runner.next() {
        let y = runner.next().unwrap().unwrap();
        let id = runner.next().unwrap().unwrap();
        if (x, y) == (-1, 0) {
            state.score = id;
        } else {
            state.board.insert((x, y), id);
        }
    }
}

#[allow(unused)]
fn print_state(state: &State) {
    // print!("\x1b[H\x1b[J\x1b[3J"); // y'ever just run `clear | hexdump -C`?
    println!("{}", state.score);
    let x_min = *state.board.keys().map(|(x, _)| x).min().unwrap();
    let x_max = *state.board.keys().map(|(x, _)| x).max().unwrap();
    let y_min = *state.board.keys().map(|(_, y)| y).min().unwrap();
    let y_max = *state.board.keys().map(|(_, y)| y).max().unwrap();
    for y in y_min..=y_max {
        for x in x_min..=x_max {
            print!(
                "{}",
                match state.board.get(&(x, y)).cloned().unwrap_or(0) {
                    0 => ' ',
                    1 => '#',
                    2 => 'B',
                    3 => '-',
                    4 => '*',
                    _ => unreachable!(),
                }
            );
        }
        println!();
    }
    println!();
}

fn main() {
    let input = util::read_intcode();

    let mut program = [0; 4096];
    program[..input.len()].copy_from_slice(&input);
    let mut runner = Runner::new(&mut program);
    let mut state = State::default();
    update_state(&mut runner, &mut state);
    println!(
        "part 1: {}",
        state.board.values().filter(|v| **v == 2).count()
    );

    let mut program = [0; 4096];
    program[..input.len()].copy_from_slice(&input);
    program[0] = 2; // coins
    let mut runner = Runner::new(&mut program);
    let mut state = State::default();
    update_state(&mut runner, &mut state);
    while !state.is_empty() {
        runner.input(match state.ball().0.cmp(&state.paddle().0) {
            Ordering::Less => -1,
            Ordering::Greater => 1,
            Ordering::Equal => 0,
        });
        update_state(&mut runner, &mut state);
    }
    println!("part 2: {}", state.score);
}
