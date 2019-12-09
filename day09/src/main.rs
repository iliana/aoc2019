use intcode::{PollExt, Runner};

fn main() {
    let program = util::read_intcode();
    let mut memory = [0; 1100];

    memory[..973].copy_from_slice(&program);
    let mut runner = Runner::new(&mut memory);
    runner.input(1);
    for value in runner {
        println!("part 1: {}", value.unwrap());
    }

    memory[..973].copy_from_slice(&program);
    let mut runner = Runner::new(&mut memory);
    runner.input(2);
    for value in runner {
        println!("part 2: {}", value.unwrap());
    }
}
