use intcode::intcode;

fn main() -> std::io::Result<()> {
    let mut program = intcode::load_stdin()?;

    program[1] = 12;
    program[2] = 2;
    let mut runner = intcode(&program);
    runner.run();
    println!("part 1: {}", runner.program()[0]);

    for noun in 0..100 {
        for verb in 0..100 {
            program[1] = noun;
            program[2] = verb;
            let mut runner = intcode(&program);
            runner.run();
            if runner.program()[0] == 19690720 {
                println!("part 2: {}", 100 * noun + verb);
                break;
            }
        }
    }
    Ok(())
}
