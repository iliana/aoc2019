use intcode::intcode;
use std::io::BufReader;

fn main() -> std::io::Result<()> {
    let mut program = intcode::load(&mut BufReader::new(std::io::stdin()))?;
    for output in intcode(&mut program, vec![1]) {
        println!("{}", output);
    }
    Ok(())
}
