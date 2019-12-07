use intcode::{PollExt, Runner};

fn main() {
    let program = util::read_intcode();

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
