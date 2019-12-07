use intcode::intcode;

fn main() -> std::io::Result<()> {
    let program = intcode::load_stdin()?;

    println!("day 1:");
    let mut runner = intcode(&program);
    runner.input(1);
    for output in runner {
        println!("{}", output);
    }

    println!("day 2:");
    let mut runner = intcode(&program);
    runner.input(5);
    for output in runner {
        println!("{}", output);
    }

    Ok(())
}
