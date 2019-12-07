use intcode::{PollExt, Runner};
use itertools::Itertools;

fn find_max_part1(program: &[i64]) -> i64 {
    let mut results = Vec::new();
    for phase_settings in (0..5).permutations(5) {
        let mut input = 0;
        for phase in &phase_settings {
            let mut program = program.to_vec();
            let mut runner = Runner::new(&mut program);
            runner.input(*phase);
            runner.next();
            runner.input(input);
            input = runner.next().unwrap().unwrap();
        }
        results.push(input);
    }
    results.into_iter().max().unwrap()
}

#[test]
fn test_find_max_part1() {
    assert_eq!(
        find_max_part1(&[3, 15, 3, 16, 1002, 16, 10, 16, 1, 16, 15, 15, 4, 15, 99, 0, 0]),
        43210
    );
    assert_eq!(
        find_max_part1(&[
            3, 23, 3, 24, 1002, 24, 10, 24, 1002, 23, -1, 23, 101, 5, 23, 23, 1, 24, 23, 23, 4, 23,
            99, 0, 0
        ]),
        54321
    );
    assert_eq!(
        find_max_part1(&[
            3, 31, 3, 32, 1002, 32, 10, 32, 1001, 31, -2, 31, 1007, 31, 0, 33, 1002, 33, 7, 33, 1,
            33, 31, 31, 1, 32, 31, 31, 4, 31, 99, 0, 0, 0
        ]),
        65210
    );
}

fn find_max_part2(program: &[i64]) -> i64 {
    let mut results = Vec::new();
    for phase_settings in (5..10).permutations(5) {
        let mut program_0 = program.to_vec();
        let mut program_1 = program.to_vec();
        let mut program_2 = program.to_vec();
        let mut program_3 = program.to_vec();
        let mut program_4 = program.to_vec();
        let mut runners = [
            Runner::new(&mut program_0),
            Runner::new(&mut program_1),
            Runner::new(&mut program_2),
            Runner::new(&mut program_3),
            Runner::new(&mut program_4),
        ];
        for i in 0..5 {
            runners[i].input(phase_settings[i]);
            runners[i].next();
        }
        let mut input = 0;
        'outer: loop {
            for runner in &mut runners {
                runner.input(input);
                if let Some(new_input) = runner.next() {
                    input = new_input.unwrap();
                } else {
                    break 'outer;
                }
            }
        }
        results.push(input);
    }
    results.into_iter().max().unwrap()
}

#[test]
fn test_find_max_part2() {
    assert_eq!(
        find_max_part2(&[
            3, 26, 1001, 26, -4, 26, 3, 27, 1002, 27, 2, 27, 1, 27, 26, 27, 4, 27, 1001, 28, -1,
            28, 1005, 28, 6, 99, 0, 0, 5
        ]),
        139629729
    );
    assert_eq!(
        find_max_part2(&[
            3, 52, 1001, 52, -5, 52, 3, 53, 1, 52, 56, 54, 1007, 54, 5, 55, 1005, 55, 26, 1001, 54,
            -5, 54, 1105, 1, 12, 1, 53, 54, 53, 1008, 54, 0, 55, 1001, 55, 1, 55, 2, 53, 55, 53, 4,
            53, 1001, 56, -1, 56, 1005, 56, 6, 99, 0, 0, 0, 0, 10
        ]),
        18216
    );
}

fn main() {
    let program = intcode::load_vec(&std::fs::read_to_string("input.txt").unwrap()).unwrap();
    println!("part 1: {}", find_max_part1(&program));
    println!("part 2: {}", find_max_part2(&program));
}
