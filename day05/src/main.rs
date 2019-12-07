use intcode::intcode;
use std::io::BufReader;

fn main() -> std::io::Result<()> {
    let program = intcode::load(&mut BufReader::new(std::io::stdin()))?;
    {
        println!("day 1:");
        let mut program = program.clone();
        for output in intcode(&mut program, vec![1]) {
            println!("{}", output);
        }
    }
    {
        println!("day 2:");
        let mut program = program.clone();
        for output in intcode(&mut program, vec![5]) {
            println!("{}", output);
        }
    }
    Ok(())
}
