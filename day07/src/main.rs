use intcode::intcode;
use itertools::Itertools;

fn find_max_part1(program: &[i64]) -> i64 {
    let mut results = Vec::new();
    for phase_settings in (0..5).permutations(5) {
        let mut input = 0;
        for phase in &phase_settings {
            let mut runner = intcode(program);
            runner.input(*phase);
            runner.input(input);
            input = runner.run()[0];
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

fn main() -> std::io::Result<()> {
    let program = intcode::load_stdin()?;
    println!("part 1: {}", find_max_part1(&program));
    Ok(())
}
