use intcode::{PollExt, Runner};

fn main() {
    let program = intcode::load_vec(&std::fs::read_to_string("input.txt").unwrap()).unwrap();

    {
        let mut program = program.clone();
        println!("day 1:");
        let mut runner = Runner::new(&mut program);
        runner.input(1);
        for output in runner {
            println!("{}", output.unwrap());
        }
    }

    {
        let mut program = program.clone();
        println!("day 2:");
        let mut runner = Runner::new(&mut program);
        runner.input(5);
        for output in runner {
            println!("{}", output.unwrap());
        }
    }
}
