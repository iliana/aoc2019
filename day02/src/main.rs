use intcode::intcode;
use std::io::BufReader;

fn main() -> std::io::Result<()> {
    let program = intcode::load(&mut BufReader::new(std::io::stdin()))?;
    for noun in 0..100 {
        for verb in 0..100 {
            let mut program = program.clone();
            program[1] = noun;
            program[2] = verb;
            intcode(&mut program);
            if program[0] == 19690720 {
                println!("{}", 100 * noun + verb);
                break;
            }
        }
    }
    Ok(())
}
